use na::{Pnt2, Pnt3, Pnt4, Vec3};
use na;
use math::Scalar;
use material::Material;

pub struct UVMaterial;

impl Material for UVMaterial {
    #[inline]
    fn ambiant(&self, _: &Pnt3<Scalar>, _: &Vec3<Scalar>, uv: &Option<Pnt2<Scalar>>) -> Pnt4<f32> {
        match *uv {
            Some(ref uvs) => {
                let ux = NumCast::from(uvs.x).expect("Conversion failed.");
                let uy = NumCast::from(uvs.y).expect("Conversion failed.");

                Pnt4::new(ux, uy, na::zero(), 1.0)
            },
            None => na::orig()
        }
    }
}

impl UVMaterial {
    #[inline]
    pub fn new() -> UVMaterial {
        UVMaterial
    }
}
