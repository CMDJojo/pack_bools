use proc_macro2::Ident;
use syn::{Error, Token};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};

use crate::pack_bools::config::global::{
    Config, FieldName, GenType, PackingStrategy, UpdateVisibilityTemplate,
};

pub enum Modifier {
    Getters(UpdateVisibilityTemplate),
    Setters(UpdateVisibilityTemplate),
    NoGetters,
    NoSetters,
    Type(PackingStrategy),
    GenType(GenType),
    Field(FieldName),
}

impl Modifier {
    pub fn modify(self, target: &mut Config) {
        match self {
            Modifier::Getters(g) => g.update(&mut target.getter),
            Modifier::NoGetters => target.skip_getter = true,
            Modifier::Setters(s) => s.update(&mut target.setter),
            Modifier::NoSetters => target.skip_setter = true,
            Modifier::Type(t) => target.packed_type = t,
            Modifier::GenType(gt) => target.gen_type = gt,
            Modifier::Field(f) => target.field_name = f,
        }
    }
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let error_msg = "Valid global attributes: \
            'get'/'getter'/'getters', \
            'no_get'/'no_getter'/'no_getters', \
            'set'/'setter'/'setters', \
            'no_set'/'no_setter'/'no_setters', \
            'type', 'inline', 'newtype', 'field'";
        let ident: Ident = Ident::parse_any(input).map_err(|e| Error::new(e.span(), error_msg))?;
        let ret = match ident.to_string().as_str() {
            "getters" | "getter" | "get" => {
                let _: Token![=] = input.parse()?;
                let at = input.parse()?;
                Modifier::Getters(at)
            }
            "setters" | "setter" | "set" => {
                let _: Token![=] = input.parse()?;
                let at = input.parse()?;
                Modifier::Setters(at)
            }
            "no_get" | "no_getter" | "no_getters" => Modifier::NoGetters,
            "no_set" | "no_setter" | "no_setters" => Modifier::NoSetters,
            "type" => {
                let _: Token![=] = input.parse()?;
                let pt: PackingStrategy = input.parse()?;
                Modifier::Type(pt)
            }
            "inline" => Modifier::GenType(GenType::Inline),
            "newtype" => {
                let lookahead = input.lookahead1();
                if lookahead.peek(Token![=]) {
                    let _: Token![=] = input.parse()?;
                    let ident: Ident = input.parse()?;
                    Modifier::GenType(GenType::NewType(Some(ident.to_string())))
                } else {
                    Modifier::GenType(GenType::NewType(None))
                }
            }
            "field" => {
                let _: Token![=] = input.parse()?;
                let f: FieldName = input.parse()?;
                Modifier::Field(f)
            }
            _ => return Err(Error::new(ident.span(), error_msg)),
        };

        Ok(ret)
    }
}
