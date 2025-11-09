use self::Shape::*;
use crate::{Containable, Object, Point, Region};
use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineSegment {
    Straight(Point),
    InwardCurve(Point),
    OutwardCurve(Point),
}

#[derive(Debug, Clone)]
pub enum Shape {
    Polygon(Point, Vec<LineSegment>),
    Line(Point, Point, f32),
    CurveOutward(Point, Point, f32),
    CurveInward(Point, Point, f32),
    SmallCircle(Point),
    BigDot(Point),
    Dot(Point),
    BigCircle(Point),
    Text(Point, String, f32),
    CenteredText(Point, String, f32),
    // FittedText(Region, String),
    Rectangle(Point, Point),
    Image(Region, String),
    RawSVG(String),
    // Tiling(Region, Box<Object>),
    Component {
        at: Point,
        size: (usize, usize),
        objects: Box<Vec<Object>>,
    },
}

impl Shape {
    pub fn translate(&mut self, dx: i32, dy: i32) {
        match self {
            Polygon(start, lines) => {
                start.translate(dx, dy);
                for line in lines {
                    match line {
                        LineSegment::InwardCurve(anchor)
                        | LineSegment::OutwardCurve(anchor)
                        | LineSegment::Straight(anchor) => {
                            anchor.translate(dx, dy)
                        }
                    }
                }
            }
            Line(start, end, _)
            | CurveInward(start, end, _)
            | CurveOutward(start, end, _)
            | Rectangle(start, end) => {
                start.translate(dx, dy);
                end.translate(dx, dy);
            }
            Text(anchor, _, _)
            | CenteredText(anchor, ..)
            | Dot(anchor)
            | BigDot(anchor) => anchor.translate(dx, dy),
            BigCircle(center) | SmallCircle(center) => center.translate(dx, dy),
            Image(region, ..) => region.translate(dx, dy),
            Component { at, .. } => at.translate(dx, dy),
            RawSVG(_) => {
                unimplemented!()
            }
        }
    }

    pub fn position(&self) -> Point {
        match self {
            Polygon(at, ..)
            | Line(at, ..)
            | CurveInward(at, ..)
            | CurveOutward(at, ..)
            | Rectangle(at, ..)
            | Text(at, ..)
            | CenteredText(at, ..)
            | Dot(at)
            | BigDot(at)
            | BigCircle(at)
            | SmallCircle(at)
            | Component { at, .. }
            | Image(Region { start: at, .. }, ..) => *at,
            RawSVG(_) => {
                unimplemented!()
            }
        }
    }

    pub fn translate_with(&mut self, delta: (i32, i32)) {
        self.translate(delta.0, delta.1)
    }

    pub fn teleport(&mut self, x: i32, y: i32) {
        let (current_x, current_y) = self.region().start.xy::<i32>();
        let delta_x = x - current_x;
        let delta_y = y - current_y;
        self.translate(delta_x, delta_y);
    }

    pub fn teleport_with(&mut self, position: (i32, i32)) {
        self.teleport(position.0, position.1)
    }

    pub fn region(&self) -> Region {
        match self {
            Polygon(start, lines) => {
                let mut region: Region = (start, start).into();
                for line in lines {
                    match line {
                        LineSegment::InwardCurve(anchor)
                        | LineSegment::OutwardCurve(anchor)
                        | LineSegment::Straight(anchor) => {
                            // println!(
                            //     "extending region {} with {}",
                            //     region,
                            //     Region::from((start, anchor))
                            // );
                            region = *region.max(&(start, anchor).into())
                        }
                    }
                }
                // println!("region for {:?} -> {}", self, region);
                region
            }
            Line(s, e, _) | CurveInward(s, e, _) | CurveOutward(s, e, _) => {
                let (x1, y1, x2, y2) = (s.x(), s.y(), e.y(), e.x());

                let region = Region::new(
                    (x1.min(x2), y1.min(y2)),
                    (x1.max(x2), y1.max(y2)),
                )
                .map_err(|e| {
                    anyhow!("Could not construct region of {self:?}: {e:?}")
                })
                .unwrap();

                region.enlarged(
                    if region.width() > 1 { -1 } else { 0 },
                    if region.height() > 1 { -1 } else { 0 },
                )
            }
            Rectangle(start, end) => {
                Region::new(*start, *end).unwrap().enlarged(-1, -1)
            }
            Text(anchor, _, _)
            | CenteredText(anchor, ..)
            | Dot(anchor)
            | BigDot(anchor)
            | SmallCircle(anchor) => anchor.region(),
            BigCircle(center) => center.region(),
            Image(region, ..) => *region,
            Component { at, size, .. } => Region::from_topleft(*at, *size)
                .expect("Invalid region for component"),
            RawSVG(_) => {
                unimplemented!()
            }
        }
    }

