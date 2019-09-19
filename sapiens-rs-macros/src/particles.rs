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
    #[no_mangle]
    pub extern "C" fn spGetEmitterTypes() -> *mut SPParticleEmitterTypeInfo {
        let mut emitters: Vec<SPParticleEmitterTypeInfo> = get_emitter_types()
            .drain(::std::ops::RangeFull)
            .map(|emitter_type| ::std::convert::TryInto::try_into(emitter_type).unwrap())
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
    #[no_mangle]
    pub extern "C" fn spGetRenderGroupTypesCount() -> ::std::os::raw::c_int {
        get_render_group_types_count() as ::std::os::raw::c_int
    }

    #func
    };

    ast.into()
}

pub fn generate_get_render_group_types(func: syn::ItemFn) -> TokenStream {
    let ast = quote! {
    #[no_mangle]
    pub extern "C" fn spGetRenderGroupTypes() -> *mut SPParticleRenderGroupInfo {
        let mut render_group_types: Vec<SPParticleRenderGroupInfo> = get_render_group_types()
            .drain(::std::ops::RangeFull)
            .map(|emitter_type| ::std::convert::TryInto::try_into(emitter_type).unwrap())
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
    #[no_mangle]
    pub extern "C" fn spEmitterWasAdded(threadState: *mut ::sapiens_sys::SPParticleThreadState,
        emitterState: *mut ::sapiens_sys::SPParticleEmitterState,
        localEmitterTypeID: u32,
    ) -> bool {
        let mut emitter_state = unsafe { &mut *emitterState };

        let mut thread_state: ::sapiens_rs::sp::particles::ThreadState = ::std::convert::TryFrom::try_from(unsafe { *threadState }).unwrap();

        emitter_was_added(&thread_state, &mut emitter_state, ::num_traits::FromPrimitive::from_u32(localEmitterTypeID).unwrap())
    }

    #func
    };

    ast.into()
}
