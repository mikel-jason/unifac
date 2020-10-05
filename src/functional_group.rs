pub struct Functional_group {
    variety: Functional_group_class,
    nu: f64,
}

impl Functional_group {
}

pub enum Functional_group_class {
    CH3,
}

impl Functional_group_class {
    pub fn id(&self) -> i32 {
        match self {
            Self::CH3 => 1,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
