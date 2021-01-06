# UNIFAC &emsp; ![action] ![license] ![version]

[version]: https://img.shields.io/crates/v/unifac.svg
[license]: https://img.shields.io/crates/l/unifac.svg
[action]: https://img.shields.io/github/workflow/status/sarcaustech/unifac/CI/main

**Implementation of the UNIFAC model to calculate activity coefficients in VLE using Rust**

---

## Table of contents
- [UNIFAC model](#unifac-model)
- [Usage](#usage)
- [Data](#data)
- [See also](#see-also)
- [Project origin](#project-origin)

## UNIFAC model
The *Universal Quasichemical Functional Group Activity Coefficients (UNIFAC)* allows to calculate the activity coefficients of substances within non-ideal mixtures. Despite other models like UNIQUAC, NRTL or van Laar, UNIFAC requires no substance-specific data. As a group contribution method, UNIFAC works with substances as a combination of functional groups. These functional groups are defined alongside the model and hold the data required to apply the model. 

For more information about the model, see the original paper *Group‐contribution estimation of activity coefficients in nonideal liquid mixtures* from Aage Fredenslund, Russell L. Jones and John M. Prausnitz (Nov 1975) in [AIChE Journal](https://aiche.onlinelibrary.wiley.com/doi/abs/10.1002/aic.690210607).

## Usage
The model assumes that the input mixture (substances, fractions, temperature) results in a vapor-liquid-equilibrium and does not check this itself. The model calculates activity coefficients either way. **Make sure you are working on a VLE yourself!**

Please see the crate's [documentation](https://crates.io/crates/unifac) for more info.

## Data
All UNIFAC parameters are obtained from [UNIFAC Consortium](http://unifac.ddbst.com/) as published [here](http://www.ddbst.com/published-parameters-unifac.html). These parameters are published originally within the following articles:

>Skjold-Joergensen S., Kolbe B., Gmehling J., Rasmussen P., "Vapor-Liquid Equilibria by UNIFAC Group Contribution. Revision and Extension", Ind.Eng.Chem. Process Des.Dev., 18(4), 714-722, 1979 \
Gmehling J., Rasmussen P., Fredenslund Aa., "Vapor-Liquid Equilibria by UNIFAC Group Contribution. Revision and Extension. 2", Ind.Eng.Chem. Process Des.Dev., 21(1), 118-127, 1982 \
Macedo E.A., Weidlich U., Gmehling J., Rasmussen P., "Vapor-Liquid Equilibria by UNIFAC Group-Contribution. Revision and Extension. 3", Ind.Eng.Chem. Process Des.Dev., 22(4), 676-678, 1983 \
Tiegs D., Gmehling J., Rasmussen P., Fredenslund A., "Vapor-Liquid Equilibria by UNIFAC Group Contribution. 4. Revision and Extension", Ind.Eng.Chem.Res., 26(1), 159-161, 1987 \
Hansen H.K., Rasmussen P., Fredenslund A., Schiller M., Gmehling J., "Vapor-Liquid Equilibria by UNIFAC Group-Contribution. 5. Revision and Extension", Ind.Eng.Chem.Res., 30(10), 2352-2355, 1991 \
Wittig R., Lohmann J., Gmehling J., "Vapor-Liquid Equilibria by UNIFAC Group Contribution. 6. Revision and Extension", Ind.Eng.Chem.Res., 42(1), 183-188, 2003

As cited from [DDBST](http://www.ddbst.com/published-parameters-unifac.html). Used parameters have been fetched on Oct 6, 2020.

Please not that all data is created by the listed articles' authors and maintained by DDBST / UNIFAC Consortium. **When using this crate, you are required to reference to not only the crate but also the data sources.**

## See also
- [unifac on GitHub](https://github.com/sarcaustech/unifac)
- [unifac on crates.io](https://crates.io/crates/unifac)
- [unifac documentation on docs.rs](https://docs.rs/unifac)
- [unifac-cli](https://github.com/sarcaustech/unifac-cli): A command-line tool leveraging this crate. As input, it uses simple YAML files as input

## Project origin
This crate originates from a University project at [Baden-Wuerttemberg Cooperative State University (DHBW)](https://www.dhbw.de/english/home) by [sarcaustech](https://github.com/sarcaustech) and [heringerp](https://github.com/heringerp).
