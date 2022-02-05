use fj::Transform;
use nalgebra::{center, Isometry3, Transform3};
use parry3d_f64::bounding_volume::AABB;

use crate::{
    debug::DebugInfo,
    kernel::{
        topology::{edges::Edges, faces::Faces},
        Shape,
    },
    math::{Point, Vector},
};

impl Shape for fj::Transform {
    fn bounding_volume(&self) -> AABB {

        self.shape.bounding_volume().transform_by(&isometry(self))

        // let ls_center = center(&self.mins, &self.maxs);
        //
        // let m = &isometry(self);
        // let center = Transform3::<f64>::transform_point(&m, &ls_center);
        //
        // let half: f64 = 0.5;
        //
        // half_extents = (self.maxs - self.mins) * half;
        //
        // m.into_inner().remove_row(4).abs(); //to_rotation_matrix().into_inner().abs() * *v
        //
        // let ws_half_extents = m.absolute_transform_vector(&self.half_extents());
    }

    fn faces(&self, tolerance: f64, debug_info: &mut DebugInfo) -> Faces {
        self.shape
            .faces(tolerance, debug_info)
            .transform(&isometry(self))
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vec<Point<3>> {
        todo!()
    }
}

fn isometry(transform: &fj::Transform) -> Transform3<f64> {
    transform.matrix
}
