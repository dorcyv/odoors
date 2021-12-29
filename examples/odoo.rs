use std::collections::HashMap;
use serde_json::Value;
use odoo::api::Response;
use odoo::odoo::Odoo;

fn main() {
    let odoo = Odoo::new_and_login(
        "http://localhost:8069",
        "infra",
        "admin",
        "admin"
    ).unwrap();
    let partners: Response<Vec<HashMap<String, Value>>> = odoo.call(
        "res.partner",
        "search_read",
        (
            [("id", ">", 2)],
            ["name", "parent_id"]
        )
    ).unwrap();
    for partner in partners.result {
        println!("{}", partner.get("name").unwrap())
    }
}
