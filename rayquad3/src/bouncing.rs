use nannou::prelude::*;
use ray2d::Ray2D;

#[derive(Debug)]
pub struct BouncingRay2D {
    pub ray_origin: Ray2D,
    pub ray: Ray2D,
    pub bounces: usize,
    pub max_bounces: usize,
    pub collisions: Vec<Vector2>,
    pub refractions: Vec<Vector2>,
    pub refl_intensity: f32,
}

impl BouncingRay2D {
    pub fn new() -> Self {
        let collisions: Vec<Vector2> = Vec::new();
        let refractions: Vec<Vector2> = Vec::new();
        BouncingRay2D {
            ray_origin: Ray2D::new(),
            ray: Ray2D::new(),
            bounces: 0,
            max_bounces: 200,
            collisions,
            refractions,
            refl_intensity: 0.0,
        }
    }

    pub fn max_bounces_reached(&self) -> bool {
        self.bounces > self.max_bounces
    }

    pub fn reset(&mut self) {
        self.bounces = 0;
        self.ray.orig = self.ray_origin.orig;
        self.ray.dir = self.ray_origin.dir;
    }
}
