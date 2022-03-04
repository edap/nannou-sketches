use crate::ray_light::Intersection;
use crate::ray_light::RayLight;
use crate::types::Curve;
use crate::types::Material;
use crate::types::SurfaceType;
use nannou::color::Mix;
#[allow(dead_code)]
use nannou::prelude::*;
use nannou::rand::Rng;
use ray2d::Ray2D;
use rayon::prelude::*;

const EPSILON: f32 = 0.05;

#[derive(Debug)]
pub struct Wraycaster {
    pub ray_lights: Vec<RayLight>,
    pub direction: Vec2,
    pub max_depth: usize,
    pub density: usize,
}

impl Wraycaster {
    pub fn new(position: Vec2, direction: Vec2, max_depth: usize, density: usize) -> Self {
        let mut ray_lights: Vec<RayLight> = Vec::new();
        for i in (0..360).step_by(density) {
            let radian = deg_to_rad(i as f32);
            let ray_light = RayLight::new(position, vec2(radian.cos(), radian.sin()), max_depth);
            ray_lights.push(ray_light);
        }
        Wraycaster {
            ray_lights,
            direction,
            max_depth,
            density,
        }
    }

    pub fn move_to(&mut self, new_pos: Vec2) {
        self.ray_lights.par_iter_mut().for_each(|r| {
            r.ray.orig.x = new_pos.x;
            r.ray.orig.y = new_pos.y
        });
    }

    pub fn animate(&mut self, win: &geom::Rect, anim_speed: f32, animation_mode: usize, time: f32) {
        match animation_mode {
            0 => {
                // Bounce orizontally
                for r in self.ray_lights.iter_mut() {
                    if self.direction.x >= 0.0 {
                        r.starting_pos.x += 0.1 * anim_speed;
                    } else {
                        r.starting_pos.x -= 0.1 * anim_speed;
                    }

                    //r.ray.orig = r.ray.orig + r.ray.dir.with_magnitude(animation_speed);
                    if r.starting_pos.x >= win.right() as f32 {
                        //r.starting_pos.x = win.left();

                        self.direction = vec2(-1.0.cos(), -1.0.sin()).normalize();
                    } else if r.starting_pos.x < win.left() as f32 {
                        self.direction = vec2(0.0.cos(), 0.0.sin()).normalize();
                        //r.starting_pos.x = win.right();
                    }
                }
            }
            1 => {
                for r in self.ray_lights.iter_mut() {
                    let radius = win.w() / 4.0;
                    let x = (time * 0.001).cos() * radius;
                    let y = (time * 0.001).sin() * radius;

                    r.starting_pos = r.starting_pos + pt2(x, y);
                }
            }
            _ => {}
        }
    }

