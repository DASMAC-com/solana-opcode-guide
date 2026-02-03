use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DeriveInput, Fields, Ident, Lit, LitInt, Meta, Token,
};

/// Maximum line length for ASM output.
const MAX_LINE_LEN: usize = 75;

/// Maximum comment length (accounting for `# ` prefix).
const MAX_COMMENT_LEN: usize = MAX_LINE_LEN - "# ".len();

/// Attribute macro for defining error code enums shared between Rust and ASM.
///
/// Automatically adds `#[repr(u64)]` to the enum. Each variant must have a doc
/// comment that will become the ASM comment. Variant names are converted from
/// PascalCase to SCREAMING_SNAKE_CASE and prefixed with `E_`.
///
/// # Example
///
/// ```ignore
/// #[error_codes]
/// pub enum ErrorCodes {
///     /// An invalid number of accounts were passed.
///     NAccounts,
///     /// The user account has nonzero data length.
///     UserData,
/// }
/// ```
///
/// Generates ASM:
///
/// ```text
/// # Error codes.
/// # ------------
/// .equ E_N_ACCOUNTS, 1 # An invalid number of accounts were passed.
/// .equ E_USER_DATA, 2 # The user account has nonzero data length.
/// ```
#[proc_macro_attribute]
pub fn error_codes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            return syn::Error::new_spanned(&input, "error_codes can only be applied to enums")
                .to_compile_error()
                .into();
        }
    };

    let mut error_entries = Vec::new();
    let mut variant_defs = Vec::new();

    for (idx, variant) in variants.iter().enumerate() {
        // Ensure no fields.
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(
                &variant.fields,
                "error_codes variants must be unit variants (no fields)",
            )
            .to_compile_error()
            .into();
        }

        // Extract doc comment.
        let doc_comment = extract_doc_comment(&variant.attrs);
        let doc_comment = match doc_comment {
            Some(doc) => doc,
            None => {
                return syn::Error::new_spanned(
                    &variant.ident,
                    format!("Variant `{}` must have a doc comment", variant.ident),
                )
                .to_compile_error()
                .into();
            }
        };

        // Validate doc comment.
        if let Err(e) = validate_doc_comment(&doc_comment) {
            return syn::Error::new_spanned(
                &variant.ident,
                format!("Variant `{}`: {}", variant.ident, e),
            )
            .to_compile_error()
            .into();
        }

        // Convert variant name to SCREAMING_SNAKE_CASE for ASM.
        let asm_name = format!("E_{}", variant.ident.to_string().to_case(Case::UpperSnake));

        // Error codes start at 1.
        let value = idx + 1;

        error_entries.push((asm_name, value, doc_comment));

        // Preserve variant with its attributes.
        let variant_ident = &variant.ident;
        let variant_attrs = &variant.attrs;
        variant_defs.push(quote! {
            #(#variant_attrs)*
            #variant_ident
        });
    }

    // Generate the to_asm() implementation.
    let asm_lines: Vec<String> = error_entries
        .iter()
        .map(|(name, value, comment)| asm_equ_line(name, value, comment))
        .collect();

    let header = asm_header("Error codes.");
    let body = asm_lines.join("\n");

    let expanded = quote! {
        #(#attrs)*
        #[repr(u64)]
        #vis enum #name {
            #(#variant_defs),*
        }

        impl #name {
            /// Generate ASM constants for this enum.
            pub fn to_asm() -> alloc::string::String {
                alloc::format!("{}\n{}\n", #header, #body)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generate an ASM section header with auto-width dashes.
fn asm_header(title: &str) -> String {
    let dash_len = title.len().min(MAX_COMMENT_LEN);
    format!("# {}\n# {}", title, "-".repeat(dash_len))
}

/// Validate a doc comment: must end with period and fit within max length.
fn validate_doc_comment(comment: &str) -> Result<(), String> {
    if !comment.ends_with('.') {
        return Err(format!("Doc comment must end with a period: {:?}", comment));
    }
    if comment.len() > MAX_COMMENT_LEN {
        return Err(format!(
            "Doc comment exceeds max length of {} chars (got {}): {:?}",
            MAX_COMMENT_LEN,
            comment.len(),
            comment
        ));
    }
    Ok(())
}

/// Format an ASM .equ line. If inline comment would exceed max line length,
/// put the comment on its own line above.
fn asm_equ_line(name: &str, value: impl std::fmt::Display, comment: &str) -> String {
    let inline = format!(".equ {}, {} # {}", name, value, comment);
    if inline.len() <= MAX_LINE_LEN {
        inline
    } else {
        format!("# {}\n.equ {}, {}", comment, name, value)
    }
}

/// Extract the doc comment from attributes.
fn extract_doc_comment(attrs: &[syn::Attribute]) -> Option<String> {
    let mut doc_parts = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        doc_parts.push(lit_str.value().trim().to_string());
                    }
                }
            }
        }
    }

    if doc_parts.is_empty() {
        None
    } else {
        Some(doc_parts.join(" "))
    }
}

