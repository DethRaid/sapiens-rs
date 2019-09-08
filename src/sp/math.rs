use sapiens_sys::{SPVec2, SPVec3, SPVec4};

#[derive(Debug)]
pub struct Vec2(SPVec2);

pub struct Vec3(SPVec3);

pub struct Vec4(SPVec4);

pub struct Mat3(SPMat3);

pub struct Mat4(SPMat4);

impl Vec2 {
    pub fn to_sp_vec2(&self) -> SPVec2 {
        self.0
    }
}

impl Vec3 {
    pub fn to_sp_vec3(&self) -> SPVec3 {
        self.0
    }
}
