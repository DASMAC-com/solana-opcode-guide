use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DeriveInput, Fields, Ident, Lit, LitInt, Meta, Token, Visibility,
};

/// Derive macro for generating ASM error code constants from an enum.
///
/// Each variant must have a doc comment that will become the ASM comment.
/// Variant names are converted from PascalCase to SCREAMING_SNAKE_CASE
/// and prefixed with `E_`.
///
/// # Example
/// ```ignore
/// #[derive(AsmErrorCodes)]
/// pub enum ErrorCodes {
///     /// An invalid number of accounts were passed.
///     NAccounts,
///     /// The user account has nonzero data length.
///     UserData,
/// }
/// ```
///
/// Generates:
/// ```text
/// # Error codes.
/// # ------------
/// .equ E_N_ACCOUNTS, 1 # An invalid number of accounts were passed.
/// .equ E_USER_DATA, 2 # The user account has nonzero data length.
/// ```
#[proc_macro_derive(AsmErrorCodes)]
pub fn derive_asm_error_codes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            return syn::Error::new_spanned(&input, "AsmErrorCodes can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let mut error_entries = Vec::new();

    for (idx, variant) in variants.iter().enumerate() {
        // Ensure no fields.
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(
                &variant.fields,
                "AsmErrorCodes variants must be unit variants (no fields)",
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

        // Validate doc comment ends with period.
        if !doc_comment.ends_with('.') {
            return syn::Error::new_spanned(
                &variant.ident,
                format!(
                    "Doc comment for `{}` must end with a period: {:?}",
                    variant.ident, doc_comment
                ),
            )
            .to_compile_error()
            .into();
        }

        // Convert variant name to SCREAMING_SNAKE_CASE.
        let asm_name = format!(
            "E_{}",
            variant.ident.to_string().to_case(Case::UpperSnake)
        );

        // Error codes start at 1.
        let value = idx + 1;

        error_entries.push((asm_name, value, doc_comment));
    }

    // Generate the to_asm() implementation.
    let asm_lines: Vec<String> = error_entries
        .iter()
        .map(|(name, value, comment)| format!(".equ {}, {} # {}", name, value, comment))
        .collect();

    let header = "# Error codes.\n# ------------";
    let body = asm_lines.join("\n");

    let expanded = quote! {
        impl #name {
            /// Generate ASM constants for this enum.
            pub fn to_asm() -> alloc::string::String {
                alloc::format!("{}\n{}\n", #header, #body)
            }
        }
    };

    TokenStream::from(expanded)
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

// ============================================================================
// asm_constants! macro
// ============================================================================

struct ConstantDef {
    doc: String,
    name: Ident,
    value: LitInt,
}

struct ConstantGroup {
    doc: String,
    vis: Visibility,
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

            // Parse visibility and `mod`.
            let vis: Visibility = input.parse()?;
            input.parse::<Token![mod]>()?;
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
                    return Err(syn::Error::new(ident.span(), "Expected 'prefix' or constant"));
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

                let const_name: Ident = content.parse()?;
                content.parse::<Token![=]>()?;
                let const_value: LitInt = content.parse()?;

                // Optional trailing comma.
                let _ = content.parse::<Token![,]>();

                constants.push(ConstantDef {
                    doc: const_doc,
                    name: const_name,
                    value: const_value,
                });
            }

            groups.push(ConstantGroup {
                doc,
                vis,
                name,
                prefix,
                constants,
            });
        }

        Ok(AsmConstantsInput { groups })
    }
}

/// Macro for defining groups of ASM constants.
///
/// # Example
/// ```ignore
/// asm_constants! {
///     /// Memory map.
///     pub mod memory_map {
///         /// Number of accounts expected.
///         N_ACCOUNTS = 2,
///         /// Offset to instruction data.
///         IX_DATA = 8,
///     }
///
///     /// Error codes.
///     pub mod error_codes {
///         prefix = "E_",
///         /// An invalid number of accounts were passed.
///         N_ACCOUNTS = 1,
///     }
/// }
/// ```
#[proc_macro]
pub fn asm_constants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AsmConstantsInput);

    let modules = input.groups.iter().map(|group| {
        let vis = &group.vis;
        let mod_name = &group.name;
        let prefix = group.prefix.as_deref().unwrap_or("");

        // Generate Rust constants.
        let const_defs = group.constants.iter().map(|c| {
            let name = &c.name;
            let value = &c.value;
            let doc = &c.doc;
            quote! {
                #[doc = #doc]
                pub const #name: u64 = #value;
            }
        });

        // Generate ASM output.
        let header_text = &group.doc;
        let header_line = "-".repeat(header_text.len());
        let header = format!("# {}\n# {}", header_text, header_line);

        let asm_lines: Vec<String> = group
            .constants
            .iter()
            .map(|c| {
                format!(
                    ".equ {}{}, {} # {}",
                    prefix,
                    c.name.to_string(),
                    c.value,
                    c.doc
                )
            })
            .collect();
        let body = asm_lines.join("\n");

        quote! {
            #vis mod #mod_name {
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
