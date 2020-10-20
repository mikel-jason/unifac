use crate::functional_group::FunctionalGroup;

#[derive(Debug, Clone)]
pub struct Substance {
    pub fraction: f64,
    pub functional_groups: Vec<FunctionalGroup>,
    pub gamma: Option<f64>,
}

impl Substance {}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
