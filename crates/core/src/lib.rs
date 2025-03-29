use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Mul, Range, Rem, Sub};

/// A trait for types that can be used with the baby-step giant-step algorithm
pub trait BsgsOps: Sized + Clone + Eq + std::hash::Hash {
    const STEPS_COUNT: u128;

    fn steps_range() -> Range<u128>;

    /// Computes the operation (typically addition for elliptic curves or multiplication for integers)
    fn baby_steps(&self) -> HashMap<Self, u128>;

    /// Computes the scalar multiplication/exponentiation
    fn scalar_mul(&self, scalar: u128) -> Self;

    /// Computes the scalar result from matched baby and giant step
    fn process_result(&self, baby: u128, giant: u128) -> u128;

    /// Returns the identity element
    fn identity() -> Self;
}

/// Implementation for u128 modular exponentiation
impl BsgsOps for u128 {
    const STEPS_COUNT: u128 = 1_048_576; // 2^20

    fn steps_range() -> Range<u128> {
        0..Self::STEPS_COUNT
    }

    fn baby_steps(&self) -> HashMap<Self, u128> {
        let mut baby_steps = HashMap::new();
        let mut current = self.clone();

        for j in 0..1_048_576 {
            baby_steps.insert(current.clone(), j);
            current *= self;
        }

        baby_steps
    }

    fn scalar_mul(&self, scalar: u128) -> Self {
        modular_exponentiation(*self, scalar, u128::MAX)
    }

    fn process_result(&self, baby: u128, giant: u128) -> u128 {
        giant * Self::STEPS_COUNT + baby
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

    let baby_steps = base.baby_steps();

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

