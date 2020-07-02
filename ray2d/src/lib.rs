#[allow(dead_code)]
use nannou::prelude::*;

pub struct Ray2D {
    orig: Vector2,
    dir: Vector2,
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

    pub fn get_dir(&self) -> &Vector2 {
        &self.dir
    }

    pub fn get_orig(&self) -> &Vector2 {
        &self.orig
    }

    pub fn set_origin(&mut self, new_orig: Vector2) {
        self.orig = new_orig
    }

    pub fn set_dir(&mut self, new_dir: Vector2) {
        self.dir = new_dir
    }

    pub fn debug_ray(&self, draw: &Draw, mag: f32) {
        draw.arrow()
            .color(RED)
            .start(self.orig)
            .end(self.dir.with_magnitude(mag));
    }

    pub fn look_at(&mut self, x: f32, y: f32) {
        self.dir.x = x - self.orig.x;
        self.dir.y = y - self.orig.y;
        self.dir.normalize();
    }

    pub fn intersect(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> Option<Vector2> {
        let x3 = self.orig.x;
        let y3 = self.orig.y;
        let x4 = self.orig.x + self.dir.x;
        let y4 = self.orig.y + self.dir.y;
        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let tri = (
            den,
            ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den,
            -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den,
        );

        match tri {
            (d, t, u) if d != 0.0 && t > 0.0 && t < 1.0 && u > 0.0 => {
                Some(vec2(x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
            }
            _ => None,
        }
    }

    pub fn intersect_r<'a>(
        &self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        collision: &'a mut Vector2,
    ) -> Option<&'a mut Vector2> {
        let x3 = self.orig.x;
        let y3 = self.orig.y;
        let x4 = self.orig.x + self.dir.x;
        let y4 = self.orig.y + self.dir.y;
        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let tri = (
            den,
            ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den,
            -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den,
        );

        match tri {
            (d, t, u) if d != 0.0 && t > 0.0 && t < 1.0 && u > 0.0 => {
                collision.x = x1 + t * (x2 - x1);
                collision.y = y1 + t * (y2 - y1);
                Some(collision)
            }
            _ => None,
        }
    }

    pub fn intersect_e<'a>(
        &self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        dist: &'a mut f32,
    ) -> Option<&'a mut f32> {
        let x3 = self.orig.x;
        let y3 = self.orig.y;
        let x4 = self.orig.x + self.dir.x;
        let y4 = self.orig.y + self.dir.y;
        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let tri = (
            den,
            ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den,
            -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den,
        );

        match tri {
            (d, t, u) if d != 0.0 && t > 0.0 && t < 1.0 && u > 0.0 => {
                *dist = t;
                Some(dist)
            }
            _ => None,
        }
    }
}
