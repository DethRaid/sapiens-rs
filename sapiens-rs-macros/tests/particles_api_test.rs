extern crate sapiens_rs_macros;
extern crate sapiens_sys;

use sapiens_rs_macros::export_to_sapiens;
use sapiens_sys::SPParticleEmitterTypeInfo;
use std::ffi::CString;

#[export_to_sapiens]
fn get_emitter_types_count() -> i32 {
    3
}

#[export_to_sapiens]
fn get_emitter_types() -> Vec<SPParticleEmitterTypeInfo> {
    vec![
        SPParticleEmitterTypeInfo {
            name: CString::new("campfire").unwrap().into_raw(),
            localID: 0,
        },
        SPParticleEmitterTypeInfo {
            name: CString::new("woodChop").unwrap().into_raw(),
            localID: 1,
        },
        SPParticleEmitterTypeInfo {
            name: CString::new("feathers").unwrap().into_raw(),
            localID: 2,
        },
    ]
}

#[export_to_sapiens]
fn get_render_group_types_count() -> u32 {
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emitter_types_count() {
        let emitter_types_count = get_emitter_types_count();
        let ffi_emitter_types_count = spGetEmitterTypesCount();

        assert_eq!(emitter_types_count, ffi_emitter_types_count);
    }

    #[test]
    fn test_get_emitter_types() {
        let emitter_types_count = get_emitter_types_count() as usize;
        let raw_ffi_emitters = spGetEmitterTypes();

        let ffi_emitters: Vec<(CString, u32)> = unsafe {
            Vec::<SPParticleEmitterTypeInfo>::from_raw_parts(
                raw_ffi_emitters,
                emitter_types_count,
                emitter_types_count,
            )
        }
        .iter()
        .map(|emitter| (unsafe { CString::from_raw(emitter.name) }, emitter.localID))
        .collect();

        let emitters: Vec<(CString, u32)> = get_emitter_types()
            .iter()
            .map(|emitter| (unsafe { CString::from_raw(emitter.name) }, emitter.localID))
            .collect();

        assert_eq!(emitters, ffi_emitters);
    }

    #[test]
    fn test_render_group_types_count() {
        let render_group_types_count = get_render_group_types_count();
        let ffi_render_group_types_count = spGetRenderGroupTypesCount() as u32;

        assert_eq!(render_group_types_count, ffi_render_group_types_count);
    }
}
