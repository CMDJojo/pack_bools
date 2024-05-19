use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Visibility as Vis;
use syn::VisRestricted;

pub use global::Config as GlobalConfig;
pub use global::GenType;
pub use local::Config as LocalConfig;

use crate::pack_bools::config::local::Accessor;

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

    pub fn getter(&self, field_name: &str, inh: &Vis) -> Option<TokenStream> {
        let (vis, ident) = match &self.local.getter {
            Accessor::Custom(custom) => {
                let (vis, ident) = custom.get_parts();
                let ident = if let Some(ident) = ident {
                    ident.clone()
                } else {
                    let s = self.global.getter.template.format(field_name);
                    format_ident!("{s}")
                };
                (vis, ident)
            }
            Accessor::Default => {
                // We use the global config, check if we should generate first
                if self.global.skip_getter {
                    return None;
                }
                self.global.getter.get_formatted_parts(field_name)
            }
            Accessor::Skip => {
                return None;
            }
        };
        let vis = vis.to_visibility(inh);
        Some(quote! {#vis fn #ident})
    }

    pub fn setter(&self, field_name: &str, inh: &Vis) -> Option<TokenStream> {
        let (vis, ident) = match &self.local.setter {
            Accessor::Custom(custom) => {
                let (vis, ident) = custom.get_parts();
                let ident = if let Some(ident) = ident {
                    ident.clone()
                } else {
                    let s = self.global.setter.template.format(field_name);
                    format_ident!("{s}")
                };
                (vis, ident)
            }
            Accessor::Default => {
                // We use the global config, check if we should generate first
                if self.global.skip_setter {
                    return None;
                }
                self.global.setter.get_formatted_parts(field_name)
            }
            Accessor::Skip => {
                return None;
            }
        };
        let vis = vis.to_visibility(inh);
        Some(quote! {#vis fn #ident})
    }
}
