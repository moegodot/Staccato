use ::std::num::TryFromIntError;
use std::ops::{Add, Sub};

/// The type of pixel unit.
pub type PointUnit = f32;

/// The rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct FRect {
    pub position: FPoint,
    pub size: FSize,
}

impl FRect {
    #[must_use]
    pub fn new(x: PointUnit, y: PointUnit, width: PointUnit, height: PointUnit) -> Self {
        Self {
            position: FPoint::new(x, y),
            size: FSize::new(width, height),
        }
    }

    /// Check if a point is in the rectangle.
    #[must_use]
    pub fn contains(&self, p: FPoint) -> bool {
        p.x >= self.position.x
            && p.x < (self.position.x + self.size.width)
            && p.y >= self.position.y
            && p.y < (self.position.y + self.size.height)
    }

    /// Add padding to the rectangle.
    ///
    /// It can avoid the pixel pollution when rendering.
    #[must_use]
    pub fn inset(&self, padding: PointUnit) -> Self {
        Self::new(
            self.position.x + padding,
            self.position.y + padding,
            self.size.width + (padding * 2.0f32),
            self.size.height + (padding * 2.0f32),
        )
    }
}

impl From<(FPoint, FSize)> for FRect {
    fn from((position, size): (FPoint, FSize)) -> Self {
        Self { position, size }
    }
}

impl AsRef<FPoint> for FRect {
    fn as_ref(&self) -> &FPoint {
        &self.position
    }
}

impl AsRef<FSize> for FRect {
    fn as_ref(&self) -> &FSize {
        &self.size
    }
}

impl Add<FPoint> for FRect {
    type Output = Self;

    fn add(self, rhs: FPoint) -> Self::Output {
        Self::from((
            self.position + rhs,
            self.size
        ))
    }
}

/// The point.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct FPoint {
    pub x: PointUnit,
    pub y: PointUnit,
}

impl FPoint {
    pub fn new(x: PointUnit, y: PointUnit) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0f32, y: 0f32 }
    }
}

impl std::ops::Add for FPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for FPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct FSize {
    pub width: PointUnit,
    pub height: PointUnit,
}

impl FSize {
    pub fn new(width: PointUnit, height: PointUnit) -> Self {
        Self { width, height }
    }

    /// Calculate the area of the rectangle.
    #[must_use]
    pub fn area(&self) -> PointUnit {
        self.width * self.height
    }

    #[must_use]
    pub fn outset(&self, padding: PointUnit) -> Self {
        Self::new(
            self.width + (padding * 2.0f32),
            self.height + (padding * 2.0f32),
        )
    }

    #[must_use]
    pub fn inset(&self, padding: PointUnit) -> Self {
        Self::new(
            self.width - (padding * 2.0f32),
            self.height - (padding * 2.0f32),
        )
    }

    #[must_use]
    pub fn max_dimension(&self, other: Self) -> Self {
        Self::new(self.width.max(other.width), self.height.max(other.height))
    }
}

impl From<FSize> for (PointUnit, PointUnit) {
    fn from(value: FSize) -> Self {
        (value.width, value.height)
    }
}

impl Add for FSize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.width + rhs.width,
            self.height + rhs.height,
        )
    }
}

impl Sub for FSize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.width - rhs.width,
            self.height - rhs.height,
        )
    }
}