struct ConstantDef {
    doc: String,
    name: Ident,
    ty: syn::Type,
    value: LitInt,
}

struct ConstantGroup {
    doc: String,
    name: Ident,
    prefix: Option<String>,
    constants: Vec<ConstantDef>,
}

struct AsmConstantsInput {
    groups: Vec<ConstantGroup>,
}

impl Parse for AsmConstantsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut groups = Vec::new();

        while !input.is_empty() {
            // Parse doc comments for the module.
            let attrs = input.call(syn::Attribute::parse_outer)?;
            let doc = extract_doc_comment(&attrs)
                .ok_or_else(|| input.error("Module must have a doc comment"))?;

            // Validate group doc comment.
            if let Err(e) = validate_doc_comment(&doc) {
                return Err(input.error(format!("Group doc comment: {}", e)));
            }

            // Parse group name (always pub).
            let name: Ident = input.parse()?;

            // Parse module body.
            let content;
            braced!(content in input);

            // Check for optional prefix.
            let prefix = if content.peek(Ident) {
                let ident: Ident = content.parse()?;
                if ident == "prefix" {
                    content.parse::<Token![=]>()?;
                    let prefix_lit: syn::LitStr = content.parse()?;
                    content.parse::<Token![,]>()?;
                    Some(prefix_lit.value())
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "Expected 'prefix' or constant",
                    ));
                }
            } else {
                None
            };

            // Parse constants.
            let mut constants = Vec::new();
            while !content.is_empty() {
                let const_attrs = content.call(syn::Attribute::parse_outer)?;
                let const_doc = extract_doc_comment(&const_attrs)
                    .ok_or_else(|| content.error("Constant must have a doc comment"))?;

                // Validate constant doc comment.
                if let Err(e) = validate_doc_comment(&const_doc) {
                    return Err(content.error(e));
                }

                let const_name: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                let const_ty: syn::Type = content.parse()?;
                content.parse::<Token![=]>()?;
                let const_value: LitInt = content.parse()?;

                // Optional trailing comma.
                let _ = content.parse::<Token![,]>();

                constants.push(ConstantDef {
                    doc: const_doc,
                    name: const_name,
                    ty: const_ty,
                    value: const_value,
                });
            }

            groups.push(ConstantGroup {
                doc,
                name,
                prefix,
                constants,
            });
        }

        Ok(AsmConstantsInput { groups })
    }
}

/// Macro for defining groups of constants shared between Rust and ASM.
///
/// Constants must specify their Rust type. Values are validated at compile time
/// to fit within i32 range (sBPF immediate constraint).
///
/// See <https://docs.rs/solana-sbpf/> for sBPF documentation. Immediates are
/// sign-extended from 32-bit, so values must fit in i32 range.
///
/// # Example
/// ```ignore
/// constant_group! {
///     /// Memory map.
///     memory_map {
///         /// Number of accounts expected.
///         N_ACCOUNTS: u64 = 2,
///         /// Offset to instruction data.
///         IX_DATA: usize = 8,
///     }
///
///     /// Stack frame offsets.
///     stack_offsets {
///         prefix = "SF_",
///         /// Offset to user pubkey.
///         USER_PUBKEY: u64 = 0,
///     }
/// }
/// ```
#[proc_macro]
pub fn constant_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AsmConstantsInput);

    let modules = input.groups.iter().map(|group| {
        let mod_name = &group.name;
        let prefix = group.prefix.as_deref().unwrap_or("");

        // Generate Rust constants with i32 bounds checking for ASM compatibility.
        let const_defs = group.constants.iter().map(|c| {
            let name = &c.name;
            let ty = &c.ty;
            let value = &c.value;
            let doc = &c.doc;
            let assert_name = Ident::new(&format!("_ASSERT_{}_FITS_I32", name), name.span());
            quote! {
                #[doc = #doc]
                pub const #name: #ty = #value;

                const #assert_name: () = assert!(
                    (#value as i64) >= (i32::MIN as i64) && (#value as i64) <= (i32::MAX as i64),
                    "ASM immediate must fit in i32 range"
                );
            }
        });

        // Generate ASM output.
        let header = asm_header(&group.doc);

        let asm_lines: Vec<String> = group
            .constants
            .iter()
            .map(|c| {
                let full_name = format!("{}{}", prefix, c.name);
                asm_equ_line(&full_name, &c.value, &c.doc)
            })
            .collect();
        let body = asm_lines.join("\n");

        quote! {
            pub mod #mod_name {
                #(#const_defs)*

                /// Generate ASM constants for this module.
                pub fn to_asm() -> alloc::string::String {
                    alloc::format!("{}\n{}\n", #header, #body)
                }
            }
        }
    });

    let expanded = quote! {
        #(#modules)*
    };

    TokenStream::from(expanded)
}
