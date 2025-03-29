use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Mul, Range, Rem, Sub};

/// A trait for types that can be used with the baby-step giant-step algorithm
/// This algorithm solves the discrete logarithm problem: finding x where target = base^x
/// (or in additive groups like elliptic curves: target = x·base)
pub trait BsgsOps {
    /// The scalar type (typically represents field elements or integers)
    type Scalar;

    /// The group element type (e.g., points on an elliptic curve)
    type El;

    /// The number of steps to use in the algorithm, affects time-space tradeoff
    const STEPS_COUNT: u128;

    /// Returns the range of steps to iterate through in the giant step phase
    fn steps_range() -> Range<u128>;

    /// Computes and stores all baby steps
    /// Returns a map from group elements to their corresponding scalar values
    fn baby_steps(&self, base: &Self::El) -> HashMap<Self::El, u128>;

    /// Defines the group operation between two elements (addition for elliptic curves)
    fn el_operation(&self, lhs: &Self::El, rhs: &Self::El) -> Self::El;

    /// Computes the giant step base: typically -(m·base) for a chosen m
    fn giant_step_base(&self, base: &Self::El) -> Self::El;

    /// Converts raw baby and giant step values into the final scalar result
    fn process_result(&self, baby: u128, giant: u128) -> Self::Scalar;

    /// The main BSGS algorithm implementation
    /// Solves for x in the equation target = x·base
    fn run(&self, base: Self::El, target: Self::El) -> Option<Self::Scalar>
    where
        Self::El: Eq + Hash,
    {
        // Precompute all baby steps and store in a hash map for O(1) lookups
        let baby_steps = self.baby_steps(&base);

        // Compute the giant step base (typically -(m·base))
        let giant_step_base = self.giant_step_base(&base);

        // Start with the target element
        let mut current = target;

        // Iterate through all giant steps
        for giant_step in Self::steps_range() {
            // Check if current element matches any baby step
            if let Some(&baby_step) = baby_steps.get(&current) {
                // Found a match! Compute the final result
                return Some(self.process_result(baby_step, giant_step));
            }

            // Apply the giant step: current = current + giant_step_base
            // (conceptually: target + j·(-m·base))
            current = self.el_operation(&current, &giant_step_base);
        }

        // No solution found
        None
    }
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

