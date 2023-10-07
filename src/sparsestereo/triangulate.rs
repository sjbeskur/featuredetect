use nalgebra as na;
use na::{ Matrix3, Matrix3x1 , Scalar, SMatrix};
use num::{ Zero, One };
use camera_models::cameras::{PinholeCamera, LinearizeProjection, PerspectiveProjection };

pub trait MyScalar: Scalar + Zero + One + na::ClosedAdd + na::ClosedSub + na::ClosedMul + na::ClosedDiv + Copy{}
impl<T: Scalar + Zero + One + na::ClosedAdd + na::ClosedSub + na::ClosedMul + na::ClosedDiv + Copy> MyScalar for T {}

type Matrix3f = SMatrix<f32, 3, 3>;
type Matrix3x1f = SMatrix<f32, 3, 1>;
type Matrix2x1f = SMatrix<f32, 2, 1>;

//pub fn triangulate<T: Scalar + Zero + One + na::ClosedAdd + na::ClosedMul + Copy>(rotation: &Matrix3<T>, translation: &Matrix3x1<T>, p1: &Matrix3x1<T>, p2: &Matrix3x1<T>)
pub fn triangulate<T: MyScalar>(rotation: &Matrix3<T>, translation: &Matrix3x1<T>, x1: &Matrix3x1<T>, x2: &Matrix3x1<T>)
-> T{
    let y  = x2.cross(&(rotation * x1));
    let z = y / y.dot(&y);
    translation.cross(&x2).dot(&z)
}

pub fn triangulate_one(R: &Matrix3f, t: &Matrix3x1f, camera: &PinholeCamera, &x1 : &Matrix3x1f, x2: &Matrix3x1f) -> f32{
    let mut d = triangulate(R, t, &x1, &x2);
    let e: f32 = 0.0;
    let u2 = camera.project(&x2);
    let jp_d = R*x1;
    let mut z: Matrix2x1f; 

    for _ in 0..3{
        let mut juv_p: na::SMatrix<f32, 2,3> = na::SMatrix::zeros();
        let tmp  = d * jp_d;
        //let u1: Matrix2x1f = camera.project_linearize(d * jp_d, juv_p);
        let u1 = camera.project_linearize(&tmp, &mut juv_p);
        let jacob: Matrix2x1f = juv_p * jp_d;
        z = u2 - u1;
        d += jacob.dot(&z) / jacob.dot(&jacob)
    }

    d
}


#[test]
fn test_triangulate()  {
    use nalgebra::SMatrix;

    type Matrix3f = SMatrix<f32, 3, 3>;
    let rot = Matrix3f::new(1.0, 0.0, 0.0, 
                            0.0, 1.0, 0.0,
                            0.0, 0.0, 1.0);

    type Matrix3x1f = SMatrix<f32, 3, 1>;
    let trans = Matrix3x1f::new(1.0, 0.0, 0.0);


    let p1 = Matrix3x1f::new(1.0f32, 2.0, 3.0);
    let p2 = Matrix3x1f::new(1.0f32, 2.0, 3.0);


    let r = triangulate(&rot, &trans, &p1, &p2);
    println!("{}", r);
}