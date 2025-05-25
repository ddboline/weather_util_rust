use criterion::{Criterion, criterion_group, criterion_main};

use weather_util_rust::{weather_data::WeatherData, weather_forecast::WeatherForecast};

pub fn criterion_benchmark(c: &mut Criterion) {
    let buf = include_str!("../tests/weather.json");
    let data: WeatherData = serde_json::from_str(buf).unwrap();

    c.bench_function("weather_data", |b| b.iter(|| data.get_current_conditions()));

    let buf = include_str!("../tests/forecast.json");
    let data: WeatherForecast = serde_json::from_str(buf).unwrap();

    c.bench_function("weather_forecast", |b| b.iter(|| data.get_forecast()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