    pub fn fillable(&self) -> bool {
        !matches!(self, Line(..) | CurveInward(..) | CurveOutward(..))
    }

    pub fn hatchable(&self) -> bool {
        self.fillable() && !matches!(self, Dot(..))
    }

    pub fn point_is_on_line(self, point: Point) -> bool {
        match (&self, point) {
            (Line(s, e, _), Point::Corner(x, y)) => {
                if !self.region().contains(&point) {
                    return false;
                }

                let (sx, sy) = s.xy::<f32>();
                let (ex, ey) = e.xy::<f32>();

                let m = (ey - sy) / (ex - sx);
                let p = sy - m * sx;

                (m * x as f32 + p) as usize == y
            }
            (Line(..), _) => panic!("Point type not supported"),
            _ => panic!("{self:?} is not a line object"),
        }
    }

    pub fn meets_endpoint_of_line(&self, point: Point) -> bool {
        match self {
            Line(s, e, _) => *s == point || *e == point,
            _ => panic!("{self:?} is not a line object"),
        }
    }

    /// Check if this line intersects with another line.
    /// Panics if either shape is not a line.
    ///
    /// ```
    /// use shapemaker::{Line, Point::Center};
    /// let line = |x1: usize, y1: usize, x2: usize, y2: usize|
    ///     Line(Center(x1, y1), Center(x2, y2), 1.0);
    /// assert!(line(1, 1, 4, 4).intersects_with(line(1, 4, 4, 1)));
    /// assert!(line(7, 6, 9, 7).intersects_with(line(7, 7, 9, 4)));
    /// assert!(line(4, 4, 6, 3).intersects_with(line(5, 2, 6, 5)));
    /// ```
    pub fn intersects_with(&self, line: Shape) -> bool {
        match (self, &line) {
            (&Line(s1, e1, _), &Line(s2, e2, _)) => {
                let parameters = |s: Point, e: Point| {
                    let (sx, sy) = s.xy::<isize>();
                    let (ex, ey) = e.xy::<isize>();
                    let a = ey - sy;
                    let b = sx - ex;
                    let c = (ex * sy) - (sx * ey);
                    (a, b, c)
                };

                let distance_to_line = |p: Point, (s, e): (Point, Point)| {
                    let (x, y) = p.xy::<isize>();
                    let (a, b, c) = parameters(s, e);

                    (a * x) + (b * y) + c
                };

                let same_side_of_line =
                    |p1: Point, p2: Point, line: (Point, Point)| {
                        let d1 = distance_to_line(p1, line);
                        let d2 = distance_to_line(p2, line);

                        if (d1 == 0) || (d2 == 0) {
                            return false;
                        }

                        d1.signum() == d2.signum()
                    };

                if same_side_of_line(s2, e2, (s1, e1)) {
                    return false;
                }

                if same_side_of_line(s1, e1, (s2, e2)) {
                    return false;
                }

                let (a1, b1, _) = parameters(s1, e1);
                let (a2, b2, _) = parameters(s2, e2);

                (a1 * b2) != (a2 * b1)
            }
            _ => {
                unimplemented!(
                    "Intersection not implemented for {self:?} and {:?}",
                    line.clone()
                )
            }
        }
    }
}
