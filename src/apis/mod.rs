use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod circuits_api;
pub use self::circuits_api::{ CircuitsApi, CircuitsApiClient };
mod dcim_api;
pub use self::dcim_api::{ DcimApi, DcimApiClient };
mod extras_api;
pub use self::extras_api::{ ExtrasApi, ExtrasApiClient };
mod ipam_api;
pub use self::ipam_api::{ IpamApi, IpamApiClient };
mod secrets_api;
pub use self::secrets_api::{ SecretsApi, SecretsApiClient };
mod tenancy_api;
pub use self::tenancy_api::{ TenancyApi, TenancyApiClient };
mod virtualization_api;
pub use self::virtualization_api::{ VirtualizationApi, VirtualizationApiClient };

pub mod configuration;
pub mod client;
