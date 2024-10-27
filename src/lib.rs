#![no_std]

mod country;
pub use country::*;

mod fifa;
mod ioc;
mod ioc_name;
mod iso_alpha2;
mod iso_alpha3;
mod iso_name;
mod uppercase;
mod serde;

mod interface;
pub use interface::*;

mod str;
#[allow(unused_imports)]
pub use str::*;
