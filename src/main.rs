#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::env;

pub mod routes;

#[launch]
fn rocket() -> _ {
    let static_path = env::var("STATIC_PATH").unwrap_or("/var/www/whoknows/static".to_string());
    rocket::build()
        .mount(
            "/",
            routes![
                routes::pages::index,
                routes::pages::about,
                routes::pages::login,
                routes::pages::register,
                routes::pages::search
            ],
        )
        .mount("/static", FileServer::from(static_path))
        .attach(Template::fairing())
}
