use kurbo::{BezPath, Rect, Shape, Vec2};

/// A union of rectangles, useful for describing an area that needs to be repainted.
#[derive(Clone, Debug)]
pub struct Region {
    rects: Vec<Rect>,
}

impl Region {
    /// The empty region.
    pub const EMPTY: Region = Region { rects: Vec::new() };

    /// Returns the collection of rectangles making up this region.
    #[inline]
    pub fn rects(&self) -> &[Rect] {
        &self.rects
    }

    /// Adds a rectangle to this region.
    pub fn add_rect(&mut self, rect: Rect) {
        self.rects.push(rect);
    }

    /// Replaces this region with a single rectangle.
    pub fn set_rect(&mut self, rect: Rect) {
        self.clear();
        self.rects.push(rect);
    }

    /// Sets this region to the empty region.
    pub fn clear(&mut self) {
        self.rects.clear();
    }

    /// Returns a rectangle containing this region.
    pub fn bounding_box(&self) -> Rect {
        if self.rects.is_empty() {
            Rect::ZERO
        } else {
            self.rects[1..]
                .iter()
                .fold(self.rects[0], |r, s| r.union(*s))
        }
    }

    /// Converts into a Bezier path. Note that this just gives the concatenation of the rectangle
    /// paths, which is not the smartest possible thing. Also, it's not the right answer for an
    /// even/odd fill rule.
    pub fn to_bez_path(&self) -> BezPath {
        let mut ret = BezPath::new();
        for rect in self.rects() {
            // Rect ignores the tolerance.
            ret.extend(rect.to_bez_path(0.0));
        }
        ret
    }
}

impl std::ops::AddAssign<Vec2> for Region {
    fn add_assign(&mut self, rhs: Vec2) {
        for r in &mut self.rects {
            *r = *r + rhs;
        }
    }
}

impl From<Rect> for Region {
    fn from(rect: Rect) -> Region {
        Region { rects: vec![rect] }
    }
}