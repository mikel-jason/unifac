use crate::functional_group::FunctionalGroup;

pub struct Substance {
    fraction: f64,
    functional_groups: Vec<FunctionalGroup>,
    gamma: Option<Result<f64, String>>,
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
