use std::iter;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Error, Field, ItemStruct, Meta, parse2, Type};
use syn::spanned::Spanned;

use crate::pack_bools::config::{CombinedConfig, GenType, GlobalConfig, LocalConfig};

pub mod config;

macro_rules! try_syn {
    ($e:expr) => {
        match $e {
            Err(err) => return err.to_compile_error(),
            Ok(ok) => ok,
        }
    };
}

#[allow(clippy::too_many_lines)]
pub fn pack_bools(config: GlobalConfig, definition: ItemStruct) -> TokenStream {
    let ItemStruct {
        attrs,
        vis,
        struct_token,
        ident,
        generics,
        fields,
        semi_token: _,
    } = definition;
    let modified: Result<Vec<_>, _> = fields.into_iter().map(ModifiedField::from_field).collect();
    let modified = try_syn!(modified);

    let (to_convert, to_keep): (Vec<_>, Vec<_>) =
        modified.into_iter().partition(ModifiedField::is_packed);

    let to_keep: Option<Vec<_>> = to_keep
        .into_iter()
        .map(ModifiedField::into_excluded)
        .collect();

    let to_keep = to_keep.unwrap();
    let new_field: TokenStream = config.field_name.0.parse().unwrap();
    let inner_type = try_syn!(config
        .packed_type
        .to_packed_type(to_convert.len(), ident.span()));
    let (packed_type, newtype) = match &config.gen_type {
        GenType::Inline => (inner_type.to_token_stream(), None),
        GenType::NewType(name) => {
            let newtype_name = if let Some(name) = name {
                name.parse().unwrap()
            } else {
                format_ident!("{ident}PackedBools").to_token_stream()
            };
            let typedef = quote! {
                #[derive(Copy, Clone, Debug)]
                #[repr(transparent)]
                struct #newtype_name (#inner_type);
            };
            (newtype_name, Some(typedef))
        }
    };

    let packed_path = if config.gen_type.is_inline() {
        new_field.clone()
    } else {
        quote! { #new_field.0 }
    };

    let mut default_bits = 0u128;
    let mut functions = vec![];
    for (idx, (field, local)) in to_convert
        .into_iter()
        .map(|f| {
            let (a, b) = f.into_packaged().unwrap();
            (a, b.unwrap_or_default())
        })
        .enumerate()
    {
        if local.default {
            if config.gen_type.is_inline() {
                return Error::new(field.span(), "#[pack_bools(default = true)] only available on `newtype` generation: use #[pack_bools(newtype)] on the struct").to_compile_error();
            }
            default_bits |= 1 << idx;
        }

        let idx = idx.to_token_stream();
        let combined = CombinedConfig::new(&config, &local);
        if let Some(getter) = combined.getter(
            field.ident.as_ref().unwrap().to_string().as_str(),
            &field.vis,
        ) {
            let getter = quote! {
                #getter (&self) -> bool {
                    self. #packed_path & 1 << #idx != 0
                }
            };
            functions.push(getter);
        }

        if let Some(setter) = combined.setter(
            field.ident.as_ref().unwrap().to_string().as_str(),
            &field.vis,
        ) {
            let setter = quote! {
                #setter (&mut self, value: bool) {
                    let val = self. #packed_path;
                    self.#packed_path = if value {
                        val | 1 << #idx
                    } else {
                        val & !(1 << #idx)
                    };
                }
            };
            functions.push(setter);
        }
    }

    let default_impl = config.gen_type.is_newtype().then(|| {
        quote! {
            impl ::std::default::Default for #packed_type {
                fn default() -> Self {
                    Self( #default_bits as #inner_type )
                }
            }
        }
    });

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let fields = to_keep
        .into_iter()
        .map(Field::into_token_stream)
        .chain(iter::once(quote! { #new_field: #packed_type }));

    let output = quote! {
        #(#attrs)* #vis #struct_token #ident #generics {
            #(#fields),*
        }

        #newtype

        #default_impl

        impl #impl_generics #ident #type_generics #where_clause {
            #(#functions)*
        }
    };

    output
}

pub enum ModifiedField {
    Excluded(Field),
    PackedField(Field, Option<LocalConfig>),
}

impl ModifiedField {
    pub fn is_packed(&self) -> bool {
        matches!(self, ModifiedField::PackedField(_, _))
    }

    pub fn into_excluded(self) -> Option<Field> {
        match self {
            ModifiedField::Excluded(e) => Some(e),
            ModifiedField::PackedField(_, _) => None,
        }
    }

    pub fn into_packaged(self) -> Option<(Field, Option<LocalConfig>)> {
        match self {
            ModifiedField::PackedField(f, c) => Some((f, c)),
            ModifiedField::Excluded(_) => None,
        }
    }
}

impl ModifiedField {
    pub fn from_field(field: Field) -> syn::Result<Self> {
        let mut config: Option<LocalConfig> = None;
        let Field {
            attrs,
            vis,
            mutability,
            ident,
            colon_token,
            ty,
        } = field;

        let mut new_attributes = Vec::with_capacity(attrs.len());
        for attr in attrs {
            if attr.path().is_ident("pack_bools") {
                // Remove this attribute
                if !is_bool_type(&ty) {
                    return Err(Error::new(
                        attr.span(),
                        "#[pack_bools] can only be used on bools",
                    ));
                } else if config.is_some() {
                    return Err(Error::new(
                        attr.span(),
                        "At most one #[pack_bools] attribute allowed per field",
                    ));
                }
                match attr.meta {
                    Meta::List(list) => {
                        let pc = parse2(list.tokens)?;
                        config = Some(pc);
                    }
                    _ => {
                        return Err(Error::new(
                            attr.span(),
                            "Use #[pack_bools(...)] for configuring fields",
                        ))
                    }
                }
            } else {
                // Keep other attributes
                new_attributes.push(attr);
            }
        }

        let is_bool = is_bool_type(&ty);
        let field = Field {
            attrs: new_attributes,
            vis,
            mutability,
            ident,
            colon_token,
            ty,
        };

        let res = if let Some(config) = config {
            if config.skip {
                Self::Excluded(field)
            } else {
                Self::PackedField(field, Some(config))
            }
        } else if is_bool {
            Self::PackedField(field, None)
        } else {
            Self::Excluded(field)
        };
        Ok(res)
    }
}

fn is_bool_type(ty: &Type) -> bool {
    if let Type::Path(p) = ty {
        p.path.is_ident("bool")
    } else {
        false
    }
}
