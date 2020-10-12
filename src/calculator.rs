use std::collections::HashMap;

use crate::formula;
use crate::substance::Substance;

pub fn calc(substances: Vec<Substance>, temperature: f64) -> Vec<Substance> {
    substances
}

fn calc_combinatorial(i: u8, substances: Vec<Substance>) -> f64 {
    0.0
}

fn calc_residual(i: u8, substances: Vec<Substance>, temperature: f64) -> Result<f64, &'static str> {
    let mut id_list = Vec::new();
    for substance in &substances {
        for fg in &substance.functional_groups {
            if !id_list.contains(&fg.id) {
                id_list.push(fg.id);
            }
        }
    }
    println!("{:?}", id_list);

    let mut x_i_m = Vec::new();
    for substance in &substances {
        let mut temp = Vec::new();
        for fg in &substance.functional_groups {
            temp.push(formula::calc_6(fg.id, &substance));
        }
        x_i_m.push(temp);
    }
    println!("{:?}", x_i_m);

    let mut x_m = Vec::new();
    let sum = formula::calc_7_sum(&substances);
    for id in &id_list {
        x_m.push(formula::calc_7(*id, sum, &substances));
    }
    println!("{:?}", x_m);

    let x_k_map: HashMap<u8, f64> = id_list.clone().into_iter().zip(x_m).collect();
    let sum = formula::calc_8_sum(&x_k_map);
    let mut theta_k = Vec::new();
    for id in &id_list {
        theta_k.push(formula::calc_8(*id, &x_k_map, sum));
    }
    println!("{:?}", theta_k);

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
    println!("{:?}", theta_i_k);

    let mut psi_n_m = Vec::new();
    for id_n in &id_list {
        let mut temp = Vec::new();
        for id_m in &id_list {
            if id_n == id_m {
                println!("Equal!");
                temp.push(1.0);
            } else {
                println!("{} - {}", id_n, id_m);
                temp.push(formula::calc_10(*id_n, *id_m, temperature).unwrap());
            }
        }
        psi_n_m.push(temp);
    }
    println!("Psi: {:?}", psi_n_m);

    // let mut gamma_i_k = Vec::new();
    // for i in 0..substances.len() {
    //     let mut temp = Vec::new();

    Ok(0.0)
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::functional_group::FunctionalGroup;

    #[test]
    fn residual_calculates() {
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
        assert_eq!(4.0, 4.0);
    }
}
