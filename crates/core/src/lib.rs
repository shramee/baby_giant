use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Mul, Range, Rem, Sub};

/// A trait for types that can be used with the baby-step giant-step algorithm
pub trait BsgsOps: Sized + Clone + Eq + std::hash::Hash {
    type Scalar;

    const ORDER: Self::Scalar;
    const ORDER_ROOT: Self::Scalar;

    fn steps_range() -> Range<Self::Scalar>;

    /// Computes the operation (typically addition for elliptic curves or multiplication for integers)
    fn operate(&self, rhs: &Self) -> Self;

    /// Computes the scalar multiplication/exponentiation
    fn scalar_mul(&self, scalar: &Self::Scalar) -> Self;

    /// Computes the scalar result from matched baby and giant step
    fn process_result(&self, baby: &Self::Scalar, giant: &Self::Scalar) -> Self::Scalar;

    /// Returns the identity element
    fn identity() -> Self;
}

/// Implementation for u128 modular exponentiation
impl BsgsOps for u128 {
    type Scalar = u128;

    const ORDER: u128 = 1_099_511_627_776; // 2^40
    const ORDER_ROOT: u128 = 1_048_576; // 2^20

    fn steps_range() -> Range<u128> {
        0..Self::ORDER_ROOT
    }

    fn operate(&self, rhs: &Self) -> Self {
        (self * rhs) % Self::Scalar::MAX
    }

    fn scalar_mul(&self, scalar: &Self::Scalar) -> Self {
        modular_exponentiation(*self, *scalar, Self::Scalar::MAX)
    }

    fn process_result(&self, baby: &Self::Scalar, giant: &Self::Scalar) -> Self::Scalar {
        (giant * Self::ORDER_ROOT + baby) % Self::ORDER
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

/// Computes the discrete logarithm using the baby-step giant-step algorithm
///
/// Solves for x in the equation: g^x ≡ h (mod p)
/// Or more generally, solves for x in: g.scalar_mul(x) == h
///
/// Returns Some(x) if a solution is found, None otherwise
pub fn baby_step_giant_step<T: BsgsOps>(
    base: &T,
    target: &T,
    modulus: &T::Scalar,
) -> Option<T::Scalar>
where
    T::Scalar: Clone
        + Copy
        + Sub<Output = T::Scalar>
        + Rem<Output = T::Scalar>
        + Mul<Output = T::Scalar>
        + PartialOrd
        + From<u32>
        + Debug
        + Eq
        + std::hash::Hash,
    std::ops::Range<<T as BsgsOps>::Scalar>: IntoIterator,
{
    // let m = num_integer::sqrt(*order);
    // let n = ((*order + m - 1) / m); // Ceiling division
    let m = T::ORDER_ROOT;
    let n = m;

    // Precompute baby steps
    let mut baby_steps = HashMap::new();
    let mut current = T::identity();

    for j in T::steps_range() {
        baby_steps.insert(current.clone(), j);
        current = current.operate(base);
    }

    // Compute g^(-m)
    let neg_m = *modulus - (m % *modulus);
    let giant_step_base = base.scalar_mul(&neg_m);

    // Compute giant steps
    let mut current = target.clone();

    for giant_step in T::steps_range() {
        if let Some(&baby_step) = baby_steps.get(&current) {
            return Some(current.process_result(&baby_step, &giant_step));
        }
        current = current.operate(&giant_step_base);
    }

    None
}

