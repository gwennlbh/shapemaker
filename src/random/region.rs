use crate::{Containable, Point, Region};
use rand::Rng;

impl Region {
    pub fn random_end(&self, start: Point) -> Point {
        // End anchors are always a square diagonal from the start anchor (for now)
        // that means taking steps of the form n * (one of (1, 1), (1, -1), (-1, 1), (-1, -1))
        // Except that the end anchor needs to stay in the bounds of the shape.

        // Determine all possible end anchors that are in a square diagonal from the start anchor
        let mut possible_end_anchors = vec![];

        // shapes can end on the next cell, since that's where they end
        let actual_region = self.enlarged(1, 1);

        for x in actual_region.mirrored_width_range() {
            for y in actual_region.mirrored_height_range() {
                let end_anchor = start.translated(x, y);

                if end_anchor == start {
                    continue;
                }

                // Check that the end anchor is in a square diagonal from the start anchor and that the end anchor is in bounds
                if x.abs() == y.abs() && actual_region.contains(&end_anchor) {
                    possible_end_anchors.push(end_anchor);
                }
            }
        }

        // Pick a random end anchor from the possible end anchors
        possible_end_anchors
            [rand::thread_rng().gen_range(0..possible_end_anchors.len())]
    }

    pub fn random(within: &Region) -> Self {
        let start = Point::random(within);
        let end = within.random_end(start);
        Region::from(if start.0 > end.0 {
            (end, start)
        } else {
            (start, end)
        })
    }

    pub fn random_point(&self) -> Point {
        Point::random(self)
    }

    pub fn random_point_except(&self, except: &Region) -> Point {
        // XXX this is probably not a good idea lmao
        loop {
            let point = self.random_point();
            if !except.contains(&point) {
                return point;
            }
        }
    }
}
