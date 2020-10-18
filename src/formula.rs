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
    let i_maingroup = FunctionalGroup::from(i, 1.0)?.main_id;
    let j_maingroup = FunctionalGroup::from(j, 1.0)?.main_id;
    println!("Maingroup of {}: {}", i, i_maingroup);
    println!("Maingroup of {}: {}", j, j_maingroup);
    if i_maingroup == j_maingroup {
        return Ok(1.0);
    }
    let interp = InteractionParameter::from(i_maingroup, j_maingroup)?;
    Ok(std::f64::consts::E.powf(-interp.a_ij / temperature))
}

pub fn calc_11_sum_1(theta_m: &Vec<f64>, psi_mk: &Vec<f64>, shadow: &Vec<bool>) -> f64 {
    let mut sum = 0.0;
    let mut theta_index = 0;
    for psi_index in 0..psi_mk.len() {
        if shadow[psi_index] {
            continue;
        }
        sum += theta_m[theta_index] * psi_mk[psi_index];
        theta_index += 1;
    }
    sum
}

pub fn calc_11_sum_2(
    k_index: usize,
    theta_m: &Vec<f64>,
    psi_nm: &Vec<Vec<f64>>,
    shadow: &Vec<bool>,
) -> f64 {
    let mut sum = 0.0;
    let mut psi_index = 0;
    for m in 0..theta_m.len() {
        while shadow[psi_index] {
            psi_index += 1;
        }
        let numerator = theta_m[m] * psi_nm[k_index][psi_index];
        let mut psi_n: Vec<f64> = Vec::new();
        for psi in psi_nm {
            psi_n.push(psi[psi_index]);
        }
        let denominator = calc_11_sum_1(theta_m, &psi_n, shadow);
        psi_index += 1;
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

pub fn calc_13(substance: &Substance, gamma_k: &Vec<f64>, gamma_i_k: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..substance.functional_groups.len() {
        sum += substance.functional_groups[i].nu * (gamma_k[i] - gamma_i_k[i]);
    }
    sum
}

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
