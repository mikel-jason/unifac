use crate::functional_group::FunctionalGroup;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone)]
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

impl Serialize for Substance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 2 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Substance", 3)?;
        state.serialize_field("Substance", &self.name)?;
        state.serialize_field("Fraction", &self.fraction)?;
        state.serialize_field("Gamma", &self.gamma)?;
        state.end()
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
