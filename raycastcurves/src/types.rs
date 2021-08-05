use nannou::prelude::*;

#[derive(Debug,Copy, Clone)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32, transparency: f32 },
    ReflectiveAndRefractive { reflectivity: f32, index: f32, transparency: f32},
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub coloration: Rgba,
    pub albedo: f32,
    pub surface: SurfaceType,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            coloration: rgba(0.0, 0.0, 1.0, 1.0),
            albedo: 1.0,
            surface: SurfaceType::Reflective { reflectivity: 1.0 }
        }
    }
}

pub struct Curve {
    pub points: Vec<Vec2>,
    pub material: Material,
}
