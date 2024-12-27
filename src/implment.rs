use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::{
    punctuated::Punctuated, Generics, ImplItem, ItemImpl, ItemTrait, TraitItem, TraitItemFn,
    Visibility,
};

pub fn expend(args: TokenStream, input: ItemImpl) -> Result<TokenStream, String> {
    let trait_name = match args.into_iter().next().unwrap() {
        TokenTree::Ident(i) => i,
        _ => Err("need trait name")?,
    };

    let target = input.self_ty;
    let generics = input.generics;
    let impl_items = input.items;
    let trait_fns = impl_items
        .iter()
        .cloned()
        .filter_map(|v| match v {
            ImplItem::Fn(v) => Some(TraitItemFn {
                attrs: v.attrs,
                sig: v.sig,
                default: None,
                semi_token: None,
            }),
            _ => None,
        })
        .map(|e| TraitItem::Fn(e))
        .collect::<Vec<_>>();
    let mut vis = Visibility::Inherited;

    let impl_items = impl_items
        .iter()
        .cloned()
        .filter_map(|v| match v {
            ImplItem::Fn(mut v) => {
                vis = high_vis(vis.clone(), v.vis);
                v.vis = Visibility::Inherited;
                Some(ImplItem::Fn(v))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let trait_ = ItemTrait {
        attrs: Vec::new(),
        vis: vis.clone(),
        unsafety: None,
        auto_token: None,
        restriction: None,
        trait_token: syn::parse2(quote! {trait}).unwrap(),
        ident: trait_name.clone(),
        generics: Generics::default(),
        colon_token: None,
        supertraits: Punctuated::new(),
        brace_token: input.brace_token.clone(),
        items: trait_fns.clone(),
    };
    let trait_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: input.impl_token.clone(),
        generics: generics.clone(),
        trait_: Some((
            None,
            syn::Path::from(trait_name.clone()),
            syn::parse2(quote! {for}).unwrap(),
        )),
        self_ty: target.clone(),
        brace_token: input.brace_token.clone(),
        items: impl_items.clone(),
    };

    Ok(quote! {
        // const _:() = {
        //     stringify!(#(#trait_fns)*);
        // };
        const _:() = {
            ::core::stringify!(#trait_impl);
        };
        // #vis trait #trait_name{
        //     #(#trait_fns)*
        // }
        #trait_

        // impl #generics #trait_name for #target{
        //     #(#impl_items)*
        // }
        #trait_impl
    })
}

fn high_vis(v1: Visibility, v2: Visibility) -> Visibility {
    match (v1, v2) {
        (Visibility::Public(p), _) => Visibility::Public(p),
        (_, Visibility::Public(p)) => Visibility::Public(p),
        (Visibility::Restricted(vis_restricted), _) => Visibility::Restricted(vis_restricted),
        (_, Visibility::Restricted(vis_restricted)) => Visibility::Restricted(vis_restricted),
        (Visibility::Inherited, Visibility::Inherited) => Visibility::Inherited,
    }
}
