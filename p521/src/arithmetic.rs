//! Pure Rust implementation of group operations on secp521r1.
//!
//! Curve parameters can be found in FIPS 186-4: Digital Signature Standard
//! (DSS): <https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf>
//!
//! See section D.1.2.4: Curve P-384.

#[macro_use]
mod macros;

pub(crate) mod field;
pub(crate) mod scalar;

use self::{field::FieldElement, scalar::Scalar};
use crate::NistP521;
use elliptic_curve::{
    AffineArithmetic, PrimeCurveArithmetic, ProjectiveArithmetic, ScalarArithmetic,
};
use weierstrass::WeierstrassCurve;

/// Elliptic curve point in affine coordinates.
pub type AffinePoint = weierstrass::AffinePoint<NistP521>;

/// Elliptic curve point in projective coordinates.
pub type ProjectivePoint = weierstrass::ProjectivePoint<NistP521>;

impl WeierstrassCurve for NistP521 {
    type FieldElement = FieldElement;

    const ZERO: FieldElement = FieldElement::ZERO;
    const ONE: FieldElement = FieldElement::ONE;

    /// a = 01ffffff ffffffff ffffffff ffffffff ffffffff ffffffff
    ///     ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff
    ///     ffffffff ffffffff ffffffff ffffffff fffc
    const EQUATION_A: FieldElement = FieldElement::from_be_hex("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc");

    /// b = 0051953e b9618e1c 9a1f929a 21a0b685 40eea2da 725b99b3
    ///     15f3b8b4 89918ef1 09e15619 3951ec7e 937b1652 c0bd3bb1
    ///     bf073573 df883d2c 34f1ef45 1fd46b50 3f00
    const EQUATION_B: FieldElement = FieldElement::from_be_hex("0051953eb9618e1c9a1f929a21a0b68540eea2da725b99b315f3b8b489918ef109e156193951ec7e937b1652c0bd3bb1bf073573df883d2c34f1ef451fd46b503f00");

    /// Base point of P-384.
    ///
    /// Defined in FIPS 186-4 § D.1.2.4:
    ///
    /// ```text
    /// Gₓ = 00c6858e 06b70404 e9cd9e3e cb662395 b4429c64 8139053f
    ///      b521f828 af606b4d 3dbaa14b 5e77efe7 5928fe1d c127a2ff
    ///      a8de3348 b3c1856a 429bf97e 7e31c2e5 bd66
    /// Gᵧ = 01183929 6a789a3b c0045c8a 5fb42c7d 1bd998f5 4449579b
    ///      446817af bd17273e 662c97ee 72995ef4 2640c550 b9013fad
    ///      0761353c 7086a272 c24088be 94769fd1 6650
    /// ```
    ///
    /// NOTE: coordinate field elements have been translated into the Montgomery
    /// domain.
    const GENERATOR: (FieldElement, FieldElement) = (
        FieldElement::from_be_hex("00c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66"),
        FieldElement::from_be_hex("011839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650"),
    );
}

impl AffineArithmetic for NistP521 {
    type AffinePoint = AffinePoint;
}

impl ProjectiveArithmetic for NistP521 {
    type ProjectivePoint = ProjectivePoint;
}

impl PrimeCurveArithmetic for NistP521 {
    type CurveGroup = ProjectivePoint;
}

impl ScalarArithmetic for NistP521 {
    type Scalar = Scalar;
}
