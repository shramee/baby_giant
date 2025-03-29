use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::AddAssign;

/// A trait for types that can be used with the baby-step giant-step algorithm
/// This algorithm solves the discrete logarithm problem: finding x where target = base^x
/// (or in additive groups like elliptic curves: target = x·base)
pub trait BsgsOps {
    /// The scalar type (typically represents field elements or integers)
    type Scalar;

    /// The group element type (e.g., points on an elliptic curve)
    type El;

    fn steps_count(&self) -> Self::Scalar;

    /// Computes and stores all baby steps
    /// Returns a map from group elements to their corresponding scalar values
    fn baby_steps(&self, base: &Self::El) -> HashMap<Self::El, Self::Scalar>;

    /// Defines the group operation between two elements (addition for elliptic curves)
    fn el_operation(&self, lhs: &Self::El, rhs: &Self::El) -> Self::El;

    /// Computes the giant step base: typically -(m·base) for a chosen m
    fn gaint_step_jump(&self, base: &Self::El) -> Self::El;

    /// Converts raw baby and giant step values into the final scalar result
    fn process_result(&self, baby: &Self::Scalar, giant: &Self::Scalar) -> Self::Scalar;

    /// The main BSGS algorithm implementation
    /// Solves for x in the equation target = x·base
    fn run(&self, base: Self::El, target: Self::El) -> Option<Self::Scalar>
    where
        Self::El: Clone + Eq + Hash + Display,
        Self::Scalar: Clone + PartialOrd + From<u32> + AddAssign + Display,
    {
        // Precompute all baby steps and store in a hash map for O(1) lookups
        let baby_steps = self.baby_steps(&base);

        // Compute the giant step base (typically -(m·base))
        let gaint_step_jump = self.gaint_step_jump(&base);

        // Start with the target element
        let mut current = target.clone();

        // Iterate through all giant steps
        let mut giant_step: Self::Scalar = 0_u32.into();
        let scalar_one: Self::Scalar = 1_u32.into();
        let steps_count = self.steps_count();
        while giant_step < steps_count {
            // Check if current element matches any baby step
            if let Some(baby_step) = baby_steps.get(&current) {
                // Found a match! Compute the final result
                return Some(self.process_result(baby_step, &giant_step));
            }

            // Apply the giant step: current = current + gaint_step_jump
            // (conceptually: target + j·(-m·base))
            current = self.el_operation(&current, &gaint_step_jump);
            giant_step += scalar_one.clone();
        }

        // No solution found
        None
    }
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct FieldU128 {
    modulus: u128,
    steps_count: u128,
}

impl FieldU128 {
    pub fn new(modulus: u128) -> Self {
        let steps_count = modulus.isqrt() + 1;
        Self {
            modulus,
            steps_count,
        }
    }
}

/// Implementation for u128 modular exponentiation
impl BsgsOps for FieldU128 {
    type El = u128;
    type Scalar = u128;

    fn steps_count(&self) -> Self::Scalar {
        self.steps_count
    }

    fn baby_steps(&self, base: &u128) -> HashMap<u128, u128> {
        let mut baby_steps = HashMap::new();
        let mut current = *base;

        let mut baby_step = 0;
        while baby_step < self.steps_count {
            baby_step += 1;
            baby_steps.insert(current, baby_step);
            current = current * base % self.modulus;
        }

        baby_steps
    }

    fn el_operation(&self, lhs: &u128, rhs: &u128) -> u128 {
        (lhs * rhs) % self.modulus
    }

        modular_exponentiation(*base, Self::STEPS_COUNT, self.modulus)
    fn gaint_step_jump(&self, base: &u128) -> u128 {
    }

    fn process_result(&self, baby: &u128, giant: &u128) -> u128 {
        let step_count = self.steps_count;
        giant * step_count + baby
    }
}

/// Modular exponentiation using square-and-multiply algorithm
fn mod_exp(base: u128, exponent: u128, modulus: u128) -> u128 {
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

#[cfg(test)]
