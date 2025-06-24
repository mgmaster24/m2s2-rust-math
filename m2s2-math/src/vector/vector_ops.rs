use crate::vector::{Vector2, Vector3, Vector4};
use num_traits::{Float, Zero};

/// Trait for 3D vector operations
pub trait Vector3Ops<T>
where
    T: Float + Copy,
{
    /// Calculate the length (magnitude) of the vector
    fn length(&self) -> T;

    /// Calculate the squared length (avoids sqrt for performance)
    fn length_squared(&self) -> T;

    /// Normalize the vector (return unit vector in same direction)
    fn normalize(&self) -> Self;

    /// Calculate dot product with another vector
    fn dot(&self, other: &Self) -> T;

    /// Calculate cross product with another vector
    fn cross(&self, other: &Self) -> Self;

    /// Get individual components
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
}

/// Trait for 2D vector operations
pub trait Vector2Ops<T>
where
    T: Float + Copy,
{
    /// Calculate the length (magnitude) of the vector
    fn length(&self) -> T;

    /// Calculate the squared length (avoids sqrt for performance)
    fn length_squared(&self) -> T;

    /// Normalize the vector (return unit vector in same direction)
    fn normalize(&self) -> Self;

    /// Calculate dot product with another vector
    fn dot(&self, other: &Self) -> T;

    /// Get individual components
    fn x(&self) -> T;
    fn y(&self) -> T;

    /// Rotate 90 degrees counter-clockwise
    fn perpendicular(&self) -> Self;
}

// Generic implementation for Vector3<T> where T: Float
impl<T> Vector3Ops<T> for Vector3<T>
where
    T: Float + Copy,
{
    fn length(&self) -> T {
        (self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2])
            .sqrt()
    }

    fn length_squared(&self) -> T {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }

    fn normalize(&self) -> Self {
        let len = self.length();
        if len == T::zero() {
            *self // Return original if zero vector
        } else {
            Vector3::new(self.data[0] / len, self.data[1] / len, self.data[2] / len)
        }
    }

    fn dot(&self, other: &Self) -> T {
        self.data[0] * other.data[0] + self.data[1] * other.data[1] + self.data[2] * other.data[2]
    }

    fn cross(&self, other: &Self) -> Self {
        Vector3::new(
            self.data[1] * other.data[2] - self.data[2] * other.data[1],
            self.data[2] * other.data[0] - self.data[0] * other.data[2],
            self.data[0] * other.data[1] - self.data[1] * other.data[0],
        )
    }

    fn x(&self) -> T {
        self.data[0]
    }
    fn y(&self) -> T {
        self.data[1]
    }
    fn z(&self) -> T {
        self.data[2]
    }
}

// Generic implementation for Vector2<T> where T: Float
impl<T> Vector2Ops<T> for Vector2<T>
where
    T: Float + Copy,
{
    fn length(&self) -> T {
        (self.data[0] * self.data[0] + self.data[1] * self.data[1]).sqrt()
    }

    fn length_squared(&self) -> T {
        self.data[0] * self.data[0] + self.data[1] * self.data[1]
    }

    fn normalize(&self) -> Self {
        let len = self.length();
        if len == T::zero() {
            *self
        } else {
            Vector2::new(self.data[0] / len, self.data[1] / len)
        }
    }

    fn dot(&self, other: &Self) -> T {
        self.data[0] * other.data[0] + self.data[1] * other.data[1]
    }

    fn x(&self) -> T {
        self.data[0]
    }
    fn y(&self) -> T {
        self.data[1]
    }

    fn perpendicular(&self) -> Self {
        Vector2::new(-self.data[1], self.data[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::{Vector2f32, Vector2f64, Vector3f32, Vector3f64};

    #[test]
    fn test_vector3_f32_operations() {
        let v = Vector3f32::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);

        let normalized = v.normalize();
        assert!((normalized.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_vector3_f64_operations() {
        let v = Vector3f64::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);

        let normalized = v.normalize();
        assert!((normalized.length() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector2_operations() {
        let v = Vector2f32::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);

        let perp = v.perpendicular();
        assert_eq!(perp.dot(&v), 0.0); // Perpendicular vectors have dot product of 0
    }

    #[test]
    fn test_cross_product_generic() {
        let v1 = Vector3f64::new(1.0, 0.0, 0.0);
        let v2 = Vector3f64::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross.as_slice(), [0.0, 0.0, 1.0]);
    }
}
