use std::f64;

// Gamma function implementation taken from statrs (https://crates.io/crates/statrs), as it was
// easyer than importing the whole crate just to use it

const TWO_SQRT_E_OVER_PI: f64 = 1.860_382_734_205_265_7;

const GAMMA_R: f64 = 10.900511;
const GAMMA_DK: &[f64] = &[
    2.485_740_891_387_535_5e-5,
    1.051_423_785_817_219_7,
    -3.456_870_972_220_162_5,
    4.512_277_094_668_948,
    -2.982_852_253_235_766_4,
    1.056_397_115_771_267,
    -1.954_287_731_916_458_7e-1,
    1.709_705_434_044_412e-2,
    -5.719_261_174_043_057e-4,
    4.633_994_733_599_057e-6,
    -2.719_949_084_886_077_2e-9,
];

fn gamma(x: f64) -> f64 {
    if x < 0.5 {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        f64::consts::PI
            / ((f64::consts::PI * x).sin()
                * s
                * TWO_SQRT_E_OVER_PI
                * ((0.5 - x + GAMMA_R) / f64::consts::E).powf(0.5 - x))
    } else {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s * TWO_SQRT_E_OVER_PI * ((x - 0.5 + GAMMA_R) / f64::consts::E).powf(x - 0.5)
    }
}

fn fact_int(x: u64) -> f64 {
    let mut count: f64 = 1.;

    if x == 0 || x == 1 {
        return 1.;
    } else if x > 170 {
        return f64::INFINITY;
    }

    for i in 2..x + 1 {
        count *= i as f64;
    }

    count
}

pub fn fact(x: f64) -> f64 {
    if x < 0. {
        f64::NAN
    } else if (x - x.trunc()).abs() < std::f64::EPSILON {
        fact_int(x.round() as u64)
    } else {
        gamma(x + 1.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fact_negative() {
        assert!(fact(-1.).is_nan());
        assert!(fact(-0.1).is_nan());
        assert!(fact(-2.5).is_nan());
    }

    #[test]
    fn fact_zero() {
        assert_eq!(fact(0.), 1.);
    }

    #[test]
    fn fact_positive_integer() {
        assert_eq!(fact(1.), 1.);
        assert_eq!(fact(2.), 2.);
        assert_eq!(fact(3.), 6.);
        assert_eq!(fact(4.), 24.);
        assert_eq!(fact(5.), 120.);
    }

    #[test]
    fn fact_positive_rational() {
        assert!((fact(2.5) - 3.3233509704478403).abs() < f64::EPSILON);
        assert!((fact(3.5) - 11.631728396567521).abs() < f64::EPSILON);
    }
}
