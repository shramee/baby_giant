use std::hash::Hash;
use std::ops::AddAssign;

/// A trait for types that can be used with the baby-step giant-step algorithm
/// This algorithm solves the discrete logarithm problem: finding x where target = base^x
/// (or in additive groups like elliptic curves: target = x·base)
pub trait BabyGiantOps {
    /// The scalar type (typically represents field elements or integers)
    type Scalar;

    /// The group element type (e.g., points on an elliptic curve)
    type El;

    fn steps_count(&self) -> Self::Scalar;

    /// Computes and stores all baby steps
    /// Returns a map from group elements to their corresponding scalar values
    fn baby_steps(&mut self, base: &Self::El);

    /// Checks if the given element is in the precomputed baby steps
    /// Returns the corresponding scalar value if found or None
    fn in_baby_steps(&self, target: &Self::El) -> Option<&Self::Scalar>;

    /// Defines the group operation between two elements (addition for elliptic curves)
    fn el_operation(&self, lhs: &Self::El, rhs: &Self::El) -> Self::El;

    /// Computes the giant step base: typically -(m·base) for a chosen m
    fn gaint_step_jump(&self, base: &Self::El) -> Self::El;

    /// Converts raw baby and giant step values into the final scalar result
    fn process_result(&self, baby: &Self::Scalar, giant: &Self::Scalar) -> Self::Scalar;

    /// The main BSGS algorithm implementation
    /// Solves for x in the equation target = x·base
    fn run(&mut self, base: Self::El, target: Self::El) -> Option<Self::Scalar>
    where
        Self::El: Clone + Eq + Hash,
        Self::Scalar: Clone + PartialOrd + From<u32> + AddAssign,
    {
        // Precompute all baby steps and store in a hash map for O(1) lookups
        self.baby_steps(&base);

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
            if let Some(baby_step) = self.in_baby_steps(&current) {
                // Found a match! Compute the final result
                return Some(self.process_result(baby_step, &giant_step));
            }
            // Apply the giant step, target + giant_step·(-m·base))
            current = self.el_operation(&current, &gaint_step_jump);
            giant_step += scalar_one.clone();
        }

        // No solution found
        None
    }
}
