use proc_macro2::Ident;

use crate::pack_bools::config::Visibility;

mod modify;
mod parse;

#[derive(Debug)]
pub struct Config {
    pub getter: Accessor,
    pub setter: Accessor,
    pub skip: bool,
    pub default: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            getter: Accessor::Default,
            setter: Accessor::Default,
            skip: false,
            default: false,
        }
    }
}

#[derive(Debug, Default)]
pub enum Accessor {
    #[default]
    Default,
    Custom(VisibilityIdent),
    None,
}

#[derive(Debug)]
pub struct VisibilityIdent {
    pub visibility: Visibility,
    pub ident: Ident,
}
