#![cfg_attr(not(feature = "std"), no_std)]

mod country;
pub use country::*;

mod fifa;
mod fifa_name;
mod ioc;
mod ioc_name;
mod iso_alpha2;

mod iso_alpha3;
pub use iso_alpha3::NAC;

mod iso_name;

#[cfg(feature = "serde")]
mod serde;

mod uppercase;

mod interface;
pub use interface::*;

mod str;
#[allow(unused_imports)]
pub use str::*;
