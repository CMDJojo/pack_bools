use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility as Vis;
use syn::VisRestricted;

pub use global::Config as GlobalConfig;
pub use global::GenType;
pub use local::Config as LocalConfig;

use crate::pack_bools::config::global::VisibilityTemplate;
use crate::pack_bools::config::local::{Accessor, VisibilityIdent};

mod global;
mod local;
pub mod parse;

#[derive(Debug)]
pub enum Visibility {
    Inherit,
    Public,
    Private,
    Restricted(VisRestricted),
}

impl Visibility {
    pub fn to_visibility(&self, inherited: &Vis) -> TokenStream {
        match self {
            Visibility::Inherit => quote! {
                #inherited
            },
            Visibility::Public => quote! {
                pub
            },
            Visibility::Private => TokenStream::new(),
            Visibility::Restricted(res) => quote! {
                #res
            },
        }
    }
}

pub struct CombinedConfig<'a> {
    global: &'a GlobalConfig,
    local: &'a LocalConfig,
}

impl<'a> CombinedConfig<'a> {
    pub fn new(global: &'a GlobalConfig, local: &'a LocalConfig) -> Self {
        Self { global, local }
    }

    pub fn setter(&self, ident: &str, vis: &Vis) -> Option<TokenStream> {
        match &self.local.setter {
            Accessor::None => None,
            Accessor::Custom(custom) => {
                let VisibilityIdent { visibility, ident } = custom;
                let vis = visibility.to_visibility(vis);
                Some(quote! {#vis fn #ident})
            }
            Accessor::Default => match &self.global.setter {
                None => None,
                Some(template) => {
                    let VisibilityTemplate {
                        visibility,
                        template,
                    } = template;
                    let vis = visibility.to_visibility(vis);
                    let name: TokenStream = template.format(ident).parse().unwrap();
                    Some(quote! {#vis fn #name})
                }
            },
        }
    }

    pub fn getter(&self, ident: &str, vis: &Vis) -> Option<TokenStream> {
        match &self.local.getter {
            Accessor::None => None,
            Accessor::Custom(custom) => {
                let VisibilityIdent { visibility, ident } = custom;
                let vis = visibility.to_visibility(vis);
                Some(quote! {#vis fn #ident})
            }
            Accessor::Default => match &self.global.getter {
                None => None,
                Some(template) => {
                    let VisibilityTemplate {
                        visibility,
                        template,
                    } = template;
                    let vis = visibility.to_visibility(vis);
                    let name: TokenStream = template.format(ident).parse().unwrap();
                    Some(quote! {#vis fn #name})
                }
            },
        }
    }
}
