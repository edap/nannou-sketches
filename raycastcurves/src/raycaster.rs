use crate::bouncing::BouncingRay2D;
use crate::scene::Curve;
#[allow(dead_code)]
use nannou::prelude::*;
use rayon::prelude::*;

const EPSILON: f32 = 0.05;

#[derive(Debug)]
pub struct Raycaster {
    pub bouncing_rays: Vec<BouncingRay2D>,
    pub direction: Vec2,
}

impl Raycaster {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        let mut bouncing_rays: Vec<BouncingRay2D> = Vec::new();
        for i in (0..360).step_by(6) {
            let radian = deg_to_rad(i as f32);
            let mut ray = BouncingRay2D::new();
            ray.primary_ray.set_dir_from_angle(radian);
            ray.primary_ray.orig = position;
            bouncing_rays.push(ray);
        }

        Raycaster {
            bouncing_rays,
            direction,
        }
    }

    pub fn move_to(&mut self, new_pos: Vec2) {
        self.bouncing_rays.par_iter_mut().for_each(|b_ray| {
            b_ray.primary_ray.orig.x = new_pos.x;
            b_ray.primary_ray.orig.y = new_pos.y
        });
    }

    pub fn bounce_horizontally(&mut self, win: &geom::Rect, anim_speed: f32) {
        for r in self.bouncing_rays.iter_mut() {
            if self.direction.x > 0.0 {
                r.primary_ray.orig.x += 0.1 * anim_speed;
            } else {
                r.primary_ray.orig.x -= 0.1 * anim_speed;
            }
            //println!("{:?}", r.primary_ray.dir.x);

            //r.primary_ray.orig = r.primary_ray.orig + r.primary_ray.dir.with_magnitude(animation_speed);
            if r.primary_ray.orig.x >= win.right() as f32 {
                r.primary_ray.orig.x = win.left();
            } else if r.primary_ray.orig.x < win.left() as f32 {
                r.primary_ray.orig.x = win.right();
            }
        }
    }

    pub fn draw_inside(
        &self,
        draw: &Draw,
        poly_weight: f32,
        weight: f32,
        cola: Rgb,
        colb: Rgb,
        colc: Rgb,
        cold: Rgb,
        cole: Rgb,
    ) {
        for b_ray in self.bouncing_rays.iter() {}
    }

    pub fn draw(&self, draw: &Draw, poly_weight: f32, weight: f32, cola: Rgb, colb: Rgb) {
        //self.bouncing_rays.iter_mut(|b_ray| {
        for b_ray in self.bouncing_rays.iter() {
            // draw.arrow()
            //     .color(cola)
            //     .weight(weight)
            //     .start(b_ray.primary_ray.orig)
            //     .end(b_ray.primary_ray.orig + b_ray.primary_ray.dir.with_magnitude(mag));

            // for coll in &b_ray.collisions {
            //     draw.ellipse().x_y(coll.x, coll.y).w_h(5.0, 5.0);
            // }
            if b_ray.collisions.len() > 0 {
                // draw.line()
                //     .start(b_ray.primary_ray.orig)
                //     .end(b_ray.collisions[0])
                //     .color(cola);
                // let ppp =
                //     b_ray
                //         .collisions
                //         .iter()
                //         .zip(b_ray.reflections.iter())
                //         .map(|(&co, &re)| {
                //             if re.x > 0.0 {
                //                 (pt2(co.x, co.y), cola)
                //             } else {
                //                 (pt2(co.x, co.y), colb)
                //             }
                //         });

                // draw.polyline().points_colored(ppp);

                let ppp =
                    b_ray
                        .collisions
                        .iter()
                        .zip(b_ray.reflections.iter())
                        .map(|(&co, &re)| {
                            if re.x > 0.0 {
                                (pt2(co.x, co.y), cola)
                            } else {
                                (pt2(co.x, co.y), colb)
                            }
                        });

                if ppp.len() > 3 {
                    draw.polygon()
                        .stroke(cola)
                        .stroke_weight(poly_weight)
                        .join_round()
                        .points_colored(ppp);
                    //draw.polygon().points_textured(&model.texture, ppp);
                }
            } else {
                let end_point = b_ray.primary_ray.orig + b_ray.primary_ray.dir.normalize() * 2000.0;
                draw.line()
                    .start(b_ray.primary_ray.orig)
                    .end(end_point)
                    .weight(weight)
                    .color(cola);
            }
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

pub fn cast_ray(
    r: &mut BouncingRay2D,
    rotation: f32,
    animation: bool,
    animation_speed: f32,
    time: f32,
    walls: &Vec<Curve>,
    win: geom::Rect,
) {
}

// TODO, make it a method of the bouncing2D
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

    //while !r.max_bounces_reached() {
    while r.bounces < 4 {
        let collision: Vec2;
        let mut distance: f32 = Float::infinity();
        let mut surface_normal: Vec2 = vec2(0.0, 0.0);
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
            collision = r.ray.orig + r.ray.dir.normalize() * distance;
            r.collisions.push(collision);
            r.bounces += 1;

            // check if the material reflect, in case add a reflcetion path

            let refl = r.ray.reflect(surface_normal);
            r.refl_intensity.push(r.ray.dir.dot(refl).abs());
            r.ray.orig = collision + refl.normalize() * EPSILON;
            r.ray.dir = refl;

            r.reflections.push(refl);
        } else {
            break;
        };
    }
    r.reset();
    //r.ray.set_dir_from_angle(rotation);
}
