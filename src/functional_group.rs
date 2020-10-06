pub struct FunctionalGroup {
    pub variety: FunctionalGroupClass,
    pub nu: f64,
    pub r: f64,
    pub q: f64,
}

impl FunctionalGroup {
    pub fn from(name: FunctionalGroupClass, amount: f64) -> FunctionalGroup {
        match name {
            FunctionalGroupClass::CH3 => FunctionalGroup {
                variety: FunctionalGroupClass::CH3,
                nu: amount,
                r: 0.9011,
                q: 0.848,
            },
        }
    }
}

pub enum FunctionalGroupClass {
    CH3,
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
