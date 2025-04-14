mod utils;
use ark_grumpkin::{Affine, Fr};
use baby_giant_core::{
    impls::grumpkin::{self, g, GrumpkinBabyGiant},
    BabyGiantOps,
};
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
pub fn grumpkin_mul() {
    let x: Fr = 4294967295_u64.into();
    let _target = g() * x;
}

#[wasm_bindgen]
pub fn baby_steps() -> Vec<String> {
    let mut grumpy_bsgs = GrumpkinBabyGiant::new(65536);
    grumpy_bsgs.baby_steps(&g());
    grumpy_bsgs
        .get_baby_steps()
        .clone()
        .into_keys()
        .map(|x| x.to_string())
        .collect()
}

#[wasm_bindgen]
pub fn grumpkin_point(x_num: u64) -> String {
    let x: Fr = x_num.into();

    let Affine { x, y, infinity: _ } = (g() * x).into();
    x.to_string() + "|" + &y.to_string()
}

#[wasm_bindgen]
pub fn grumpkin_log_test(x_num: u64) -> u64 {
    let x: Fr = if x_num == 0 {
        4294967295_u64.into()
    } else {
        x_num.into()
    };

    let target: Affine = (g() * x).into();

    grumpkin::grumpkin_bsgs_32(target)
}

#[wasm_bindgen]
pub fn grumpkin_bsgs_str_(x: &str, y: &str) -> u64 {
    grumpkin::grumpkin_bsgs_32(grumpkin::grumpkin_str_to_point(x, y))
}
