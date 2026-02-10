use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{discouraged::Speculative, Parse, ParseStream},
    parse_macro_input, Ident, Lit, LitInt, Meta, Token,
};

/// Maximum line length for ASM output.
const MAX_LINE_LEN: usize = 75;

/// BPF alignment requirement (align_of::<u128>()).
const BPF_ALIGN: i64 = 8;

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
        let value = (idx + 1) as u32;

        variant_defs.push(quote! {
            #[doc = #doc]
            #variant_name = #value
        });

        asm_lines.push(asm_equ_line(&asm_name, value, doc));
    }

    let header = asm_header("Error codes.");
    let body = asm_lines.join("\n");

    let expanded = quote! {
        pub mod error_codes {
            #[repr(u32)]
            #[allow(non_camel_case_types)]
            pub enum error {
                #(#variant_defs),*
            }

            impl From<error> for u32 {
                fn from(e: error) -> u32 {
                    e as u32
                }
            }

            impl From<error> for u64 {
                fn from(e: error) -> u64 {
                    e as u64
                }
            }

            /// Generate ASM constants for error codes.
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

enum ConstantKind {
    /// Regular constant with explicit type and value.
    Value {
        ty: syn::Type,
        value: syn::Expr,
        /// Original literal string for ASM output (preserves hex/binary).
        literal_repr: Option<String>,
    },
    /// Offset derived from struct field path (i16 validated).
    /// Name gets `_OFF` suffix appended.
    Offset {
        struct_name: Ident,
        field_path: Vec<Ident>,
    },
    /// Negative offset from end of a stack frame struct (i16 validated).
    /// Computed as `offset_of!(Struct, field) - size_of::<Struct>()`.
    /// Name gets `_OFF` suffix (aligned) or `_UOFF` suffix (unaligned).
    /// Array element types are resolved via the `__<Struct>_fields` companion module
    /// generated by `#[stack_frame]`.
    StackFrameOffset {
        struct_name: Ident,
        field_path_tokens: proc_macro2::TokenStream,
        array_index: Option<ArrayIndexInfo>,
        aligned: bool,
    },
    /// Pubkey chunk offset (i16 validated).
    /// Base offset + chunk_index * 8 for 8-byte register loads.
    /// Name gets `_OFF_{chunk_index}` suffix appended.
    PubkeyChunkOffset {
        struct_name: Ident,
        field_path: Vec<Ident>,
        chunk_index: usize,
    },
}

/// Array index info for stack frame offset computation.
/// Supports expression indices (e.g., `CpiAccountIndex::User`) and
/// optional inner field access (e.g., `.is_writable`).
struct ArrayIndexInfo {
    /// The array field name on the struct (for type alias lookup).
    array_field_name: Ident,
    /// The index expression (e.g., `0`, `CpiAccountIndex::User`).
    index_expr: syn::Expr,
    /// Optional inner field path after the array index (e.g., `is_writable`).
    inner_field_path: Option<proc_macro2::TokenStream>,
}

struct ConstantDef {
    doc: String,
    name: Ident,
    kind: ConstantKind,
}

struct ConstantGroup {
    doc: String,
    name: Ident,
    prefix: Option<String>,
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

        // Parse optional header parameter: prefix = "..."
        let prefix = parse_group_params(&content)?;

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

            let ident: Ident = content.parse()?;

            // pubkey_offset!(NAME, Struct.field) expands to 4 chunk constants.
            if ident == "pubkey_offset" {
                content.parse::<Token![!]>()?;
                let inner;
                syn::parenthesized!(inner in content);
                let base_name: Ident = inner.parse()?;
                inner.parse::<Token![,]>()?;
                let struct_name: Ident = inner.parse()?;
                let mut field_path = Vec::new();
                while inner.peek(Token![.]) {
                    inner.parse::<Token![.]>()?;
                    field_path.push(inner.parse::<Ident>()?);
                }
                if field_path.is_empty() {
                    return Err(inner.error("Expected at least one field after struct name"));
                }
                let _ = content.parse::<Token![,]>();

                let base_doc = const_doc.trim_end_matches('.');
                for chunk in 0..4usize {
                    let chunk_name =
                        Ident::new(&format!("{}_OFF_{}", base_name, chunk), base_name.span());
                    let chunk_doc = format!("{} (chunk index {}).", base_doc, chunk);
                    if let Err(e) = validate_doc_comment(&chunk_doc) {
                        return Err(content.error(e));
                    }
                    constants.push(ConstantDef {
                        doc: chunk_doc,
                        name: chunk_name,
                        kind: ConstantKind::PubkeyChunkOffset {
                            struct_name: struct_name.clone(),
                            field_path: field_path.clone(),
                            chunk_index: chunk,
                        },
                    });
                }
                continue;
            }

            // Support `offset!(NAME, Struct.field)`, `NAME: type = value`,
            // and `NAME = expr as Type` forms.
            let (const_name, kind) = if ident == "offset" {
                // offset!(NAME, Struct.field.nested.path)
                content.parse::<Token![!]>()?;
                let inner;
                syn::parenthesized!(inner in content);
                let base_name: Ident = inner.parse()?;
                inner.parse::<Token![,]>()?;
                let struct_name: Ident = inner.parse()?;
                let mut field_path = Vec::new();
                while inner.peek(Token![.]) {
                    inner.parse::<Token![.]>()?;
                    field_path.push(inner.parse::<Ident>()?);
                }
                if field_path.is_empty() {
                    return Err(inner.error("Expected at least one field after struct name"));
                }
                let full_name = Ident::new(&format!("{}_OFF", base_name), base_name.span());
                (
                    full_name,
                    ConstantKind::Offset {
                        struct_name,
                        field_path,
                    },
                )
            } else if ident == "stack_frame_offset" || ident == "stack_frame_offset_unaligned" {
                let aligned = ident == "stack_frame_offset";
                let suffix = if aligned { "_OFF" } else { "_UOFF" };
                content.parse::<Token![!]>()?;
                let inner;
                syn::parenthesized!(inner in content);
                let base_name: Ident = inner.parse()?;
                inner.parse::<Token![,]>()?;
                let struct_name: Ident = inner.parse()?;
                let parsed = parse_stack_frame_field_path(&inner)?;
                let full_name = Ident::new(&format!("{}{}", base_name, suffix), base_name.span());
                (
                    full_name,
                    ConstantKind::StackFrameOffset {
                        struct_name,
                        field_path_tokens: parsed.field_path_tokens,
                        array_index: parsed.array_index,
                        aligned,
                    },
                )
            } else if content.peek(Token![:]) {
                // NAME: type = value
                content.parse::<Token![:]>()?;
                let ty: syn::Type = content.parse()?;
                content.parse::<Token![=]>()?;

                // Try literal first to preserve hex/binary representation.
                let fork = content.fork();
                if let Ok(lit) = fork.parse::<LitInt>() {
                    content.advance_to(&fork);
                    let repr = lit.to_string();
                    let expr = syn::Expr::Lit(syn::ExprLit {
                        attrs: vec![],
                        lit: Lit::Int(lit),
                    });
                    (
                        ident,
                        ConstantKind::Value {
                            ty,
                            value: expr,
                            literal_repr: Some(repr),
                        },
                    )
                } else {
                    let expr: syn::Expr = content.parse()?;
                    (
                        ident,
                        ConstantKind::Value {
                            ty,
                            value: expr,
                            literal_repr: None,
                        },
                    )
                }
            } else {
                // NAME = expr (type inferred from `as Type` cast).
                content.parse::<Token![=]>()?;
                let expr: syn::Expr = content.parse()?;

                let ty = if let syn::Expr::Cast(cast) = &expr {
                    (*cast.ty).clone()
                } else {
                    return Err(content.error(
                        "Expression must include `as Type` when type annotation is omitted",
                    ));
                };

                (
                    ident,
                    ConstantKind::Value {
                        ty,
                        value: expr,
                        literal_repr: None,
                    },
                )
            };

            // Optional trailing comma.
            let _ = content.parse::<Token![,]>();

            constants.push(ConstantDef {
                doc: const_doc,
                name: const_name,
                kind,
            });
        }

        // Reject multiple groups in a single macro invocation.
        if !input.is_empty() {
            return Err(input.error(
                "Only one constant group per macro invocation; use separate constant_group! calls",
            ));
        }

        Ok(ConstantGroup {
            doc,
            name,
            prefix,
            constants,
        })
    }
}

