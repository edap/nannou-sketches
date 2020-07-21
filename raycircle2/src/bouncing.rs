use nannou::prelude::*;
use ray2d::Ray2D;

#[derive(Debug)]
pub struct BouncingRay2D {
    pub primary_ray: Ray2D,
    pub ray: Ray2D,
    pub bounces: usize,
    pub max_bounces: usize,
    pub collisions: Vec<Vector2>,
    pub reflections: Vec<Vector2>,
    pub refractions: Vec<Vector2>,
    pub refl_intensity: Vec<f32>,
}

impl BouncingRay2D {
    pub fn new() -> Self {
        let collisions: Vec<Vector2> = Vec::new();
        let reflections: Vec<Vector2> = Vec::new();
        let refractions: Vec<Vector2> = Vec::new();
        let refl_intensity: Vec<f32> = Vec::new();
        BouncingRay2D {
            primary_ray: Ray2D::new(),
            ray: Ray2D::new(),
            bounces: 0,
            max_bounces: 20,
            collisions,
            reflections,
            refractions,
            refl_intensity: refl_intensity,
        }
    }

    pub fn max_bounces_reached(&self) -> bool {
        self.bounces > self.max_bounces
    }

    pub fn reset(&mut self) {
        self.bounces = 0;
        self.ray.orig = self.primary_ray.orig;
        self.ray.dir = self.primary_ray.dir;
    }
}
