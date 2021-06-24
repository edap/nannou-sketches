use nannou::prelude::*;
use ray2d::Ray2D;
#[derive(Debug)]

pub struct Intersection {
    pub pos: Vec2,
    pub color: Rgba,
    pub depth: usize,
}
#[derive(Debug)]
pub struct RayLight {
    pub intersections: Vec<Intersection>,
    pub color: Rgba,
    pub max_depth: usize,
    pub ray: Ray2D,
}

impl RayLight {
    pub fn new(origin: Vec2, direction: Vec2) -> Self{
        let intersections: Vec<Intersection> = Vec::new();
        let ray = Ray2D::new();
        ray.orig = origin;
        ray.dir = direction;
        RayLight {
            intersections,
            color: rgba(1.0,1.0,1.0,1.0),
            max_depth: 4,
            ray,
        }
    }

}