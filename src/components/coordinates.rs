use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};
use bevy::prelude::Component;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
#[derive(Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (rhs_x, rhs_y): (i8, i8)) -> Self::Output {
        let x = ((self.x as i16) + rhs_x as i16) as u16;
        let y = ((self.y as i16) + rhs_y as i16) as u16;
        Self { x, y }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y)
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Coordinates: {}, {}", self.x, self.y)
    }
}