use crate::functional_group::FunctionalGroup;
use serde::{Deserialize, Serialize};

/// A substance within a VLE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Substance {
    /// Verbose name of the substance
    pub name: String,
    /// - Fraction (molar) of the substance in the containing mixture
    pub fraction: f64,
    /// - Functional groups defining the substance
    pub functional_groups: Vec<FunctionalGroup>,
    /// - Calculated activity coefficient
    pub gamma: Option<f64>,
}

impl Substance {
    /// Create a new substance
    ///
    /// To set a verbose name, use [`Substance::from_name`]
    ///
    /// # Arguments
    /// - `fraction` - Fraction (molar) of the substance in the containing mixture
    /// - `functional_groups` - Functional groups defining the substance
    ///
    /// # Example
    /// ```
    /// use unifac::*;
    /// let pentane = Substance::from(
    ///     0.5,
    ///     vec![
    ///         FunctionalGroup::from(1, 2.0).unwrap(),
    ///         FunctionalGroup::from(2, 3.0).unwrap(),
    ///     ],
    /// );
    /// ```
    pub fn from(fraction: f64, functional_groups: Vec<FunctionalGroup>) -> Substance {
        Substance {
            name: String::from(""),
            fraction,
            functional_groups,
            gamma: None,
        }
    }

    /// Create a new substance
    ///
    /// # Arguments
    /// - `name ` - Verbose name
    /// - `fraction` - Fraction (molar) of the substance in the containing mixture
    /// - `functional_groups` - Functional groups defining the substance
    ///
    /// # Example
    /// ```
    /// use unifac::*;
    /// let pentane = Substance::from_name(
    ///     "pentane",
    ///     0.5,
    ///     vec![
    ///         FunctionalGroup::from(1, 2.0).unwrap(),
    ///         FunctionalGroup::from(2, 3.0).unwrap(),
    ///     ],
    /// );
    /// ```
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
