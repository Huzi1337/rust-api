#![allow(unused_variables)]
mod utils;
use rand::Rng;

#[get("/calculateDisselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
pub fn fuel_route(distance: i32, yearOfProduction: i32, fuelUsagePer100KM: i32) -> String {
    let totalFuelConsumption: i32 = utils::calculate_fuel_usage(distance, fuelUsagePer100KM);
    return format!("Fuel used: {}", totalFuelConsumption);
}

#[get("/probabilityOfUnitInjectorFail?<vin>")]
pub fn fail_route(vin: &str) -> String {
    let RndBackedValue: f32 = rand::thread_rng().gen_range(0.00..1.00);
    return format!("Fail probability: {:.2}", RndBackedValue);
}
