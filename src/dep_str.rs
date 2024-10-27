//! This module provides a way to convert between ISO 3166-1 alpha-3 codes and IOC codes.
//! `&str` is used as the underlying type storing 3-letter code, either ISO or IOC/FIFA.

use crate::{
    fifa,
    interface::IocIsoFifa,
    ioc, iso_alpha2,
    iso_alpha3::{self, country_to_alpha3},
    uppercase::uppercase,
};

impl IocIsoFifa<&'static str> for str {
    fn from_alpha2(alpha2: &str) -> Option<&'static str> {
        let mut buffer = [0u8; 2];
        let candidate = uppercase(alpha2, &mut buffer)?;
        iso_alpha2::alpha2_to_country(candidate).and_then(country_to_alpha3)
    }

    fn from_alpha3(alpha3: &str) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(alpha3, &mut buffer)?;
        iso_alpha3::alpha3_to_alpha3(&candidate)
    }

    fn from_ioc(ioc: &str) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(ioc, &mut buffer)?;
        ioc::ioc_to_country(candidate).and_then(country_to_alpha3)
    }

    fn from_fifa(fifa: &str) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(fifa, &mut buffer)?;
        fifa::fifa_to_country(candidate).and_then(country_to_alpha3)
    }

    fn from_alpha3_ioc(code: &str) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(code, &mut buffer)?;
        iso_alpha3::alpha3_to_country(candidate)
            .or_else(|| ioc::ioc_to_country(candidate))
            .and_then(country_to_alpha3)
    }

    fn from_alpha3_fifa(code: &str) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(code, &mut buffer)?;
        iso_alpha3::alpha3_to_country(candidate)
            .or_else(|| fifa::fifa_to_country(candidate))
            .and_then(country_to_alpha3)
    }

    fn from_ioc_alpha3(code: &str) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(code, &mut buffer)?;
        ioc::ioc_to_country(candidate)
            .or_else(|| iso_alpha3::alpha3_to_country(candidate))
            .and_then(country_to_alpha3)
    }

    fn from_fifa_alpha3(code: &str) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(code, &mut buffer)?;
        fifa::fifa_to_country(candidate)
            .or_else(|| iso_alpha3::alpha3_to_country(candidate))
            .and_then(country_to_alpha3)
    }

    fn as_alpha2(&self) -> Option<&'static str> {
        iso_alpha3::alpha3_to_country(self).and_then(iso_alpha2::country_to_alpha2)
    }

    fn as_alpha3(&self) -> Option<&'static str> {
        iso_alpha3::alpha3_to_alpha3(self)
    }

    fn as_ioc(&self) -> Option<&'static str> {
        iso_alpha3::alpha3_to_country(self).and_then(ioc::country_to_ioc)
    }

    fn as_fifa(&self) -> Option<&'static str> {
        iso_alpha3::alpha3_to_country(self).and_then(fifa::country_to_fifa)
    }

    fn as_alpha3_ioc(&self) -> Option<&'static str> {
        iso_alpha3::alpha3_to_country(self).or_else(|| ioc::ioc_to_country(self)).and_then(country_to_alpha3)
    }

    fn as_alpha3_fifa(&self) -> Option<&'static str> {
        iso_alpha3::alpha3_to_country(self).or_else(|| fifa::fifa_to_fifa(self))
    }

    fn as_ioc_alpha3(&self) -> Option<&'static str> {
        self.as_ioc()
            .or_else(|| iso_alpha3::alpha3_to_country(self))
    }

    fn as_fifa_alpha3(&self) -> Option<&'static str> {
        self.as_fifa()
            .or_else(|| iso_alpha3::alpha3_to_country(self))
    }

    fn as_iso_name(&self) -> Option<&'static str> {
        ioc::ioc_to_name(self).or_else(|| fifa::fifa_to_name(self))
    }
}
