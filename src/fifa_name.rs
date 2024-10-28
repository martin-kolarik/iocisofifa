use crate::country::Country;

type CountryName = (Country, &'static str);

const KOS: CountryName = (Country::XKX, "Kosovo");
const ENG: CountryName = (Country::ENG, "England");
const NIR: CountryName = (Country::NIR, "Northern Ireland");
const SCO: CountryName = (Country::SCO, "Scotland");
const WAL: CountryName = (Country::WAL, "Wales");

const COUNTRY_NAME: [&CountryName; 5] = [&KOS, &ENG, &NIR, &SCO, &WAL];

pub fn name_to_country(name: &str) -> Option<Country> {
    COUNTRY_NAME
        .iter()
        .find_map(|country_name| match country_name {
            (country, candidate) if *candidate == name => Some(*country),
            _ => None,
        })
}

#[cfg(feature = "std")]
pub fn name_to_country_caseless(name: &str) -> Option<Country> {
    let name = name.to_uppercase();
    COUNTRY_NAME
        .iter()
        .find_map(|country_name| match country_name {
            (country, candidate) if candidate.to_uppercase() == name => Some(*country),
            _ => None,
        })
}

pub fn country_to_name(country: Country) -> Option<&'static str> {
    COUNTRY_NAME
        .iter()
        .find_map(|country_name| match country_name {
            (candidate, name) if *candidate == country => Some(*name),
            _ => None,
        })
}
