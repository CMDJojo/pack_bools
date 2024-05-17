use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;

use crate::pack_bools::config::local::{Config, VisibilityIdent};
use crate::pack_bools::config::local::modify::Modifier;

impl Parse for VisibilityIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = input.parse()?;
        let ident = input.parse()?;
        Ok(VisibilityIdent { visibility, ident })
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
