extern crate lambdaworks_math;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::field_extension::BLS12381PrimeField;
use lambdaworks_math::elliptic_curve::short_weierstrass::traits::IsShortWeierstrass;
use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::cyclic_group::IsGroup;

#[derive(Clone, Debug)]
pub struct BLS12381Curve;

impl IsEllipticCurve for BLS12381Curve {
    type BaseField = BLS12381PrimeField;
    type PointRepresentation = ShortWeierstrassProjectivePoint<Self>;
    fn generator() -> Self::PointRepresentation {
        Self::PointRepresentation::new([
            FieldElement::<Self::BaseField>::new_base("17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb"),
            FieldElement::<Self::BaseField>::new_base("8b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1"),
            FieldElement::one()
        ])
    }
}
impl IsShortWeierstrass for BLS12381Curve {
    fn a() -> FieldElement<Self::BaseField> {
        FieldElement::from(0)
    }

    fn b() -> FieldElement<Self::BaseField> {
        FieldElement::from(4)
    }
}
fn main() {
    let g = BLS12381Curve::generator();
    let private_key_hex = "6C616D6264617370";

    let private_key = match u64::from_str_radix(private_key_hex, 16) {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Error parsing private key: {}", e);
            return;
        }
    };
    let public_key = g.operate_with_self(private_key);

    // get x and y from affine coordinates
    let public_key_affine = public_key.to_affine();
    let x = public_key_affine.x();
    let y = public_key_affine.y();
    println!("Public key (x, y): ({}, {})", x, y);

}