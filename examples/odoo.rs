use serde_json::Value;
use odoors::api::Response;
use odoors::odoo::Odoo;

fn main() {
    let odoo = Odoo::new_and_login(
        "http://localhost:8069",
        "infra",
        "admin",
        "admin",
    ).unwrap();
    let partners: Response<Vec<Value>> = odoo.search_read(
        "res.partner",
        (("name", "ilike", "a"),),
        Some(vec!["name", "email"]),
        Some(5),
        None,
    ).unwrap();

    for partner in partners.result {
        let partner = partner.as_object().unwrap();
        println!("{}", partner.get("name").unwrap())
    }
}
