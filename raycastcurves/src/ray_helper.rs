pub use crate::bouncing::BouncingRay2D;
pub use crate::raycaster::Raycaster;
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
            r.primary_ray.dir = Vector2::from_angle(0.0);
            x = win.left();
        } else {
            r.primary_ray.dir = Vector2::from_angle(-PI);
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
    rays: &mut Vec<Raycaster>,
    win: &geom::Rect,
    tile_count_w: u32,
    n_caster: u32, // 0 even, 1 random rotation, 2 one in the middle, 4 diamond
) {
    rays.clear();
    let padding = win.h() as u32 / n_caster;

    //for _y in 0..(win.h() as u32 / side as u32) {
    for _y in 0..n_caster {
        let x;
        let dir;
        //r.primary_ray.dir = Vector2::from_angle(random_range(-PI, PI));

        if _y % 2 == 0 {
            dir = Vector2::from_angle(0.0);
            x = win.left();
        } else {
            dir = Vector2::from_angle(-PI);
            x = win.right();
        }

        let pos = vec2(x, (_y * padding) as f32 - win.h() / 2 as f32);
        let r = Raycaster::new(pos, dir);
        rays.push(r);
    }
}
