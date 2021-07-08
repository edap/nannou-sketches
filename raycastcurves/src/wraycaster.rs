use crate::ray_light::RayLight;
use ray2d::Ray2D;
use crate::ray_light::Intersection;
use crate::types::Curve;
#[allow(dead_code)]
use nannou::prelude::*;
use rayon::prelude::*;

const EPSILON: f32 = 0.05;

#[derive(Debug)]
pub struct Wraycaster {
    pub ray_lights: Vec<RayLight>,
    pub direction: Vec2,
    pub max_depth: usize,
}

impl Wraycaster {
    pub fn new(position: Vec2, direction: Vec2, max_depth: usize) -> Self {
        let mut ray_lights: Vec<RayLight> = Vec::new();
        for i in (0..360).step_by(6) {
            let radian = deg_to_rad(i as f32);
            let mut ray_light = RayLight::new(position, vec2(radian.cos(), radian.sin()), max_depth);
            ray_lights.push(ray_light);
        }

        Wraycaster {
            ray_lights,
            direction,
            max_depth,
        }
    }

    pub fn move_to(&mut self, new_pos: Vec2) {
        self.ray_lights.par_iter_mut().for_each(|r| {
            r.ray.orig.x = new_pos.x;
            r.ray.orig.y = new_pos.y
        });
    }

    pub fn bounce_horizontally(&mut self, win: &geom::Rect, anim_speed: f32) {
        for r in self.ray_lights.iter_mut() {
            if self.direction.x > 0.0 {
                r.ray.orig.x += 0.1 * anim_speed;
            } else {
                r.ray.orig.x -= 0.1 * anim_speed;
            }
            //println!("{:?}", r.primary_ray.dir.x);

            //r.primary_ray.orig = r.primary_ray.orig + r.primary_ray.dir.with_magnitude(animation_speed);
            if r.ray.orig.x >= win.right() as f32 {
                r.ray.orig.x = win.left();
            } else if r.ray.orig.x < win.left() as f32 {
                r.ray.orig.x = win.right();
            }
        }
    }

    pub fn draw_inside(&self, draw: &Draw, poly_weight: f32, weight: f32, cola: Rgb, colb: Rgb, colc: Rgb, cold: Rgb, cole: Rgb) {
        for ray in self.ray_lights.iter() {

        }
    }

    pub fn draw(&self, draw: &Draw, poly_weight: f32, weight: f32, cola: Rgb, colb: Rgb) {
        //self.ray_lights.iter_mut(|b_ray| {
        for pray in self.ray_lights.iter() {
            // draw.arrow()
            //     .color(cola)
            //     .weight(weight)
            //     .start(b_ray.primary_ray.orig)
            //     .end(b_ray.primary_ray.orig + b_ray.primary_ray.dir.with_magnitude(mag));

            // for coll in &b_ray.collisions {
            //     draw.ellipse().x_y(coll.x, coll.y).w_h(5.0, 5.0);
            // }
            if pray.intersections.len() > 0 {
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
                    pray
                        .intersections
                        .iter()
                        // .zip(b_ray.reflections.iter())
                        // .map(|(&co, &re)| {
                        .map(|inter| {
                            (pt2(inter.pos.x, inter.pos.y), inter.color)
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
                let end_point =
                    pray.ray.orig + pray.ray.dir.normalize() * 2000.0;
                draw.line()
                    .start(pray.ray.orig)
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
        self.ray_lights.par_iter_mut().for_each(|pray| {
            // ray_collides(
            //     pray,
            //     rotation,
            //     animation,
            //     animation_speed,
            //     time,
            //     walls,
            //     win,
            // )
            pray.reset();
            cast_ray(
                &mut pray.ray, &mut pray.count_depth, self.max_depth, &mut pray.intersections, walls)
        })
    }
}

// look at a whitted rt and implement thiss
pub fn cast_ray(
    ray: &mut Ray2D,
    depth: &mut usize,
    max_depth: usize,
    intersections: &mut Vec<Intersection>,
    walls: &Vec<Curve>,
) {
    if *depth < max_depth {
        let collision: Vec2;
        let mut distance: f32 = Float::infinity();
        let mut surface_normal: Vec2 = vec2(0.0, 0.0);
        // find the closest intersection point between the ray and the walls
        for curve in walls.iter() {
            if let Some(collision) = ray.intersect_polyline(&curve.points) {
                // save the closest possible collision
                if collision.0 < distance {
                    distance = collision.0;
                    surface_normal = collision.1;
                }
            }
        }

        if distance < Float::infinity() {
            // collision point
            collision = ray.orig + ray.dir.normalize() * distance;
            let intersection = Intersection::new(collision,rgba(1.0, 0.0, 1.0, 1.0), 3);
            intersections.push(intersection);
            *depth = *depth + 1 ;



            // check if the material reflect, in case add a reflcetion path
            let refl = ray.reflect(surface_normal);
            // r.refl_intensity.push(r.ray.dir.dot(refl).abs());
            ray.orig = collision + refl.normalize() * EPSILON;
            ray.dir = refl;
            cast_ray(ray, depth, max_depth, intersections, walls);

            // check if the material refract, in case add a refraction path
            // check if the material transmit, in case add a transmission path




            // r.reflections.push(refl);
            
        }

    }
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-overview/light-transport-ray-tracing-whitted
// questo metodo dovfebbe essere ricorsivo, raycollides dovrebbe accumulare intersezioni per un singolo raylight
pub fn ray_collides(
    r: &mut RayLight,
    rotation: f32,
    animation: bool,
    animation_speed: f32,
    time: f32,
    walls: &Vec<Curve>,
    win: geom::Rect,
) {
    r.intersections.clear();


    //while !r.max_bounces_reached() {
    while r.count_depth< r.max_depth {
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
            let intersection = Intersection::new(collision,rgba(1.0, 0.0, 1.0, 1.0), 3);
            r.intersections.push(intersection);
            r.count_depth += 1;

            // check if the material reflect, in case add a reflcetion path
            // check if the material refract, in case add a refraction path
            // check if the material transmit, in case add a transmission path


            let refl = r.ray.reflect(surface_normal);
            // r.refl_intensity.push(r.ray.dir.dot(refl).abs());
            // r.ray.orig = collision + refl.normalize() * EPSILON;
            // r.ray.dir = refl;

            // r.reflections.push(refl);
            
        } else {
            break;
        };
    }
    //r.reset();
    //r.ray.set_dir_from_angle(rotation);
}
