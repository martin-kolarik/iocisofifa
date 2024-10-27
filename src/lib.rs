#![cfg_attr(all(feature = "iso3166", not(feature = "isocountry")), no_std)]

mod country;
mod fifa;
mod ioc;
mod ioc_name;
mod iso_alpha2;
mod iso_alpha3;
mod iso_name;
mod uppercase;

mod interface;
pub use interface::*;

mod dep_str;
#[allow(unused_imports)]
pub use dep_str::*;

// #[cfg(feature = "iso3166")]
// mod dep_iso3166;
// #[cfg(feature = "iso3166")]
// pub use dep_iso3166::*;

#[cfg(feature = "isocountry")]
mod dep_isocountry;
#[cfg(feature = "isocountry")]
pub use dep_isocountry::*;
