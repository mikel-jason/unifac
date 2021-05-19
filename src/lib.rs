//! # UNIFAC
//! The `unifac` crate allows to calculate non-ideal vapor-liquid-equilibria (VLE) using the
//! *Universal Quasichemical Functional Group Activity Coefficients (UNIFAC)* model. This model,
//! first published by Fredenslund et. al. in November 1975 in [AIChE Journal](https://aiche.onlinelibrary.wiley.com/doi/abs/10.1002/aic.690210607),
//! calculates the fractions of each substance in a mixture for a given temperature. Therefore,
//! it only requires each subtance to be described as a combination of functional groups.
//! Relying on empirical data for these groups, the model does not need substance-specific data.
//! Therefore, it can be used to predict VLE without any further substance / mixture specific data.
//!
//! The following table explains used terms in this crate
//!
//! | Term | Explanation |
//! |------|-------------|
//! | Substance | A chemical substance, based on covalent bonds  (e.g. ethane C2H6) |
//! | Functional Group | A characteristic group of bonded atoms in organic substances. These are pre-defined by UNIFAC model. |
//! | Mixture | A set of substances, leading to a VLE at given temperature. |
//!
//! ## Restrictions
//! As UNIFAC model itself, `unifac` crate will calculate results even if the given mixture does not have a VLE present at all or at given temperature.
//! The project owners / devs / users are required to make sure the calculated results do apply.
//!
//! ## Data
//! All UNIFAC parameters are obtained from [UNIFAC Consortium](http://unifac.ddbst.com/) as published [here](http://www.ddbst.com/published-parameters-unifac.html).
//! These parameters are published originally within the following articles:
//!
//! >Skjold-Joergensen S., Kolbe B., Gmehling J., Rasmussen P., "Vapor-Liquid Equilibria by UNIFAC Group Contribution. Revision and Extension", Ind.Eng.Chem. Process Des.Dev., 18(4), 714-722, 1979 \
//! Gmehling J., Rasmussen P., Fredenslund Aa., "Vapor-Liquid Equilibria by UNIFAC Group Contribution. Revision and Extension. 2", Ind.Eng.Chem. Process Des.Dev., 21(1), 118-127, 1982 \
//! Macedo E.A., Weidlich U., Gmehling J., Rasmussen P., "Vapor-Liquid Equilibria by UNIFAC Group-Contribution. Revision and Extension. 3", Ind.Eng.Chem. Process Des.Dev., 22(4), 676-678, 1983 \
//! Tiegs D., Gmehling J., Rasmussen P., Fredenslund A., "Vapor-Liquid Equilibria by UNIFAC Group Contribution. 4. Revision and Extension", Ind.Eng.Chem.Res., 26(1), 159-161, 1987 \
//! Hansen H.K., Rasmussen P., Fredenslund A., Schiller M., Gmehling J., "Vapor-Liquid Equilibria by UNIFAC Group-Contribution. 5. Revision and Extension", Ind.Eng.Chem.Res., 30(10), 2352-2355, 1991 \
//! Wittig R., Lohmann J., Gmehling J., "Vapor-Liquid Equilibria by UNIFAC Group Contribution. 6. Revision and Extension", Ind.Eng.Chem.Res., 42(1), 183-188, 2003
//!
//! As cited from [DDBST](http://www.ddbst.com/published-parameters-unifac.html). Used parameters have been fetched on Oct 6, 2020.
//!
//! Please not that all data is created by the listed articles' authors and maintained by DDBST / UNIFAC Consortium. **When using this crate,
//! you are required to reference to not only the crate but also the data sources.**

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
