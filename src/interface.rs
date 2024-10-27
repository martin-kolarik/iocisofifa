pub enum IsoIocFifaCode {
    None,
    Iso(&'static str),
    Ioc(&'static str),
    Fifa(&'static str),
}

impl IsoIocFifaCode {
    pub const fn as_iso(&self) -> Option<&'static str> {
        match self {
            IsoIocFifaCode::Iso(iso) => Some(iso),
            _ => None,
        }
    }

    pub const fn as_ioc(&self) -> Option<&'static str> {
        match self {
            IsoIocFifaCode::Ioc(iso) => Some(iso),
            _ => None,
        }
    }

    pub const fn as_fifa(&self) -> Option<&'static str> {
        match self {
            IsoIocFifaCode::Fifa(iso) => Some(iso),
            _ => None,
        }
    }
}

pub trait IocIsoFifa<T> {
    fn from_alpha2(alpha2: &str) -> Option<T>;
    fn from_alpha3(alpha3: &str) -> Option<T>;
    fn from_iso_name(name: &str) -> Option<T>;
    fn from_ioc(ioc: &str) -> Option<T>;
    fn from_ioc_name(name: &str) -> Option<T>;
    fn from_fifa(fifa: &str) -> Option<T>;

    fn from_alpha3_ioc(code: &str) -> Option<T>;
    fn from_alpha3_fifa(code: &str) -> Option<T>;
    fn from_iso_ioc_name(name: &str) -> Option<T>;
    fn from_ioc_alpha3(code: &str) -> Option<T>;
    fn from_ioc_iso_name(name: &str) -> Option<T>;
    fn from_fifa_alpha3(code: &str) -> Option<T>;

    fn as_alpha2(&self) -> Option<&'static str>;
    fn as_alpha3(&self) -> Option<&'static str>;
    fn as_iso_name(&self) -> Option<&'static str>;
    fn as_ioc(&self) -> Option<&'static str>;
    fn as_ioc_name(&self) -> Option<&'static str>;
    fn as_fifa(&self) -> Option<&'static str>;

    /// Returns ISO 3166-1 alpha-3 code or IOC code as a backup.
    fn as_alpha3_ioc(&self) -> Option<&'static str>;
    fn as_iso_ioc_name(&self) -> Option<&'static str>;

    /// Returns ISO 3166-1 alpha-3 code or FIFA code as a backup.
    fn as_alpha3_fifa(&self) -> Option<&'static str>;

    /// Returns IOC code or ISO 3166-1 alpha-3 as a backup.
    fn as_ioc_alpha3(&self) -> Option<&'static str>;
    fn as_ioc_iso_name(&self) -> Option<&'static str>;

    /// Returns FIFA code or ISO 3166-1 alpha-3 a backup.
    fn as_fifa_alpha3(&self) -> Option<&'static str>;
}
