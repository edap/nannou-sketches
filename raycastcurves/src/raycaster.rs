use crate::bouncing::BouncingRay2D;
#[allow(dead_code)]
use nannou::prelude::*;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Raycaster {
    pub bouncing_rays: Vec<BouncingRay2D>,
    pub position: Vector2,
}

impl Raycaster {
    pub fn new(position: Vector2) -> Self {
        let bouncing_rays: Vec<Vector2> = Vec::new();
        for i in 0..360.step_by(2) {
            let radian = deg_to_rad(i as f32);
            let ray = BouncingRay2D::new();
            ray.primary_ray.set_dir_from_angle(radian);
            ray.primary_ray.orig = position;
            bouncing_rays.push(ray);
        }

        Raycaster {
            bouncing_rays,
            position,
        }
    }

    pub fn move_to(&mut self, new_pos: Vector2) {
        self.position = new_pos;
        for b_ray in self.bouncing_rays.par_iter_mut() {
            b_ray.primary_ray.orig = self.position;
        }
    }
}
