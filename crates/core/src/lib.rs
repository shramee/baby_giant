/// A trait for types that can be used with the baby-step giant-step algorithm
pub trait BsgsOps: Sized + Clone + Eq + std::hash::Hash {
    type Scalar;

    /// Computes the operation (typically addition for elliptic curves or multiplication for integers)
    fn operate(&self, rhs: &Self) -> Self;

    /// Computes the scalar multiplication/exponentiation
    fn scalar_mul(&self, scalar: &Self::Scalar) -> Self;

    /// Returns the identity element
    fn identity() -> Self;
}

/// Implementation for u128 modular exponentiation
impl BsgsOps for u128 {
    type Scalar = u128;

    fn operate(&self, rhs: &Self) -> Self {
        (self * rhs) % Self::Scalar::MAX
    }

    fn scalar_mul(&self, scalar: &Self::Scalar) -> Self {
        modular_exponentiation(*self, *scalar, Self::Scalar::MAX)
    }

    fn identity() -> Self {
        1
    }
}

/// Modular exponentiation using square-and-multiply algorithm
fn modular_exponentiation(base: u128, exponent: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exponent;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    result
}

