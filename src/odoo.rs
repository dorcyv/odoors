use std::collections::HashMap;
use serde::de::DeserializeOwned;
use serde::{Serialize};
use crate::error::Error;
use crate::api::{Request, Response};

pub struct Odoo {
    host: String,
    database: String,
    uid: Option<u32>,
    password: Option<String>,
}


impl Odoo {
    pub fn new(host: &str, database: &str) -> Odoo {
        Odoo {
            host: host.to_string(),
            database: database.to_string(),
            uid: None,
            password: None,
        }
    }

    pub fn new_and_login(host: &str, database: &str, login: &str, password: &str) -> Result<Odoo, Error> {
        let mut odoo = Odoo::new(host, database);
        odoo.login(login, password)?;
        Ok(odoo)
    }

    pub fn login(&mut self, login: &str, password: &str) -> Result<u32, Error> {
        let request = Request::new("common", Some("authenticate"), (
            self.database.as_str(),
            login,
            password,
            ""
        ));
        let response: Response<u32> = self.send(&request, None).map_err(|e| Error(e.to_string()))?;
        self.uid = Some(response.result);
        self.password = Some(password.to_string());
        Ok(response.result)
    }

    pub fn start(&self) -> Result<HashMap<String, String>, Error> {
        let request: Request<()> = Request::new("common", Some("start"), ());

        let response: Response<HashMap<String, String>> = self.send(&request, Some("start")).map_err(|e| Error(e.to_string()))?;

        Ok(response.result)
    }

    fn send<T: Serialize, U: DeserializeOwned>(&self, request: &Request<T>, url: Option<&str>) -> Result<Response<U>, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/{}", self.host, url.unwrap_or("jsonrpc"));
        let resp = client.post(&url)
            .json(&request)
            .send()?;
        let response: Response<U> = resp.json()?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::odoo::Odoo;

    fn get_odoo() -> Odoo {
        let odoo = Odoo::new("https://demo.odoo.com", "");
        let values = odoo.start().unwrap();
        Odoo::new_and_login(
            values.get("host").unwrap(),
            values.get("database").unwrap(),
            values.get("user").unwrap(),
            values.get("password").unwrap(),
        ).unwrap()
    }

    #[test]
    fn test_start() {
        let odoo = Odoo::new("https://demo.odoo.com", "");
        let values = odoo.start().unwrap();
        assert_eq!(values.is_empty(), false);
        assert_eq!(values.contains_key("host"), true);
        assert_eq!(values.contains_key("database"), true);
        assert_eq!(values.contains_key("user"), true);
        assert_eq!(values.contains_key("password"), true);
    }

    #[test]
    fn test_login() {
        let odoo = get_odoo();
        assert_ne!(odoo.uid.unwrap(), 0);
    }
}
