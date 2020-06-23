use kurbo::{Rect, Vec2};

#[derive(Clone, Debug)]
pub struct Region {
    rects: Vec<Rect>,
}

impl Region {
    pub const EMPTY: Region = Region { rects: Vec::new() };

    #[inline]
    pub fn rects(&self) -> &[Rect] {
        &self.rects
    }

    pub fn add_rect(&mut self, rect: Rect) {
        self.rects.push(rect);
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.clear();
        self.rects.push(rect);
    }

    pub fn clear(&mut self) {
        self.rects.clear();
    }

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
