use crate::bouncing::BouncingRay2D;
use crate::types::Curve;
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
        let mut bouncing_rays: Vec<BouncingRay2D> = Vec::new();
        for i in (0..360).step_by(2) {
            let radian = deg_to_rad(i as f32);
            let mut ray = BouncingRay2D::new();
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
        self.bouncing_rays.par_iter_mut().for_each(|b_ray| {
            b_ray.primary_ray.orig.x = new_pos.x;
            b_ray.primary_ray.orig.y = new_pos.y
        });
    }

    pub fn draw(&self, draw: &Draw, mag: f32, weight: f32, col: Rgb) {
        //self.bouncing_rays.iter_mut(|b_ray| {
        for b_ray in self.bouncing_rays.iter() {
            draw.arrow()
                .color(col)
                .weight(weight)
                .start(b_ray.primary_ray.orig)
                .end(b_ray.primary_ray.dir.with_magnitude(mag));
        }
    }

    pub fn collide(
        &mut self,
        rotation: f32,
        animation: bool,
        animation_speed: f32,
        time: f32,
        walls: &Vec<Curve>,
        win: geom::Rect,
    ) {
        self.bouncing_rays.par_iter_mut().for_each(|b_ray| {
            ray_collides(
                b_ray,
                rotation,
                animation,
                animation_speed,
                time,
                walls,
                win,
            )
        })
    }
}

pub fn ray_collides(
    r: &mut BouncingRay2D,
    rotation: f32,
    animation: bool,
    animation_speed: f32,
    time: f32,
    walls: &Vec<Curve>,
    win: geom::Rect,
) {
    r.collisions.clear();
    r.reflections.clear();
    r.refl_intensity.clear();
    if animation {
        if r.primary_ray.dir.x > 0.0 {
            r.primary_ray.orig.x += 0.1 * animation_speed;
        } else {
            r.primary_ray.orig.x -= 0.1 * animation_speed;
        }
        //r.primary_ray.orig = r.primary_ray.orig + r.primary_ray.dir.with_magnitude(animation_speed);
        if r.primary_ray.orig.x >= win.right() as f32 {
            r.primary_ray.orig.x = win.left();
        } else if r.primary_ray.orig.x <= win.left() as f32 {
            r.primary_ray.orig.x = win.right();
        }
    }
    while !r.max_bounces_reached() {
        let collision: Vector2;
        let mut distance: f32 = Float::infinity();
        let mut surface_normal: Vector2 = vec2(0.0, 0.0);
        // find the closest intersection point between the ray and the walls
        for curve in walls.iter() {
            if let Some(collision) = r.ray.intersect_polyline(&curve.points) {
                // save the closest possible collision
                if collision.0 < distance {
                    distance = collision.0;
                    surface_normal = collision.1;
                }
            }
        }
        if distance < Float::infinity() {
            // collision point
            collision = r.ray.orig + r.ray.dir.with_magnitude(distance);
            r.bounces += 1;
            let refl = r.ray.reflect(surface_normal);
            r.refl_intensity.push(r.ray.dir.dot(refl).abs());
            r.ray.orig = collision + refl.with_magnitude(0.03);
            r.ray.dir = refl;
            r.collisions.push(collision);
            r.reflections.push(refl);
        } else {
            break;
        };
    }
    r.reset();
    //r.ray.set_dir_from_angle(rotation);
}
