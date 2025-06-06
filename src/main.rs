#[cfg(feature = "cli")]
use weather_util_rust::{Error, config::Config, weather_opts::WeatherOpts};

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::init_config(None)?;

    match tokio::spawn(async move { WeatherOpts::parse_opts(&config).await })
        .await
        .unwrap()
    {
        Ok(()) => Ok(()),
        Err(Error::InvalidInputError(e)) => {
            let help_message = WeatherOpts::api_help_msg();
            println!("{e}\n{help_message}");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[cfg(not(feature = "cli"))]
fn main() -> Result<(), Error> {
    Ok(())
}
