pub trait Rotate {
    /// Create a rotation
    ///
    /// Create a rotation that rotates `shape` by `angle` around an axis defined
    /// by `axis`.
    fn rotate(&self, axis: [f64; 3], angle: f64) -> crate::Transform;
}

impl<T> Rotate for T
where
    T: Clone + Into<crate::Shape3d>,
{
    fn rotate(&self, axis: [f64; 3], angle: f64) -> crate::Transform {
        let shape = self.clone().into();
        let matrix: Transform3<f64> = Transform3::from_matrix_unchecked(UnitQuaternion::from_axis_angle(axis, angle).to_homogeneous());
        crate::Transform {
            shape,
            matrix,
        }
    }
}

pub trait Sketch {
    fn sketch(&self) -> crate::Sketch;
}

impl<T> Sketch for T
where
    T: AsRef<[[f64; 2]]>,
{
    fn sketch(&self) -> crate::Sketch {
        crate::Sketch::from_points(self.as_ref().to_vec())
    }
}

pub trait Difference {
    fn difference<Other>(&self, other: &Other) -> crate::Difference
    where
        Other: Clone + Into<crate::Shape3d>;
}

impl<T> Difference for T
where
    T: Clone + Into<crate::Shape3d>,
{
    fn difference<Other>(&self, other: &Other) -> crate::Difference
    where
        Other: Clone + Into<crate::Shape3d>,
    {
        let a = self.clone().into();
        let b = other.clone().into();

        crate::Difference { a, b }
    }
}

pub trait Sweep {
    fn sweep(&self, length: f64) -> crate::Sweep;
}

impl<T> Sweep for T
where
    T: Clone + Into<crate::Shape2d>,
{
    fn sweep(&self, length: f64) -> crate::Sweep {
        let shape = self.clone().into();
        crate::Sweep { shape, length }
    }
}

pub trait Translate {
    /// Create a translation
    ///
    /// Create a translation that translates `shape` by `offset`.
    fn translate(&self, offset: [f64; 3]) -> crate::Transform;
}

impl<T> Translate for T
where
    T: Clone + Into<crate::Shape3d>,
{
    fn translate(&self, offset: [f64; 3]) -> crate::Transform {
        let shape = self.clone().into();
        let matrix: Transform3<f64> = Transform3::from_matrix_unchecked(Translation::from(offset).to_homogeneous());
        crate::Transform {
            shape,
            matrix,
        }
    }
}

pub trait Union {
    fn union<Other>(&self, other: &Other) -> crate::Union
    where
        Other: Clone + Into<crate::Shape3d>;
}

impl<T> Union for T
where
    T: Clone + Into<crate::Shape3d>,
{
    fn union<Other>(&self, other: &Other) -> crate::Union
    where
        Other: Clone + Into<crate::Shape3d>,
    {
        let a = self.clone().into();
        let b = other.clone().into();

        crate::Union { a, b }
    }
}
