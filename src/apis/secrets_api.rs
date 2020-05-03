/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 2.7
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use reqwest;

use super::{configuration, Error};

pub struct SecretsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl SecretsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> SecretsApiClient {
        SecretsApiClient { configuration }
    }
}

pub trait SecretsApi {
    fn secrets_choices_list(&self) -> Result<(), Error>;
    fn secrets_choices_read(&self, id: &str) -> Result<(), Error>;
    fn secrets_generate_rsa_key_pair_list(&self) -> Result<(), Error>;
    fn secrets_get_session_key_create(&self) -> Result<(), Error>;
    fn secrets_secret_roles_create(
        &self,
        data: crate::models::SecretRole,
    ) -> Result<crate::models::SecretRole, Error>;
    fn secrets_secret_roles_delete(&self, id: i32) -> Result<(), Error>;
    fn secrets_secret_roles_list(
        &self,
        id: Option<&str>,
        name: Option<&str>,
        slug: Option<&str>,
        q: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<crate::models::InlineResponse20052, Error>;
    fn secrets_secret_roles_partial_update(
        &self,
        id: i32,
        data: crate::models::SecretRole,
    ) -> Result<crate::models::SecretRole, Error>;
    fn secrets_secret_roles_read(&self, id: i32) -> Result<crate::models::SecretRole, Error>;
    fn secrets_secret_roles_update(
        &self,
        id: i32,
        data: crate::models::SecretRole,
    ) -> Result<crate::models::SecretRole, Error>;
    fn secrets_secrets_create(
        &self,
        data: crate::models::WritableSecret,
    ) -> Result<crate::models::Secret, Error>;
    fn secrets_secrets_delete(&self, id: i32) -> Result<(), Error>;
    fn secrets_secrets_list(
        &self,
        name: Option<&str>,
        created: Option<&str>,
        created__gte: Option<&str>,
        created__lte: Option<&str>,
        last_updated: Option<&str>,
        last_updated__gte: Option<&str>,
        last_updated__lte: Option<&str>,
        id__in: Option<&str>,
        q: Option<&str>,
        role_id: Option<&str>,
        role: Option<&str>,
        device_id: Option<&str>,
        device: Option<&str>,
        tag: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<crate::models::InlineResponse20053, Error>;
    fn secrets_secrets_partial_update(
        &self,
        id: i32,
        data: crate::models::WritableSecret,
    ) -> Result<crate::models::Secret, Error>;
    fn secrets_secrets_read(&self, id: i32) -> Result<crate::models::Secret, Error>;
    fn secrets_secrets_update(
        &self,
        id: i32,
        data: crate::models::WritableSecret,
    ) -> Result<crate::models::Secret, Error>;
}

impl SecretsApi for SecretsApiClient {
    fn secrets_choices_list(&self) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/_choices/", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn secrets_choices_read(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/secrets/_choices/{id}/",
            configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn secrets_generate_rsa_key_pair_list(&self) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/generate-rsa-key-pair/", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn secrets_get_session_key_create(&self) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/get-session-key/", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn secrets_secret_roles_create(
        &self,
        data: crate::models::SecretRole,
    ) -> Result<crate::models::SecretRole, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/secret-roles/", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&data);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn secrets_secret_roles_delete(&self, id: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/secrets/secret-roles/{id}/",
            configuration.base_path,
            id = id
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn secrets_secret_roles_list(
        &self,
        id: Option<&str>,
        name: Option<&str>,
        slug: Option<&str>,
        q: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<crate::models::InlineResponse20052, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/secret-roles/", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = id {
            req_builder = req_builder.query(&[("id", &s.to_string())]);
        }
        if let Some(ref s) = name {
            req_builder = req_builder.query(&[("name", &s.to_string())]);
        }
        if let Some(ref s) = slug {
            req_builder = req_builder.query(&[("slug", &s.to_string())]);
        }
        if let Some(ref s) = q {
            req_builder = req_builder.query(&[("q", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("limit", &s.to_string())]);
        }
        if let Some(ref s) = offset {
            req_builder = req_builder.query(&[("offset", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn secrets_secret_roles_partial_update(
        &self,
        id: i32,
        data: crate::models::SecretRole,
    ) -> Result<crate::models::SecretRole, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/secrets/secret-roles/{id}/",
            configuration.base_path,
            id = id
        );
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&data);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn secrets_secret_roles_read(&self, id: i32) -> Result<crate::models::SecretRole, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/secrets/secret-roles/{id}/",
            configuration.base_path,
            id = id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn secrets_secret_roles_update(
        &self,
        id: i32,
        data: crate::models::SecretRole,
    ) -> Result<crate::models::SecretRole, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/secrets/secret-roles/{id}/",
            configuration.base_path,
            id = id
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&data);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn secrets_secrets_create(
        &self,
        data: crate::models::WritableSecret,
    ) -> Result<crate::models::Secret, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/secrets/", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&data);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn secrets_secrets_delete(&self, id: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/secrets/{id}/", configuration.base_path, id = id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn secrets_secrets_list(
        &self,
        name: Option<&str>,
        created: Option<&str>,
        created__gte: Option<&str>,
        created__lte: Option<&str>,
        last_updated: Option<&str>,
        last_updated__gte: Option<&str>,
        last_updated__lte: Option<&str>,
        id__in: Option<&str>,
        q: Option<&str>,
        role_id: Option<&str>,
        role: Option<&str>,
        device_id: Option<&str>,
        device: Option<&str>,
        tag: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<crate::models::InlineResponse20053, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/secrets/", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = name {
            req_builder = req_builder.query(&[("name", &s.to_string())]);
        }
        if let Some(ref s) = created {
            req_builder = req_builder.query(&[("created", &s.to_string())]);
        }
        if let Some(ref s) = created__gte {
            req_builder = req_builder.query(&[("created__gte", &s.to_string())]);
        }
        if let Some(ref s) = created__lte {
            req_builder = req_builder.query(&[("created__lte", &s.to_string())]);
        }
        if let Some(ref s) = last_updated {
            req_builder = req_builder.query(&[("last_updated", &s.to_string())]);
        }
        if let Some(ref s) = last_updated__gte {
            req_builder = req_builder.query(&[("last_updated__gte", &s.to_string())]);
        }
        if let Some(ref s) = last_updated__lte {
            req_builder = req_builder.query(&[("last_updated__lte", &s.to_string())]);
        }
        if let Some(ref s) = id__in {
            req_builder = req_builder.query(&[("id__in", &s.to_string())]);
        }
        if let Some(ref s) = q {
            req_builder = req_builder.query(&[("q", &s.to_string())]);
        }
        if let Some(ref s) = role_id {
            req_builder = req_builder.query(&[("role_id", &s.to_string())]);
        }
        if let Some(ref s) = role {
            req_builder = req_builder.query(&[("role", &s.to_string())]);
        }
        if let Some(ref s) = device_id {
            req_builder = req_builder.query(&[("device_id", &s.to_string())]);
        }
        if let Some(ref s) = device {
            req_builder = req_builder.query(&[("device", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("tag", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("limit", &s.to_string())]);
        }
        if let Some(ref s) = offset {
            req_builder = req_builder.query(&[("offset", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn secrets_secrets_partial_update(
        &self,
        id: i32,
        data: crate::models::WritableSecret,
    ) -> Result<crate::models::Secret, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/secrets/{id}/", configuration.base_path, id = id);
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&data);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn secrets_secrets_read(&self, id: i32) -> Result<crate::models::Secret, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/secrets/{id}/", configuration.base_path, id = id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn secrets_secrets_update(
        &self,
        id: i32,
        data: crate::models::WritableSecret,
    ) -> Result<crate::models::Secret, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/secrets/secrets/{id}/", configuration.base_path, id = id);
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&data);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
