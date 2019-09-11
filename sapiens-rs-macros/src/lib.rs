extern crate proc_macro;
extern crate proc_quote;

use crate::generation::generate_binding;
use proc_macro::TokenStream;
use proc_quote::quote;

mod generation;
mod particles;

/// Generates code to allow Sapiens to call your function
///
/// This macro converts `*mut T` to `&mut T`, and it can convert the `u32`s that Sapiens passes in for the
/// `emitter_type` and `
///
/// This macro looks at your function name to determine which Sapiens hook to generate code for. The following function
/// names are supported:
/// - get_emitter_types_count -> spGetEmitterTypesCount
/// - get_emitter_types -> spGetEmitterTypes
/// - get_render_group_types_count -> spGetRenderGroupTypesCount
/// - get_render_group_types -> spGetRenderGroupTypes
/// - emitter_was_added -> spEmitterWasAdded
/// - update_emitter -> spUpdateEmitter
/// - update_particle -> spUpdateParticle
///
/// TODO: Look at the enum types used for the emitter type and render group type to determine how to convert from
/// Sapiens ints to your enums
///
/// TODO: validate that the enums used for the emitter type and render group type are the same as the enums marked as
/// the emitter type and render group type
#[proc_macro_attribute]
pub fn export_for_sapiens(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse2(item.into()).unwrap();

    match input {
        syn::Item::Fn(func) => {
            let mut generated_func: TokenStream = generate_binding(func);
            generated_func
        }
        _ => quote! {compile_error!("export_for_sapiens may only be used for functions!")}.into(),
    }
}

// TODO: Macros to mark your emitter type and render group type enums and generate `get_emitter_types_count`,
// `get_emitter_types`, `get_render_group_types`, and `get_render_groups`
