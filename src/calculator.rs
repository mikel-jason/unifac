use std::collections::HashMap;

use crate::formula;
use crate::functional_group::FunctionalGroup;
use crate::substance::Substance;

pub fn calc(substances: Vec<Substance>, temperature: f64) -> Result<Vec<Substance>, &'static str> {
    let combinatorial = calc_combinatorial(substances.clone())?;
    let residual = calc_residual(substances.clone(), temperature)?;

    let mut calced_substances: Vec<Substance> = Vec::new();
    for i in 0..substances.len() {
        let gamma = (combinatorial[i] + residual[i]).exp();

        calced_substances.push(Substance {
            fraction: substances[i].fraction.clone(),
            functional_groups: substances[i].functional_groups.clone(),
            gamma: Some(gamma),
        })
    }
    Ok(calced_substances)
}

fn calc_combinatorial(substances: Vec<Substance>) -> Result<Vec<f64>, &'static str> {
    let mut r_i: Vec<f64> = Vec::new();
    let mut q_i: Vec<f64> = Vec::new();
    let mut phi_i: Vec<f64> = Vec::new();
    let mut theta_i: Vec<f64> = Vec::new();
    let mut l_i: Vec<f64> = Vec::new();
    let mut ln_gamma_c_i: Vec<f64> = Vec::new();

    for substance in &substances {
        r_i.push(formula::calc_1(&substance));
        q_i.push(formula::calc_2(&substance));
    }

    for i in 0..substances.len() {
        phi_i.push(formula::calc_3(i, &substances, &r_i));
        theta_i.push(formula::calc_4(i, &substances, &q_i));
        l_i.push(formula::calc_5(r_i[i], q_i[i]));
    }

    let sum_lx = formula::calc_15_sum(&substances, &l_i);
    for i in 0..substances.len() {
        ln_gamma_c_i.push(formula::calc_15(
            substances[i].fraction,
            q_i[i],
            phi_i[i],
            theta_i[i],
            l_i[i],
            sum_lx,
        ))
    }

    Ok(ln_gamma_c_i)
}

