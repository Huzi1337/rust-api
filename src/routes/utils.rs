pub fn calculate_fuel_usage(distance: i32, fuelUsagePer100KM: i32) -> i32 {
    distance/100*fuelUsagePer100KM
}