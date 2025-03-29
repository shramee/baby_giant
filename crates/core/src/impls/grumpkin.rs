#[cfg(test)]
mod tests {
    use ark_grumpkin::{Affine, Fr, GrumpkinConfig, Projective, G_GENERATOR_X, G_GENERATOR_Y};
    use std::time::Instant;

    #[test]
    fn affine_product() {
        let g = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);

        let now = Instant::now();
        let x: Fr = 292084935932564429263800859041549340427_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        let x: Fr = 64429263800932549340427859041292084935_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        let x: Fr = 329211671161606102760987063494629384547_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        let x: Fr = 193256442926382920849393404200859041547_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        let x: Fr = 098706349462938454732921167116160610276_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        println!("Affine product took: {:.2?}", now.elapsed());
        assert!(false);
        // grumpkin
    }

    #[test]
    fn projective_product() {
        let g: Projective = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y).into();

        let now = Instant::now();
        let x: Fr = 292084935932564429263800859041549340427_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        let x: Fr = 64429263800932549340427859041292084935_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        let x: Fr = 329211671161606102760987063494629384547_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        let x: Fr = 193256442926382920849393404200859041547_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        let x: Fr = 098706349462938454732921167116160610276_u128
            .try_into()
            .unwrap();
        let _p = g * x;

        println!("Projective product took: {:.2?}", now.elapsed());
        assert!(false);
        // grumpkin
    }
}