/// Macro for defining groups of constants shared between Rust and ASM.
///
/// Values are validated at compile time to fit within i32 range (sBPF immediate constraint).
/// The prefix is automatically joined with an underscore.
///
/// Two syntaxes are supported:
/// - `NAME: type = value` — explicit type, literal values preserve hex/binary in ASM.
/// - `NAME = expr as Type` — type inferred from `as` cast, value computed at build time.
///
/// # Example
/// ```ignore
/// constant_group! {
///     /// Input buffer layout.
///     input_buffer {
///         /// Number of accounts expected.
///         N_ACCOUNTS: u64 = 2,
///         /// Expected data length of system program account.
///         SYSTEM_PROGRAM_DATA_LEN = b"system_program".len() as u64,
///     }
/// }
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
    let mut const_defs = Vec::new();
    let mut const_value_strs: Vec<Option<String>> = Vec::new();

    for c in &group.constants {
        let name = &c.name;
        let doc = &c.doc;

        match &c.kind {
            ConstantKind::Value {
                ty,
                value,
                literal_repr,
            } => {
                let assert_name = Ident::new(&format!("_ASSERT_{}_FITS_I32", name), name.span());
                const_value_strs.push(literal_repr.clone());

                if literal_repr.is_some() {
                    // Literal value - no scope wrapper needed.
                    const_defs.push(quote! {
                        #[doc = #doc]
                        pub const #name: #ty = #value;

                        const #assert_name: () = assert!(
                            (#value as i64) >= (i32::MIN as i64) && (#value as i64) <= (i32::MAX as i64),
                            "ASM immediate must fit in i32 range"
                        );
                    });
                } else {
                    // Expression value - use super::* for scope access.
                    const_defs.push(quote! {
                        #[doc = #doc]
                        pub const #name: #ty = { use super::*; #value };

                        const #assert_name: () = assert!(
                            ({ use super::*; #value } as i64) >= (i32::MIN as i64)
                                && ({ use super::*; #value } as i64) <= (i32::MAX as i64),
                            "ASM immediate must fit in i32 range"
                        );
                    });
                }
            }
            ConstantKind::Offset {
                struct_name,
                field_path,
            } => {
                let assert_name = Ident::new(&format!("_ASSERT_{}_FITS", name), name.span());
                const_value_strs.push(None);

                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #name: i16 = core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i16;

                    const #assert_name: () = assert!(
                        (core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i64) >= (i16::MIN as i64)
                            && (core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i64) <= (i16::MAX as i64),
                        "Offset must fit in i16 range"
                    );
                });
            }
            ConstantKind::StackFrameOffset {
                struct_name,
                field_path_tokens,
                array_index,
                aligned,
            } => {
                let (const_def, literal_repr) = gen_stack_frame_offset_code(
                    name,
                    doc,
                    struct_name,
                    field_path_tokens,
                    array_index,
                    *aligned,
                );
                const_value_strs.push(literal_repr);
                const_defs.push(const_def);
            }
            ConstantKind::PubkeyChunkOffset {
                struct_name,
                field_path,
                chunk_index,
            } => {
                const_value_strs.push(None);
                const_defs.push(gen_pubkey_chunk_offset_code(
                    name,
                    doc,
                    struct_name,
                    field_path,
                    *chunk_index,
                ));
            }
        }
    }

    let const_names: Vec<String> = group.constants.iter().map(|c| c.name.to_string()).collect();
    let const_idents: Vec<&Ident> = group.constants.iter().map(|c| &c.name).collect();
    let const_docs: Vec<String> = group.constants.iter().map(|c| c.doc.clone()).collect();

    let value_str_opts: Vec<_> = const_value_strs
        .iter()
        .map(|opt| match opt {
            Some(s) => quote! { Some(#s) },
            None => quote! { None },
        })
        .collect();

    // Generate to_asm function signature based on whether prefix is baked in.
    let to_asm_fn = if let Some(ref prefix) = group.prefix {
        let name_format = quote! { format!("{}_{}", #prefix, names[i]) };
        quote! {
            /// Generate ASM constants for this module.
            pub fn to_asm() -> alloc::string::String {
                use alloc::string::String;
                use alloc::format;

                let mut result = String::from(#header);
                result.push('\n');

                let names = [#(#const_names),*];
                let computed_values: &[i64] = &[#(#const_idents as i64),*];
                let literal_values: &[Option<&str>] = &[#(#value_str_opts),*];
                let docs = [#(#const_docs),*];

                for i in 0..names.len() {
                    let full_name = #name_format;
                    let value_str = match literal_values[i] {
                        Some(lit) => String::from(lit),
                        None => format!("{}", computed_values[i]),
                    };
                    let inline = format!(".equ {}, {} # {}", full_name, value_str, docs[i]);
                    if inline.len() <= #max_line_len {
                        result.push_str(&inline);
                    } else {
                        result.push_str(&format!("# {}\n.equ {}, {}", docs[i], full_name, value_str));
                    }
                    result.push('\n');
                }

                result
            }
        }
    } else {
        quote! {
            /// Generate ASM constants for this module with the given prefix.
            /// Prefix is automatically joined with underscore (e.g., "IB" -> "IB_NAME").
            pub fn to_asm(prefix: &str) -> alloc::string::String {
                use alloc::string::String;
                use alloc::format;

                let mut result = String::from(#header);
                result.push('\n');

                let names = [#(#const_names),*];
                let computed_values: &[i64] = &[#(#const_idents as i64),*];
                let literal_values: &[Option<&str>] = &[#(#value_str_opts),*];
                let docs = [#(#const_docs),*];

                for i in 0..names.len() {
                    let full_name = if prefix.is_empty() {
                        String::from(names[i])
                    } else {
                        format!("{}_{}", prefix, names[i])
                    };
                    let value_str = match literal_values[i] {
                        Some(lit) => String::from(lit),
                        None => format!("{}", computed_values[i]),
                    };
                    let inline = format!(".equ {}, {} # {}", full_name, value_str, docs[i]);
                    if inline.len() <= #max_line_len {
                        result.push_str(&inline);
                    } else {
                        result.push_str(&format!("# {}\n.equ {}, {}", docs[i], full_name, value_str));
                    }
                    result.push('\n');
                }

                result
            }
        }
    };

    let expanded = quote! {
        pub mod #mod_name {
            #(#const_defs)*

            #to_asm_fn
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
    /// Literal value (i32 validated) - preserves original representation (hex, etc.).
    Literal(LitInt),
    /// Expression value (i32 validated) - computed at runtime, shown as decimal.
    Expr(syn::Expr),
    /// Offset derived from struct field path (i16 validated).
    /// Name gets `_OFF` suffix appended.
    /// Supports nested fields like `Struct.field1.field2.field3`.
    Offset {
        struct_name: Ident,
        field_path: Vec<Ident>,
    },
    /// Negative offset from end of a stack frame struct (i16 validated).
    /// Computed as `offset_of!(Struct, field) - size_of::<Struct>()`.
    /// Name gets `_OFF` suffix (aligned) or `_UOFF` suffix (unaligned).
    /// Array element types are resolved via the `__<Struct>_fields` companion module
    /// generated by `#[stack_frame]`.
    StackFrameOffset {
        struct_name: Ident,
        field_path_tokens: proc_macro2::TokenStream,
        array_index: Option<ArrayIndexInfo>,
        aligned: bool,
    },
    /// Pubkey chunk offset (i16 validated).
    /// Base offset + chunk_index * 8 for 8-byte register loads.
    /// Name gets `_OFF_{chunk_index}` suffix appended.
    PubkeyChunkOffset {
        struct_name: Ident,
        field_path: Vec<Ident>,
        chunk_index: usize,
    },
}

/// Input for asm_constant_group! macro.
struct AsmConstantGroupInput {
    doc: String,
    name: Ident,
    prefix: Option<String>,
    constants: Vec<AsmConstantDef>,
}

/// Parse a dot-separated field path for `offset_of!`.
///
/// Expects tokens like `.field` or `.field1.field2.field3`.
/// Returns a `TokenStream` suitable for use inside `offset_of!(Type, <path>)`.
fn parse_field_path(inner: ParseStream) -> syn::Result<proc_macro2::TokenStream> {
    let mut tokens = proc_macro2::TokenStream::new();
    let mut has_field = false;

    while inner.peek(Token![.]) {
        inner.parse::<Token![.]>()?;
        let field: Ident = inner.parse()?;
        if has_field {
            tokens.extend(quote! { . #field });
        } else {
            tokens.extend(quote! { #field });
            has_field = true;
        }
    }

    if !has_field {
        return Err(inner.error("Expected at least one field in path"));
    }

    Ok(tokens)
}

/// Result of parsing a stack frame field path.
struct StackFrameFieldPath {
    /// Token stream for `offset_of!(Struct, <path>)` (fields before any bracket).
    field_path_tokens: proc_macro2::TokenStream,
    /// Optional array index info (bracket expression, inner field access).
    array_index: Option<ArrayIndexInfo>,
}

/// Parse a stack frame field path with optional array indexing and inner field access.
///
/// Supports:
/// - `Struct.field` → simple field access
/// - `Struct.field[expr]` → array element access
/// - `Struct.field[expr].inner` → array element + inner field access
/// - `Struct.a.b.c` → nested field access (no array)
///
/// Array element types are resolved via `__<Struct>_fields::<array_field>` type aliases
/// generated by `#[stack_frame]`, so the element type is not required in the syntax.
fn parse_stack_frame_field_path(inner: ParseStream) -> syn::Result<StackFrameFieldPath> {
    let mut fields: Vec<Ident> = Vec::new();
    let mut tokens = proc_macro2::TokenStream::new();

    // Parse dot-separated field segments until we hit a bracket or end.
    while inner.peek(Token![.]) {
        inner.parse::<Token![.]>()?;
        let field: Ident = inner.parse()?;

        if !fields.is_empty() {
            tokens.extend(quote! { . });
        }
        tokens.extend(quote! { #field });
        fields.push(field);

        // Check for array index after this field.
        if inner.peek(syn::token::Bracket) {
            let bracket_content;
            bracketed!(bracket_content in inner);
            let index_expr: syn::Expr = bracket_content.parse()?;

            // Parse optional inner field path.
            let inner_field_path = if inner.peek(Token![.]) {
                Some(parse_field_path(inner)?)
            } else {
                None
            };

            let array_field_name = fields.last().unwrap().clone();

            return Ok(StackFrameFieldPath {
                field_path_tokens: tokens,
                array_index: Some(ArrayIndexInfo {
                    array_field_name,
                    index_expr,
                    inner_field_path,
                }),
            });
        }
    }

    if fields.is_empty() {
        return Err(inner.error("Expected at least one field in path"));
    }

    Ok(StackFrameFieldPath {
        field_path_tokens: tokens,
        array_index: None,
    })
}

/// Extract the last path segment name from a type (for auto-generating constant names).
fn extract_type_name(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .last()
            .map(|seg| seg.ident.to_string())
    } else {
        None
    }
}

/// Convert a type name to UPPER_SNAKE_CASE for ASM constants.
///
/// Uses `convert_case` but fixes primitive types like `u8`, `i16` etc.
/// where the default conversion inserts an unwanted underscore (`U_8` → `U8`).
fn type_name_to_upper_snake(name: &str) -> String {
    let s = name.to_case(Case::UpperSnake);
    // Primitive types: single letter + digits (e.g. U_8, I_16, F_32).
    // Remove the underscore between the letter and digits.
    if s.starts_with(|c: char| c.is_ascii_uppercase())
        && s.as_bytes().get(1) == Some(&b'_')
        && s[2..].chars().all(|c| c.is_ascii_digit())
    {
        format!("{}{}", &s[..1], &s[2..])
    } else {
        s
    }
}

/// Parse optional `prefix = "..."` group header parameter.
fn parse_group_params(content: ParseStream) -> syn::Result<Option<String>> {
    // Check for prefix = "..." before constants.
    if content.peek(Ident) {
        let fork = content.fork();
        let ident: Ident = fork.parse()?;
        if ident == "prefix" && fork.peek(Token![=]) {
            content.parse::<Ident>()?;
            content.parse::<Token![=]>()?;
            let prefix_lit: syn::LitStr = content.parse()?;
            content.parse::<Token![,]>()?;
            return Ok(Some(prefix_lit.value()));
        }
    }
    Ok(None)
}

/// Parse ASM constants (shared between asm_constant_group! and extend_constant_group!).
fn parse_asm_constants(content: ParseStream) -> syn::Result<Vec<AsmConstantDef>> {
    let mut constants = Vec::new();
    while !content.is_empty() {
        let const_attrs = content.call(syn::Attribute::parse_outer)?;
        let const_doc = extract_doc_comment(&const_attrs)
            .ok_or_else(|| content.error("Constant must have a doc comment"))?;

        if let Err(e) = validate_doc_comment(&const_doc) {
            return Err(content.error(e));
        }

        // Check for identifier.
        let lookahead = content.lookahead1();
        if !lookahead.peek(Ident) {
            return Err(lookahead.error());
        }
        let ident: Ident = content.parse()?;

        // pubkey_offset!(NAME, Struct.field) expands to 4 chunk constants.
        if ident == "pubkey_offset" {
            content.parse::<Token![!]>()?;
            let inner;
            syn::parenthesized!(inner in content);
            let base_name: Ident = inner.parse()?;
            inner.parse::<Token![,]>()?;
            let struct_name: Ident = inner.parse()?;
            let mut field_path = Vec::new();
            while inner.peek(Token![.]) {
                inner.parse::<Token![.]>()?;
                field_path.push(inner.parse::<Ident>()?);
            }
            if field_path.is_empty() {
                return Err(inner.error("Expected at least one field after struct name"));
            }
            let _ = content.parse::<Token![,]>();

            let base_doc = const_doc.trim_end_matches('.');
            for chunk in 0..4usize {
                let chunk_name =
                    Ident::new(&format!("{}_OFF_{}", base_name, chunk), base_name.span());
                let chunk_doc = format!("{} (chunk index {}).", base_doc, chunk);
                if let Err(e) = validate_doc_comment(&chunk_doc) {
                    return Err(content.error(e));
                }
                constants.push(AsmConstantDef {
                    doc: chunk_doc,
                    name: chunk_name,
                    kind: AsmConstantKind::PubkeyChunkOffset {
                        struct_name: struct_name.clone(),
                        field_path: field_path.clone(),
                        chunk_index: chunk,
                    },
                });
            }
            continue;
        }

        let (const_name, kind) = if ident == "offset" {
                // Parse offset!(NAME, Struct.field.nested.path)
                content.parse::<Token![!]>()?;
                let inner;
                syn::parenthesized!(inner in content);
                let base_name: Ident = inner.parse()?;
                inner.parse::<Token![,]>()?;
                let struct_name: Ident = inner.parse()?;
                // Parse field path (one or more fields separated by dots).
                let mut field_path = Vec::new();
                while inner.peek(Token![.]) {
                    inner.parse::<Token![.]>()?;
                    field_path.push(inner.parse::<Ident>()?);
                }
                if field_path.is_empty() {
                    return Err(inner.error("Expected at least one field after struct name"));
                }
                // Append _OFF suffix to the name.
                let full_name = Ident::new(&format!("{}_OFF", base_name), base_name.span());
                (
                    full_name,
                    AsmConstantKind::Offset {
                        struct_name,
                        field_path,
                    },
                )
            } else if ident == "stack_frame_offset" || ident == "stack_frame_offset_unaligned" {
                let aligned = ident == "stack_frame_offset";
                let suffix = if aligned { "_OFF" } else { "_UOFF" };
                content.parse::<Token![!]>()?;
                let inner;
                syn::parenthesized!(inner in content);
                let base_name: Ident = inner.parse()?;
                inner.parse::<Token![,]>()?;
                let struct_name: Ident = inner.parse()?;
                let parsed = parse_stack_frame_field_path(&inner)?;
                let full_name = Ident::new(&format!("{}{}", base_name, suffix), base_name.span());
                (
                    full_name,
                    AsmConstantKind::StackFrameOffset {
                        struct_name,
                        field_path_tokens: parsed.field_path_tokens,
                        array_index: parsed.array_index,
                        aligned,
                    },
                )
            } else {
                // Regular NAME = value syntax.
                content.parse::<Token![=]>()?;
                // Try to parse as literal first (to preserve hex/binary representation),
                // otherwise parse as expression (for constants like NON_DUP_MARKER).
                let fork = content.fork();
                if let Ok(lit) = fork.parse::<LitInt>() {
                    content.advance_to(&fork);
                    (ident, AsmConstantKind::Literal(lit))
                } else {
                    let expr: syn::Expr = content.parse()?;
                    (ident, AsmConstantKind::Expr(expr))
                }
            };

        // Optional trailing comma.
        let _ = content.parse::<Token![,]>();

        constants.push(AsmConstantDef {
            doc: const_doc,
            name: const_name,
            kind,
        });
    }
    Ok(constants)
}

impl Parse for AsmConstantGroupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse doc comment for the group.
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let doc = extract_doc_comment(&attrs)
            .ok_or_else(|| input.error("Group must have a doc comment"))?;

        if let Err(e) = validate_doc_comment(&doc) {
            return Err(input.error(format!("Group doc comment: {}", e)));
        }

        // Parse group name.
        let name: Ident = input.parse()?;

        // Parse body.
        let content;
        braced!(content in input);

        // Parse optional header parameter: prefix = "..."
        let prefix = parse_group_params(&content)?;

        // Parse constants.
        let constants = parse_asm_constants(&content)?;

        // Reject multiple groups.
        if !input.is_empty() {
            return Err(input.error("Only one group per macro invocation"));
        }

        Ok(AsmConstantGroupInput {
            doc,
            name,
            prefix,
            constants,
        })
    }
}

/// Generate code for a `stack_frame_offset!` constant.
///
/// Returns `(const_def_tokens, literal_repr)` where literal_repr is always `None`
/// (offsets are always computed).
fn gen_stack_frame_offset_code(
    name: &Ident,
    doc: &str,
    struct_name: &Ident,
    field_path_tokens: &proc_macro2::TokenStream,
    array_index: &Option<ArrayIndexInfo>,
    aligned: bool,
) -> (proc_macro2::TokenStream, Option<String>) {
    let assert_name = Ident::new(&format!("_ASSERT_{}_FITS", name), name.span());
    let fields_mod = Ident::new(&format!("__{}_fields", struct_name), struct_name.span());

    let offset_expr = match array_index {
        Some(info) => {
            let array_field_name = &info.array_field_name;
            let index_expr = &info.index_expr;
            let inner_offset = match &info.inner_field_path {
                Some(path) => quote! {
                    + core::mem::offset_of!(#fields_mod::#array_field_name, #path) as i64
                },
                None => quote! {},
            };
            quote! {
                core::mem::offset_of!(#struct_name, #field_path_tokens) as i64
                + (#index_expr) as i64 * core::mem::size_of::<#fields_mod::#array_field_name>() as i64
                #inner_offset
            }
        }
        None => quote! {
            core::mem::offset_of!(#struct_name, #field_path_tokens) as i64
        },
    };

    let align_assertion = if aligned {
        let bpf_align = BPF_ALIGN;
        quote! {
            assert!(
                result % #bpf_align == 0,
                "Stack frame offset must be aligned"
            );
        }
    } else {
        quote! {}
    };

    let const_def = quote! {
        #[doc = #doc]
        pub const #name: i16 = {
            use super::*;
            (#offset_expr - core::mem::size_of::<#struct_name>() as i64) as i16
        };

        const #assert_name: () = {
            use super::*;
            let result = #offset_expr - core::mem::size_of::<#struct_name>() as i64;
            assert!(
                result >= (i16::MIN as i64) && result <= (i16::MAX as i64),
                "Stack frame offset must fit in i16"
            );
            assert!(result < 0, "Stack frame offset must be negative");
            #align_assertion
        };
    };

    (const_def, None)
}

/// Generate code for a `pubkey_offset!` constant (single chunk).
///
/// Computes `offset_of!(Struct, field) + chunk_index * 8` as an i16 constant.
fn gen_pubkey_chunk_offset_code(
    name: &Ident,
    doc: &str,
    struct_name: &Ident,
    field_path: &[Ident],
    chunk_index: usize,
) -> proc_macro2::TokenStream {
    let assert_name = Ident::new(&format!("_ASSERT_{}_FITS", name), name.span());
    let chunk_byte_offset = (chunk_index * BPF_ALIGN as usize) as i64;

    quote! {
        #[doc = #doc]
        pub const #name: i16 = (core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i64 + #chunk_byte_offset) as i16;

        const #assert_name: () = assert!(
            (core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i64 + #chunk_byte_offset) >= (i16::MIN as i64)
                && (core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i64 + #chunk_byte_offset) <= (i16::MAX as i64),
            "Offset must fit in i16 range"
        );
    }
}

/// Macro for defining ASM-only constant groups.
///
/// Constants don't need types - values are `i64`, offsets are `i16`.
/// All values are validated at compile time to fit within i32 range (sBPF constraint).
///
/// Supports two constant syntaxes:
/// - `NAME = value` - direct value (i64, validated for i32 range)
/// - `offset!(NAME, Struct.field)` - offset (i16, validated for i16 range, `_OFF` suffix added)
///
/// # Example
/// ```ignore
/// asm_constant_group! {
///     /// Miscellaneous constants.
///     misc {
///         prefix = "M",
///         /// Data length of zero.
///         DATA_LENGTH_ZERO = 0,
///     }
/// }
/// // Creates `misc` module with:
/// // - pub const DATA_LENGTH_ZERO: i64 = 0;
/// // - to_asm() outputs ".equ M_DATA_LENGTH_ZERO, 0 # ..."
/// ```
#[proc_macro]
pub fn asm_constant_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AsmConstantGroupInput);

    let mod_name = &input.name;
    let max_line_len = MAX_LINE_LEN;
    let header = asm_header(&input.doc);

    // Generate constant definitions and collect info for ASM.
    let mut const_defs = Vec::new();
    let mut const_names = Vec::new();
    let mut const_docs = Vec::new();
    // Track value representations: Some(literal_str) for values, None for offsets.
    let mut const_value_strs: Vec<Option<String>> = Vec::new();

    for c in &input.constants {
        let name = &c.name;
        let doc = &c.doc;
        let name_str = name.to_string();
        let assert_name = Ident::new(&format!("_ASSERT_{}_FITS", name), name.span());

        const_names.push(name_str);
        const_docs.push(doc.clone());

        match &c.kind {
            AsmConstantKind::Literal(value) => {
                // Preserve original literal representation (hex, binary, etc.).
                const_value_strs.push(Some(value.to_string()));
                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #name: i64 = #value;

                    const #assert_name: () = assert!(
                        (#value as i64) >= (i32::MIN as i64) && (#value as i64) <= (i32::MAX as i64),
                        "ASM immediate must fit in i32 range"
                    );
                });
            }
            AsmConstantKind::Expr(expr) => {
                // Expression (e.g., constant from another crate) - computed at runtime.
                // Use super::* to access imports from parent scope.
                const_value_strs.push(None);
                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #name: i64 = { use super::*; #expr as i64 };

                    const #assert_name: () = assert!(
                        ({ use super::*; #expr } as i64) >= (i32::MIN as i64)
                            && ({ use super::*; #expr } as i64) <= (i32::MAX as i64),
                        "ASM immediate must fit in i32 range"
                    );
                });
            }
            AsmConstantKind::Offset {
                struct_name,
                field_path,
            } => {
                // Offsets are computed at runtime, no literal to preserve.
                const_value_strs.push(None);
                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #name: i16 = core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i16;

                    const #assert_name: () = assert!(
                        (core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i64) >= (i16::MIN as i64)
                            && (core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i64) <= (i16::MAX as i64),
                        "Offset must fit in i16 range"
                    );
                });
            }
            AsmConstantKind::StackFrameOffset {
                struct_name,
                field_path_tokens,
                array_index,
                aligned,
            } => {
                let (const_def, literal_repr) = gen_stack_frame_offset_code(
                    name,
                    doc,
                    struct_name,
                    field_path_tokens,
                    array_index,
                    *aligned,
                );
                const_value_strs.push(literal_repr);
                const_defs.push(const_def);
            }
            AsmConstantKind::PubkeyChunkOffset {
                struct_name,
                field_path,
                chunk_index,
            } => {
                const_value_strs.push(None);
                const_defs.push(gen_pubkey_chunk_offset_code(
                    name,
                    doc,
                    struct_name,
                    field_path,
                    *chunk_index,
                ));
            }
        }
    }

    let const_idents: Vec<_> = input.constants.iter().map(|c| &c.name).collect();

    // Generate name formatting logic based on whether prefix is present.
    let name_format = match &input.prefix {
        Some(prefix) => quote! { format!("{}_{}", #prefix, names[i]) },
        None => quote! { String::from(names[i]) },
    };

    // Generate value string options for preserving hex/binary literals.
    let value_str_opts: Vec<_> = const_value_strs
        .iter()
        .map(|opt| match opt {
            Some(s) => quote! { Some(#s) },
            None => quote! { None },
        })
        .collect();

    let expanded = quote! {
        pub mod #mod_name {
            use alloc::string::String;
            use alloc::format;

            #(#const_defs)*

            /// Generate ASM constants.
            pub fn to_asm() -> String {
                let mut result = String::from(#header);
                result.push('\n');

                let names: &[&str] = &[#(#const_names),*];
                let computed_values: &[i64] = &[#(#const_idents as i64),*];
                let literal_values: &[Option<&str>] = &[#(#value_str_opts),*];
                let docs: &[&str] = &[#(#const_docs),*];

                for i in 0..names.len() {
                    let full_name = #name_format;
                    // Use original literal if available, otherwise use computed value.
                    let value_str = match literal_values[i] {
                        Some(lit) => String::from(lit),
                        None => format!("{}", computed_values[i]),
                    };
                    let inline = format!(".equ {}, {} # {}", full_name, value_str, docs[i]);
                    if inline.len() <= #max_line_len {
                        result.push_str(&inline);
                    } else {
                        result.push_str(&format!("# {}\n.equ {}, {}", docs[i], full_name, value_str));
                    }
                    result.push('\n');
                }

                result
            }
        }
    };

    TokenStream::from(expanded)
}

/// Input for extend_constant_group! macro.
struct ExtendConstantGroupInput {
    name: Ident,
    prefix: Option<String>,
    constants: Vec<AsmConstantDef>,
}

impl Parse for ExtendConstantGroupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse module name.
        let name: Ident = input.parse()?;

        // Parse body.
        let content;
        braced!(content in input);

        // Parse optional header parameter: prefix = "..."
        let prefix = parse_group_params(&content)?;

        // Parse constants using shared parser.
        let constants = parse_asm_constants(&content)?;

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
    let max_line_len = MAX_LINE_LEN;

    // Generate constant definitions and collect info for ASM.
    let mut const_defs = Vec::new();
    let mut const_names = Vec::new();
    let mut const_docs = Vec::new();
    // Track value representations: Some(literal_str) for values, None for offsets.
    let mut const_value_strs: Vec<Option<String>> = Vec::new();

    for c in &input.constants {
        let name = &c.name;
        let doc = &c.doc;
        let name_str = name.to_string();
        let assert_name = Ident::new(&format!("_ASSERT_{}_FITS", name), name.span());

        const_names.push(name_str);
        const_docs.push(doc.clone());

        match &c.kind {
            AsmConstantKind::Literal(value) => {
                // Preserve original literal representation (hex, binary, etc.).
                const_value_strs.push(Some(value.to_string()));
                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #name: i32 = #value;

                    const #assert_name: () = assert!(
                        (#value as i64) >= (i32::MIN as i64) && (#value as i64) <= (i32::MAX as i64),
                        "ASM immediate must fit in i32 range"
                    );
                });
            }
            AsmConstantKind::Expr(expr) => {
                // Expression (e.g., constant from another crate) - computed at runtime.
                // Use super::* to access imports from parent scope.
                const_value_strs.push(None);
                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #name: i32 = { use super::*; #expr as i32 };

                    const #assert_name: () = assert!(
                        ({ use super::*; #expr } as i64) >= (i32::MIN as i64)
                            && ({ use super::*; #expr } as i64) <= (i32::MAX as i64),
                        "ASM immediate must fit in i32 range"
                    );
                });
            }
            AsmConstantKind::Offset {
                struct_name,
                field_path,
            } => {
                // Offsets are computed at runtime, no literal to preserve.
                const_value_strs.push(None);
                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #name: i16 = core::mem::offset_of!(super::#struct_name, #(#field_path).*)  as i16;

                    const #assert_name: () = assert!(
                        (core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i64) >= (i16::MIN as i64)
                            && (core::mem::offset_of!(super::#struct_name, #(#field_path).*) as i64) <= (i16::MAX as i64),
                        "Offset must fit in i16 range"
                    );
                });
            }
            AsmConstantKind::StackFrameOffset {
                struct_name,
                field_path_tokens,
                array_index,
                aligned,
            } => {
                let (const_def, literal_repr) = gen_stack_frame_offset_code(
                    name,
                    doc,
                    struct_name,
                    field_path_tokens,
                    array_index,
                    *aligned,
                );
                const_value_strs.push(literal_repr);
                const_defs.push(const_def);
            }
            AsmConstantKind::PubkeyChunkOffset {
                struct_name,
                field_path,
                chunk_index,
            } => {
                const_value_strs.push(None);
                const_defs.push(gen_pubkey_chunk_offset_code(
                    name,
                    doc,
                    struct_name,
                    field_path,
                    *chunk_index,
                ));
            }
        }
    }

    // Collect const idents for ASM output.
    let const_idents: Vec<_> = input.constants.iter().map(|c| &c.name).collect();

    // Generate name formatting logic based on whether prefix is present.
    let base_prefix = match &input.prefix {
        Some(prefix) => quote! { #prefix },
        None => quote! { "" },
    };
    let name_format = match &input.prefix {
        Some(prefix) => quote! { format!("{}_{}", #prefix, names[i]) },
        None => quote! { String::from(names[i]) },
    };

    // Generate value string options for preserving hex/binary literals.
    let value_str_opts: Vec<_> = const_value_strs
        .iter()
        .map(|opt| match opt {
            Some(s) => quote! { Some(#s) },
            None => quote! { None },
        })
        .collect();

    let expanded = quote! {
        pub mod #mod_name {
            use alloc::string::String;
            use alloc::format;

            // Re-export base group's constants.
            pub use crate::common::#mod_name::*;

            #(#const_defs)*

            /// Generate combined ASM (base + extension).
            pub fn to_asm() -> String {
                // Base group adds header and its constants.
                let mut result = crate::common::#mod_name::to_asm(#base_prefix);

                // Add extension constants (no separate header).
                let names: &[&str] = &[#(#const_names),*];
                let computed_values: &[i64] = &[#(#const_idents as i64),*];
                let literal_values: &[Option<&str>] = &[#(#value_str_opts),*];
                let docs: &[&str] = &[#(#const_docs),*];

                for i in 0..names.len() {
                    let full_name = #name_format;
                    // Use original literal if available, otherwise use computed value.
                    let value_str = match literal_values[i] {
                        Some(lit) => String::from(lit),
                        None => format!("{}", computed_values[i]),
                    };
                    let inline = format!(".equ {}, {} # {}", full_name, value_str, docs[i]);
                    if inline.len() <= #max_line_len {
                        result.push_str(&inline);
                    } else {
                        result.push_str(&format!("# {}\n.equ {}, {}", docs[i], full_name, value_str));
                    }
                    result.push('\n');
                }

                result
            }
        }
    };

    TokenStream::from(expanded)
}

/// Input for sizes! macro.
struct SizesInput {
    types: Vec<syn::Type>,
}

impl Parse for SizesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut types = Vec::new();
        while !input.is_empty() {
            let ty: syn::Type = input.parse()?;
            types.push(ty);
            // Optional trailing comma.
            let _ = input.parse::<Token![,]>();
        }
        Ok(SizesInput { types })
    }
}

/// Macro for generating type size constants for ASM.
///
/// Takes a list of types and generates `SIZE_OF_<UPPER_SNAKE_NAME>` constants.
/// Creates a `sizes` module with a `to_asm()` function for build-time ASM generation.
///
/// # Example
/// ```ignore
/// sizes! {
///     u8,
///     SolAccountMeta,
///     Rent,
/// }
/// ```
///
/// Generates ASM:
/// ```text
/// # Type sizes.
/// # -----------
/// .equ SIZE_OF_U8, 1 # Size of u8.
/// .equ SIZE_OF_SOL_ACCOUNT_META, 24 # Size of SolAccountMeta.
/// .equ SIZE_OF_RENT, 17 # Size of Rent.
/// ```
#[proc_macro]
pub fn sizes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SizesInput);

    let max_line_len = MAX_LINE_LEN;
    let header = asm_header("Type sizes.");

    let mut const_defs = Vec::new();
    let mut const_name_strs = Vec::new();
    let mut const_doc_strs = Vec::new();

    for ty in &input.types {
        let type_name =
            extract_type_name(ty).unwrap_or_else(|| panic!("Expected a named type path in sizes!"));
        let screaming = type_name_to_upper_snake(&type_name);
        let name_str = format!("SIZE_OF_{}", screaming);
        let name = Ident::new(&name_str, proc_macro2::Span::call_site());
        let assert_name = Ident::new(
            &format!("_ASSERT_{}_FITS", name_str),
            proc_macro2::Span::call_site(),
        );
        let doc = format!("Size of {}.", type_name);

        const_defs.push(quote! {
            #[doc = #doc]
            pub const #name: i64 = { use super::*; core::mem::size_of::<#ty>() as i64 };

            const #assert_name: () = assert!(
                ({ use super::*; core::mem::size_of::<#ty>() } as i64) >= (i32::MIN as i64)
                    && ({ use super::*; core::mem::size_of::<#ty>() } as i64) <= (i32::MAX as i64),
                "ASM immediate must fit in i32 range"
            );
        });

        const_name_strs.push(name_str);
        const_doc_strs.push(doc);
    }

    let const_idents: Vec<_> = const_name_strs
        .iter()
        .map(|n| Ident::new(n, proc_macro2::Span::call_site()))
        .collect();

    let expanded = quote! {
        pub mod sizes {
            use alloc::string::String;
            use alloc::format;

            #(#const_defs)*

            /// Generate ASM constants for type sizes.
            pub fn to_asm() -> String {
                let mut result = String::from(#header);
                result.push('\n');

                let names: &[&str] = &[#(#const_name_strs),*];
                let values: &[i64] = &[#(#const_idents as i64),*];
                let docs: &[&str] = &[#(#const_doc_strs),*];

                for i in 0..names.len() {
                    let inline = format!(".equ {}, {} # {}", names[i], values[i], docs[i]);
                    if inline.len() <= #max_line_len {
                        result.push_str(&inline);
                    } else {
                        result.push_str(&format!("# {}\n.equ {}, {}", docs[i], names[i], values[i]));
                    }
                    result.push('\n');
                }

                result
            }
        }
    };

    TokenStream::from(expanded)
}

/// Macro for generating pubkey chunking offset constants.
///
/// A pubkey (Address) is 32 bytes. For 8-byte register loads, it is
/// accessed in 4 chunks of 8 bytes each. This macro generates a
/// `pubkey_chunk` module with `OFF_0` through `OFF_3` constants
/// and a `to_asm()` function.
///
/// # Example
/// ```ignore
/// pubkey_chunk_group!();
/// ```
///
/// Generates ASM:
/// ```text
/// # Pubkey chunking offsets.
/// # ------------------------
/// .equ PUBKEY_CHUNK_OFF_0, 0 # Offset for the first 8 bytes.
/// .equ PUBKEY_CHUNK_OFF_1, 8 # Offset for the second 8 bytes.
/// .equ PUBKEY_CHUNK_OFF_2, 16 # Offset for the third 8 bytes.
/// .equ PUBKEY_CHUNK_OFF_3, 24 # Offset for the fourth 8 bytes.
/// ```
#[proc_macro]
pub fn pubkey_chunk_group(_input: TokenStream) -> TokenStream {
    const PUBKEY_SIZE: usize = 32;
    const CHUNK_SIZE: usize = BPF_ALIGN as usize;
    const N_CHUNKS: usize = PUBKEY_SIZE / CHUNK_SIZE;
    const ORDINALS: [&str; N_CHUNKS] = ["first", "second", "third", "fourth"];

    let max_line_len = MAX_LINE_LEN;
    let header = asm_header("Pubkey chunking offsets.");

    let mut const_defs = Vec::new();
    let mut const_name_strs = Vec::new();
    let mut const_doc_strs = Vec::new();

    for (i, ordinal) in ORDINALS.iter().enumerate() {
        let offset = (i * CHUNK_SIZE) as i64;
        let name_str = format!("PUBKEY_CHUNK_OFF_{}", i);
        let name = Ident::new(&format!("OFF_{}", i), proc_macro2::Span::call_site());
        let doc = format!("Offset for the {} 8 bytes.", ordinal);

        const_defs.push(quote! {
            #[doc = #doc]
            pub const #name: i64 = #offset;
        });

        const_name_strs.push(name_str);
        const_doc_strs.push(doc);
    }

    let const_idents: Vec<_> = (0..N_CHUNKS)
        .map(|i| Ident::new(&format!("OFF_{}", i), proc_macro2::Span::call_site()))
        .collect();

    let expanded = quote! {
        pub mod pubkey_chunk {
            use alloc::string::String;
            use alloc::format;

            #(#const_defs)*

            /// Generate ASM constants for pubkey chunking offsets.
            pub fn to_asm() -> String {
                let mut result = String::from(#header);
                result.push('\n');

                let names: &[&str] = &[#(#const_name_strs),*];
                let values: &[i64] = &[#(#const_idents as i64),*];
                let docs: &[&str] = &[#(#const_doc_strs),*];

                for i in 0..names.len() {
                    let inline = format!(".equ {}, {} # {}", names[i], values[i], docs[i]);
                    if inline.len() <= #max_line_len {
                        result.push_str(&inline);
                    } else {
                        result.push_str(&format!("# {}\n.equ {}, {}", docs[i], names[i], values[i]));
                    }
                    result.push('\n');
                }

                result
            }
        }
    };

    TokenStream::from(expanded)
}

/// Attribute macro for stack frame structs.
///
/// Adds `#[repr(C, align(8))]` to ensure C layout and 8-byte alignment.
/// Any existing `#[repr(...)]` attributes are removed.
///
/// Also generates a companion module `__<StructName>_fields` containing type aliases
/// for each array field's element type. This allows `stack_frame_offset!` to resolve
/// element types without requiring them in the syntax.
///
/// # Example
/// ```ignore
/// #[stack_frame]
/// struct InitStackFrame {
///     data: [u8; 32],
///     metas: [SolAccountMeta; 2],
/// }
/// ```
///
/// Generates:
/// ```ignore
/// #[repr(C, align(8))]
/// struct InitStackFrame { ... }
///
/// mod __InitStackFrame_fields {
///     use super::*;
///     pub type metas = SolAccountMeta;
///     pub type data = u8;
/// }
/// ```
#[proc_macro_attribute]
pub fn stack_frame(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemStruct);

    // Remove existing repr attributes to avoid conflicts.
    input.attrs.retain(|attr| !attr.path().is_ident("repr"));

    let struct_name = &input.ident;
    let fields_mod = Ident::new(&format!("__{}_fields", struct_name), struct_name.span());

    // Extract array element types for each array field.
    let mut type_aliases = Vec::new();
    if let syn::Fields::Named(ref fields) = input.fields {
        for field in &fields.named {
            if let Some(ref field_name) = field.ident {
                if let syn::Type::Array(array_type) = &field.ty {
                    let elem_type = &*array_type.elem;
                    type_aliases.push(quote! {
                        #[allow(non_camel_case_types)]
                        pub type #field_name = #elem_type;
                    });
                }
            }
        }
    }

    let expanded = quote! {
        #[repr(C, align(8))]
        #input

        #[doc(hidden)]
        #[allow(non_snake_case)]
        mod #fields_mod {
            use super::*;
            #(#type_aliases)*
        }
    };

    TokenStream::from(expanded)
}
