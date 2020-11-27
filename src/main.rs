use influxdb::Client;
use std::str;
use std::{thread, time};
use influxdb::InfluxDbWriteable;
mod models;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() {
    loop {
        let client = Client::new("http://localhost:8086", "toon");

        let toon_data = match download_device_list().await {
            Ok(json) => parse_devices(&json).await,
            Err(_) => return (),
        };

        tokio::spawn(async move {
            let toon_readings = ToonInflux {
                time: Utc::now(),
                electricity_flow_low_tariff: toon_data.dev35.current_electricity_flow.parse().unwrap(),
                electricity_flow_normal_tariff: toon_data.dev33.current_electricity_flow.parse().unwrap(),
                electricity_quantity_low_tariff: toon_data.dev35.current_electricity_quantity.parse().unwrap(),
                electricity_quantity_normal_tariff: toon_data.dev33.current_electricity_quantity.parse().unwrap(),
                gas_flow: toon_data.dev31.current_gas_flow.parse().unwrap(),
                gas_quantity: toon_data.dev31.current_gas_quantity.parse().unwrap(),
            };

            let write_result  = client.query(
                &toon_readings.into_query("toon_readings")
            ).await;
        }); 
        
        thread::sleep(time::Duration::from_secs(30));
    }
}

async fn download_device_list() -> Result<String, Box<dyn std::error::Error>> {
    let url = "http://10.0.0.8/hdrv_zwave?action=getDevices.json";

    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    Ok(body)
}

async fn parse_devices(content: &str) -> models::Toon {
    match serde_json::from_str(content) {
        Ok(x) => x,
        Err(err) => panic!("error while parsing to Device struct: {:?}", err),
    }
}

#[derive(InfluxDbWriteable)]
pub struct ToonInflux {
    pub time: DateTime<Utc>,
    pub electricity_quantity_low_tariff: f64,
    pub electricity_quantity_normal_tariff: f64,
    pub electricity_flow_low_tariff: f32,
    pub electricity_flow_normal_tariff: f32,
    pub gas_quantity: f64,
    pub gas_flow: f32,
}