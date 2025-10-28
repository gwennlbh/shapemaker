use rand::{Rng, distr::uniform::SampleRange};

use crate::{LineSegment, Object, Point, Region};

impl Object {
    pub fn random_starting_at<R: rand::Rng>(
        rng: &mut R,
        start: Point,
        region: &Region,
        line_width: f32,
        polygon_vertices_range: impl SampleRange<usize>,
    ) -> Self {
        match rng.random_range(1..=7) {
            1 => Self::random_polygon(rng, region, polygon_vertices_range),
            2 => Self::BigCircle(start),
            3 => Self::SmallCircle(start),
            4 => Self::Dot(start),
            5 => Self::CurveInward(
                start,
                region.random_end(rng, start),
                line_width,
            ),
            6 => Self::CurveOutward(
                start,
                region.random_end(rng, start),
                line_width,
            ),
            7 => Self::Line(
                Point::random(rng, region),
                Point::random(rng, region),
                line_width,
            ),
            _ => unreachable!(),
        }
    }

    pub fn random_polygon<R: rand::Rng>(
        rng: &mut R,
        region: &Region,
        vertices_range: impl SampleRange<usize>,
    ) -> Object {
        let number_of_anchors = rng.random_range(vertices_range);
        let start = Point::random(rng, region);
        let mut lines: Vec<LineSegment> = vec![];
        for _ in 0..number_of_anchors {
            let next_anchor = Point::random(rng, region);
            lines.push(Self::random_line_segment(rng, next_anchor));
        }
        Object::Polygon(start, lines)
    }

    pub fn random_line_segment<R: rand::Rng>(
        rng: &mut R,
        end: Point,
    ) -> LineSegment {
        match rng.random_range(1..=3) {
            1 => LineSegment::Straight(end),
            2 => LineSegment::InwardCurve(end),
            3 => LineSegment::OutwardCurve(end),
            _ => unreachable!(),
        }
    }

    pub fn random(
        rng: &mut impl Rng,
        region: &Region,
        line_width: f32,
        polygon_vertices_range: impl SampleRange<usize>,
    ) -> Object {
        let start = Point::random(rng, region);
        Object::random_starting_at(
            rng,
            start,
            region,
            line_width,
            polygon_vertices_range,
        )
    }

    pub fn random_curve_within(
        rng: &mut impl Rng,
        region: &Region,
        line_width: f32,
    ) -> Object {
        let start = region.random_point(rng);
        match rng.random_range(1..=3) {
            1 => Object::CurveInward(
                start,
                region.random_end(rng, start),
                line_width,
            ),
            2 => Object::CurveOutward(
                start,
                region.random_end(rng, start),
                line_width,
            ),
            3 => Object::Line(
                Point::random(rng, region),
                Point::random(rng, region),
                line_width,
            ),
            _ => unreachable!(),
        }
    }
}
