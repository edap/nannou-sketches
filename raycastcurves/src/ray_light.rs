use nannou::prelude::*;
use ray2d::Ray2D;
#[derive(Debug)]

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

#[derive(Debug)]
pub struct RayLight {
    pub intersections: Vec<Intersection>,
    pub color: Rgba,
    pub max_depth: usize,
    pub count_depth: usize,
    pub ray: Ray2D,
    pub starting_pos: Vec2,
    pub starting_dir: Vec2,
}

impl RayLight {
    pub fn new(origin: Vec2, direction: Vec2, max_depth: usize) -> Self{
        let intersections: Vec<Intersection> = Vec::new();
        let mut ray = Ray2D::new();
        ray.orig = origin;
        ray.dir = direction;
        let starting_pos = origin;
        let starting_dir = direction;
        RayLight {
            intersections,
            color: rgba(1.0,1.0,1.0,1.0),
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