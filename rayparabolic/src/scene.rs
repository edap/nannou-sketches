use nannou::prelude::*;
use nannou_ray2d::{BoundingVolume, Ray2D};

#[derive(Debug, Copy, Clone)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { ior: f32 },
    ReflectiveAndRefractive { reflectivity: f32, ior: f32 },
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
#[derive(Debug)]
pub struct Curve {
    pub points: Vec<Vec2>,
    pub material: Material,
    pub ray_anchor_point: Option<Vec2>,
    pub bounding_volume: Option<BoundingVolume>,
}
#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
    pub position: Vec2,
    pub material: Material,
    pub ray_anchor_point: Option<Vec2>,
    pub bounding_volume: Option<BoundingVolume>,
}

#[derive(Debug)]
pub enum Element {
    Curve(Curve),
    Circle(Circle),
}

impl Element {
    pub fn material(&self) -> &Material{
        match *self {
            Element::Curve(ref cu) => &cu.material,
            Element::Circle(ref ci) => &ci.material,
        }
    }
    pub fn bounding_volume(&self) -> Option<&BoundingVolume>{
        match *self {
            Element::Curve(ref cu) => cu.bounding_volume.as_ref(),
            Element::Circle(ref ci) => ci.bounding_volume.as_ref(),
        }
    }
    pub fn ray_anchor_point(&self) -> Option<&Vec2>{
        match *self {
            Element::Curve(ref cu) => cu.ray_anchor_point.as_ref(),
            Element::Circle(ref ci) => ci.ray_anchor_point.as_ref(),
        }
    }
    pub fn material_mut(&mut self) -> &mut Material {
        match *self {
            Element::Curve(ref mut cu) => &mut cu.material,
            Element::Circle(ref mut ci) => &mut ci.material,
        }
    }

    pub fn draw(&self, draw: &Draw, wall_width: &f32) {
        //println!("{:?}", curve.points.len());
        match *self {
            Element::Curve(ref curve) => {
                draw.polyline()
                .weight(*wall_width)
                .color(curve.material.coloration)
                .points(curve.points.clone());
            },
            Element::Circle(ref circle) => {
                draw.ellipse()
                .no_fill()
                .x_y(circle.position.x, circle.position.y)
                .w_h(circle.radius * 2.0, circle.radius * 2.0)
                .color(circle.material.coloration)
                .stroke_weight(*wall_width);
            }
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray : &Ray2D) -> Option<(f32, Vec2)>;
}

impl Intersectable for Element {
    fn intersect(&self, ray : &Ray2D) -> Option<(f32, Vec2)>{
        match *self {
            Element::Curve(ref cu) => cu.intersect(ray),
            Element::Circle(ref ci) => ci.intersect(ray),
        }
    }
}

impl Intersectable for Circle {
    fn intersect(&self, ray : &Ray2D) -> Option<(f32, Vec2)>{
        ray.intersect_circle(&self.position, &self.radius)
    }
}

impl Intersectable for Curve {
    fn intersect(&self, ray : &Ray2D) -> Option<(f32, Vec2)>{
            // if a bounding volume is present, use it to pre-test the intersection
            match &self.bounding_volume {
                Some(volume) => {
                    let pretest = ray.intersect_bounding_volume(volume);
                    match pretest {
                        Some(_) => {
                            ray.intersect_polyline(&self.points)
                        }
                        None => None
                    }
                }
                // There is no acceleration structure available to pre-test the intersection
                // proceed to test every segment in the polyline.
                None => {
                    ray.intersect_polyline(&self.points)
                }
            }
        
    }   
}