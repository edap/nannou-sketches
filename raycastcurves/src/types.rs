use nannou::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { ior: f32 },
    ReflectiveAndRefractive { reflectivity: f32, ior: f32 },
}

#[derive(Debug, Copy, Clone)]
pub enum BoundingVolume {
    Sphere { position: Vec2, radius: f32 },
    BoundingBox { position: Vec2, dimension: Vec2 },
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub coloration: Rgba,
    pub albedo: f32,
    pub surface: SurfaceType,
}

impl Default for Material {
    fn default() -> Self {
        let coin = random_range(0.0, 1.0);
        let mut sur;
        if coin > 0.5 {
            sur = SurfaceType::ReflectiveAndRefractive {
                reflectivity: 1.0,
                ior: 1.4,
            };
        } else {
            sur = SurfaceType::Diffuse;
        }

        Material {
            coloration: rgba(0.0, 0.0, 1.0, 1.0),
            albedo: 1.0,
            //surface: SurfaceType::Diffuse
            surface: sur,
        }
    }
}

// TODO, add a bouding box
pub struct Curve {
    pub points: Vec<Vec2>,
    pub material: Material,
    pub ray_anchor_point: Option<Vec2>,
    pub bounding_volume: Option<BoundingVolume>,
}
