use std::ops;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2(pub isize, pub isize);

impl Vec2 {
    pub const ZERO: Vec2 = Vec2(0, 0);
    pub const DOWN: Vec2 = Vec2(0, -1);
    pub const UP: Vec2 = Vec2(0, 1);
    pub const LEFT: Vec2 = Vec2(-1, 0);
    pub const RIGHT: Vec2 = Vec2(1, 0);

    /// Chebyshev distance between two 2D vectors
    pub fn distance(&self, rhs: Vec2) -> usize {
        ((self.0 - rhs.0).abs().max((self.1 - rhs.1).abs())) as usize
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = Vec2(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_works() {
        let first = Vec2(0, 0);
        let second = Vec2(0, 1);
        assert_eq!(first.distance(second), 1);

        let first = Vec2(0, 0);
        let second = Vec2(1, 0);
        assert_eq!(first.distance(second), 1);

        let first = Vec2(0, 0);
        let second = Vec2(1, 1);
        assert_eq!(first.distance(second), 1);
    }
}
