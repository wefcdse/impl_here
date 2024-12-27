use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
mod implment;
use implment::*;

/// This macro converts a impl block to a trait and impl it for the foreign type.
/// # Example
/// ```
/// use impl_here::impl_here;
///
/// #[impl_here(ArrayTrait)]
/// impl<T, const L:usize> [T;L]{
///     const TOTAL_SIZE: usize = size_of::<Self>();
///     fn length(&self) -> usize{
///         L
///     }
/// }
/// assert_eq!([0.3; 125].length(), 125);
/// assert_eq!(<[String; 2024]>::TOTAL_SIZE, 2024 * 24);
///
/// #[impl_here(I32Square)]
/// impl i32 {
///     pub fn square(self) -> i32 {
///         self * self
///     }
/// }
/// // I32Square is public because fn square is public
/// assert_eq!(13.square(), 13 * 13);
///
/// ```
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
