# weather_util_rust

[![crates.io](https://meritbadge.herokuapp.com/weather_util_rust)](https://crates.io/crates/weather_util_rust)
[![Build Status](https://github.com/ddboline/weather_util_rust/workflows/Rust/badge.svg?branch=master)](https://github.com/ddboline/weather_util_rust/actions?branch=master)
[![Documentation](https://docs.rs/weather_util_rust/badge.svg)](https://docs.rs/weather_util_rust/0.1.3/s3_ext/)
[![codecov](https://codecov.io/gh/ddboline/weather_util_rust/branch/master/graph/badge.svg)](https://codecov.io/gh/ddboline/weather_util_rust)

CLI tool to pull weather data from openweathermap.org, intended to replace the debian package `weather-util`.

You will need to obtain an api key by signing up at openweathermap.org.

Usage:

```bash
weather_util_rust
Utility to retrieve and format weather data from openweathermap.org

USAGE:
    weather-util-rust [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --api-key <api-key>              Api key (optional but either this or API_KEY environment variable must exist)
        --city-name <city-name>          City Name
    -c, --country-code <country-code>    Country Code (optional), if not specified `us` will be assumed
        --lat <lat>                      Latitude (must also specify Longitude)
        --lon <lon>                      Longitude (must also specify Latitude)
    -z, --zipcode <zipcode>              Zipcode (optional)
```

Output:

```bash
Current conditions Astoria US 40.76N -73.93E
Last Updated 2020-01-19 15:18:52 -05:00
        Temperature: 39.45 F (4.14 C)
        Relative Humidity: 48%
        Wind: 300 degrees at 14.99 mph
        Conditions: broken clouds
        Sunrise: 2020-01-19 07:15:59 -05:00
        Sunset: 2020-01-19 16:56:14 -05:00

Forecast:
        2020-01-19 High: 35.76 F / 2.09 C         Low: 26.06 F / -3.30 C
        2020-01-20 High: 29.21 F / -1.55 C        Low: 19.58 F / -6.90 C
        2020-01-21 High: 31.21 F / -0.44 C        Low: 20.79 F / -6.23 C
        2020-01-22 High: 34.68 F / 1.49 C         Low: 24.96 F / -3.91 C
        2020-01-23 High: 38.43 F / 3.57 C         Low: 30.25 F / -0.97 C
        2020-01-24 High: 42.04 F / 5.58 C         Low: 31.55 F / -0.25 C
```
