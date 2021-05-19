use crate::formula;
use crate::functional_group::FunctionalGroup;
use crate::substance::Substance;

use itertools::Itertools;
use rayon::prelude::*;

/// Calculates activity coeficients for the given mixture
///
/// # Arugments
///
/// # Example
/// ```
/// use unifac::*;
/// let diethyl_ether = Substance::from(
///     0.9,
///     vec![
///         FunctionalGroup::from(1, 2.0).unwrap(),  // CH3
///         FunctionalGroup::from(2, 1.0).unwrap(),  // CH2
///         FunctionalGroup::from(25, 1.0).unwrap(), // CH2O
///     ],
/// );
/// let benzene = Substance::from(
///     0.1,
///     vec![
///         FunctionalGroup::from(9, 6.0).unwrap(), // CH3
///     ],
/// );
/// ```
pub fn calc(substances: Vec<Substance>, temperature: f64) -> Result<Vec<Substance>, &'static str> {
    let combinatorial = calc_combinatorial(substances.clone())?;
    let residual = calc_residual(substances.clone(), temperature)?;

    let calced_substances: Vec<Substance> = (0..substances.len())
        .into_par_iter()
        .map(|i| Substance {
            name: substances[i].name.clone(),
            fraction: substances[i].fraction.clone(),
            functional_groups: substances[i].functional_groups.clone(),
            gamma: Some((combinatorial[i] + residual[i]).exp()),
        })
        .collect();

    Ok(calced_substances)
}

fn calc_combinatorial(substances: Vec<Substance>) -> Result<Vec<f64>, &'static str> {
    let (r_i, q_i): (Vec<f64>, Vec<f64>) = substances
        .par_iter()
        .map(|s| (formula::calc_1(s), formula::calc_2(s)))
        .collect::<Vec<(f64, f64)>>()
        .par_iter()
        .cloned()
        .unzip();

    // since unzip only supports tuples with 2 elements, the formula results are
    // nested and unzipped twice
    let (phi_i, theta_l): (Vec<f64>, Vec<(f64, f64)>) = (0..substances.len())
        .into_par_iter()
        .map(|i| {
            (
                formula::calc_3(i, &substances, &r_i),
                (
                    formula::calc_4(i, &substances, &q_i),
                    formula::calc_5(r_i[i], q_i[i]),
                ),
            )
        })
        .collect::<Vec<(f64, (f64, f64))>>()
        .par_iter()
        .cloned()
        .unzip();
    let (theta_i, l_i): (Vec<f64>, Vec<f64>) = theta_l.par_iter().cloned().unzip();

    let sum_lx = formula::calc_15_sum(&substances, &l_i);
    Ok((0..substances.len())
        .into_par_iter()
        .map(|i| {
            formula::calc_15(
                substances[i].fraction,
                q_i[i],
                phi_i[i],
                theta_i[i],
                l_i[i],
                sum_lx,
            )
        })
        .collect())
}

