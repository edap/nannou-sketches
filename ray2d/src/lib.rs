#[allow(dead_code)]
use nannou::prelude::*;

#[derive(Debug)]
pub struct Ray2D {
    pub orig: Vector2,
    pub dir: Vector2,
}

impl Ray2D {
    pub fn new() -> Self {
        Ray2D {
            orig: vec2(0.0, 0.0),
            dir: vec2(1.0, 0.0),
        }
    }

    pub fn reflect(&self, surface_normal: Vector2) -> Vector2 {
        //I - 2.0 * dot(N, I) * N
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/reflect.xhtml
        //
        self.dir - surface_normal.with_magnitude(2.0 * surface_normal.dot(self.dir))
    }

    pub fn refract(&self, surface_normal: Vector2, ior: f32) -> Vector2 {
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel

        let mut cosi = clamp(-1.0, 1.0, self.dir.dot(surface_normal));
        let (mut etai, mut etat) = (1.0, ior);
        let mut n = surface_normal;
        if cosi < 0.0 {
            cosi = -cosi;
        } else {
            std::mem::swap(&mut etai, &mut etat);
            n = -surface_normal;
        }
        let eta = etai / etat;
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        if k < 0.0 {
            //vec2(0.0, 0.0)
            self.dir.with_magnitude(0.0)
        } else {
            self.dir.with_magnitude(eta) + n.with_magnitude(eta * cosi - k.sqrt())
        }
    }

    pub fn draw(&self, draw: &Draw, mag: f32, weight: f32, col: Rgb) {
        draw.arrow()
            .color(col)
            .weight(weight)
            .start(self.orig)
            .end(self.dir.with_magnitude(mag));
    }

    pub fn look_at(&mut self, x: f32, y: f32) {
        self.dir.x = x - self.orig.x;
        self.dir.y = y - self.orig.y;
        self.dir = self.dir.normalize();
    }

    pub fn set_dir_from_angle(&mut self, a_radians: f32) {
        self.dir = Vector2::from_angle(a_radians);
    }

    pub fn intersect_segment(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> Option<f32> {
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

    pub fn intersect_circle(&self, center: Vector2, radius: f32) -> Option<f32> {
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

        let l = center - self.orig;
        let adj = l.dot(self.dir);
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = radius * radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }
        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }
}
