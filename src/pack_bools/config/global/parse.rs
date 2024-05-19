use syn::{Error, Ident, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use crate::pack_bools::config::global::{
    Config, FieldName, GenType, PackedType, PackingStrategy, Template, UpdateVisibilityTemplate,
    VisibilityTemplate,
};
use crate::pack_bools::config::global::modify::Modifier;
use crate::pack_bools::config::Visibility;

impl Parse for Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let res = if lookahead.peek(Token![%]) {
            // Before is empty
            let _: Token![%] = input.parse()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let after: Ident = input.parse()?;
                Template::new(String::new(), after.to_string())
            } else {
                Template::from_str("", "")
            }
        } else if lookahead.peek(syn::Ident) {
            let before: Ident = input.parse()?;
            let _: Token![%] = input.parse()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let after: Ident = input.parse()?;
                Template::new(before.to_string(), after.to_string())
            } else {
                Template::new(before.to_string(), String::new())
            }
        } else {
            return Err(Error::new(
                input.span(),
                "Templates must be valid identifiers with % to substitute the name",
            ));
        };

        Ok(res)
    }
}

impl Parse for PackingStrategy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let token: Ident = input.parse()?;
        let ty = match token.to_string().as_str() {
            "u8" => PackedType::U8,
            "u16" => PackedType::U16,
            "u32" => PackedType::U32,
            "u64" => PackedType::U64,
            "u128" => PackedType::U128,
            "auto" => return Ok(PackingStrategy::Auto),
            _ => {
                return Err(Error::new(
                    token.span(),
                    "Type must be auto, u8, u16, u32, u64 or u128",
                ))
            }
        };
        Ok(PackingStrategy::FixedType(ty))
    }
}

impl Parse for VisibilityTemplate {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        let template: Template = input.parse()?;
        Ok(VisibilityTemplate {
            visibility,
            template,
        })
    }
}

impl Parse for UpdateVisibilityTemplate {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        let template: Option<Template> = input.parse().ok();
        Ok(UpdateVisibilityTemplate {
            visibility,
            template,
        })
    }
}

impl Parse for FieldName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        Ok(FieldName(ident.to_string()))
    }
}

impl Parse for GenType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let res = match ident.to_string().as_str() {
            "inline" => GenType::Inline,
            "newtype" => {
                let lookahead = input.lookahead1();
                if lookahead.peek(Token![=]) {
                    let _: Token![=] = input.parse()?;
                    let ident: Ident = input.parse()?;
                    GenType::NewType(Some(ident.to_string()))
                } else {
                    GenType::NewType(None)
                }
            }
            _ => {
                return Err(Error::new(
                    ident.span(),
                    "Only 'inline'/'newtype' as generator types",
                ));
            }
        };
        Ok(res)
    }
}

impl Parse for Config {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let modifiers = Punctuated::<Modifier, Token![,]>::parse_terminated(input)?;
        let mut default = Config::default();
        for modifier in modifiers {
            modifier.modify(&mut default);
        }
        Ok(default)
    }
}