fn calc_residual(substances: Vec<Substance>, temperature: f64) -> Result<Vec<f64>, &'static str> {
    let fg_ids: Vec<u8> = substances
        .par_iter()
        .map(|s| {
            s.functional_groups
                .par_iter()
                .map(|fg| fg.id)
                .collect::<Vec<u8>>()
        })
        .into_par_iter()
        .flatten()
        .collect::<Vec<u8>>()
        .into_iter()
        .unique()
        .collect();

    let x_i_m = substances
        .par_iter()
        .map(|s| {
            s.functional_groups
                .par_iter()
                .map(|fg| formula::calc_6(fg.id, &s))
                .collect::<Result<Vec<f64>, &'static str>>()
        })
        .collect::<Result<Vec<Vec<f64>>, &'static str>>()?;

    let sum = formula::calc_7_sum(&substances);
    let x_m: Vec<f64> = fg_ids
        .par_iter()
        .map(|id| formula::calc_7(*id, &substances, sum))
        .collect();

    let sum = formula::calc_8_sum(fg_ids.clone(), x_m.clone())?;
    let theta_k = fg_ids
        .par_iter()
        .map(|id| formula::calc_8(*id, fg_ids.clone(), x_m.clone(), sum))
        .collect::<Result<Vec<f64>, &'static str>>()?;

    let theta_i_k = (0..substances.len())
        .into_par_iter()
        .map(|i| {
            let (ids, x_i_m_param): (Vec<u8>, Vec<f64>) =
                (0..substances[i].functional_groups.len())
                    .into_par_iter()
                    .map(|j| (substances[i].functional_groups[j].id, x_i_m[i][j]))
                    .collect::<Vec<(u8, f64)>>()
                    .par_iter()
                    .cloned()
                    .unzip();
            let sum = formula::calc_8_sum(ids.clone(), x_i_m_param.clone())?;
            substances[i]
                .functional_groups
                .par_iter()
                .map(|fg| formula::calc_8(fg.id, ids.clone(), x_i_m_param.clone(), sum))
                .collect::<Result<Vec<f64>, &'static str>>()
        })
        .collect::<Result<Vec<Vec<f64>>, &'static str>>()?;

    let psi_n_m: Vec<Vec<f64>> = fg_ids
        .par_iter()
        .map(|id_n| {
            fg_ids
                .par_iter()
                .map(|id_m| formula::calc_10(*id_n, *id_m, temperature))
                .collect::<Result<Vec<f64>, &'static str>>()
        })
        .collect::<Result<Vec<Vec<f64>>, &'static str>>()?;

    let psi_m_n: Vec<Vec<f64>> = (0..fg_ids.len())
        .into_par_iter()
        .map(|i| psi_n_m.par_iter().map(|psi_m| psi_m[i]).collect())
        .collect();

    let gamma_i_k: Vec<Vec<f64>> = (0..substances.len())
        .into_par_iter()
        .map(|i| {
            let fg_ids_of_current_substance: Vec<u8> = substances[i]
                .functional_groups
                .par_iter()
                .map(|fg| fg.id)
                .collect();
            let shadow: Vec<bool> = fg_ids
                .par_iter()
                .map(|id| !fg_ids_of_current_substance.contains(id))
                .collect();
            substances[i]
                .functional_groups
                .par_iter()
                .map(|fg| {
                    let index_of_fg_in_fg_ids = fg_ids
                        .iter()
                        .position(|&x| x == fg.id)
                        .ok_or_else(|| "Unknown functional group")?;
                    let sum_1 = formula::calc_11_sum_1(
                        &theta_i_k[i],
                        &psi_m_n[index_of_fg_in_fg_ids],
                        &shadow,
                    );
                    let sum_2 = formula::calc_11_sum_2(
                        index_of_fg_in_fg_ids,
                        &theta_i_k[i],
                        &psi_n_m,
                        &shadow,
                    );
                    Ok(formula::calc_11(fg.q, sum_1, sum_2))
                })
                .collect::<Result<Vec<f64>, &'static str>>()
        })
        .collect::<Result<Vec<Vec<f64>>, &'static str>>()?;

    let shadow_nothing = &vec![false; fg_ids.len()];
    let gamma_k: Vec<f64> = (0..fg_ids.len())
        .into_par_iter()
        .map(|i| {
            let sum_1 = formula::calc_11_sum_1(&theta_k, &psi_m_n[i], &shadow_nothing);
            let sum_2 = formula::calc_11_sum_2(i, &theta_k, &psi_n_m, &shadow_nothing);
            let q_k = FunctionalGroup::from(fg_ids[i], 1.0)?.q;
            Ok(formula::calc_11(q_k, sum_1, sum_2))
        })
        .collect::<Result<Vec<f64>, &'static str>>()?;

    let gamma: Vec<f64> = (0..substances.len())
        .into_par_iter()
        .map(|i| {
            let gamma_k_subst: Vec<f64> = substances[i]
                .functional_groups
                .par_iter()
                .map(|fg| {
                    let index = fg_ids
                        .par_iter()
                        .position_any(|&x| x == fg.id)
                        .ok_or_else(|| "Unknown functional group")?;
                    Ok(gamma_k[index])
                })
                .collect::<Result<Vec<f64>, &'static str>>()?;
            Ok(formula::calc_13(
                &substances[i],
                &gamma_k_subst,
                &gamma_i_k[i],
            ))
        })
        .collect::<Result<Vec<f64>, &'static str>>()?;

    Ok(gamma)
}

