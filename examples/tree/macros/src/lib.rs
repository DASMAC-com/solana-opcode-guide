use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, Lit, LitInt, Meta, Token,
};

/// Maximum line length for ASM output.
const MAX_LINE_LEN: usize = 75;

/// Maximum comment length (accounting for `# ` prefix).
const MAX_COMMENT_LEN: usize = MAX_LINE_LEN - "# ".len();

/// Error code entry: doc comment + snake_case name.
struct ErrorCodeEntry {
    doc: String,
    name: Ident,
}

/// Input for error_codes! macro.
struct ErrorCodesInput {
    entries: Vec<ErrorCodeEntry>,
}

impl Parse for ErrorCodesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();

        while !input.is_empty() {
            let attrs = input.call(syn::Attribute::parse_outer)?;
            let doc = extract_doc_comment(&attrs)
                .ok_or_else(|| input.error("Error code must have a doc comment"))?;

            if let Err(e) = validate_doc_comment(&doc) {
                return Err(input.error(e));
            }

            let name: Ident = input.parse()?;

            // Optional trailing comma.
            let _ = input.parse::<Token![,]>();

            entries.push(ErrorCodeEntry { doc, name });
        }

        Ok(ErrorCodesInput { entries })
    }
}

/// Macro for defining error codes shared between Rust and ASM.
///
/// Creates an `Error` enum with `#[repr(u32)]` and auto-numbered variants starting at 1.
/// Variant names are SCREAMING_SNAKE_CASE. ASM names have `E_` prefix added.
///
/// # Example
/// ```ignore
/// error_codes! {
///     /// An invalid number of accounts were passed.
///     N_ACCOUNTS_INVALID,
///     /// The user account has nonzero data length.
///     USER_HAS_DATA,
/// }
/// ```
///
/// Generates:
/// - Rust: `enum Error { N_ACCOUNTS_INVALID, USER_HAS_DATA }` with `From<Error> for u32`
/// - ASM: `.equ E_N_ACCOUNTS_INVALID, 1` and `.equ E_USER_HAS_DATA, 2`
#[proc_macro]
pub fn error_codes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ErrorCodesInput);

    let mut variant_defs = Vec::new();
    let mut asm_lines = Vec::new();

    for (idx, entry) in input.entries.iter().enumerate() {
        let doc = &entry.doc;
        let variant_name = &entry.name;
        // Just add E_ prefix for ASM.
        let asm_name = format!("E_{}", entry.name);
        let value = idx + 1;

        variant_defs.push(quote! {
            #[doc = #doc]
            #variant_name
        });

        asm_lines.push(asm_equ_line(&asm_name, &value, doc));
    }

    let header = asm_header("Error codes.");
    let body = asm_lines.join("\n");

    let expanded = quote! {
        #[repr(u32)]
        #[allow(non_camel_case_types)]
        pub enum Error {
            #(#variant_defs),*
        }

        impl From<Error> for u32 {
            fn from(e: Error) -> u32 {
                e as u32
            }
        }

        impl Error {
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

impl Parse for ConstantGroup {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

        // Reject multiple groups in a single macro invocation.
        if !input.is_empty() {
            return Err(input.error("Only one constant group per macro invocation; use separate constant_group! calls"));
        }

        Ok(ConstantGroup {
            doc,
            name,
            constants,
        })
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
    let group = parse_macro_input!(input as ConstantGroup);

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

    let expanded = quote! {
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
    };

    TokenStream::from(expanded)
}

/// ASM-only constant (no Rust type needed).
struct AsmConstantDef {
    doc: String,
    name: Ident,
    kind: AsmConstantKind,
}

/// Kind of ASM constant.
enum AsmConstantKind {
    /// Direct value (i32 validated).
    Value(LitInt),
    /// Offset derived from struct field (i16 validated).
    /// Name gets `_OFF` suffix appended.
    Offset { struct_name: Ident, field_name: Ident },
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
        // Supports two syntaxes:
        //   NAME = value -> direct value (i32 validated)
        //   offset!(NAME, Struct.field) -> offset (i16 validated, _OFF suffix)
        let mut constants = Vec::new();
        while !content.is_empty() {
            let const_attrs = content.call(syn::Attribute::parse_outer)?;
            let const_doc = extract_doc_comment(&const_attrs)
                .ok_or_else(|| content.error("Constant must have a doc comment"))?;

            if let Err(e) = validate_doc_comment(&const_doc) {
                return Err(content.error(e));
            }

            // Check for offset!(NAME, Struct.field) syntax.
            let lookahead = content.lookahead1();
            let (const_name, kind) = if lookahead.peek(Ident) {
                let ident: Ident = content.parse()?;
                if ident == "offset" {
                    // Parse offset!(NAME, Struct.field)
                    content.parse::<Token![!]>()?;
                    let inner;
                    syn::parenthesized!(inner in content);
                    let base_name: Ident = inner.parse()?;
                    inner.parse::<Token![,]>()?;
                    let struct_name: Ident = inner.parse()?;
                    inner.parse::<Token![.]>()?;
                    let field_name: Ident = inner.parse()?;
                    // Append _OFF suffix to the name.
                    let full_name = Ident::new(&format!("{}_OFF", base_name), base_name.span());
                    (full_name, AsmConstantKind::Offset { struct_name, field_name })
                } else {
                    // Regular NAME = value syntax.
                    content.parse::<Token![=]>()?;
                    let value: LitInt = content.parse()?;
                    (ident, AsmConstantKind::Value(value))
                }
            } else {
                return Err(lookahead.error());
            };

            // Optional trailing comma.
            let _ = content.parse::<Token![,]>();

            constants.push(AsmConstantDef {
                doc: const_doc,
                name: const_name,
                kind,
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
/// Supports two constant syntaxes:
/// - `NAME = value` - direct value (validated for i32 range)
/// - `offset!(NAME, Struct.field)` - offset (validated for i16 range, `_OFF` suffix added)
///
/// # Example
/// ```ignore
/// extend_constant_group!(input_buffer {
///     prefix = "IB",
///     /// Offset to number of accounts field.
///     offset!(N_ACCOUNTS, InputBuffer.n_accounts),
/// });
/// // Creates `input_buffer` module that:
/// // - Re-exports all constants from crate::common::input_buffer
/// // - Adds N_ACCOUNTS_OFF constant derived from offset_of!(InputBuffer, n_accounts)
/// // - to_asm() outputs ".equ IB_N_ACCOUNTS_OFF, 0 # ..."
/// ```
#[proc_macro]
pub fn extend_constant_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExtendConstantGroupInput);

    let mod_name = &input.name;
    let prefix = &input.prefix;
    let max_line_len = MAX_LINE_LEN;

    // Generate constant definitions and collect info for ASM.
    let mut const_defs = Vec::new();
    let mut const_names = Vec::new();
    let mut const_docs = Vec::new();

    for c in &input.constants {
        let name = &c.name;
        let doc = &c.doc;
        let name_str = name.to_string();
        let assert_name = Ident::new(&format!("_ASSERT_{}_FITS", name), name.span());

        const_names.push(name_str);
        const_docs.push(doc.clone());

        match &c.kind {
            AsmConstantKind::Value(value) => {
                // Direct value - validate i32 range.
                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #name: i32 = #value;

                    const #assert_name: () = assert!(
                        (#value as i64) >= (i32::MIN as i64) && (#value as i64) <= (i32::MAX as i64),
                        "ASM immediate must fit in i32 range"
                    );
                });
            }
            AsmConstantKind::Offset { struct_name, field_name } => {
                // Offset from struct field - validate i16 range.
                // Use super:: to access struct from parent module.
                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #name: i16 = core::mem::offset_of!(super::#struct_name, #field_name) as i16;

                    const #assert_name: () = assert!(
                        (core::mem::offset_of!(super::#struct_name, #field_name) as i64) >= (i16::MIN as i64)
                            && (core::mem::offset_of!(super::#struct_name, #field_name) as i64) <= (i16::MAX as i64),
                        "Offset must fit in i16 range"
                    );
                });
            }
        }
    }

    // Collect const idents for ASM output.
    let const_idents: Vec<_> = input.constants.iter().map(|c| &c.name).collect();

    let expanded = quote! {
        pub mod #mod_name {
            use alloc::string::String;
            use alloc::format;

            // Re-export base group's constants.
            pub use crate::common::#mod_name::*;

            #(#const_defs)*

            /// Generate combined ASM (base + extension) with prefix.
            pub fn to_asm() -> String {
                // Base group adds header and its constants.
                let mut result = crate::common::#mod_name::to_asm(#prefix);

                // Add extension constants (no separate header).
                let names = [#(#const_names),*];
                let values = [#(#const_idents as i64),*];
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
