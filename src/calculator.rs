use std::collections::HashMap;

use crate::formula;
use crate::functional_group::FunctionalGroup;
use crate::substance::Substance;

pub fn calc(substances: Vec<Substance>, temperature: f64) -> Vec<Substance> {
    substances
}

fn calc_combinatorial(i: u8, substances: Vec<Substance>) -> f64 {
    0.0
}

fn calc_residual(
    i: u8,
    substances: Vec<Substance>,
    temperature: f64,
) -> Result<Vec<f64>, &'static str> {
    let mut id_list = Vec::new();
    for substance in &substances {
        for fg in &substance.functional_groups {
            if !id_list.contains(&fg.id) {
                id_list.push(fg.id);
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
    for id in &id_list {
        x_m.push(formula::calc_7(*id, sum, &substances));
    }

    let x_k_map: HashMap<u8, f64> = id_list.clone().into_iter().zip(x_m).collect();
    let sum = formula::calc_8_sum(&x_k_map);
    let mut theta_k = Vec::new();
    for id in &id_list {
        theta_k.push(formula::calc_8(*id, &x_k_map, sum));
    }

    let mut theta_i_k = Vec::new();
    for i in 0..substances.len() {
        let mut temp = Vec::new();
        let mut x_ki_map: HashMap<u8, f64> = HashMap::new();
        for j in 0..substances[i].functional_groups.len() {
            x_ki_map.insert(substances[i].functional_groups[j].id, x_i_m[i][j]);
        }
        let sum = formula::calc_8_sum(&x_ki_map);
        for fg in &substances[i].functional_groups {
            temp.push(formula::calc_9(fg.id, &x_ki_map, sum));
        }
        theta_i_k.push(temp);
    }

    let mut psi_n_m = Vec::new();
    for id_n in &id_list {
        let mut temp = Vec::new();
        for id_m in &id_list {
            if id_n == id_m {
                temp.push(1.0);
            } else {
                temp.push(formula::calc_10(*id_n, *id_m, temperature).unwrap());
            }
        }
        psi_n_m.push(temp);
    }

    let mut psi_m_k = Vec::new();
    for i in 0..id_list.len() {
        let mut temp = Vec::new();
        for psi_m in &psi_n_m {
            temp.push(psi_m[i]);
        }
        psi_m_k.push(temp);
    }

    let mut gamma_i_k = Vec::new();
    for i in 0..substances.len() {
        let mut temp = Vec::new();
        let mut fg_ids = Vec::new();
        for fg in &substances[i].functional_groups {
            fg_ids.push(fg.id);
        }
        let mut shadow = Vec::new();
        for id in &id_list {
            if fg_ids.contains(id) {
                shadow.push(false);
            } else {
                shadow.push(true);
            }
        }
        for fg in &substances[i].functional_groups {
            let index = id_list.iter().position(|&x| x == fg.id).unwrap();
            let sum_1 = formula::calc_11_sum_1(&theta_i_k[i], &psi_m_k[index], &shadow);
            let sum_2 = formula::calc_11_sum_2(index, &theta_i_k[i], &psi_n_m, &shadow);
            temp.push(formula::calc_11(fg.q, sum_1, sum_2));
        }
        gamma_i_k.push(temp);
    }

    let mut gamma_k = Vec::new();
    for i in 0..id_list.len() {
        let sum_1 = formula::calc_11_sum_1(&theta_k, &psi_m_k[i], &vec![false; id_list.len()]);
        let sum_2 = formula::calc_11_sum_2(i, &theta_k, &psi_n_m, &vec![false; id_list.len()]);
        let q_k = FunctionalGroup::from(id_list[i], 1.0).unwrap().q;
        gamma_k.push(formula::calc_12(q_k, sum_1, sum_2));
    }

    let mut gamma = Vec::new();
    for i in 0..substances.len() {
        let mut gamma_k_subst = Vec::new();
        for fg in &substances[i].functional_groups {
            let index = id_list.iter().position(|&x| x == fg.id).unwrap();
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
#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.001;

    #[test]
    fn residual_calculates() {
        let acetone_value = 0.6709271125908701;
        let pentane_value = 0.00177787069083582;
        let acetone = Substance {
            fraction: 0.047,
            functional_groups: vec![
                FunctionalGroup::from(1, 1.0).unwrap(),  // CH3
                FunctionalGroup::from(18, 1.0).unwrap(), // CH3CO
            ],
            gamma: None,
        };
        let pentane = Substance {
            fraction: 0.953,
            functional_groups: vec![
                FunctionalGroup::from(1, 2.0).unwrap(), // CH3
                FunctionalGroup::from(2, 3.0).unwrap(), // CH2
            ],
            gamma: None,
        };

        let resid = calc_residual(0, vec![acetone, pentane], 307.0).unwrap();
        assert!((resid[0] - acetone_value).abs() < EPSILON);
        assert!((resid[1] - pentane_value).abs() < EPSILON);
    }
}
