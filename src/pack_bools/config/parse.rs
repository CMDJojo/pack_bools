use syn::parse::{Parse, ParseStream};
use syn::Token;

use crate::pack_bools::config::Visibility;

impl Parse for Visibility {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::Visibility as SV;

        let lh = input.lookahead1();
        if lh.peek(Token![self]) {
            let _: Token![self] = input.parse()?;
            return Ok(Visibility::Inherit);
        }

        let visibility = SV::parse(input)?;
        let vis = match visibility {
            SV::Public(_) => Visibility::Public,
            SV::Restricted(vis_res) => Visibility::Restricted(vis_res),
            SV::Inherited => Visibility::Private,
        };
        Ok(vis)
    }
}
