use num_traits::Num;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T>
where
    T: Num,
{
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn up(self) -> Self {
        Self {
            x: self.x,
            y: self.y + T::one(),
        }
    }

    pub fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y - T::one(),
        }
    }

    pub fn right(self) -> Self {
        Self {
            x: self.x + T::one(),
            y: self.y,
        }
    }

    pub fn left(self) -> Self {
        Self {
            x: self.x - T::one(),
            y: self.y,
        }
    }
}
