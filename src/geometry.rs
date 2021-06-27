#[derive(Clone, Copy)]
pub struct V2 {
    pub x: f32,
    pub y: f32,
}

impl V2 {
    pub fn zero() -> V2 {
        V2 { x: 0.0, y: 0.0 }
    }

    pub fn add(self, other: V2) -> V2 {
        V2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(self, other: V2) -> V2 {
        V2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn mul(self, multiplier: f32) -> V2 {
        V2 {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }

    pub fn div(self, divisor: f32) -> V2 {
        V2 {
            x: self.x / divisor,
            y: self.y / divisor,
        }
    }

    pub fn diff(b: V2, a: V2) -> V2 {
        V2 {
            x: b.x - a.x,
            y: b.y - a.y,
        }
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x.powf(2.0) + self.y.powf(2.0)
    }

    pub fn distance(a: V2, b: V2) -> f32 {
        V2::diff(b, a).length()
    }

    pub fn distance_squared(a: V2, b: V2) -> f32 {
        V2::diff(b, a).length_squared()
    }

    pub fn distance_to(self, target: V2) -> f32 {
        V2::distance(self, target)
    }

    pub fn distance_to_squared(self, target: V2) -> f32 {
        V2::distance_squared(self, target)
    }

    pub fn unit(self) -> V2 {
        let length = self.length();
        if length > 0.0 {
            self.div(length)
        } else {
            self
        }
    }

    pub fn towards(self, target: V2, max_step: f32) -> V2 {
        let diff = V2::diff(target, self);
        let distance = diff.length();
        if distance < max_step {
            return target;
        } else if distance > 0.0 {
            return self.add(diff.mul(max_step / distance));
        } else {
            return self;
        }
    }
}