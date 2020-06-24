use kurbo::{Rect, Vec2};

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
}

impl std::ops::AddAssign<Vec2> for Region {
    fn add_assign(&mut self, rhs: Vec2) {
        for r in &mut self.rects {
            *r = *r + rhs;
        }
    }
}