fn calc_residual(substances: Vec<Substance>, temperature: f64) -> Result<Vec<f64>, &'static str> {
    let mut fg_ids = Vec::new();
    for substance in &substances {
        for fg in &substance.functional_groups {
            if !fg_ids.contains(&fg.id) {
                fg_ids.push(fg.id);
            }
        }
    }

    let mut x_i_m = Vec::new();
    for substance in &substances {
        let mut temp = Vec::new();
        for fg in &substance.functional_groups {
            temp.push(formula::calc_6(fg.id, &substance));
        }
        x_i_m.push(temp);
    }

    let mut x_m = Vec::new();
    let sum = formula::calc_7_sum(&substances);
    for id in &fg_ids {
        x_m.push(formula::calc_7(*id, &substances, sum));
    }

    let x_k_map: HashMap<u8, f64> = fg_ids.clone().into_iter().zip(x_m).collect();
    let sum = formula::calc_8_sum(&x_k_map)?;
    let mut theta_k = Vec::new();
    for id in &fg_ids {
        theta_k.push(formula::calc_8(*id, &x_k_map, sum)?);
    }

    let mut theta_i_k = Vec::new();
    for i in 0..substances.len() {
        let mut temp = Vec::new();
        let mut x_i_k_map: HashMap<u8, f64> = HashMap::new();
        for j in 0..substances[i].functional_groups.len() {
            x_i_k_map.insert(substances[i].functional_groups[j].id, x_i_m[i][j]);
        }
        let sum = formula::calc_8_sum(&x_i_k_map)?;
        for fg in &substances[i].functional_groups {
            temp.push(formula::calc_8(fg.id, &x_i_k_map, sum)?);
        }
        theta_i_k.push(temp);
    }

    let mut psi_n_m = Vec::new();
    for id_n in &fg_ids {
        let mut temp = Vec::new();
        for id_m in &fg_ids {
            if id_n == id_m {
                temp.push(1.0);
            } else {
                temp.push(formula::calc_10(*id_n, *id_m, temperature)?);
            }
        }
        psi_n_m.push(temp);
    }

    let mut psi_m_n = Vec::new();
    for i in 0..fg_ids.len() {
        let mut temp = Vec::new();
        for psi_m in &psi_n_m {
            temp.push(psi_m[i]);
        }
        psi_m_n.push(temp);
    }

    let mut gamma_i_k = Vec::new();
    for i in 0..substances.len() {
        let mut temp = Vec::new();
        let mut fg_ids_of_current_substance = Vec::new();
        for fg in &substances[i].functional_groups {
            fg_ids_of_current_substance.push(fg.id);
        }
        let mut shadow = Vec::new();
        for id in &fg_ids {
            if fg_ids_of_current_substance.contains(id) {
                shadow.push(false);
            } else {
                shadow.push(true);
            }
        }
        for fg in &substances[i].functional_groups {
            let index_of_fg_in_fg_ids = fg_ids
                .iter()
                .position(|&x| x == fg.id)
                .ok_or_else(|| "Could not find fg in fgs of mixture.")?;
            let sum_1 =
                formula::calc_11_sum_1(&theta_i_k[i], &psi_m_n[index_of_fg_in_fg_ids], &shadow);
            let sum_2 =
                formula::calc_11_sum_2(index_of_fg_in_fg_ids, &theta_i_k[i], &psi_n_m, &shadow);
            temp.push(formula::calc_11(fg.q, sum_1, sum_2));
        }
        gamma_i_k.push(temp);
    }

    let mut gamma_k = Vec::new();
    for i in 0..fg_ids.len() {
        // calc_11_sum1 and 2 accept a vector of booleans to shadow unneeded values
        // since all values are needed a vector consisting of false is used
        let shadow_nothing = &vec![false; fg_ids.len()];
        let sum_1 = formula::calc_11_sum_1(&theta_k, &psi_m_n[i], &shadow_nothing);
        let sum_2 = formula::calc_11_sum_2(i, &theta_k, &psi_n_m, &shadow_nothing);
        let q_k = FunctionalGroup::from(fg_ids[i], 1.0)?.q;
        gamma_k.push(formula::calc_11(q_k, sum_1, sum_2));
    }

    let mut gamma = Vec::new();
    for i in 0..substances.len() {
        let mut gamma_k_subst = Vec::new();
        for fg in &substances[i].functional_groups {
            let index = fg_ids
                .iter()
                .position(|&x| x == fg.id)
                .ok_or_else(|| "Could not find fg in fgs of mixture.")?;
            gamma_k_subst.push(gamma_k[index]);
        }
        gamma.push(formula::calc_13(
            &substances[i],
            &gamma_k_subst,
            &gamma_i_k[i],
        ))
    }

    Ok(gamma)
}

