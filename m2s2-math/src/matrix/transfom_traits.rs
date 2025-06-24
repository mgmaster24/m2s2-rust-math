use crate::vector::{Vector3, Vector4};
use num_traits::{Float, Zero, One};

/// Trait for 4x4 transformation matrices
pub trait Transform4x4<T> 
where 
    T: Float + Copy,
{
    /// Create a perspective projection matrix
    fn perspective(fov_y_radians: T, aspect_ratio: T, near: T, far: T) -> Self;
    
    /// Create a look-at view matrix
    fn look_at(eye: Vector3<T>, target: Vector3<T>, up: Vector3<T>) -> Self;
    
    /// Create a translation matrix
    fn translation(translation: Vector3<T>) -> Self;
    
    /// Create a uniform scale matrix
    fn uniform_scale(scale: T) -> Self;
    
    /// Create a non-uniform scale matrix
    fn scale(scale: Vector3<T>) -> Self;
    
    /// Create rotation matrix around X axis
    fn rotation_x(angle_radians: T) -> Self;
    
    /// Create rotation matrix around Y axis
    fn rotation_y(angle_radians: T) -> Self;
    
    /// Create rotation matrix around Z axis
    fn rotation_z(angle_radians: T) -> Self;
    
    /// Create rotation matrix around arbitrary axis
    fn rotation_axis_angle(axis: Vector3<T>, angle_radians: T) -> Self;
    
    /// Create orthographic projection matrix
    fn orthographic(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self;
}

/// Trait for 3x3 transformation matrices (2D + homogeneous)
pub trait Transform3x3<T>
where
    T: Float + Copy,
{
    /// Create 2D translation matrix
    fn translation_2d(translation: crate::vector::Vector2<T>) -> Self;
    
    /// Create 2D rotation matrix
    fn rotation_2d(angle_radians: T) -> Self;
    
    /// Create 2D scale matrix
    fn scale_2d(scale: crate::vector::Vector2<T>) -> Self;
    
    /// Create 2D uniform scale matrix
    fn uniform_scale_2d(scale: T) -> Self;
}

/// Trait for 2x2 rotation matrices
pub trait Transform2x2<T>
where
    T: Float + Copy,
{
    /// Create 2D rotation matrix (pure rotation, no translation)
    fn rotation_2d(angle_radians: T) -> Self;
}
