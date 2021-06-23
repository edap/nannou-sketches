use nannou::prelude::*;

#[derive(Debug,Copy, Clone)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32, transparency: f32 },
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub coloration: Rgba,
    pub albedo: f32,
    pub surface: SurfaceType,
}


pub struct Curve {
    pub points: Vec<Vector2>,
    pub material: Material,
}
