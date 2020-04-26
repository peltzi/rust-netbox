use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    circuits_api: Box<dyn crate::apis::CircuitsApi>,
    dcim_api: Box<dyn crate::apis::DcimApi>,
    extras_api: Box<dyn crate::apis::ExtrasApi>,
    ipam_api: Box<dyn crate::apis::IpamApi>,
    secrets_api: Box<dyn crate::apis::SecretsApi>,
    tenancy_api: Box<dyn crate::apis::TenancyApi>,
    virtualization_api: Box<dyn crate::apis::VirtualizationApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            circuits_api: Box::new(crate::apis::CircuitsApiClient::new(rc.clone())),
            dcim_api: Box::new(crate::apis::DcimApiClient::new(rc.clone())),
            extras_api: Box::new(crate::apis::ExtrasApiClient::new(rc.clone())),
            ipam_api: Box::new(crate::apis::IpamApiClient::new(rc.clone())),
            secrets_api: Box::new(crate::apis::SecretsApiClient::new(rc.clone())),
            tenancy_api: Box::new(crate::apis::TenancyApiClient::new(rc.clone())),
            virtualization_api: Box::new(crate::apis::VirtualizationApiClient::new(rc.clone())),
        }
    }

    pub fn circuits_api(&self) -> &dyn crate::apis::CircuitsApi{
        self.circuits_api.as_ref()
    }

    pub fn dcim_api(&self) -> &dyn crate::apis::DcimApi{
        self.dcim_api.as_ref()
    }

    pub fn extras_api(&self) -> &dyn crate::apis::ExtrasApi{
        self.extras_api.as_ref()
    }

    pub fn ipam_api(&self) -> &dyn crate::apis::IpamApi{
        self.ipam_api.as_ref()
    }

    pub fn secrets_api(&self) -> &dyn crate::apis::SecretsApi{
        self.secrets_api.as_ref()
    }

    pub fn tenancy_api(&self) -> &dyn crate::apis::TenancyApi{
        self.tenancy_api.as_ref()
    }

    pub fn virtualization_api(&self) -> &dyn crate::apis::VirtualizationApi{
        self.virtualization_api.as_ref()
    }

}
