use crate::functional_group::FunctionalGroup;
use crate::interaction::InteractionParameter;
use crate::substance::Substance;

use rayon::prelude::*;

/// Calc 1: Substance's r
///
/// # Arguments
/// - `substance` - Target substance
pub fn calc_1(substance: &Substance) -> f64 {
    substance
        .functional_groups
        .par_iter()
        .map(|fg| fg.nu * fg.r)
        .sum()
}

/// Calc 2: Substance's q
///
/// # Arguments
/// - `substance` - Target substance
pub fn calc_2(substance: &Substance) -> f64 {
    substance
        .functional_groups
        .par_iter()
        .map(|fg| fg.nu * fg.q)
        .sum()
}

/// Calc 3: Substance's intermediate value phi
///
/// # Arguments
/// - `id` - Index of the substance, phi should be calced for
/// - `substances` - All substances
/// - `r_i` - Vec containing the substances' r_i (ordered as `substances`)
pub fn calc_3(id: usize, substances: &Vec<Substance>, r_i: &Vec<f64>) -> f64 {
    let sum: f64 = (0..substances.len())
        .into_par_iter()
        .map(|i| substances[i].fraction * r_i[i])
        .sum();
    r_i[id] * substances[id].fraction / sum
}

/// Calc 4: Substance's intermediate value theta
///
/// # Arguments
/// - `id` - Index of the substance in `substances`, phi should be calced for
/// - `substances` - All substances
/// - `q_i` - Vec containing the substances' q_i (ordered as `substances`)
pub fn calc_4(id: usize, substances: &Vec<Substance>, q_i: &Vec<f64>) -> f64 {
    let sum: f64 = (0..substances.len())
        .into_par_iter()
        .map(|i| substances[i].fraction * q_i[i])
        .sum();
    q_i[id] * substances[id].fraction / sum
}

/// Calc 5: Substance's intermediate value l
///
/// Implicitly uses constant `z` as 10
///
/// # Arguments
/// - `r` - Target substance's `r_i`
/// - `q` - Target substance's `q_i`
pub fn calc_5(r: f64, q: f64) -> f64 {
    5.0 * (r - q) - (r - 1.0)
}

/// Calc 6: Substances's intermediate value X
///
/// # Arguments
/// - `id` - id of target functional group
/// - `substance` - target substance
pub fn calc_6(id: u8, substance: &Substance) -> Result<f64, &'static str> {
    let sum: f64 = substance.functional_groups.par_iter().map(|fg| fg.nu).sum();
    let i = substance
        .functional_groups
        .par_iter()
        .position_any(|fg| fg.id == id)
        .ok_or_else(|| "Unknown functional group id")?;
    Ok(substance.functional_groups[i].nu / sum)
}

/// Calc 7 sum: Functional group's intermediate sum needed for value X
///
/// # Arguments
/// - `substances` - vector of all substances in mixture
pub fn calc_7_sum(substances: &Vec<Substance>) -> f64 {
    substances
        .par_iter()
        .map(|s| {
            s.functional_groups
                .par_iter()
                .map(|fg| s.fraction * fg.nu)
                .sum::<f64>()
        })
        .sum()
}

/// Calc 7: Functional group's intermediate value X
///
/// # Arguments
/// - `id` - id of target functional group
/// - `sum` - intermediate sum calculated using `calc_7_sum()`
/// - `substances` - vector of all substances in mixture
pub fn calc_7(id: u8, substances: &Vec<Substance>, sum: f64) -> f64 {
    let single_sum: f64 = substances
        .par_iter()
        .map(|s| {
            s.functional_groups
                .par_iter()
                .filter(|fg| fg.id == id)
                .map(|fg| s.fraction * fg.nu)
                .sum::<f64>()
        })
        .sum();
    single_sum / sum
}

/// Calc 8 sum: Functional group's intermediate sum for value THETA
///
/// # Arguments
/// - `ids` - Vector of all functional group ids
/// - `x_m` - Vector pf all groups' X_m
pub fn calc_8_sum(ids: Vec<u8>, x_m: Vec<f64>) -> Result<f64, &'static str> {
    Ok((0..ids.len())
        .into_par_iter()
        .map(|i| FunctionalGroup::from(ids[i], 0.0).map_or_else(|_| 0.0, |fg| fg.q) * x_m[i])
        .sum())
}

/// Calc 8: Functional group's intermediate value THETA
///
/// # Arguments
/// - `id` - id of target functional group
/// - `ids` - Vector of all functional group ids
/// - `x_m` - Vector pf all groups' X_m
/// - `sum` - intermediate sum calculated using `calc_8_sum()`
pub fn calc_8(id: u8, ids: Vec<u8>, x_m: Vec<f64>, sum: f64) -> Result<f64, &'static str> {
    let i: usize = ids
        .par_iter()
        .position_any(|&x| x == id)
        .ok_or_else(|| "Unknown functional group id")?;
    let fg = FunctionalGroup::from(id, 0.0)?;
    Ok(fg.q * x_m[i] / sum)
}

/// Calc 10: Functional groups' interaction parameter value psi
///
/// If i and j are the same, the result is set to 1.
///
/// # Arguments
/// - `i` - id of first functional group
/// - `j` - id of second functional group
/// - `temperature` - temperature in Kelvin
pub fn calc_10(i: u8, j: u8, temperature: f64) -> Result<f64, &'static str> {
    let i_maingroup = FunctionalGroup::from(i, 1.0)?.main_id;
    let j_maingroup = FunctionalGroup::from(j, 1.0)?.main_id;
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

