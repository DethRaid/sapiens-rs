use sapiens_sys::{SPVec2, SPVec3, SPVec4};

#[derive(Debug)]
pub struct Vec2(SPVec2);

pub struct Vec3(SPVec3);

pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub struct Mat3{}

pub struct Mat4{}

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
