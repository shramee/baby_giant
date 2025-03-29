use ark_grumpkin::{Fr, Projective};

use std::collections::HashMap;
use std::hash::Hash;

use crate::BabyGiantOps;

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct GrumpkinBabyGiant {
    steps_count: u64,
}

impl GrumpkinBabyGiant {
    pub fn new(steps_count: u64) -> Self {
        Self { steps_count }
    }
}

/// Implementation for u128 modular exponentiation
impl BabyGiantOps for GrumpkinBabyGiant {
    type El = Projective;
    type Scalar = u64;

    fn steps_count(&self) -> Self::Scalar {
        self.steps_count
    }

    fn baby_steps(&self, base: &Self::El) -> HashMap<Self::El, u64> {
        let mut baby_steps = HashMap::new();
        let mut current = *base;

        let mut baby_step = 0;
        while baby_step < self.steps_count {
            baby_step += 1;
            baby_steps.insert(current, baby_step);
            current = current + base;
        }

        baby_steps
    }

    fn el_operation(&self, lhs: &Self::El, rhs: &Self::El) -> Self::El {
        lhs + rhs
    }

    fn gaint_step_jump(&self, base: &Self::El) -> Self::El {
        let m: Fr = self.steps_count.into();
        -(*base * m)
    }

    fn process_result(&self, baby: &u64, giant: &u64) -> u64 {
        let step_count = self.steps_count;
        giant * step_count + baby
    }
}

#[cfg(test)]
mod tests {
    use ark_grumpkin::{Affine, Fr, Projective, G_GENERATOR_X, G_GENERATOR_Y};
    use std::time::Instant;

    use crate::{impls::grumpkin::GrumpkinBabyGiant, BabyGiantOps};

    #[test]
    fn grumpkin_bsgs_32() {
        let g: Projective = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y).into();
        let grumpy_bsgs = GrumpkinBabyGiant::new(65536);

        let x_num = 4294967295_u64;
        let x: Fr = x_num.into();
        let target = g * x;

        let now = Instant::now();

        let res = grumpy_bsgs.run(g, target);

        println!("Result: {:?}", res);
        println!("\n\nGrumpkin BSGS took: {:.2?}", now.elapsed());

        assert!(res.unwrap() == x_num, "Incorrect result");
        // assert!(false, "\n\nAll good, but this test is disabled for now\n\n");
    }
}
