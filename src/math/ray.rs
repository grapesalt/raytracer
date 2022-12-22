use super::vec::Vec3;

#[derive(Debug, Clone, Copy)]
struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    fn at(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
