mod utils;

#[get("/calculateDisselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
pub fn fuel_route(distance: i32, yearOfProduction: i32, fuelUsagePer100KM: i32) -> String {
    let totalFuelConsumption: i32 = utils::calculate_fuel_usage(distance, fuelUsagePer100KM);
    return format!("Fuel used: {}", totalFuelConsumption);
}

