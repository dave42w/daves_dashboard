
use handlebars::Handlebars;

use dotenvy::dotenv;
use std::env;


pub fn init_templates(dir: String) -> Handlebars<'static> {
    let mut handlebars: Handlebars= Handlebars::new();

    handlebars
        .register_templates_directory(".hbs", dir)
        .unwrap();

    return handlebars
}

pub fn get_templates() -> Handlebars<'static> {
    dotenv().expect(".env file not found");
    
    init_templates(env::var("templates").unwrap())
}