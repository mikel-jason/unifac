use std::collections::HashMap;

use crate::functional_group::FunctionalGroup;
use crate::interaction::InteractionParameter;
use crate::substance::Substance;

/// Calc 1: Substance's r
///
/// Arguments
/// - `substance` - Target substance
pub fn calc_1(substance: &Substance) -> f64 {
    let mut sum = 0.0;
    for fg in &substance.functional_groups {
        sum += fg.nu * fg.r;
    }
    sum
}

/// Calc 2: Substance's q
///
/// Arguments
/// - `substance` - Target substance
pub fn calc_2(substance: &Substance) -> f64 {
    let mut sum = 0.0;
    for fg in &substance.functional_groups {
        sum += fg.nu * fg.q;
    }
    sum
}

/// Calc 3: Substance's intermediate value phi
///
/// Arguments
/// - `id` - Index of the substance, phi should be calced for
/// - `substances` - All substances
/// - `r_i` - Vec containing the substances' r_i (ordered as `substances`)
pub fn calc_3(id: usize, substances: &Vec<Substance>, r_i: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..substances.len() {
        sum += substances[i].fraction * r_i[i];
    }
    r_i[id] / sum
}

/// Calc 4: Substance's intermediate value theta
///
/// Arguments
/// - `id` - Index of the substance in `substances`, phi should be calced for
/// - `substances` - All substances
/// - `q_i` - Vec containing the substances' q_i (ordered as `substances`)
pub fn calc_4(id: usize, substances: &Vec<Substance>, q_i: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..substances.len() {
        sum += substances[i].fraction * q_i[i];
    }
    q_i[id] / sum
}

/// Calc 5: Substance's intermediate value l
///
/// Implicitly uses constant `z` as 10
///
/// Arguments
/// - `r` - Target substance's `r_i`
/// - `q` - Target substance's `q_i`
pub fn calc_5(r: f64, q: f64) -> f64 {
    5.0 * (r - q) - (r - 1.0)
}

pub fn calc_6(id: u8, substance: &Substance) -> f64 {
    let mut sum = 0.0;
    let mut nu_m = 0.0;
    for fg in &substance.functional_groups {
        sum += fg.nu;
        if fg.id == id {
            nu_m = fg.nu;
        }
    }
    nu_m / sum
}

pub fn calc_7_sum(substances: &Vec<Substance>) -> f64 {
    let mut sum = 0.0;
    for substance in substances {
        for fg in &substance.functional_groups {
            sum += substance.fraction * fg.nu;
        }
    }
    sum
}

pub fn calc_7(id: u8, sum: f64, substances: &Vec<Substance>) -> f64 {
    let mut single_sum = 0.0;
    for substance in substances {
        for fg in &substance.functional_groups {
            if fg.id == id {
                single_sum += substance.fraction * fg.nu;
            }
        }
    }
    single_sum / sum
}

pub fn calc_8_sum(x_k: &HashMap<u8, f64>) -> f64 {
    let mut sum = 0.0;
    for (id, x) in x_k {
        let fg = FunctionalGroup::from(*id, 0.0).unwrap();
        sum += fg.q * x;
    }
    sum
}

pub fn calc_8(id: u8, x_k: &HashMap<u8, f64>, sum: f64) -> f64 {
    let fg = FunctionalGroup::from(id, 0.0).unwrap();
    fg.q * x_k[&id] / sum
}

pub fn calc_9(id: u8, x_ki: &HashMap<u8, f64>, sum: f64) -> f64 {
    calc_8(id, x_ki, sum)
}

pub fn calc_10(i: u8, j: u8, temperature: f64) -> Result<f64, &'static str> {
    let interp = InteractionParameter::from(i, j)?;
    Ok(std::f64::consts::E.powf(-interp.a_ij / temperature))
}

pub fn calc_11_sum_1(theta_m: &Vec<f64>, psi_mk: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for (theta, psi) in theta_m.iter().zip(psi_mk) {
        sum += theta * psi
    }
    sum
}

pub fn calc_11_sum_2(k_index: usize, theta_m: &Vec<f64>, psi_nm: &Vec<Vec<f64>>) -> f64 {
    let mut sum = 0.0;
    for m in 0..theta_m.len() {
        let numerator = theta_m[m] * psi_nm[k_index][m];
        let mut psi_n: Vec<f64> = Vec::new();
        for psi in psi_nm {
            psi_n.push(psi[m]);
        }
        let denominator = calc_11_sum_1(theta_m, psi_n);
        sum += numerator / denominator;
    }
    sum
}

pub fn calc_11(q_k: f64, sum_1: f64, sum_2: f64) -> f64 {
    q_k * (1.0 - sum_1.ln() - sum_2)
}

