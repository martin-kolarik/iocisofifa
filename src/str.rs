//! This module provides a way to convert between ISO 3166-1 alpha-3 codes and IOC codes.
//! `&str` is used as the underlying type storing 3-letter code, either ISO or IOC/FIFA.

use crate::{country::Country, fifa, interface::IocIsoFifa, ioc, iso_alpha3, uppercase::uppercase};

impl IocIsoFifa for &str {
    fn from_alpha2(alpha2: &str) -> Option<&'static str> {
        Country::from_alpha2(alpha2)
            .as_ref()
            .and_then(Country::alpha3)
    }

    fn from_alpha3(alpha3: &str) -> Option<&'static str> {
        Country::from_alpha3(alpha3)
            .as_ref()
            .and_then(Country::alpha3)
    }

    fn from_iso_name(name: &str) -> Option<&'static str> {
        Country::from_iso_name(name)
            .as_ref()
            .and_then(Country::alpha3)
    }

    fn from_ioc(ioc: &str) -> Option<&'static str> {
        Country::from_ioc(ioc)
            .as_ref()
            .and_then(Country::alpha3_ioc)
    }

    fn from_ioc_name(name: &str) -> Option<&'static str> {
        Country::from_ioc_name(name)
            .as_ref()
            .and_then(Country::alpha3_ioc)
    }

    fn from_fifa(fifa: &str) -> Option<&'static str> {
        Country::from_fifa(fifa)
            .as_ref()
            .and_then(Country::alpha3_fifa)
    }

    fn from_alpha3_ioc(code: &str) -> Option<&'static str> {
        Country::from_alpha3_ioc(code)
            .as_ref()
            .and_then(Country::alpha3_ioc)
    }

    fn from_alpha3_fifa(code: &str) -> Option<&'static str> {
        Country::from_alpha3_fifa(code)
            .as_ref()
            .and_then(Country::alpha3_fifa)
    }

    fn from_iso_ioc_name(name: &str) -> Option<&'static str> {
        Country::from_iso_ioc_name(name)
            .as_ref()
            .and_then(Country::alpha3_ioc)
    }

    fn from_ioc_alpha3(code: &str) -> Option<&'static str> {
        Country::from_ioc_alpha3(code)
            .as_ref()
            .and_then(Country::alpha3_ioc)
    }

    fn from_ioc_iso_name(name: &str) -> Option<&'static str> {
        Country::from_iso_ioc_name(name)
            .as_ref()
            .and_then(Country::alpha3_ioc)
    }

    fn from_fifa_alpha3(code: &str) -> Option<&'static str> {
        Country::from_fifa_alpha3(code)
            .as_ref()
            .and_then(Country::alpha3_fifa)
    }

    fn alpha2(&self) -> Option<&'static str> {
        Country::from_alpha3(self)
            .as_ref()
            .and_then(Country::alpha2)
    }

    fn alpha3(&self) -> Option<&'static str> {
        Country::from_alpha3(self)
            .as_ref()
            .and_then(Country::alpha3)
    }

    fn iso_name(&self) -> Option<&'static str> {
        Country::from_alpha3(self)
            .as_ref()
            .and_then(Country::iso_name)
    }

    fn ioc(&self) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(self, &mut buffer)?;
        ioc::ioc_to_country(candidate)
            .or_else(|| iso_alpha3::alpha3_to_country(candidate))
            .or_else(|| fifa::fifa_to_country(candidate))
            .as_ref()
            .and_then(Country::ioc)
    }

    fn ioc_name(&self) -> Option<&'static str> {
        ioc::ioc_to_country(self)
            .or_else(|| iso_alpha3::alpha3_to_country(self))
            .or_else(|| fifa::fifa_to_country(self))
            .as_ref()
            .and_then(Country::ioc_name)
    }

    fn fifa(&self) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(self, &mut buffer)?;
        fifa::fifa_to_country(candidate)
            .or_else(|| iso_alpha3::alpha3_to_country(candidate))
            .or_else(|| ioc::ioc_to_country(candidate))
            .as_ref()
            .and_then(Country::fifa)
    }

    fn alpha3_ioc(&self) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(self, &mut buffer)?;
        ioc::ioc_to_country(candidate)
            .or_else(|| iso_alpha3::alpha3_to_country(candidate))
            .or_else(|| fifa::fifa_to_country(candidate))
            .as_ref()
            .and_then(Country::alpha3_ioc)
    }

    fn iso_ioc_name(&self) -> Option<&'static str> {
        iso_alpha3::alpha3_to_country(self)
            .or_else(|| ioc::ioc_to_country(self))
            .or_else(|| fifa::fifa_to_country(self))
            .as_ref()
            .and_then(Country::iso_ioc_name)
    }

    fn alpha3_fifa(&self) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(self, &mut buffer)?;
        fifa::fifa_to_country(candidate)
            .or_else(|| iso_alpha3::alpha3_to_country(candidate))
            .or_else(|| ioc::ioc_to_country(candidate))
            .as_ref()
            .and_then(Country::alpha3_fifa)
    }

    fn ioc_alpha3(&self) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(self, &mut buffer)?;
        ioc::ioc_to_country(candidate)
            .or_else(|| iso_alpha3::alpha3_to_country(candidate))
            .or_else(|| fifa::fifa_to_country(candidate))
            .as_ref()
            .and_then(Country::ioc_alpha3)
    }

    fn ioc_iso_name(&self) -> Option<&'static str> {
        ioc::ioc_to_country(self)
            .or_else(|| iso_alpha3::alpha3_to_country(self))
            .or_else(|| fifa::fifa_to_country(self))
            .as_ref()
            .and_then(Country::ioc_iso_name)
    }

    fn fifa_alpha3(&self) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(self, &mut buffer)?;
        fifa::fifa_to_country(candidate)
            .or_else(|| iso_alpha3::alpha3_to_country(candidate))
            .or_else(|| ioc::ioc_to_country(candidate))
            .as_ref()
            .and_then(Country::fifa_alpha3)
    }
}
