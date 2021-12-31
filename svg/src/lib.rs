//! A suite of common interpolation functions often referred to as "easing" and "tweening"
//! functions. This API is provided by the [**pennereq** crate](https://docs.rs/pennereq).
//use nannou::cgmath::{Transform, Vector3};
use nannou::color::{rgba, Rgba};
use nannou::lyon::math::Point;
use nannou::lyon::path::PathEvent;
use nannou::lyon::tessellation::{LineCap, LineJoin};
//use nannou::math::{Matrix3, Matrix4};
use usvg;

use nannou::prelude::*;

pub struct StrokeStyle {
    pub color: Rgba,
    pub weight: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
}

pub struct SvgPath {
    pub events: Vec<PathEvent>,
    pub fill_color: Option<Rgba>,
    pub stroke_style: Option<StrokeStyle>,
    pub width: f32,
    pub height: f32,
}

pub struct Svg {
    pub paths: Vec<SvgPath>,
}

impl Svg {
    pub fn load(asset: std::path::PathBuf) -> Result<Self, usvg::Error> {
        let opt = usvg::Options::default();
        let rtree = usvg::Tree::from_file(&asset, &opt)?;
        let view_box = rtree.svg_node().view_box;
        let mut paths = Vec::new();

        for node in rtree.root().descendants() {
            dbg!(&node);
            if let usvg::NodeKind::Path(ref p) = *node.borrow() {
                let t = p.transform;
                // let mut matrix: Matrix4::<f64> = Matrix3::from_cols(
                //     Vector3::new(t.a, t.b, 0.0),
                //     Vector3::new(t.c, t.d, 0.0),
                //     Vector3::new(t.e, t.f, 1.0)).into();

                let convert_color = |c: usvg::Color, opacity: usvg::NormalizedValue| -> Rgba {
                    rgba(
                        c.red as f32 / 255.0,
                        c.green as f32 / 255.0,
                        c.blue as f32 / 255.0,
                        opacity.value() as f32,
                    )
                };

                let fill_color = match &p.fill {
                    None => None,
                    Some(fill) => {
                        let opacity = fill.opacity;
                        match fill.paint {
                            usvg::Paint::Color(c) => Some(convert_color(c, opacity)),
                            _ => Some(rgba(0.0, 0.0, 0.0, 0.0)),
                        }
                    }
                };

                let stroke_style = match &p.stroke {
                    None => None,
                    Some(stroke) => {
                        let opacity = stroke.opacity;
                        let color = match stroke.paint {
                            usvg::Paint::Color(c) => convert_color(c, opacity),
                            _ => rgba(0.0, 0.0, 0.0, 0.0),
                        };
                        let line_cap = match stroke.linecap {
                            usvg::LineCap::Butt => LineCap::Butt,
                            usvg::LineCap::Round => LineCap::Round,
                            usvg::LineCap::Square => LineCap::Square,
                        };
                        let line_join = match stroke.linejoin {
                            usvg::LineJoin::Miter => LineJoin::Miter,
                            usvg::LineJoin::Round => LineJoin::Round,
                            usvg::LineJoin::Bevel => LineJoin::Bevel,
                        };
                        Some(StrokeStyle {
                            color,
                            weight: stroke.width.value() as f32,
                            line_cap,
                            line_join,
                        })
                    }
                };

                let path = convert_path(p);
                let mut path_events = Vec::new();
                for e in path {
                    path_events.push(e);
                }
                let width = view_box.rect.size().width() as f32;
                let height = view_box.rect.size().height() as f32;

                paths.push(SvgPath {
                    events: path_events,
                    fill_color,
                    stroke_style,
                    width,
                    height,
                })
            }
        }

        Ok(Svg { paths })
        // Ok(Svg {
        //     events: Vec::new(),
        //     fill_color: None,
        //     stroke_style: None,
        //     width: 0.0,
        //     height: 0.0,
        // })
    }
}

/// Some glue between usvg's iterators and lyon's.

fn point(x: &f64, y: &f64) -> Point {
    Point::new((*x) as f32, (*y) as f32)
}

pub struct PathConvIter<'a> {
    sub_path_iter: usvg::SubPathIter<'a>,
    segment_iter: Option<std::slice::Iter<'a, usvg::PathSegment>>,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
}

impl<'l> Iterator for PathConvIter<'l> {
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }

        loop {
            if self.segment_iter.is_none() {
                self.segment_iter = Some(self.sub_path_iter.next()?.0.iter());
            }

            //let mut matrix = Matrix4::<f64>::from_scale(1.0);
            //let flip = Matrix4::<f64>::from_nonuniform_scale(1.0, -1.0, 1.0);
            //matrix = matrix * flip;
            //dbg!(matrix.transform_point(cgmath::Point3::new(10.0,50.0,2.0)));

            let next = self.segment_iter.as_mut().and_then(|it| it.next());
            match next {
                Some(usvg::PathSegment::MoveTo { x, y }) => {
                    //let (x, y) = matrix.transform_point(point(x, y)).into();
                    if self.needs_end {
                        let last = self.prev;
                        let first = self.first;
                        self.needs_end = false;
                        self.prev = point(x, y);
                        self.deferred = Some(PathEvent::Begin { at: self.prev });
                        self.first = self.prev;
                        return Some(PathEvent::End {
                            last,
                            first,
                            close: false,
                        });
                    } else {
                        self.first = point(x, y);
                        return Some(PathEvent::Begin { at: self.first });
                    }
                }
                Some(usvg::PathSegment::LineTo { x, y }) => {
                    self.needs_end = true;
                    let from = self.prev;
                    self.prev = point(x, y);
                    return Some(PathEvent::Line {
                        from,
                        to: self.prev,
                    });
                }
                Some(usvg::PathSegment::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                }) => {
                    self.needs_end = true;
                    let from = self.prev;
                    self.prev = point(x, y);
                    return Some(PathEvent::Cubic {
                        from,
                        ctrl1: point(x1, y1),
                        ctrl2: point(x2, y2),
                        to: self.prev,
                    });
                }
                Some(usvg::PathSegment::ClosePath) => {
                    self.needs_end = false;
                    self.prev = self.first;
                    return Some(PathEvent::End {
                        last: self.prev,
                        first: self.first,
                        close: true,
                    });
                }
                None => {
                    self.segment_iter = None;
                    if self.needs_end {
                        self.needs_end = false;
                        let last = self.prev;
                        let first = self.first;
                        return Some(PathEvent::End {
                            last,
                            first,
                            close: false,
                        });
                    }
                }
            }
        }
    }
}

pub fn convert_path<'a>(p: &'a usvg::Path) -> PathConvIter<'a> {
    PathConvIter {
        sub_path_iter: p.data.subpaths(),
        segment_iter: None,
        first: Point::new(0.0, 0.0),
        prev: Point::new(0.0, 0.0),
        deferred: None,
        needs_end: false,
    }
}
