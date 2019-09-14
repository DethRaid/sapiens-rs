use proc_macro::TokenStream;
use quote::quote;
use std::iter::Extend;

pub fn generate_get_emitter_type_count_func(func: syn::ItemFn) -> TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetEmitterTypesCount() -> ::std::os::raw::c_int {
        get_emitter_types_count()
    }

    #func
    };

    ast.into()
}

pub fn generate_get_emitter_types(func: syn::ItemFn) -> TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetEmitterTypes() -> *mut SPParticleEmitterTypeInfo {
        let mut emitters: Vec<SPParticleEmitterTypeInfo> = get_emitter_types()
            .iter()
            .map(|emitter_type| emitter_type.into())
            .collect();

        emitters.shrink_to_fit();
        let ptr = emitters.as_mut_ptr();
        ::std::mem::forget(emitters);

        ptr
    }

    #func
    };

    ast.into()
}

pub fn generate_get_render_group_types_count(func: syn::ItemFn) -> TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetRenderGroupTypesCount() -> ::std::os::raw::c_int {
        get_render_group_types_count() as ::std::os::raw::c_int
    }

    #func
    };

    ast.into()
}

pub fn generate_get_render_group_types(func: syn::ItemFn) -> TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetRenderGroupTypes() -> *mut SPParticleRenderGroupInfo {
        let mut render_group_types: Vec<SPParticleRenderGroupInfo> = get_render_group_types()
            .iter()
            .map(|emitter_type| emitter_type.into())
            .collect();

        render_group_types.shrink_to_fit();
        assert!(render_group_types.len() == render_group_types.capacity());
        let ptr = render_group_types.as_mut_ptr();
        ::std::mem::forget(render_group_types);

        ptr
    }

    #func
    };

    ast.into()
}

pub fn generate_emitter_was_added(func: syn::ItemFn) -> TokenStream {
    let ast = quote! {
    pub extern "C" fn spEmitterWasAdded(threadState: *mut SPParticleThreadState,
        emitterState: *mut SPParticleEmitterState,
        localEmitterTypeID: u32,
    ) -> bool {
        let thread_state = unsafe{ *threadState };
        let emitter_state = unsafe { *emitterState };

        emitter_was_added(&thread_state, &emitter_state, localEmitterTypeID)
    }

    #func
    };

    ast.into()
}
