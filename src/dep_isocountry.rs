pub use isocountry::CountryCode;

use crate::{fifa, ioc, uppercase::uppercase, IocIsoFifa};

impl IocIsoFifa<CountryCode> for CountryCode {
    fn from_alpha2(alpha2: &str) -> Option<CountryCode> {
        let mut buffer = [0u8; 2];
        let iso_candidate = uppercase(alpha2, &mut buffer)?;
        Self::for_alpha2(&iso_candidate).ok()
    }

    fn from_alpha3(alpha3: &str) -> Option<CountryCode> {
        let mut buffer = [0u8; 3];
        let iso_candidate = uppercase(alpha3, &mut buffer)?;
        Self::for_alpha3(iso_candidate).ok()
    }

    fn from_ioc(ioc: &str) -> Option<CountryCode> {
        let mut buffer = [0u8; 3];
        let ioc = uppercase(ioc, &mut buffer)?;
        ioc::ioc_to_alpha3(ioc).and_then(|iso| Self::for_alpha3(iso).ok())
    }

    fn from_fifa(fifa: &str) -> Option<CountryCode> {
        let mut buffer = [0u8; 3];
        let fifa = uppercase(fifa, &mut buffer)?;
        fifa::fifa_to_alpha3(fifa).and_then(|iso| Self::for_alpha3(iso).ok())
    }

    fn from_alpha3_ioc(code: &str) -> Option<CountryCode> {
        let mut buffer = [0u8; 3];
        let code = uppercase(code, &mut buffer)?;
        Self::for_alpha3(code)
            .ok()
            .or_else(|| ioc::ioc_to_alpha3(code).and_then(|iso| Self::for_alpha3(iso).ok()))
    }

    fn from_alpha3_fifa(code: &str) -> Option<CountryCode> {
        let mut buffer = [0u8; 3];
        let code = uppercase(code, &mut buffer)?;
        Self::for_alpha3(code)
            .ok()
            .or_else(|| fifa::fifa_to_alpha3(code).and_then(|iso| Self::for_alpha3(iso).ok()))
    }

    fn from_ioc_alpha3(code: &str) -> Option<CountryCode> {
        let mut buffer = [0u8; 3];
        let code = uppercase(code, &mut buffer)?;
        ioc::ioc_to_alpha3(code)
            .or(Some(code))
            .and_then(|iso| Self::for_alpha3(iso).ok())
    }

    fn from_fifa_alpha3(code: &str) -> Option<CountryCode> {
        let mut buffer = [0u8; 3];
        let code = uppercase(code, &mut buffer)?;
        fifa::fifa_to_alpha3(code)
            .or(Some(code))
            .and_then(|iso| Self::for_alpha3(iso).ok())
    }

    fn as_alpha2(&self) -> Option<&'static str> {
        Some(self.alpha2())
    }

    fn as_alpha3(&self) -> Option<&'static str> {
        Some(self.alpha3())
    }

    fn as_ioc(&self) -> Option<&'static str> {
        ioc::alpha3_to_ioc(self.alpha3())
    }

    fn as_fifa(&self) -> Option<&'static str> {
        fifa::alpha3_to_fifa(self.alpha3())
    }

    #[inline]
    fn as_alpha3_ioc(&self) -> Option<&'static str> {
        self.as_alpha3()
    }

    #[inline]
    fn as_alpha3_fifa(&self) -> Option<&'static str> {
        self.as_alpha3()
    }

    #[inline]
    fn as_ioc_alpha3(&self) -> Option<&'static str> {
        self.as_ioc()
    }

    #[inline]
    fn as_fifa_alpha3(&self) -> Option<&'static str> {
        self.as_fifa()
    }

    fn as_international_name(&self) -> Option<&'static str> {
        ioc::ioc_to_name(self.alpha3())
            .or_else(|| fifa::fifa_to_name(self.alpha3()))
            .or(Some(self.name()))
    }
}