pub fn calc_12(q_k: f64, sum_1: f64, sum_2: f64) -> f64 {
    calc_11(q_k, sum_1, sum_2)
}

/// Calc 15: Sustance's combinatorial activity coeff. part
///
/// Calcs the sum across all substances used as denominator
///
/// Arguments
/// - `substances` - All substances
/// - `l_i` - Substances' l_i (ordered as `substances`)
pub fn calc_15_sum(substances: &Vec<Substance>, l_i: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..substances.len() {
        sum += substances[i].fraction * l_i[i];
    }
    sum
}

/// Calc 15: Sustance's combinatorial activity coeff. part
///
/// Calcs the sum across all substances used as denominator
///
/// Arguments
/// - `x` - Substance's fraction
/// - `q` - Substance's `q`
/// - `phi` - Substances `phi`
/// - `theta` - Substances `theta`
/// - `l` - Substances `l`
/// - `sum_lx` - Sum (denominator), see calc_15_sum
pub fn calc_15(x: f64, q: f64, phi: f64, theta: f64, l: f64, sum_lx: f64) -> f64 {
    let mut res = (phi / x).log2(); // 4
    res += 5.0 * q * (theta / phi).log2();
    res += l;
    res - phi / x * sum_lx
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_1() {
        let substance = Substance {
            fraction: 1.0,
            functional_groups: vec![
                FunctionalGroup::from(1, 1.0).unwrap(),
                FunctionalGroup::from(2, 1.0).unwrap(),
                FunctionalGroup::from(4, 3.0).unwrap(),
            ],
            gamma: None,
        };
        let test = super::calc_1(&substance);
        assert_eq!(test, 2.234);
    }

    #[test]
    fn calc_2() {
        let substance = Substance {
            fraction: 1.0,
            functional_groups: vec![
                FunctionalGroup::from(1, 1.0).unwrap(),
                FunctionalGroup::from(2, 1.0).unwrap(),
                FunctionalGroup::from(4, 3.0).unwrap(),
            ],
            gamma: None,
        };
        let test = super::calc_2(&substance);
        assert_eq!(test, 1.388);
    }

    #[test]
    fn calc_3() {
        let ethane = Substance {
            fraction: 0.5,
            functional_groups: vec![FunctionalGroup::from(1, 2.0).unwrap()],
            gamma: None,
        };
        let benzene = Substance {
            fraction: 0.5,
            functional_groups: vec![FunctionalGroup::from(9, 6.0).unwrap()],
            gamma: None,
        };
        let substances = vec![ethane, benzene];
        let r_i = vec![0.9011 * 2.0, 0.5313 * 6.0];
        let val = super::calc_3(0, &substances, &r_i);
        let rounded = (val * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.7223);
    }

    #[test]
    fn calc_4() {
        let ethane = Substance {
            fraction: 0.5,
            functional_groups: vec![FunctionalGroup::from(1, 2.0).unwrap()],
            gamma: None,
        };
        let benzene = Substance {
            fraction: 0.5,
            functional_groups: vec![FunctionalGroup::from(9, 6.0).unwrap()],
            gamma: None,
        };
        let substances = vec![ethane, benzene];
        let q_i = vec![0.8480 * 2.0, 0.4 * 6.0];
        let val = super::calc_4(0, &substances, &q_i);
        let rounded = (val * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.8281);
    }

    #[test]
    fn calc_5() {
        let test = super::calc_5(2.0, 3.0);
        assert_eq!(test, -6.0);
    }

    #[test]
    fn calc_15_sum() {
        let ethane = Substance {
            fraction: 2.0 / 3.0,
            functional_groups: vec![],
            gamma: None,
        };
        let benzene = Substance {
            fraction: 1.0 / 3.0,
            functional_groups: vec![],
            gamma: None,
        };
        let substances = vec![ethane, benzene];
        let l_i = vec![1.6, 2.2]; // fake values
        let test = super::calc_15_sum(&substances, &l_i);
        assert_eq!(test, 1.8);
    }

    #[test]
    fn calc_15() {
        let ethane = Substance {
            fraction: 2.0 / 3.0,
            functional_groups: vec![],
            gamma: None,
        };
        let benzene = Substance {
            fraction: 1.0 / 3.0,
            functional_groups: vec![],
            gamma: None,
        };
        let substances = vec![ethane, benzene];
        let l_i = vec![1.6, 2.2]; // fake values
        let val = super::calc_15(0.125, 1.0 / 3.0, 2.0, 8.0, 1.4, 0.25);
        let rounded = (val * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 4.7333);
    }
}
