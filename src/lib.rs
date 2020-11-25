mod calculator;
mod formula;
mod functional_group;
mod interaction;
mod substance;

pub use calculator::calc;
pub use functional_group::FunctionalGroup;
pub use substance::Substance;

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
