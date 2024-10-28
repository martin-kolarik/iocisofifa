pub enum Precedence {
    IsoIocFifa,
    IsoFifaIoc,
    IocIsoFifa,
    IocFifaIso,
    FifaIocIso,
    FifaIsoIoc,
}

pub trait IocIsoFifa: Sized {
    fn from_numeric(numeric: u32) -> Option<Self>;
    fn from_alpha2(alpha2: &str) -> Option<Self>;
    fn from_alpha3(alpha3: &str) -> Option<Self>;
    fn from_iso_name(name: &str) -> Option<Self>;
    fn from_ioc(ioc: &str) -> Option<Self>;
    fn from_ioc_name(name: &str) -> Option<Self>;
    fn from_fifa(fifa: &str) -> Option<Self>;

    fn from_code(code: &str, precedence: Precedence) -> Option<Self>;

    fn from_name(name: &str, precedence: Precedence) -> Option<Self>;
    #[cfg(feature = "std")]
    fn from_name_caseless(name: &str, precedence: Precedence) -> Option<Self>;

    fn numeric(&self) -> Option<u32>;
    fn alpha2(&self) -> Option<&'static str>;
    fn alpha3(&self) -> Option<&'static str>;
    fn iso_name(&self) -> Option<&'static str>;
    fn ioc(&self) -> Option<&'static str>;
    fn ioc_name(&self) -> Option<&'static str>;
    fn fifa(&self) -> Option<&'static str>;

    fn code(&self, precedence: Precedence) -> Option<&'static str>;
    fn name(&self, precedence: Precedence) -> Option<&'static str>;
}
