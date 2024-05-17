use syn::parse::{Parse, ParseStream};

use crate::pack_bools::config::Visibility;

impl Parse for Visibility {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::Visibility as SV;
        let visibility = SV::parse(input)?;
        let vis = match visibility {
            SV::Public(_) => Visibility::Public,
            SV::Restricted(vis_res) => Visibility::Restricted(vis_res),
            SV::Inherited => Visibility::Private,
        };
        Ok(vis)
    }
}
