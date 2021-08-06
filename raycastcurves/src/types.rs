use nannou::prelude::*;

#[derive(Debug,Copy, Clone)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { ior: f32},
    ReflectiveAndRefractive { reflectivity: f32, ior: f32},
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
            //surface: SurfaceType::Refractive {ior: 1.5}
            surface: SurfaceType::ReflectiveAndRefractive {reflectivity: 1.0, ior: 1.5}
            
        }
    }
}

pub struct Curve {
    pub points: Vec<Vec2>,
    pub material: Material,
}
