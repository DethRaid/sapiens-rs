#[macro_use]
use proc_quote::quote;

pub fn generate_get_emitter_type_count_func(func: syn::ItemFn) -> proc_macro::TokenStream {
    let ast = quote! {
    pub extern "C" fn spGetEmitterTypeCount() -> ::std::os::raw::c_int {
        get_emitter_types_count()
    }

    #func
    };

    ast.into()
}
