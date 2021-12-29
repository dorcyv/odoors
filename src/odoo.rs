use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use crate::error::Error;

pub struct Odoo {
    host: String,
    database: String,
    pub uid: Option<u32>,
    password: String,
}

#[derive(Serialize, Debug)]
struct RequestParams<T> {
    service: String,
    method: String,
    args: T,
}

#[derive(Serialize, Debug)]
struct Request<T> {
    jsonrpc: String,
    method: String,
    id: u32,
    params: RequestParams<T>,
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    pub id: u32,
    result: T,
}

impl<T> Request<T> {
    pub fn new(service: &str, method: Option<&str>, args: T) -> Request<T> {
        let mut rng = thread_rng();
        Request {
            jsonrpc: String::from("2.0"),
            method: String::from("call"),
            params: RequestParams {
                service: service.to_string(),
                method: method.unwrap_or("execute_kw").to_string(),
                args,
            },
            id: rng.gen_range(1..10000)
        }
    }
}

impl Odoo {
    pub fn new(host: &str, database: &str, username: &str, password: &str) -> Result<Odoo, Error> {
        let mut odoo = Odoo {
            host: host.to_string(),
            database: database.to_string(),
            uid: None,
            password: password.to_string(),
        };
        odoo.uid = Some(odoo.login(username)?);

        Ok(odoo)
    }

    pub fn login(&self, login: &str) -> Result<u32, Error> {
        let request = Request::new("common", Some("authenticate"), (
            self.database.as_str(),
            login,
            self.password.as_str(),
            ""
        ));
        let response: Response<u32> = self.send(&request).map_err(|e| Error(e.to_string()))?;

        Ok(response.result)
    }

    fn send<T: Serialize, U: DeserializeOwned>(&self, request: &Request<T>) -> Result<Response<U>, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/jsonrpc", self.host);
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

    #[test]
    fn test_login() {
        let conn = Odoo::new(
            "http://localhost:8069",
            "infra",
            "admin",
            "admin",
        ).unwrap();
        assert_eq!(conn.uid.unwrap(), 2);
    }
}
