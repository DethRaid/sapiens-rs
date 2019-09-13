use quote::quote;
use std::iter::Extend;

pub fn generate_get_emitter_type_count_func(
    attr: proc_macro::TokenStream,
    func: syn::ItemFn,
) -> proc_macro::TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetEmitterTypesCount() -> ::std::os::raw::c_int {
        get_emitter_types_count()
    }

    #func
    };

    let mut output = attr.clone();
    output.extend(ast);

    output
}

pub fn generate_get_emitter_types(
    attr: proc_macro::TokenStream,
    func: syn::ItemFn,
) -> proc_macro::TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetEmitterTypes() -> *mut SPParticleEmitterTypeInfo {
        let mut emitters = get_emitter_types();

        emitters.as_mut_ptr()
    }

    #func
    };

    let mut output = attr.clone();
    output.extend(ast.into());

    output
}

pub fn generate_get_render_group_types_count(
    attr: proc_macro::TokenStream,
    func: syn::ItemFn,
) -> proc_macro::TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetRenderGroupTypesCount() -> ::std::os::raw::c_int {
        get_render_group_types_count() as ::std::os::raw::c_int
    }

    #func
    };

    let mut output = attr.clone();
    output.extend(ast.into());

    output
}

pub fn generate_get_render_group_types(
    attr: proc_macro::TokenStream,
    func: syn::ItemFn,
) -> proc_macro::TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetRenderGroupTypes() -> *mut SPRenderGroupTypeInfo {
        let mut render_group_types = get_render_group_types();

        render_group_types.as_mut_ptr()
    }

    #func
    };

    let mut output = attr.clone();
    output.extend(ast.into());

    output
}