// Unit tests
// for reference values, see http://ddbonline.ddbst.de/UNIFACCalculation/UNIFACCalculationCGI.exe
#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.001;

    #[test]
    fn calc_mixture_acetone_ethanole_50_50() {
        let acetone_value = 1.2140;
        let ethanol_value = 1.2168;
        let temperature = 323.0;
        let acetone = Substance {
            fraction: 0.5,
            functional_groups: vec![
                FunctionalGroup::from(1, 1.0).unwrap(),  // CH3
                FunctionalGroup::from(18, 1.0).unwrap(), // CH3CO
            ],
            gamma: None,
        };
        let ethanol = Substance {
            fraction: 0.5,
            functional_groups: vec![
                FunctionalGroup::from(1, 1.0).unwrap(),  // CH3
                FunctionalGroup::from(2, 1.0).unwrap(),  // CH2
                FunctionalGroup::from(14, 1.0).unwrap(), // OH
            ],
            gamma: None,
        };

        let resid = calc(vec![acetone, ethanol], temperature).unwrap();

        assert!((resid[0].gamma.unwrap() - acetone_value).abs() < EPSILON);
        assert!((resid[1].gamma.unwrap() - ethanol_value).abs() < EPSILON);
    }

    #[test]
    fn calc_mixture_acetone_ethanole_70_30() {
        let acetone_value = 1.0726;
        let ethanol_value = 1.4652;
        let temperature = 323.0;
        let acetone = Substance {
            fraction: 0.7,
            functional_groups: vec![
                FunctionalGroup::from(1, 1.0).unwrap(),  // CH3
                FunctionalGroup::from(18, 1.0).unwrap(), // CH3CO
            ],
            gamma: None,
        };
        let ethanol = Substance {
            fraction: 0.3,
            functional_groups: vec![
                FunctionalGroup::from(1, 1.0).unwrap(),  // CH3
                FunctionalGroup::from(2, 1.0).unwrap(),  // CH2
                FunctionalGroup::from(14, 1.0).unwrap(), // OH
            ],
            gamma: None,
        };

        let resid = calc(vec![acetone, ethanol], temperature).unwrap();

        assert!((resid[0].gamma.unwrap() - acetone_value).abs() < EPSILON);
        assert!((resid[1].gamma.unwrap() - ethanol_value).abs() < EPSILON);
    }

    #[test]
    fn calc_mixture_diethyl_ether_benzene_90_10() {
        let diethyl_ether_value = 1.0007;
        let benzene_value = 1.1012;
        let temperature = 298.0;
        let diethyl_ether = Substance {
            fraction: 0.9,
            functional_groups: vec![
                FunctionalGroup::from(1, 2.0).unwrap(),  // CH3
                FunctionalGroup::from(2, 1.0).unwrap(),  // CH2
                FunctionalGroup::from(25, 1.0).unwrap(), // CH2O
            ],
            gamma: None,
        };
        let benzene = Substance {
            fraction: 0.1,
            functional_groups: vec![
                FunctionalGroup::from(9, 6.0).unwrap(), // CH3
            ],
            gamma: None,
        };

        let resid = calc(vec![diethyl_ether, benzene], temperature).unwrap();

        assert!((resid[0].gamma.unwrap() - diethyl_ether_value).abs() < EPSILON);
        assert!((resid[1].gamma.unwrap() - benzene_value).abs() < EPSILON);
    }

    #[test]
    fn combinatorial_calculates() {
        let acetone_value = 0.9837063754024332;
        let pentane_value = 0.9999607147028443;
        let temperature = 298.0;

        let acetone = Substance {
            fraction: 0.047,
            functional_groups: vec![
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
            gamma: None,
        };
        let pentane = Substance {
            fraction: 0.953,
            functional_groups: vec![
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
            gamma: None,
        };

        let resid = calc(vec![acetone, pentane], temperature).unwrap();

        assert!((resid[0].gamma.unwrap() - acetone_value).abs() < EPSILON);
        assert!((resid[1].gamma.unwrap() - pentane_value).abs() < EPSILON);
    }

    #[test]
    fn residual_calculates() {
        let acetone_value = 1.6605607656292078;
        let pentane_value = 0.00534819401527342;

        let acetone = Substance {
            fraction: 0.047,
            functional_groups: vec![
                FunctionalGroup::from(1, 1.0).unwrap(),
                FunctionalGroup::from(18, 1.0).unwrap(),
            ],
            gamma: None,
        };
        let pentane = Substance {
            fraction: 0.953,
            functional_groups: vec![
                FunctionalGroup::from(1, 2.0).unwrap(),
                FunctionalGroup::from(2, 3.0).unwrap(),
            ],
            gamma: None,
        };

        let resid = calc_residual(vec![acetone, pentane], 307.0).unwrap();
        assert!((resid[0] - acetone_value).abs() < EPSILON);
        assert!((resid[1] - pentane_value).abs() < EPSILON);
    }
}
