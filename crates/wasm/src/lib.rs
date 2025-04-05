mod utils;
use core::{impls::grumpkin::GrumpkinBabyGiant, BabyGiantOps};
use std::str::FromStr;

use ark_ff::BigInt;
use ark_grumpkin::{Affine, Fq, Fr, G_GENERATOR_X, G_GENERATOR_Y};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen]
pub fn greet() {
    set_panic_hook();
    log("Hello - BSGS wasm.".into());
}

#[wasm_bindgen]
pub fn grumpkin_ecmul() {
    let g = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);
    let x: Fr = 4294967295_u64.into();
    let _target = g * x;
}

#[wasm_bindgen]
pub fn baby_steps() -> Vec<u64> {
    let g = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);
    let grumpy_bsgs = GrumpkinBabyGiant::new(65536);
    let hashmap = grumpy_bsgs.baby_steps(&g);
    hashmap.into_values().collect()
}

#[wasm_bindgen]
pub fn grumpkin_log_test(x_num: u64) -> u64 {
    let g = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);

    let x: Fr = if x_num == 0 {
        4294967295_u64.into()
    } else {
        x_num.into()
    };

    let target: Affine = (g * x).into();

    let res = grumpkin_log(&target.x.to_string(), &target.y.to_string());

    res
}

#[wasm_bindgen]
pub fn grumpkin_log(x: &str, y: &str) -> u64 {
    let target = Affine::new_unchecked(
        Fq::new_unchecked(BigInt::from_str(x).unwrap()),
        Fq::new_unchecked(BigInt::from_str(y).unwrap()),
    );
    grumpkin_log_point(target)
}

#[wasm_bindgen]
pub fn grumpkin_point(x_num: u64) -> String {
    let g = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);

    let x: Fr = x_num.into();

    let Affine { x, y, infinity: _ } = (g * x).into();
    x.to_string() + "|" + &y.to_string()
}

fn grumpkin_log_point(target: Affine) -> u64 {
    let g = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);
    let grumpy_bsgs = GrumpkinBabyGiant::new(65536);

    let res = grumpy_bsgs.run(g, target.into());

    log(format!("Result:"));
    log(format!("{:?}", res));

    match res {
        Some(res) => res,
        None => 0,
    }
}
