#[allow(dead_code)]
use nannou::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum BoundingVolume {
    Circle { position: Vec2, radius: f32 },
    Aabb { min: Vec2, max: Vec2 },
}

#[derive(Debug, Clone, Copy)]
pub struct Ray2D {
    pub orig: Vec2,
    pub dir: Vec2,
}

// TODO, all this methods should accept both f32 and f64

impl Ray2D {
    pub fn new() -> Self {
        Ray2D {
            orig: vec2(0.0, 0.0),
            dir: vec2(1.0, 0.0),
        }
    }

    pub fn reflect(&self, surface_normal: Vec2) -> Vec2 {
        //I - 2.0 * dot(N, I) * N
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/reflect.xhtml
        //
        self.dir - surface_normal.normalize() * (2.0 * surface_normal.dot(self.dir))
    }

    pub fn refract(&self, surface_normal: Vec2, ior: f32) -> Vec2 {
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel

        let mut cosi = clamp(-1.0, 1.0, self.dir.dot(surface_normal));
        let (mut etai, mut etat) = (1.0, ior);
        let mut n = surface_normal.normalize();
        if cosi < 0.0 {
            cosi = -cosi;
        } else {
            std::mem::swap(&mut etai, &mut etat);
            n = -surface_normal;
        }
        let eta = etai / etat;
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        if k < f32::zero() {
            self.dir.normalize() * 0.0
        } else {
            self.dir.normalize() * eta + n.normalize() * (eta * cosi - k.sqrt())
        }
    }

    // in case of material like glass, that are both refractive and reflective, fresnel equation find out how much
    // light is refracted and how much light is reflected
    // reference https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel
    pub fn fresnel(&self, surface_normal: Vec2, ior: f32) -> f64 {
        let i_dot_n = self.dir.dot(surface_normal) as f64;
        let mut eta_i = 1.0;
        let mut eta_t = ior as f64;
        if i_dot_n > 0.0 {
            eta_i = eta_t;
            eta_t = 1.0;
        }

        let sin_t = eta_i / eta_t * (1.0 as f64 - i_dot_n * i_dot_n).max(0.0).sqrt();
        if sin_t > 1.0 {
            //Total internal reflection
            return 1.0;
        } else {
            let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
            let cos_i = cos_t.abs();
            let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
            let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
            return (r_s * r_s + r_p * r_p) / 2.0;
        }
    }

    pub fn draw(&self, draw: &Draw, mag: f32, weight: f32, col: Rgb) {
        draw.arrow()
            .color(col)
            .weight(weight)
            .start(self.orig)
            .end(self.dir.normalize() * mag);
    }

    pub fn look_at(&mut self, x: f32, y: f32) {
        self.dir.x = x - self.orig.x;
        self.dir.y = y - self.orig.y;
        self.dir = self.dir.normalize();
    }

    pub fn set_dir_from_angle(&mut self, angle_in_radians: f32) {
        self.dir = vec2(angle_in_radians.cos(), angle_in_radians.sin())
    }

    pub fn intersect_segment(&self, x1: &f32, y1: &f32, x2: &f32, y2: &f32) -> Option<f32> {
        let x3 = self.orig.x;
        let y3 = self.orig.y;
        let x4 = self.orig.x + self.dir.x;
        let y4 = self.orig.y + self.dir.y;
        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

        if den != 0.0 && t > 0.0 && t < 1.0 && u > 0.0 {
            Some(u)
        } else {
            None
        }
    }

    pub fn intersect_polyline(&self, points: &Vec<Vec2>) -> Option<(f32, Vec2)> {
        if points.len() <= 1 {
            return None;
        }
        let mut distance: f32 = Float::infinity();
        let mut surface_normal: Vec2 = vec2(0.0, 0.0);
        // find the closest intersection point between the ray and the walls
        for index in 0..points.len() - 1 {
            if let Some(collision_distance) = self.intersect_segment(
                &points[index].x,
                &points[index].y,
                &points[index + 1].x,
                &points[index + 1].y,
            ) {
                if collision_distance < distance {
                    let segment_dir = (points[index] - points[index + 1]).normalize();
                    surface_normal = vec2(segment_dir.y, -segment_dir.x);
                    distance = collision_distance;
                }
            }
        }
        if distance < Float::infinity() {
            return Some((distance, surface_normal));
        } else {
            return None;
        }
    }

    pub fn intersect_circle(&self, center: &Vec2, radius: &f32) -> Option<(f32, Vec2)> {
        // let h = center - self.orig;
        // let lf = self.dir.dot(h);
        // let mut s = radius.powi(2) - h.dot(h) + lf.powi(2);

        // if s < 0.0 {
        //     None
        // } else {
        //     s = s.sqrt();
        //     let first_collision: f32;
        //     if lf < s && lf + s >= f32::zero() {
        //         s = -s;
        //         first_collision = lf + s;
        //     } else {
        //         first_collision = lf - s;
        //     }
        //     Some(first_collision)
        // }

        let l = *center - self.orig;
        let adj = l.dot(self.dir);
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = radius * radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;
        if t0 < f32::zero() && t1 < f32::zero() {
            return None;
        }
        let inside = self.orig.distance(*center) <= *radius;
        let distance = if t0 < t1 && !inside { t0 } else { t1 };
        // TODO, test this surface_normal
        let surface_normal = ((self.orig * distance)  - *center).normalize();
        Some((distance, surface_normal))
    }

    // https://github.com/rustgd/collision-rs/blob/master/src/volume/aabb/aabb2.rs
    pub fn intersect_aabb(&self, min: &Vec2, max: &Vec2) -> Option<f32> {
        let mut tmax: f32 = Float::infinity();
        let mut tmin: f32 = Float::neg_infinity();
        if self.dir.x != f32::zero() {
            let tx1 = (min.x - self.orig.x) / self.dir.x;
            let tx2 = (max.x - self.orig.x) / self.dir.x;
            tmin = tmin.max(tx1.min(tx2));
            tmax = tmax.min(tx1.max(tx2));
        } else if self.orig.x <= min.x || self.orig.x >= max.x {
            return None;
        }

        if self.dir.y != 0.0 {
            let ty1 = (min.y - self.orig.y) / self.dir.y;
            let ty2 = (max.y - self.orig.y) / self.dir.y;
            tmin = tmin.max(ty1.min(ty2));
            tmax = tmax.min(ty1.max(ty2));
        } else if self.orig.y <= min.y || self.orig.y >= max.y {
            return None;
        }

        if (tmin < f32::zero() && tmax < f32::zero()) || tmax < tmin {
            None
        } else {
            let t = if tmin >= f32::zero() { tmin } else { tmax };
            Some(t)
        }
    }

    pub fn intersect_bounding_volume(&self, volume: &BoundingVolume) -> Option<f32> {
        match volume {
            BoundingVolume::Circle { position, radius } => {
                match self.intersect_circle(position, radius){
                    Some((dist, _surface_normal)) => Some(dist), 
                    _ => None
                }
            },
            BoundingVolume::Aabb { min, max } => self.intersect_aabb(min, max),
        }
    }
}

// pub trait Intersectable {
//     fn intersect(&self, ray: &Ray2D) -> Option<f32>;
// }

// pub enum Element {
//     Circle(Vec2, f32),
//     Polyline(&Vec<Vec2>),
//     Segment(f32, f32, f32, f32),
//     //Plane(Plane),
// }

//let sphere = Element::Circle(vec2(10.0, 2.0),12,5);
// WIP https://github.com/bheisler/raytracer/blob/master/src/scene.rs
