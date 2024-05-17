use proc_macro2::Ident;
use syn::{Error, Token};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};

use crate::pack_bools::config::local::{Accessor, Config, VisibilityIdent};

#[derive(Debug)]
pub enum Modifier {
    Skip,
    Getter(VisibilityIdent),
    NoGetter,
    Setter(VisibilityIdent),
    NoSetter,
    SetDefault(bool),
}

impl Modifier {
    pub fn modify(self, target: &mut Config) {
        match self {
            Modifier::Skip => target.skip = true,
            Modifier::Getter(g) => target.getter = Accessor::Custom(g),
            Modifier::NoGetter => target.getter = Accessor::None,
            Modifier::Setter(s) => target.setter = Accessor::Custom(s),
            Modifier::NoSetter => target.setter = Accessor::None,
            Modifier::SetDefault(v) => target.default = v,
        }
    }
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let error_msg = "Valid local attributes: \
            'skip', \
            'get'/'getter', \
            'no_get'/'no_getter', \
            'set'/'setter', \
            'no_set'/'no_setter', \
            'default'";
        let ident: Ident = Ident::parse_any(input).map_err(|e| Error::new(e.span(), error_msg))?;
        let ret = match ident.to_string().as_str() {
            "getter" | "get" => {
                let _: Token![=] = input.parse()?;
                let at = input.parse()?;
                Modifier::Getter(at)
            }
            "setter" | "set" => {
                let _: Token![=] = input.parse()?;
                let at = input.parse()?;
                Modifier::Setter(at)
            }
            "no_get" | "no_getter" => Modifier::NoGetter,
            "no_set" | "no_setter" => Modifier::NoSetter,
            "skip" => Modifier::Skip,
            "default" => {
                let _: Token![=] = input.parse()?;
                let ident = Ident::parse_any(input)?;
                let def = match ident.to_string().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            "Expected true/false as default values",
                        ))
                    }
                };
                Modifier::SetDefault(def)
            }
            _ => return Err(Error::new(ident.span(), error_msg)),
        };

        Ok(ret)
    }
}
