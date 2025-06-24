use crate::matrix::transfom_traits::{Transform2x2, Transform3x3, Transform4x4};
use crate::matrix::{Matrix2x2, Matrix3x3, Matrix4x4};
use crate::vector::vector_ops::{Vector2Ops, Vector3Ops};
use crate::vector::{Vector2, Vector3, Vector4};
use num_traits::{Float, One, Zero};

// Macro to implement Transform4x4 for any 4x4 matrix with Float type
macro_rules! impl_transform_4x4 {
    ($matrix_type:ident, $float_type:ty) => {
        impl Transform4x4<$float_type> for $matrix_type<$float_type> {
            fn perspective(
                fov_y_radians: $float_type,
                aspect_ratio: $float_type,
                near: $float_type,
                far: $float_type,
            ) -> Self {
                let f = <$float_type>::one()
                    / (fov_y_radians / (<$float_type>::one() + <$float_type>::one())).tan();

                Self::from_2d_array([
                    [
                        f / aspect_ratio,
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        f,
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        far / (near - far),
                        (near * far) / (near - far),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        -<$float_type>::one(),
                        <$float_type>::zero(),
                    ],
                ])
            }

            fn look_at(
                eye: Vector3<$float_type>,
                target: Vector3<$float_type>,
                up: Vector3<$float_type>,
            ) -> Self {
                let forward = (target - eye).normalize();
                let right = forward.cross(&up).normalize();
                let up = right.cross(&forward);

                Self::from_2d_array([
                    [right.x(), up.x(), -forward.x(), <$float_type>::zero()],
                    [right.y(), up.y(), -forward.y(), <$float_type>::zero()],
                    [right.z(), up.z(), -forward.z(), <$float_type>::zero()],
                    [
                        -right.dot(&eye),
                        -up.dot(&eye),
                        forward.dot(&eye),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn translation(translation: Vector3<$float_type>) -> Self {
                Self::from_2d_array([
                    [
                        <$float_type>::one(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        translation.x(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::one(),
                        <$float_type>::zero(),
                        translation.y(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                        translation.z(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn uniform_scale(scale: $float_type) -> Self {
                Self::from_2d_array([
                    [
                        scale,
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        scale,
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        scale,
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn scale(scale: Vector3<$float_type>) -> Self {
                Self::from_2d_array([
                    [
                        scale.x(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        scale.y(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        scale.z(),
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn rotation_x(angle_radians: $float_type) -> Self {
                let cos_a = angle_radians.cos();
                let sin_a = angle_radians.sin();

                Self::from_2d_array([
                    [
                        <$float_type>::one(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                    ],
                    [<$float_type>::zero(), cos_a, -sin_a, <$float_type>::zero()],
                    [<$float_type>::zero(), sin_a, cos_a, <$float_type>::zero()],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn rotation_y(angle_radians: $float_type) -> Self {
                let cos_a = angle_radians.cos();
                let sin_a = angle_radians.sin();

                Self::from_2d_array([
                    [cos_a, <$float_type>::zero(), sin_a, <$float_type>::zero()],
                    [
                        <$float_type>::zero(),
                        <$float_type>::one(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                    ],
                    [-sin_a, <$float_type>::zero(), cos_a, <$float_type>::zero()],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn rotation_z(angle_radians: $float_type) -> Self {
                let cos_a = angle_radians.cos();
                let sin_a = angle_radians.sin();

                Self::from_2d_array([
                    [cos_a, -sin_a, <$float_type>::zero(), <$float_type>::zero()],
                    [sin_a, cos_a, <$float_type>::zero(), <$float_type>::zero()],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn rotation_axis_angle(axis: Vector3<$float_type>, angle_radians: $float_type) -> Self {
                let axis = axis.normalize();
                let cos_a = angle_radians.cos();
                let sin_a = angle_radians.sin();
                let one_minus_cos = <$float_type>::one() - cos_a;

                let x = axis.x();
                let y = axis.y();
                let z = axis.z();

                Self::from_2d_array([
                    [
                        cos_a + x * x * one_minus_cos,
                        x * y * one_minus_cos - z * sin_a,
                        x * z * one_minus_cos + y * sin_a,
                        <$float_type>::zero(),
                    ],
                    [
                        y * x * one_minus_cos + z * sin_a,
                        cos_a + y * y * one_minus_cos,
                        y * z * one_minus_cos - x * sin_a,
                        <$float_type>::zero(),
                    ],
                    [
                        z * x * one_minus_cos - y * sin_a,
                        z * y * one_minus_cos + x * sin_a,
                        cos_a + z * z * one_minus_cos,
                        <$float_type>::zero(),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn orthographic(
                left: $float_type,
                right: $float_type,
                bottom: $float_type,
                top: $float_type,
                near: $float_type,
                far: $float_type,
            ) -> Self {
                let two = <$float_type>::one() + <$float_type>::one();
                Self::from_2d_array([
                    [
                        two / (right - left),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        -(right + left) / (right - left),
                    ],
                    [
                        <$float_type>::zero(),
                        two / (top - bottom),
                        <$float_type>::zero(),
                        -(top + bottom) / (top - bottom),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        -two / (far - near),
                        -(far + near) / (far - near),
                    ],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }
        }
    };
}

// Macro to implement Transform3x3 for any 3x3 matrix with Float type
macro_rules! impl_transform_3x3 {
    ($matrix_type:ident, $float_type:ty) => {
        impl Transform3x3<$float_type> for $matrix_type<$float_type> {
            fn translation_2d(translation: Vector2<$float_type>) -> Self {
                Self::from_2d_array([
                    [<$float_type>::one(), <$float_type>::zero(), translation.x()],
                    [<$float_type>::zero(), <$float_type>::one(), translation.y()],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn rotation_2d(angle_radians: $float_type) -> Self {
                let cos_a = angle_radians.cos();
                let sin_a = angle_radians.sin();

                Self::from_2d_array([
                    [cos_a, -sin_a, <$float_type>::zero()],
                    [sin_a, cos_a, <$float_type>::zero()],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn scale_2d(scale: Vector2<$float_type>) -> Self {
                Self::from_2d_array([
                    [scale.x(), <$float_type>::zero(), <$float_type>::zero()],
                    [<$float_type>::zero(), scale.y(), <$float_type>::zero()],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }

            fn uniform_scale_2d(scale: $float_type) -> Self {
                Self::from_2d_array([
                    [scale, <$float_type>::zero(), <$float_type>::zero()],
                    [<$float_type>::zero(), scale, <$float_type>::zero()],
                    [
                        <$float_type>::zero(),
                        <$float_type>::zero(),
                        <$float_type>::one(),
                    ],
                ])
            }
        }
    };
}

// Macro to implement Transform2x2 for any 2x2 matrix with Float type
macro_rules! impl_transform_2x2 {
    ($matrix_type:ident, $float_type:ty) => {
        impl Transform2x2<$float_type> for $matrix_type<$float_type> {
            fn rotation_2d(angle_radians: $float_type) -> Self {
                let cos_a = angle_radians.cos();
                let sin_a = angle_radians.sin();

                Self::from_2d_array([[cos_a, -sin_a], [sin_a, cos_a]])
            }
        }
    };
}

// Apply the macros to all your float matrix types
impl_transform_4x4!(Matrix4x4, f32);
impl_transform_4x4!(Matrix4x4, f64);

impl_transform_3x3!(Matrix3x3, f32);
impl_transform_3x3!(Matrix3x3, f64);

impl_transform_2x2!(Matrix2x2, f32);
impl_transform_2x2!(Matrix2x2, f64);
