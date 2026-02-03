use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta};

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
