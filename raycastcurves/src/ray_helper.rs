pub use crate::bouncing::BouncingRay2D;
use crate::scene::Element;
pub use crate::wraycaster::Wraycaster;
use nannou::image::math;
use nannou::prelude::*;

pub fn make_rays(
    rays: &mut Vec<BouncingRay2D>,
    win: &geom::Rect,
    tile_count_w: u32,
    n_caster: u32, // 0 even, 1 random rotation, 2 one in the middle, 4 diamond
) {
    rays.clear();
    let padding = win.h() as u32 / n_caster;

    //for _y in 0..(win.h() as u32 / side as u32) {
    for _y in 0..n_caster {
        let mut r = BouncingRay2D::new();
        let x;
        //r.primary_ray.dir = Vector2::from_angle(random_range(-PI, PI));
        if _y % 2 == 0 {
            r.primary_ray.dir = vec2(0.0.cos(), 0.0.sin());
            x = win.left() + random_range(0.0, win.w() / 2.0);
        } else {
            r.primary_ray.dir = vec2(-PI.cos(), -PI.sin());
            x = win.right();
        }
        r.ray.dir = r.primary_ray.dir;

        let o = vec2(x, (_y * padding) as f32 - win.h() / 2 as f32);
        r.primary_ray.orig = o;
        r.ray.orig = o;
        rays.push(r);
    }
}

pub fn make_raycasters(
    rays: &mut Vec<Wraycaster>,
    win: &geom::Rect,
    tile_count_w: u32,
    n_caster: u32, // 0 even, 1 random rotation, 2 one in the middle, 4 diamond
    max_reflection: usize,
    density: usize,
    walls: &Vec<Element>,
    rays_position_mode: usize,
    rays_probability: f32,
) {
    match rays_position_mode {
        0 => left_and_right_alternate(rays, win, tile_count_w, n_caster, max_reflection, density),
        1 => inside_the_walls(rays, &walls, rays_probability, density, max_reflection),
        _ => {}
    }
}

fn inside_the_walls(
    raycasters: &mut Vec<Wraycaster>,
    walls: &Vec<Element>,
    rays_probability: f32,
    density: usize,
    max_depth: usize,
) {
    raycasters.clear();
    for wall in walls.iter() {
        match wall.ray_anchor_point() {
            Some(pt) => {
                let coin = random_range(0.0, 1.0);
                if coin <= rays_probability {
                    let dir = vec2(random_f32().cos(), random_f32().sin()).normalize();
                    let raycaster = Wraycaster::new(*pt, dir, max_depth, density);
                    raycasters.push(raycaster);
                }
            }
            _ => {}
        }
    }
}

fn left_and_right_alternate(
    rays: &mut Vec<Wraycaster>,
    win: &geom::Rect,
    tile_count_w: u32,
    n_caster: u32, // 0 even, 1 random rotation, 2 one in the middle, 4 diamond
    max_reflection: usize,
    density: usize,
) {
    rays.clear();
    let padding = win.h() as u32 / n_caster;

    //for _y in 0..(win.h() as u32 / side as u32) {
    for _y in 0..n_caster {
        let x;
        let dir;

        if _y % 2 == 0 {
            dir = vec2(0.0.cos(), 0.0.sin());
            x = win.left() + 1.0;
        } else {
            dir = vec2(-1.0.cos(), -1.0.sin());
            x = win.right() - 1.0;
        }

        let pos = vec2(x, (_y * padding) as f32 - win.h() / 2 as f32);
        let r = Wraycaster::new(pos, dir, max_reflection, density);
        rays.push(r);
    }
}
