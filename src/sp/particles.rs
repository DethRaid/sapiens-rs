use sapiens_sys::*;

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum RenderGroupVertexDescriptionType {
    Float = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_float,
    Vec2 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2,
    Vec3 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3,
    Vec4 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4,
}

pub struct EmitterTypeInfo(SPParticleEmitterTypeInfo);

impl EmitterTypeInfo {
    pub fn into_sp_info(&self) -> SPParticleEmitterTypeInfo {
        self.0
    }
}

pub fn get_emitter_types_count() -> i32 {
    unsafe { spGetEmitterTypesCount() }
}

pub fn get_emitter_types() -> Vec<EmitterTypeInfo> {
    let num_emitters = get_emitter_types_count();
    let emitters = unsafe { spGetEmitterTypes() };

    Vec::<EmitterTypeInfo>::from_raw_parts(emitters, num_emitters, num_emitters)
}

