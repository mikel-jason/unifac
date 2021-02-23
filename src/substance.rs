use crate::functional_group::FunctionalGroup;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Substance {
    pub name: String,
    pub fraction: f64,
    pub functional_groups: Vec<FunctionalGroup>,
    pub gamma: Option<f64>,
}

impl Substance {
    pub fn from(fraction: f64, functional_groups: Vec<FunctionalGroup>) -> Substance {
        Substance {
            name: String::from(""),
            fraction,
            functional_groups,
            gamma: None,
        }
    }

    pub fn from_name(
        name: &str,
        fraction: f64,
        functional_groups: Vec<FunctionalGroup>,
    ) -> Substance {
        Substance {
            name: String::from(name),
            fraction,
            functional_groups,
            gamma: None,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
