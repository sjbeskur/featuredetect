#![allow(dead_code)]

use na::{SMatrix, U3, DMatrix, OMatrix, Dyn};
use nalgebra as na;


pub mod triangulate;
pub mod akaze_stereo;
pub mod traits;

type ProjectionMatrix = SMatrix<f32, 3, 4>;
type IntrinsicKMatrix = SMatrix<f32, 3, 4>;
type ExtrinsicTMatrix = SMatrix<f32, 4, 4>;
type RMatrix = SMatrix<f32, 3, 3>;

// Three Rows x N columns
// x x x x.....
// y y y y.....
// z z z z.....
type PointCloud = OMatrix<f32, U3, Dyn>;  // consider promoting this to more of a first class construct.


/// The intrinsic matrix is a transformation matrix
/// that converts points from the camera coordinate
/// system to the pixel coordinate system.
type KMat = IntrinsicKMatrix;

/// The extrinsic matrix is a 4x4 transformation matrix that
/// converts points from the world coordinate system to the
/// camera coordinate system. The camera extrinsic matrix changes
/// if the physical location/orientation of the camera is changed.
type TMat = ExtrinsicTMatrix;



