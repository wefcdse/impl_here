use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
mod implment;
use implment::*;

#[proc_macro_attribute]
pub fn impl_here(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let a = || -> Result<proc_macro::TokenStream, String> {
        let input: TokenStream = input.into();
        let d = syn::parse2(input.clone()).map_err(|e| e.to_string())?;
        let out: TokenStream = expend(args.into(), d)?.into_token_stream();
        Ok(quote!(
            #out
        )
        .into())
    };
    match a() {
        Ok(v) => v,
        // Err(e) => quote! {compile_error!(#e);}.into(),
        Err(e) => msg(&e).into(),
    }
}

fn msg(msg: &str) -> TokenStream {
    quote! {
        const _: () = {
            #msg;
        };
    }
}
