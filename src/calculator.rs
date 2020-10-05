use crate::substance::Substance;

pub fn calc(
    substances: Vec<Substance>,
    temperature: f64) -> Vec<Substance> {
    substances
}

fn calc_combinatorial(i: u32, substances: Vec<Substance>) -> f64 {
    0.0
}

fn calc_residual(i: u32, substances: Vec<Substance>) -> f64 {
    0.0
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
