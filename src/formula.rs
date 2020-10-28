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

/// Calc 6: Substances's intermediate value X
///
/// # Arguments
/// - `id` - id of target functional group
/// - `substance` - target substance
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

/// Calc 7 sum: Functional group's intermediate sum needed for value X
///
/// # Arguments
/// - `substances` - vector of all substances in mixture
pub fn calc_7_sum(substances: &Vec<Substance>) -> f64 {
    let mut sum = 0.0;
    for substance in substances {
        for fg in &substance.functional_groups {
            sum += substance.fraction * fg.nu;
        }
    }
    sum
}

/// Calc 7: Functional group's intermediate value X
///
/// # Arguments
/// - `id` - id of target functional group
/// - `sum` - intermediate sum calculated using `calc_7_sum()`
/// - `substances` - vector of all substances in mixture
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

/// Calc 8 sum: Functional group's intermediate sum for value THETA
///
/// # Arguments
/// - `x_k` - Hash map with pairs of functional group id and corresponding X
pub fn calc_8_sum(x_k: &HashMap<u8, f64>) -> f64 {
    let mut sum = 0.0;
    for (id, x) in x_k {
        let fg = FunctionalGroup::from(*id, 0.0).unwrap();
        sum += fg.q * x;
    }
    sum
}

/// Calc 8: Functional group's intermediate value THETA
///
/// # Arguments
/// - `id` - id of target functional group
/// - `x_k` - Hash map with pairs of functional group id and corresponding X
/// - `sum` - intermediate sum calculated using `calc_8_sum()`
pub fn calc_8(id: u8, x_k: &HashMap<u8, f64>, sum: f64) -> f64 {
    let fg = FunctionalGroup::from(id, 0.0).unwrap();
    fg.q * x_k[&id] / sum
}

/// Calc 9: Substance's intermediate value THETA
///
/// # Arguments
/// - `id` - id of target functional group
/// - `x_ki` - Hash map with pairs of functional group id and corresponding X (substance's X), only use functional
/// groups of target substance
/// - `sum` - intermediate sum calculated using `calc_8_sum()`
pub fn calc_9(id: u8, x_ki: &HashMap<u8, f64>, sum: f64) -> f64 {
    calc_8(id, x_ki, sum)
}

/// Calc 10: Functional groups' interaction parameter value psi
///
/// # Arguments
/// - `i` - id of first functional group
/// - `j` - id of second functional group
/// - `temperature` - temperature in Kelvin
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

/// Calc 11 sum 1: Substance's intermediate first sum needed for value ln GAMMA
///
/// # Arguments
/// - `theta_m` - vector with THETA of substance
/// - `psi_mk` - vector containing psi values for target functional group (as first functional
/// group)
/// - `shadow` - vector containing `True` for places in psi_mk which should be skipped (because
/// functional group at that place is not part of target substance)
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

/// Calc 11 sum 2: Substance's intermediate second sum needed for value ln GAMMA
///
/// # Arguments
/// - `k_index` - index of target functional group in psi_nm (as first functional group)
/// - `theta_m` - vector with THETA of substance
/// - `psi_nm` - 2d-vector containing psi values
/// - `shadow` - vector containing `True` for places in psi_nm which should be skipped (because
/// functional group at that place is not part of target substance)
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

/// Calc 11: Substance's intermediate value ln GAMMA
///
/// # Arguments
/// - `q_k` - value Q of target functional group
/// - `sum_1` - value calculated using `calc_11_sum_1()`
/// - `sum_2` - value calculated using `calc_11_sum_2()`
pub fn calc_11(q_k: f64, sum_1: f64, sum_2: f64) -> f64 {
    q_k * (1.0 - sum_1.ln() - sum_2)
}

/// Calc 12: Functional group's intermediate value ln GAMMA
///
/// # Arguments
/// - `q_k` - value Q of target functional group
/// - `sum_1` - value calculated using `calc_11_sum_1()`
/// - `sum_2` - value calculated using `calc_11_sum_2()`
pub fn calc_12(q_k: f64, sum_1: f64, sum_2: f64) -> f64 {
    calc_11(q_k, sum_1, sum_2)
}

/// Calc 13: Substance's intermediate value ln gamma r
///
/// # Arguments
/// - `substance` - target substance
/// - `gamma_k` - vector of GAMMA of functional groups contained in substance
/// - `gamma_i_k` - vector of GAMMA of substance and functional groups
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
    use super::*;
    const EPSILON: f64 = 0.001;

    #[test]
    fn calc_6() {
        let expected = 1.0;
        let benzene = Substance {
            fraction: 1.0,
            functional_groups: vec![FunctionalGroup::from(9, 6.0).unwrap()],
            gamma: None,
        };
        let actual = super::calc_6(9, &benzene);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_7_sum() {
        let expected = 4.0;
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
        let actual = super::calc_7_sum(&substances);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_7() {
        let expected = 0.25;
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
        let actual = super::calc_7(1, 4.0, &substances);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_8_sum() {
        let expected = 0.617;
        let mut x_k_map = HashMap::new();
        x_k_map.insert(1, 0.25);
        x_k_map.insert(2, 0.75);
        let actual = super::calc_8_sum(&x_k_map);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_8() {
        let expected = 0.343598;
        let mut x_k_map = HashMap::new();
        x_k_map.insert(1, 0.25);
        x_k_map.insert(2, 0.75);
        let actual = super::calc_8(1, &x_k_map, 0.617);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_9() {
        let expected = 0.343598;
        let mut x_k_map = HashMap::new();
        x_k_map.insert(1, 0.25);
        x_k_map.insert(2, 0.75);
        let actual = super::calc_9(1, &x_k_map, 0.617);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_10() {
        let expected = 0.81565;
        let actual = super::calc_10(1, 9, 300.0).unwrap();
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_11_sum_1() {
        let expected = 1.0925;
        let theta = vec![3.2, 0.65];
        let psi = vec![0.25, 0.3, 0.45];
        let shadow = vec![false, true, false];
        let actual = super::calc_11_sum_1(&theta, &psi, &shadow);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_11_sum_2() {
        let expected = 2.219;
        let theta = vec![1.23, 0.68];
        let psi = vec![vec![0.5, 0.34, 0.62], vec![0.9, 0.13, 0.47]];
        let shadow = vec![false, true, false];
        let actual = super::calc_11_sum_2(1, &theta, &psi, &shadow);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_11() {
        let expected = -0.8237;
        let actual = super::calc_11(0.63, 1.0925, 2.219);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_12() {
        let expected = -0.58836;
        let actual = super::calc_12(0.45, 1.0925, 2.219);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_13() {
        let expected = 0.26;
        let ethane = Substance {
            fraction: 0.5,
            functional_groups: vec![FunctionalGroup::from(1, 2.0).unwrap()],
            gamma: None,
        };
        let gamma_k = vec![0.45];
        let gamma_i_k = vec![0.32];
        let actual = super::calc_13(&ethane, &gamma_k, &gamma_i_k);
        assert!((expected - actual).abs() < EPSILON);
    }
}