// Unit tests
// for reference values, see http://ddbonline.ddbst.de/UNIFACCalculation/UNIFACCalculationCGI.exe
#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.001;

    fn get_acetone(fraction: f64) -> Substance {
        Substance::from(
            fraction,
            vec![
                FunctionalGroup::from(1, 1.0).unwrap(),  // CH3
                FunctionalGroup::from(18, 1.0).unwrap(), // CH3CO
            ],
        )
    }

    fn get_ethanol(fraction: f64) -> Substance {
        Substance::from(
            fraction,
            vec![
                FunctionalGroup::from(1, 1.0).unwrap(),  // CH3
                FunctionalGroup::from(2, 1.0).unwrap(),  // CH2
                FunctionalGroup::from(14, 1.0).unwrap(), // OH
            ],
        )
    }
    fn get_pentane(fraction: f64) -> Substance {
        Substance::from(
            fraction,
            vec![
                FunctionalGroup::from(1, 2.0).unwrap(),
                FunctionalGroup::from(2, 3.0).unwrap(),
            ],
        )
    }

    #[test]
    fn calc_mixture_acetone_ethanole_50_50() {
        let acetone_value = 1.2140;
        let ethanol_value = 1.2168;
        let temperature = 323.0;
        let acetone = get_acetone(0.5);
        let ethanol = get_ethanol(0.5);

        let resid = calc(vec![acetone, ethanol], temperature).unwrap();

        assert!((resid[0].gamma.unwrap() - acetone_value).abs() < EPSILON);
        assert!((resid[1].gamma.unwrap() - ethanol_value).abs() < EPSILON);
    }

    #[test]
    fn calc_mixture_acetone_ethanole_70_30() {
        let acetone_value = 1.0726;
        let ethanol_value = 1.4652;
        let temperature = 323.0;
        let acetone = get_acetone(0.7);
        let ethanol = get_ethanol(0.3);

        let resid = calc(vec![acetone, ethanol], temperature).unwrap();

        assert!((resid[0].gamma.unwrap() - acetone_value).abs() < EPSILON);
        assert!((resid[1].gamma.unwrap() - ethanol_value).abs() < EPSILON);
    }

    #[test]
    fn calc_mixture_diethyl_ether_benzene_90_10() {
        let diethyl_ether_value = 1.0007;
        let benzene_value = 1.1012;
        let temperature = 298.0;
        let diethyl_ether = Substance::from(
            0.9,
            vec![
                FunctionalGroup::from(1, 2.0).unwrap(),  // CH3
                FunctionalGroup::from(2, 1.0).unwrap(),  // CH2
                FunctionalGroup::from(25, 1.0).unwrap(), // CH2O
            ],
        );
        let benzene = Substance::from(
            0.1,
            vec![
                FunctionalGroup::from(9, 6.0).unwrap(), // CH3
            ],
        );

        let resid = calc(vec![diethyl_ether, benzene], temperature).unwrap();

        assert!((resid[0].gamma.unwrap() - diethyl_ether_value).abs() < EPSILON);
        assert!((resid[1].gamma.unwrap() - benzene_value).abs() < EPSILON);
    }

    #[test]
    fn combinatorial_calculates() {
        let acetone_value = 0.9837063754024332;
        let pentane_value = 0.9999607147028443;
        let temperature = 298.0;

        let acetone = Substance::from(
            0.047,
            vec![
                FunctionalGroup {
                    id: 1,
                    subgroup: "CH3".to_string(),
                    main_id: 1,
                    nu: 2.0,
                    r: 0.9011,
                    q: 0.8480,
                }, // CH3
                FunctionalGroup {
                    id: 8,
                    subgroup: "CO".to_string(),
                    main_id: 0,
                    nu: 1.0,
                    r: 0.7713,
                    q: 0.64,
                },
            ],
        );
        let pentane = Substance::from(
            0.953,
            vec![
                FunctionalGroup {
                    id: 1,
                    subgroup: "CH3".to_string(),
                    main_id: 1,
                    nu: 2.0,
                    r: 0.9011,
                    q: 0.8480,
                }, // CH3
                FunctionalGroup {
                    id: 2,
                    subgroup: "CH2".to_string(),
                    main_id: 1,
                    nu: 3.0,
                    r: 0.6744,
                    q: 0.5400,
                }, // CH2
            ],
        );

        let resid = calc(vec![acetone, pentane], temperature).unwrap();

        assert!((resid[0].gamma.unwrap() - acetone_value).abs() < EPSILON);
        assert!((resid[1].gamma.unwrap() - pentane_value).abs() < EPSILON);
    }

    #[test]
    fn residual_calculates() {
        let acetone_value = 1.6605607656292078;
        let pentane_value = 0.00534819401527342;

        let acetone = get_acetone(0.047);
        let pentane = get_pentane(0.953);

        let resid = calc_residual(vec![acetone, pentane], 307.0).unwrap();
        assert!((resid[0] - acetone_value).abs() < EPSILON);
        assert!((resid[1] - pentane_value).abs() < EPSILON);
    }
}
