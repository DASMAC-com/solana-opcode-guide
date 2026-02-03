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
/// Automatically adds `#[repr(u32)]` to the enum. Each variant must have a doc
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
        #[repr(u32)]
        #vis enum #name {
            #(#variant_defs),*
        }

        impl From<#name> for u32 {
            fn from(e: #name) -> u32 {
                e as u32
            }
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

            // Parse group name.
            let name: Ident = input.parse()?;

            // Parse module body.
            let content;
            braced!(content in input);

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
                constants,
            });
        }

        Ok(AsmConstantsInput { groups })
    }
}

/// Macro for defining groups of constants shared between Rust and ASM.
///
/// Constants must specify their Rust type. Values are validated at compile time
/// to fit within i32 range (sBPF immediate constraint). The prefix is automatically
/// joined with an underscore.
///
/// # Example
/// ```ignore
/// constant_group! {
///     /// Input buffer layout.
///     input_buffer {
///         /// Number of accounts expected.
///         N_ACCOUNTS: u64 = 2,
///     }
/// }
/// // Usage: input_buffer::to_asm("IB") -> ".equ IB_N_ACCOUNTS, 2 # ..."
/// ```
///
/// To extend a group with ASM-only constants, use `extend_constant_group!`.
#[proc_macro]
pub fn constant_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AsmConstantsInput);

    let modules = input.groups.iter().map(|group| {
        let mod_name = &group.name;
        let max_line_len = MAX_LINE_LEN;
        let header = asm_header(&group.doc);

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

        let const_names: Vec<String> = group.constants.iter().map(|c| c.name.to_string()).collect();
        let const_values: Vec<String> = group.constants.iter().map(|c| c.value.to_string()).collect();
        let const_docs: Vec<String> = group.constants.iter().map(|c| c.doc.clone()).collect();

        quote! {
            pub mod #mod_name {
                #(#const_defs)*

                /// Generate ASM constants for this module with the given prefix.
                /// Prefix is automatically joined with underscore (e.g., "IB" -> "IB_NAME").
                pub fn to_asm(prefix: &str) -> alloc::string::String {
                    use alloc::string::String;
                    use alloc::format;

                    let mut result = String::from(#header);
                    result.push('\n');

                    let names = [#(#const_names),*];
                    let values = [#(#const_values),*];
                    let docs = [#(#const_docs),*];

                    for i in 0..names.len() {
                        let full_name = format!("{}_{}", prefix, names[i]);
                        let inline = format!(".equ {}, {} # {}", full_name, values[i], docs[i]);
                        if inline.len() <= #max_line_len {
                            result.push_str(&inline);
                        } else {
                            result.push_str(&format!("# {}\n.equ {}, {}", docs[i], full_name, values[i]));
                        }
                        result.push('\n');
                    }

                    result
                }
            }
        }
    });

    let expanded = quote! {
        #(#modules)*
    };

    TokenStream::from(expanded)
}

/// ASM-only constant (no Rust type needed).
struct AsmConstantDef {
    doc: String,
    name: Ident,
    value: LitInt,
}

/// Input for extend_constant_group! macro.
struct ExtendConstantGroupInput {
    name: Ident,
    prefix: String,
    constants: Vec<AsmConstantDef>,
}

impl Parse for ExtendConstantGroupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse module name.
        let name: Ident = input.parse()?;

        // Parse body.
        let content;
        braced!(content in input);

        // Parse prefix = "..."
        let ident: Ident = content.parse()?;
        if ident != "prefix" {
            return Err(syn::Error::new(
                ident.span(),
                "First item must be 'prefix = \"...\"'",
            ));
        }
        content.parse::<Token![=]>()?;
        let prefix_lit: syn::LitStr = content.parse()?;
        let prefix = prefix_lit.value();
        content.parse::<Token![,]>()?;

        // Parse constants (ASM-only, no type needed).
        let mut constants = Vec::new();
        while !content.is_empty() {
            let const_attrs = content.call(syn::Attribute::parse_outer)?;
            let const_doc = extract_doc_comment(&const_attrs)
                .ok_or_else(|| content.error("Constant must have a doc comment"))?;

            if let Err(e) = validate_doc_comment(&const_doc) {
                return Err(content.error(e));
            }

            let const_name: Ident = content.parse()?;
            content.parse::<Token![=]>()?;
            let const_value: LitInt = content.parse()?;

            // Optional trailing comma.
            let _ = content.parse::<Token![,]>();

            constants.push(AsmConstantDef {
                doc: const_doc,
                name: const_name,
                value: const_value,
            });
        }

        Ok(ExtendConstantGroupInput {
            name,
            prefix,
            constants,
        })
    }
}

/// Macro for extending a constant group with ASM-only constants.
///
/// This creates a module that re-exports the base group's constants from
/// `crate::common::{name}` and adds ASM-only constants. The `to_asm()` function
/// combines both under one header. The prefix is automatically joined with an underscore.
///
/// # Example
/// ```ignore
/// extend_constant_group!(input_buffer {
///     prefix = "IB",
///     /// Offset to number of accounts field.
///     N_ACCOUNTS_OFF = 0,
/// });
/// // Creates `input_buffer` module that:
/// // - Re-exports all constants from crate::common::input_buffer
/// // - Adds ASM-only constants (N_ACCOUNTS_OFF)
/// // - to_asm() outputs all constants with "IB_" prefix under one header
/// ```
#[proc_macro]
pub fn extend_constant_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExtendConstantGroupInput);

    let mod_name = &input.name;
    let prefix = &input.prefix;
    let max_line_len = MAX_LINE_LEN;

    let const_names: Vec<String> = input.constants.iter().map(|c| c.name.to_string()).collect();
    let const_values: Vec<String> = input.constants.iter().map(|c| c.value.to_string()).collect();
    let const_docs: Vec<String> = input.constants.iter().map(|c| c.doc.clone()).collect();

    let expanded = quote! {
        pub mod #mod_name {
            use alloc::string::String;
            use alloc::format;

            // Re-export base group's constants.
            pub use crate::common::#mod_name::*;

            /// Generate combined ASM (base + extension) with prefix.
            pub fn to_asm() -> String {
                // Base group adds header and its constants.
                let mut result = crate::common::#mod_name::to_asm(#prefix);

                // Add extension constants (no separate header).
                let names = [#(#const_names),*];
                let values = [#(#const_values),*];
                let docs = [#(#const_docs),*];

                for i in 0..names.len() {
                    let full_name = format!("{}_{}", #prefix, names[i]);
                    let inline = format!(".equ {}, {} # {}", full_name, values[i], docs[i]);
                    if inline.len() <= #max_line_len {
                        result.push_str(&inline);
                    } else {
                        result.push_str(&format!("# {}\n.equ {}, {}", docs[i], full_name, values[i]));
                    }
                    result.push('\n');
                }

                result
            }
        }
    };

    TokenStream::from(expanded)
}
