mod matrix;
mod quaternian;
mod vector;

pub use crate::matrix::{
    Matrix2x2f32, Matrix2x2f64, Matrix2x2i32, Matrix2x2i64, Matrix3x3f32, Matrix3x3f64,
    Matrix3x3i32, Matrix3x3i64, Matrix4x4f32, Matrix4x4f64, Matrix4x4i32, Matrix4x4i64,
};
pub use crate::vector::{
    Vector2f32, Vector2f64, Vector2i32, Vector2i64, Vector3f32, Vector3f64, Vector3i32, Vector3i64,
    Vector4f32, Vector4f64, Vector4i32, Vector4i64,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_2d_i32_vecs() {
        let v1 = Vector2i32::new(1, 2);
        let v2 = Vector2i32::new(2, 1);
        assert_eq!(v1 + v2, Vector2i32::new(3, 3));
    }

    #[test]
    fn add_2d_i64_vecs() {
        let v1 = Vector2i64::new(10000, 10000);
        let v2 = Vector2i64::new(20000, 20000);
        assert_eq!(v1 + v2, Vector2i64::new(30000, 30000));
    }

    #[test]
    fn mul_mat3x3_with_vc3() {
        let m3x3 = Matrix3x3f32::identity();
        let v3 = Vector3f32::new(1.0, 0.0, 0.0);
        let res = m3x3 * v3;
        assert_eq!(res, Vector3f32::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn mul_mat3x3_with_mat3x3() {
        let m3x31 = Matrix3x3f32::identity();
        let m3x32 = Matrix3x3f32::identity();
        let res = m3x31 * m3x32;
        assert_eq!(res, Matrix3x3f32::identity());
    }
}
