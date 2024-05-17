use std::fmt::Display;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{TokenStreamExt, ToTokens};
use syn::{Error, parse_quote};

use crate::pack_bools::config::Visibility;

mod modify;
mod parse;

#[derive(Debug)]
pub struct Config {
    pub getter: Option<VisibilityTemplate>,
    pub setter: Option<VisibilityTemplate>,
    pub packed_type: PackingStrategy,
    pub field_name: FieldName,
    pub gen_type: GenType,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct FieldName(pub String);

#[derive(Debug, PartialEq, Eq)]
pub enum GenType {
    Inline,
    NewType(Option<String>),
}

impl GenType {
    pub fn is_inline(&self) -> bool {
        self == &GenType::Inline
    }

    pub fn is_newtype(&self) -> bool {
        !self.is_inline()
    }
}

#[derive(Debug)]
pub enum PackingStrategy {
    Auto,
    FixedType(PackedType),
}

impl PackingStrategy {
    pub fn to_packed_type(&self, length: usize, span: Span) -> syn::Result<PackedType> {
        match self {
            PackingStrategy::Auto => PackedType::smallest_larger_than(length).ok_or_else(|| {
                Error::new(
                    span,
                    "This struct contains more than 128 bools and doesn't fit in a u128",
                )
            }),
            PackingStrategy::FixedType(t) => {
                let t = *t;
                if (t.bit_width() as usize) < length {
                    Err(Error::new(span, "This struct contains more bools than would fit in your specified bit width"))
                } else {
                    Ok(t)
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PackedType {
    U8,
    U16,
    U32,
    U64,
    U128,
}

static PACKED_TYPE_VARIANTS: &[PackedType] = &[
    PackedType::U8,
    PackedType::U16,
    PackedType::U32,
    PackedType::U64,
    PackedType::U128,
];

impl PackedType {
    pub fn bit_width(self) -> u8 {
        match self {
            PackedType::U8 => 8,
            PackedType::U16 => 16,
            PackedType::U32 => 32,
            PackedType::U64 => 64,
            PackedType::U128 => 128,
        }
    }

    pub fn smallest_larger_than(length: usize) -> Option<Self> {
        PACKED_TYPE_VARIANTS
            .iter()
            .find(|variant| variant.bit_width() as usize >= length)
            .copied()
    }
}

impl ToTokens for PackedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PackedType::U8 => tokens.append::<Ident>(parse_quote! {u8}),
            PackedType::U16 => tokens.append::<Ident>(parse_quote! {u16}),
            PackedType::U32 => tokens.append::<Ident>(parse_quote! {u32}),
            PackedType::U64 => tokens.append::<Ident>(parse_quote! {u64}),
            PackedType::U128 => tokens.append::<Ident>(parse_quote! {u128}),
        }
    }
}

#[derive(Debug)]
pub struct Template {
    pub before: String,
    pub after: String,
}

impl Template {
    pub fn new(before: String, after: String) -> Self {
        Self { before, after }
    }

    pub fn from_str(before: &str, after: &str) -> Self {
        Self::new(before.to_string(), after.to_string())
    }

    pub fn format<T: Display>(&self, item: T) -> String {
        let Self { before, after } = self;
        format!("{before}{item}{after}")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            getter: Some(VisibilityTemplate {
                template: Template::from_str("get_", ""),
                visibility: Visibility::Inherit,
            }),
            setter: Some(VisibilityTemplate {
                template: Template::from_str("set_", ""),
                visibility: Visibility::Inherit,
            }),
            packed_type: PackingStrategy::Auto,
            field_name: FieldName("packed_bools".to_string()),
            gen_type: GenType::Inline,
        }
    }
}

#[derive(Debug)]
pub struct VisibilityTemplate {
    pub visibility: Visibility,
    pub template: Template,
}
