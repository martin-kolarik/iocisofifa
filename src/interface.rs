pub trait IocIsoFifa: Sized {
    fn from_alpha2(alpha2: &str) -> Option<Self>;
    fn from_alpha3(alpha3: &str) -> Option<Self>;
    fn from_iso_name(name: &str) -> Option<Self>;
    fn from_ioc(ioc: &str) -> Option<Self>;
    fn from_ioc_name(name: &str) -> Option<Self>;
    fn from_fifa(fifa: &str) -> Option<Self>;

    fn from_alpha3_ioc(code: &str) -> Option<Self>;
    fn from_alpha3_fifa(code: &str) -> Option<Self>;
    fn from_iso_ioc_name(name: &str) -> Option<Self>;
    fn from_ioc_alpha3(code: &str) -> Option<Self>;
    fn from_ioc_iso_name(name: &str) -> Option<Self>;
    fn from_fifa_alpha3(code: &str) -> Option<Self>;

    fn alpha2(&self) -> Option<&'static str>;
    fn alpha3(&self) -> Option<&'static str>;
    fn iso_name(&self) -> Option<&'static str>;
    fn ioc(&self) -> Option<&'static str>;
    fn ioc_name(&self) -> Option<&'static str>;
    fn fifa(&self) -> Option<&'static str>;

    /// Returns ISO 3166-1 alpha-3 code or IOC code as a backup.
    fn alpha3_ioc(&self) -> Option<&'static str>;
    fn iso_ioc_name(&self) -> Option<&'static str>;

    /// Returns ISO 3166-1 alpha-3 code or FIFA code as a backup.
    fn alpha3_fifa(&self) -> Option<&'static str>;

    /// Returns IOC code or ISO 3166-1 alpha-3 as a backup.
    fn ioc_alpha3(&self) -> Option<&'static str>;
    fn ioc_iso_name(&self) -> Option<&'static str>;

    /// Returns FIFA code or ISO 3166-1 alpha-3 a backup.
    fn fifa_alpha3(&self) -> Option<&'static str>;
}
