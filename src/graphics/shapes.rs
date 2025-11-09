use self::Shape::*;
use crate::{Containable, Point, Region};
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

    /// Check if this line intersects with another line.
    /// Panics if either shape is not a line.
    /// 
    /// ```
    /// use shapemaker::{Line, Point::Corner};
    /// let line = |x1: u32, y1: u32, x2: u32, y2: u32| Line(Center(x1, y1), Center(x2, y2), 1.0);
    /// assert!(line(1, 1, 4, 4).intersects_with(line(1, 4, 4, 1)));
    /// assert!(line(7, 6, 9, 7).intersects_with(line(7, 7, 9, 4)));
    /// assert!(line(4, 4, 6, 3).intersects_with(line(5, 2, 6, 5)));
    /// ```
    pub fn intersects_with(&self, line: Shape) -> bool {
        match (self, &line) {
            (&Line(s1, e1, _), &Line(s2, e2, _)) => {
                let (dx1, dy1) = e1 - s1;
                let (dx2, dy2) = e2 - s2;
                let (dx3, dy3) = s2 - s1;

                let det = dx1 * dy2 - dy1 * dx2;

                let det1 = dx1 * dy3 - dx3 * dy1;
                let det2 = dx2 * dy3 - dx3 * dy2;

                if det == 0 {
                    if det1 != 0 || det2 != 0 {
                        return false;
                    }

                    let (x1, y1) = s1.xy::<isize>();
                    let (x2, y2) = e1.xy::<isize>();
                    let (x3, y3) = s2.xy::<isize>();

                    if dx1 != 0 && (x1 < x3 && x3 < x2 || x1 > x3 && x3 > x2) {
                        return true;
                    }

                    if dx1 == 0 && (y1 < y3 && y3 < y2 || y1 > y3 && y3 > y2) {
                        return true;
                    }

                    return false;
                }

                let frac_less_than_one = |num: isize, den: isize| {
                    if num.signum() != den.signum() {
                        return false;
                    }

                    num.abs() <= den.abs()
                };

                frac_less_than_one(det1, det) && frac_less_than_one(det2, det)
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
