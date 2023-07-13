#[macro_use]
extern crate dotenv_codegen;

mod http;
mod lights;
mod message;
mod wifi;

use esp_idf_hal::{delay::FreeRtos, prelude::Peripherals};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::{EspNvsPartition, NvsDefault},
};
use log::*;
use std::sync::mpsc;
use tokio::runtime::Runtime;

const NUM_LEDS: u8 = 23;
const LED_PIN: u32 = 15;

fn main() -> ! {
    let rt = Runtime::new().unwrap();

    rt.block_on(async { tmain().await })
}

async fn tmain() -> ! {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Message channel
    let (info_sender, info_reciever) = mpsc::channel();

    let light_handle = tokio::task::spawn(lights::run_lights(info_reciever));

    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspNvsPartition::<NvsDefault>::take().unwrap();

    let peripherals = Peripherals::take().unwrap();

    let wifi_res = wifi::wifi(peripherals.modem, sysloop.clone(), nvs);
    let _http_server = http::init_http_server(info_sender);

    if let Ok(wifi) = &wifi_res {
        let ip_info = wifi.sta_netif().get_ip_info().unwrap();
        info!("connected on ip {}", ip_info.ip);
    } else {
        warn!("Connecting to wifi failed");
    }

    light_handle.await.unwrap();
}
