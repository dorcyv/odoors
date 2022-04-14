use serde::Deserialize;

use odoors::odoo::{deserialize_odoo_nullable, Odoo};

#[derive(Deserialize)]
struct ProductTemplate {
    pub id: u32,
    pub name: String,
    #[serde(deserialize_with = "deserialize_odoo_nullable")]
    pub default_code: Option<String>,
}

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

fn main() {
    let odoo = get_odoo();
    let product_template: Vec<ProductTemplate> = odoo.search_read("product.template",
                                                                  (),
                                                                  Some(vec!["name", "default_code"]),
                                                                  None, None,
    ).unwrap().result;

    for product in product_template.iter() {
        println!("[{}] {}", product.default_code.as_ref().unwrap_or(&String::from("")), product.name);
    }
}
