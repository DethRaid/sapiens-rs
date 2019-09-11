use quote::quote;

pub fn generate_get_emitter_type_count_func(func: syn::ItemFn) -> proc_macro::TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetEmitterTypesCount() -> ::std::os::raw::c_int {
        get_emitter_types_count()
    }

    #func
    };

    ast.into()
}

pub fn generate_get_emitter_types(func: syn::ItemFn) -> proc_macro::TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetEmitterTypes() -> *mut SPParticleEmitterTypeInfo {
        let mut emitters = get_emitter_types();

        emitters.as_mut_ptr()
    }

    #func
    };

    ast.into()
}