/// Calc 13: Substance's intermediate value ln gamma r
///
/// # Arguments
/// - `substance` - target substance
/// - `gamma_k` - vector of GAMMA of functional groups contained in substance
/// - `gamma_i_k` - vector of GAMMA of substance and functional groups
pub fn calc_13(substance: &Substance, gamma_k: &Vec<f64>, gamma_i_k: &Vec<f64>) -> f64 {
    (0..substance.functional_groups.len())
        .into_par_iter()
        .map(|i| substance.functional_groups[i].nu * (gamma_k[i] - gamma_i_k[i]))
        .sum()
}

/// Calc 15: Sustance's combinatorial activity coeff. part
///
/// Calcs the sum across all substances used as denominator
///
/// # Arguments
/// - `substances` - All substances
/// - `l_i` - Substances' l_i (ordered as `substances`)
pub fn calc_15_sum(substances: &Vec<Substance>, l_i: &Vec<f64>) -> f64 {
    (0..substances.len())
        .into_par_iter()
        .map(|i| substances[i].fraction * l_i[i])
        .sum()
}

/// Calc 15: Sustance's combinatorial activity coeff. part
///
/// Calcs the sum across all substances used as denominator
///
/// # Arguments
/// - `x` - Substance's fraction
/// - `q` - Substance's `q`
/// - `phi` - Substances `phi`
/// - `theta` - Substances `theta`
/// - `l` - Substances `l`
/// - `sum_lx` - Sum (denominator), see calc_15_sum
pub fn calc_15(x: f64, q: f64, phi: f64, theta: f64, l: f64, sum_lx: f64) -> f64 {
    let mut res = (phi / x).ln();
    res += 5.0 * q * (theta / phi).ln();
    res += l;
    res - phi / x * sum_lx
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.001;

    fn create_substance(fraction: f64, fgs: Vec<(u8, f64)>) -> Substance {
        Substance {
            fraction,
            functional_groups: fgs
                .iter()
                .map(|x| FunctionalGroup::from(x.0, x.1).unwrap())
                .collect(),
            gamma: None,
        }
    }

    fn create_ethane(fraction: f64) -> Substance {
        create_substance(fraction, vec![(1, 2.0)])
    }

    fn create_benzene(fraction: f64) -> Substance {
        create_substance(fraction, vec![(9, 6.0)])
    }

    #[test]
    fn calc_1() {
        let substance = create_substance(1.0, vec![(1, 1.0), (2, 1.0), (4, 3.0)]);
        let test = super::calc_1(&substance);
        assert!((test - 2.234).abs() < EPSILON);
    }

    #[test]
    fn calc_2() {
        let substance = create_substance(1.0, vec![(1, 1.0), (2, 1.0), (4, 3.0)]);
        let test = super::calc_2(&substance);
        assert!((test - 1.388).abs() < EPSILON);
    }

    #[test]
    fn calc_3() {
        let ethane = create_ethane(0.5);
        let benzene = create_benzene(0.5);
        let substances = vec![ethane, benzene];
        let r_i = vec![0.9011 * 2.0, 0.5313 * 6.0];
        let test = super::calc_3(0, &substances, &r_i);
        assert!((test - 0.3612).abs() < EPSILON);
    }

    #[test]
    fn calc_4() {
        let ethane = create_ethane(0.5);
        let benzene = create_benzene(0.5);
        let substances = vec![ethane, benzene];
        let q_i = vec![0.8480 * 2.0, 0.4 * 6.0];
        let test = super::calc_4(0, &substances, &q_i);
        assert!((test - 0.4141).abs() < EPSILON);
    }

    #[test]
    fn calc_5() {
        let test = super::calc_5(2.0, 3.0);
        assert!((test + 6.0).abs() < EPSILON);
    }

    #[test]
    fn calc_6() {
        let expected = 1.0;
        let benzene = create_benzene(1.0);
        let actual = super::calc_6(9, &benzene).unwrap();
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_7_sum() {
        let expected = 4.0;
        let ethane = create_ethane(0.5);
        let benzene = create_benzene(0.5);
        let substances = vec![ethane, benzene];
        let actual = super::calc_7_sum(&substances);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_7() {
        let expected = 0.25;
        let ethane = create_ethane(0.5);
        let benzene = create_benzene(0.5);
        let substances = vec![ethane, benzene];
        let actual = super::calc_7(1, &substances, 4.0);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_8_sum() {
        let expected = 0.617;
        let actual = super::calc_8_sum(vec![1, 2], vec![0.25, 0.75]).unwrap();
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_8() {
        let expected = 0.343598;
        let actual = super::calc_8(1, vec![1, 2], vec![0.25, 0.75], 0.617).unwrap();
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
    fn calc_13() {
        let expected = 0.26;
        let ethane = create_ethane(0.5);
        let gamma_k = vec![0.45];
        let gamma_i_k = vec![0.32];
        let actual = super::calc_13(&ethane, &gamma_k, &gamma_i_k);
        assert!((expected - actual).abs() < EPSILON);
    }

    #[test]
    fn calc_15_sum() {
        let ethane = create_ethane(2.0 / 3.0);
        let benzene = create_benzene(1.0 / 3.0);
        let substances = vec![ethane, benzene];
        let l_i = vec![1.6, 2.2]; // fake values
        let test = super::calc_15_sum(&substances, &l_i);
        assert!((test - 1.8).abs() < EPSILON);
    }

    #[test]
    fn calc_15() {
        let x = 0.047;
        let q = 2.336;
        let phi = 0.0321;
        let theta = 0.0336;
        let l = -0.3860;
        let sum = 0.047 * 0.3860 + 0.953 * 0.2784;
        let test = super::calc_15(x, q, phi, theta, l, sum);
        assert!((test + 0.42746).abs() < EPSILON);
    }
}
