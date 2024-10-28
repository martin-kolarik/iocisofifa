//! This module provides a way to convert between ISO 3166-1 alpha-3 codes and IOC codes.
//! `&str` is used as the underlying type storing 3-letter code, either ISO or IOC/FIFA.

use crate::{
    country::Country, fifa, interface::IocIsoFifa, ioc, iso_alpha3, uppercase::uppercase,
    Precedence,
};

impl IocIsoFifa for &str {
    fn from_numeric(numeric: u32) -> Option<Self> {
        Country::from_numeric(numeric)
            .as_ref()
            .and_then(|country| country.code(Precedence::IsoIocFifa))
    }

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
            .and_then(|country| country.code(Precedence::IsoIocFifa))
    }

    fn from_ioc_name(name: &str) -> Option<&'static str> {
        Country::from_ioc_name(name)
            .as_ref()
            .and_then(|country| country.code(Precedence::IsoIocFifa))
    }

    fn from_fifa(fifa: &str) -> Option<&'static str> {
        Country::from_fifa(fifa)
            .as_ref()
            .and_then(|country| country.code(Precedence::IsoIocFifa))
    }

    fn from_code(code: &str, precedence: Precedence) -> Option<&'static str> {
        Country::from_code(code, precedence)
            .as_ref()
            .and_then(|country| country.code(Precedence::IsoIocFifa))
    }

    fn from_name(name: &str, precedence: Precedence) -> Option<&'static str> {
        Country::from_name(name, precedence)
            .as_ref()
            .and_then(|country| country.code(Precedence::IsoIocFifa))
    }

    #[cfg(feature = "std")]
    fn from_name_caseless(name: &str, precedence: Precedence) -> Option<&'static str> {
        Country::from_name_caseless(name, precedence)
            .as_ref()
            .and_then(|country| country.code(Precedence::IsoIocFifa))
    }

    fn numeric(&self) -> Option<u32> {
        iso_alpha3::code_to_country(self)
            .or_else(|| ioc::code_to_country(self))
            .or_else(|| fifa::code_to_country(self))
            .as_ref()
            .and_then(Country::numeric)
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
        let code = uppercase(self, &mut buffer)?;
        iso_alpha3::code_to_country(code)
            .or_else(|| ioc::code_to_country(code))
            .or_else(|| fifa::code_to_country(code))
            .as_ref()
            .and_then(Country::ioc)
    }

    fn ioc_name(&self) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let code = uppercase(self, &mut buffer)?;
        iso_alpha3::code_to_country(code)
            .or_else(|| ioc::code_to_country(code))
            .or_else(|| fifa::code_to_country(code))
            .as_ref()
            .and_then(Country::ioc_name)
    }

    fn fifa(&self) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let code = uppercase(self, &mut buffer)?;
        iso_alpha3::code_to_country(code)
            .or_else(|| ioc::code_to_country(code))
            .or_else(|| fifa::code_to_country(code))
            .as_ref()
            .and_then(Country::fifa)
    }

    fn code(&self, precedence: Precedence) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let code = uppercase(self, &mut buffer)?;
        iso_alpha3::code_to_country(code)
            .or_else(|| ioc::code_to_country(code))
            .or_else(|| fifa::code_to_country(code))
            .as_ref()
            .and_then(|country| country.code(precedence))
    }

    fn name(&self, precedence: Precedence) -> Option<&'static str> {
        let mut buffer = [0u8; 3];
        let code = uppercase(self, &mut buffer)?;
        iso_alpha3::code_to_country(code)
            .or_else(|| ioc::code_to_country(code))
            .or_else(|| fifa::code_to_country(code))
            .as_ref()
            .and_then(|country| country.name(precedence))
    }
}
