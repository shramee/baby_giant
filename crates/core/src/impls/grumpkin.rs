use ark_ff::BigInt;
use ark_grumpkin::{Affine, Fq, Fr, G_GENERATOR_X, G_GENERATOR_Y};
use std::{collections::HashMap, hash::Hash, str::FromStr};

use crate::BabyGiantOps;

/// Grumpkin generator point
pub fn g() -> Affine {
    Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y)
}

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
    type El = Affine;
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
            current = (current + base).into();
        }

        baby_steps
    }

    fn el_operation(&self, lhs: &Self::El, rhs: &Self::El) -> Self::El {
        (*lhs + *rhs).into()
    }

    fn gaint_step_jump(&self, base: &Self::El) -> Self::El {
        let m: Fr = self.steps_count.into();
        (-(*base * m)).into()
    }

    fn process_result(&self, baby: &u64, giant: &u64) -> u64 {
        let step_count = self.steps_count;
        giant * step_count + baby
    }
}

pub fn grumpkin_bsgs(target: Affine, size: u64) -> u64 {
    let g = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);
    let grumpy_bsgs = GrumpkinBabyGiant::new(size);

    let res = grumpy_bsgs.run(g, target.into());

    match res {
        Some(res) => res,
        None => 0,
    }
}

pub fn grumpkin_bsgs_32(target: Affine) -> u64 {
    grumpkin_bsgs(target, 65_536)
}

pub fn grumpkin_bsgs_40(target: Affine) -> u64 {
    grumpkin_bsgs(target, 1_048_576)
}

pub fn grumpkin_str_to_point(x: &str, y: &str) -> Affine {
    Affine::new_unchecked(
        Fq::new(BigInt::from_str(x).unwrap()),
        Fq::new(BigInt::from_str(y).unwrap()),
    )
}

#[cfg(test)]
mod tests {
    use crate::{impls::grumpkin::GrumpkinBabyGiant, BabyGiantOps};

    // #[test]
    // fn grumpkin_bsgs_40() {
    //     let g = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);
    //     let grumpy_bsgs = GrumpkinBabyGiant::new(1_048_576);

    //     let x_num = 840368900803_u64;
    //     let x: Fr = x_num.into();
    //     let target = (g * x).into();

    //     let now = Instant::now();

    //     let res = grumpy_bsgs.run(g, target);

    //     println!("Result: {:?}", res);
    //     println!("\n\nGrumpkin BSGS took: {:.2?}", now.elapsed());

    //     assert!(res.unwrap() == x_num, "Incorrect result");
    // }

    // #[test]
    // fn grumpkin_bsgs_32() {
    //     let g = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);
    //     let grumpy_bsgs = GrumpkinBabyGiant::new(65536);

    //     let x_num = 4294967295_u64;
    //     let x: Fr = x_num.into();
    //     let target = (g * x).into();

    //     let now = Instant::now();

    //     let res = grumpy_bsgs.run(g, target);

    //     println!("Result: {:?}", res);
    //     println!("\n\nGrumpkin BSGS took: {:.2?}", now.elapsed());

    //     assert!(res.unwrap() == x_num, "Incorrect result");
    // }

    // #[test]
    // pub fn grumpkin_bsgs_str() {
    //     let pt_coords = (
    //         "18404411293574529506939754020345889193409751730106812839425939610401353525304",
    //         "18142326956230734014841872757503391093517090171806600353970481267506310918723",
    //     );

    //     let r = super::grumpkin_bsgs_str(pt_coords.0, pt_coords.1);

    //     assert!(r == 35235);
    // }

    #[test]
    pub fn grumpkin_baby_steps() {
        let grumpy_bsgs = GrumpkinBabyGiant::new(32);

        let baby_steps = grumpy_bsgs.baby_steps(&super::g());

        println!("Baby steps: {:?}", baby_steps);
    }
}
