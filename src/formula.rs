use std::collections::HashMap;

use crate::functional_group::FunctionalGroup;
use crate::interaction::InteractionParameter;
use crate::substance::Substance;

pub fn calc_1(substance: Substance) -> f64 {
    0.0
}

pub fn calc_2(substance: Substance) -> f64 {
    0.0
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

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