    pub fn draw_polygon(
        &self,
        draw: &Draw,
        poly_weight: f32,
        weight: f32,
        draw_not_colliding_rays: bool,
        mode: usize,
    ) {
        let expe = true;
        match mode {
            0 => {
                let mut all_intersections: Vec<Intersection> = Vec::new();
                for pray in self.ray_lights.iter() {
                    if pray.intersections.len() > 0 {
                        //all_intersections.extend(pray.intersections);
                        all_intersections.extend(pray.intersections.iter())
                        //all_intersections.iter().chain(pray.intersections.iter()).collect::<Intersection>();
                    }
                }

                if all_intersections.len() > 3 {
                    let pp = all_intersections
                        .iter()
                        .map(|&inter| (inter.pos, inter.color));
                    if poly_weight > 0.5 {
                        draw.polygon()
                            .stroke_weight(poly_weight)
                            .caps_round()
                            .stroke(all_intersections[0].color)
                            .join_round()
                            .points_colored(pp);
                    } else {
                        draw.polygon().points_colored(pp);
                    }
                }
            }
            1 => {
                for pray in self.ray_lights.iter() {
                    match pray.intersections.len() {
                        1 => {
                            let first_two_points =
                                vec![pray.starting_pos, pray.intersections[0].pos];
                            let colors = vec![pray.color, pray.intersections[0].color];
                            let first_two_points_colored = first_two_points
                                .iter()
                                .zip(colors.iter())
                                .map(|(&pt, &col)| (pt, col));
                            if poly_weight > 0.5 {
                                draw.polygon()
                                    .stroke_weight(poly_weight)
                                    .caps_round()
                                    .join_round()
                                    .stroke(pray.color)
                                    .points_colored(first_two_points_colored);
                            } else {
                                draw.polygon().points_colored(first_two_points_colored);
                            }
                        }
                        2..=200 => {
                            let first_point = vec![pray.starting_pos];
                            let first_color = vec![pray.color];
                            let intersection_points_pos: &Vec<Vec2> =
                                &pray.intersections.iter().map(|inter| inter.pos).collect();
                            let intersection_points_col: &Vec<Hsla> =
                                &pray.intersections.iter().map(|inter| inter.color).collect();

                            let points: Vec<&Vec2> = first_point
                                .iter()
                                .chain(intersection_points_pos.iter())
                                .collect();
                            let colors: Vec<&Hsla> = first_color
                                .iter()
                                .chain(intersection_points_col.iter())
                                .collect();
                            let points_colored = points
                                .into_iter()
                                .zip(colors.into_iter())
                                .map(|(&pt, &col)| (pt, col));

                            if poly_weight > 0.5 {
                                draw.polygon()
                                    .stroke_weight(poly_weight)
                                    .caps_round()
                                    .join_round()
                                    .stroke(pray.color)
                                    .points_colored(points_colored);
                            } else {
                                draw.polygon().points_colored(points_colored);
                            }
                        }
                        // 0 if draw_not_colliding_rays=> {
                        //     let end_point = pray.ray.orig + pray.ray.dir.normalize() * 2000.0;
                        //     draw.line()
                        //     .start(pray.starting_pos)
                        //     .end(end_point)
                        //     .stroke_weight(poly_weight)
                        //     .caps_round()
                        //     .color(pray.color);
                        // }
                        _ => {}
                    }
                }
            }
            2 => {
                for pray in self.ray_lights.iter() {
                    if pray.intersections.len() > 3 {
                        let pp = pray
                            .intersections
                            .iter()
                            .map(|&inter| (inter.pos, inter.color));
                        if poly_weight > 0.5 {
                            draw.polygon()
                                .stroke_weight(poly_weight)
                                .caps_round()
                                .stroke(pray.intersections[0].color)
                                .join_round()
                                .points_colored(pp);
                        } else {
                            draw.polygon().points_colored(pp);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn draw_arrows(&self, draw: &Draw, weight: f32) {
        for pray in self.ray_lights.iter() {
            if pray.intersections.len() > 1 {
                draw.arrow()
                    .start(pray.starting_pos)
                    .end(pray.intersections[0].pos)
                    .stroke_weight(weight)
                    .color(pray.color);
            }

            if pray.intersections.len() > 2 {
                for n in 0..pray.intersections.len() - 1 {
                    draw.arrow()
                        .start(pray.intersections[n].pos)
                        .end(pray.intersections[n + 1].pos)
                        .stroke_weight(weight)
                        .color(pray.intersections[n].color);
                }
            }
        }

        // for pray in self.ray_lights.iter() {

        //     if pray.intersections.len() > 1 {
        //         draw.arrow()
        //         .start(pray.starting_pos)
        //         .end(pray.intersections[0].pos)
        //         .stroke_weight(weight)
        //         .color(pray.color);

        //     }

        //     if pray.intersections.len() > 2 {
        //         for n in 0..pray.intersections.len() -1 {
        //             draw.arrow()
        //             .start(pray.intersections[n].pos)
        //             .end(pray.intersections[n+1].pos)
        //             .stroke_weight(weight)
        //             .color(pray.intersections[n].color);

        //         }
        //     }
        // }
    }

    pub fn draw_rays(&self, draw: &Draw, weight: f32, draw_not_colliding_rays: bool) {
        for pray in self.ray_lights.iter() {
            match pray.intersections.len() {
                1 => {
                    let first_two_points = vec![pray.starting_pos, pray.intersections[0].pos];
                    let colors = vec![pray.color, pray.intersections[0].color];
                    let first_two_points_colored = first_two_points
                        .iter()
                        .zip(colors.iter())
                        .map(|(&pt, &col)| (pt, col));
                    draw.polyline()
                        .stroke_weight(weight)
                        .caps_round()
                        .join_round()
                        .points_colored(first_two_points_colored);
                }
                2..=200 => {
                    let first_point = vec![pray.starting_pos];
                    let first_color = vec![pray.color];
                    let intersection_points_pos: &Vec<Vec2> =
                        &pray.intersections.iter().map(|inter| inter.pos).collect();
                    let intersection_points_col: &Vec<Hsla> =
                        &pray.intersections.iter().map(|inter| inter.color).collect();

                    let points: Vec<&Vec2> = first_point
                        .iter()
                        .chain(intersection_points_pos.iter())
                        .collect();
                    let colors: Vec<&Hsla> = first_color
                        .iter()
                        .chain(intersection_points_col.iter())
                        .collect();
                    let points_colored = points
                        .into_iter()
                        .zip(colors.into_iter())
                        .map(|(&pt, &col)| (pt, col));
                    draw.polyline()
                        .stroke_weight(weight)
                        .caps_round()
                        .join_round()
                        .points_colored(points_colored);
                }
                0 if draw_not_colliding_rays => {
                    let end_point = pray.ray.orig + pray.ray.dir.normalize() * f32::MAX;
                    draw.line()
                        .start(pray.starting_pos)
                        .end(end_point)
                        .stroke_weight(weight)
                        .caps_round()
                        .color(pray.color);
                }
                _ => {}
            }
        }
    }

    pub fn collide(
        &mut self,
        rotation: f32,
        animation: bool,
        animation_speed: f32,
        time: f32,
        scene: &Vec<Curve>,
        win: geom::Rect,
    ) {
        self.ray_lights.par_iter_mut().for_each(|pray| {
            pray.reset();
            //println!("DDD {:?}", pray.count_depth);
            // principle of light conservation. The amount of light does not decrease.
            // but at the same time it does not increase. If a ray generates 2 secondary rays like
            // in a refractive and reflective surface, the amount of light is split among the 2
            // secondary rays.
            // This parameter is used as alpha channel of the color.
            let light_amount = 1.0;
            cast_ray(
                &mut pray.ray,
                &mut pray.count_depth,
                pray.max_depth,
                &mut pray.intersections,
                scene,
                light_amount,
            )
        })
    }
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-overview/light-transport-ray-tracing-whitted
// look at a whitted rt and implement thiss
pub fn cast_ray(
    ray: &mut Ray2D,
    depth: &mut usize,
    max_depth: usize,
    intersections: &mut Vec<Intersection>,
    scene: &Vec<Curve>,
    light_amount: f64,
) {
    if *depth < max_depth {
        let collision: Vec2;
        let mut distance: f32 = Float::infinity();
        let mut surface_normal: Vec2 = vec2(0.0, 0.0);
        let mut material: Material = Material::default();
        // find the closest intersection point between the ray and the scene
        for curve in scene.iter() {
            // if a bounding volume is present, use it to pre-test the intersection
            // otherwise test everything
            match &curve.bounding_volume {
                Some(volume) => {
                    let pretest = ray.intersect_bounding_volume(volume);
                    match pretest {
                        Some(_) => {
                            if let Some(collision) = ray.intersect_polyline(&curve.points) {
                                // save the closest possible collision
                                if collision.0 < distance {
                                    distance = collision.0;
                                    surface_normal = collision.1;
                                    material = curve.material;
                                }
                            }
                        }

                        None => {}
                    }
                }
                None => {
                    if let Some(collision) = ray.intersect_polyline(&curve.points) {
                        // save the closest possible collision
                        if collision.0 < distance {
                            distance = collision.0;
                            surface_normal = collision.1;
                            material = curve.material;
                        }
                    }
                }
            }
        }

        if distance < Float::infinity() {
            *depth = *depth + 1;
            // collision point
            collision = ray.orig + ray.dir.normalize() * distance;

            let mut hsla = get_color(&surface_normal, &ray.dir.normalize(), &material);
            hsla.alpha = light_amount as f32;

            //secondary rays.

            match material.surface {
                SurfaceType::Reflective { reflectivity } => {
                    let refl = ray.reflect(surface_normal);
                    // let intensity = ray.dir.dot(refl).abs();
                    // hsla.lightness = hsla.lightness * intensity;
                    //println!("light {:?}", hsla.lightness);
                    let intersection = Intersection::new(collision, hsla, *depth);
                    intersections.push(intersection);

                    // r.refl_intensity.push(r.ray.dir.dot(refl).abs());
                    ray.orig = collision + refl.normalize() * EPSILON;
                    ray.dir = refl;
                    cast_ray(ray, depth, max_depth, intersections, scene, light_amount);
                }
                SurfaceType::Refractive { ior } => {
                    let intersection = Intersection::new(collision, hsla, *depth);
                    intersections.push(intersection);

                    let refr = ray.refract(surface_normal, ior);
                    ray.orig = collision + refr.normalize() * EPSILON;
                    ray.dir = refr;
                    cast_ray(ray, depth, max_depth, intersections, scene, light_amount);
                }
                SurfaceType::ReflectiveAndRefractive { reflectivity, ior } => {
                    let fresnel = ray.fresnel(surface_normal, ior);
                    let outside = ray.dir.dot(surface_normal) < 0.0;
                    let refl = ray.reflect(surface_normal);

                    // always refract, as we are dealong most with segments and curves and they don't have an inside or outside
                    // side
                    let refr = ray.refract(surface_normal, ior);
                    ray.dir = refr;
                    ray.orig = collision + refr.normalize() * EPSILON;
                    cast_ray(
                        ray,
                        depth,
                        max_depth,
                        intersections,
                        scene,
                        light_amount * (1.0 - fresnel),
                    );
                    // refl
                    ray.dir = refl;
                    ray.orig = collision + refl.normalize() * EPSILON;
                    cast_ray(
                        ray,
                        depth,
                        max_depth,
                        intersections,
                        scene,
                        light_amount * fresnel,
                    );

                    //2nd way, check if outside or inside refl und refr
                    // compute refraction if it is not a case of total internal reflection
                    // if fresnel < 1.0{
                    //     //refraction
                    //     let refr = ray.refract(surface_normal, ior);
                    //     if outside{
                    //         ray.orig = collision - refr.normalize() * EPSILON;
                    //     }else{
                    //         ray.orig = collision + refr.normalize() * EPSILON;
                    //     }
                    //     ray.dir = refr;
                    //     cast_ray(ray, depth, max_depth, intersections, scene, light_amount * (1.0 - fresnel));
                    // }

                    // //reflection
                    // if outside{
                    //     ray.orig = collision + refl.normalize() * EPSILON;
                    // }else{
                    //     ray.orig = collision - refl.normalize() * EPSILON;
                    // }
                    // ray.dir = refl;
                    // cast_ray(ray, depth, max_depth, intersections, scene, light_amount * fresnel);

                    let intersection = Intersection::new(collision, hsla, *depth);
                    intersections.push(intersection);
                }
                SurfaceType::Diffuse => {
                    let intersection = Intersection::new(collision, hsla, *depth);
                    intersections.push(intersection);
                }
            }

            // 100% transmit?
        }
    }

    // at the moment this method is doing nothing
    fn get_color(surface_direction: &Vec2, ray_direction: &Vec2, material: &Material) -> Hsla {
        // let mut alpha : f32 = 1.0 - ( *depth as f32 / max_depth as f32);
        // alpha = alpha.min(0.0).max(1.0);
        let mut hsla: Hsla = material.coloration.into();
        // TODO, is there a function to get the inverse of a vector?
        let inverted_ray_dir = vec2(ray_direction.x * -1.0, ray_direction.y * -1.0);
        let diffuse_component = surface_direction.dot(inverted_ray_dir).clamp(0.0, 1.0);
        hsla.lightness = hsla.lightness * diffuse_component;

        return hsla;
    }
}
