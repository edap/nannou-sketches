use nannou::prelude::*;
use nannou_raycast::ray2d::Ray2d;
#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub pos: Vec2,
    pub color: Hsla,
    pub depth: usize,
}

impl Intersection{
    pub fn new(pos: Vec2, color: Hsla, depth: usize) -> Self { 
        Intersection {
            pos,
            color,
            depth,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RayLight {
    pub intersections: Vec<Intersection>,
    pub color: Hsla,
    pub max_depth: usize,
    pub count_depth: usize,
    pub ray: Ray2d,
    pub starting_pos: Vec2,
    pub starting_dir: Vec2,
}

impl RayLight {
    pub fn new(origin: Vec2, direction: Vec2, max_depth: usize) -> Self{
        let intersections: Vec<Intersection> = Vec::new();
        let mut ray = Ray2d::default();
        let color: Hsla = hsla(random_range(0.0, 1.0), 1.0, random_range(0.5, 1.0)  , 1.0);
        ray.orig = origin;
        ray.dir = direction;
        let starting_pos = origin;
        let starting_dir = direction;
        RayLight {
            intersections,
            color,
            max_depth,
            count_depth: 0,
            ray,
            starting_pos,
            starting_dir,
        }
    }

    pub fn reset(&mut self){
        self.ray.orig = self.starting_pos;
        self.ray.dir = self.starting_dir;
        self.intersections.clear();
        self.count_depth = 0;
    }

}