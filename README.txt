In order to run the API ensure you have Rust installed on your system.
To start the application navigate to the project's directory and type "cargo run" in the command line.

The server will run on localhost:8000 by default.

In that case below you'll find working mock URLs for reference:
Fuel consumption:
http://localhost:8000/calculateDisselUsageForDistance?distance=500&yearOfProduction=1999&fuelUsagePer100KM=10
Probability of unit injector fail:
http://localhost:8000/probabilityOfUnitInjectorFail?vin=asdas12asdasdsad
