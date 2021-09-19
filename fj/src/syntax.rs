//! Extension traits that provide simplified syntax for various operations
//!
//! Import the prelude (`use fj::prelude::*;`) to make these traits available to
//! you.

use nalgebra::SVector;

use crate::{geometry::operations, model};

/// Provides convenient syntax for [`operations::Difference`]
///
/// This trait is implemented for tuples with two entries. The call
/// `(a, b).difference()` will return the difference of `a` and `b`.
pub trait Difference<A, B> {
    fn difference(self) -> operations::Difference<A, B>;
}

impl<A, B> Difference<A, B> for (A, B) {
    fn difference(self) -> operations::Difference<A, B> {
        operations::Difference {
            a: self.0,
            b: self.1,
        }
    }
}

/// Provides convenient syntax for [`model::WithResolution`]
///
/// This trait is implemented for all types. The call `geometry.resolution(res)`
/// will wrap `geometry` in a `model::WithResolution` struct, which can then be
/// converted into a triangle mesh, using the resolution specified.
pub trait Resolution: Sized {
    fn resolution(self, resolution: f32) -> model::WithResolution<Self> {
        model::WithResolution {
            geometry: self,
            resolution,
        }
    }
}

impl<Geometry> Resolution for Geometry {}

/// Provides convenient syntax for [`operations::Sweep`]
///
/// This trait is implemented for all types. The call `shape.sweep(distance)`
/// will create a sweep of `shape` over `distance`.
pub trait Sweep<T, Path> {
    fn sweep(self, distance: Path) -> operations::Sweep<T, Path>;
}

impl<T, Path> Sweep<T, Path> for T {
    fn sweep(self, distance: Path) -> operations::Sweep<T, Path> {
        operations::Sweep {
            shape: self,
            path: distance,
        }
    }
}

/// Provides convenient syntax for [`operations::Translate`]
///
/// This trait is implemented for all types. The call `shape.translate(offset)`
/// will translate `shape` by `offset`.
pub trait Translate<T, const D: usize> {
    fn translate(self, offset: SVector<f32, D>) -> operations::Translate<T, D>;
}

impl<T, const D: usize> Translate<T, D> for T {
    fn translate(self, offset: SVector<f32, D>) -> operations::Translate<T, D> {
        operations::Translate {
            shape: self,
            offset,
        }
    }
}
