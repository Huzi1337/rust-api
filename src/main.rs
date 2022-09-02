#![allow(non_snake_case)]
#[macro_use] extern crate rocket;


#[get("/calculateDisselUsageForDistance")]
fn fuel_route() -> String {
    return format!("Fuel used: {}", 20);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![fuel_route])
}
