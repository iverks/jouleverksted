#[macro_use]
extern crate dotenv_codegen;

mod http;
mod wifi;
mod lights;
mod message;

use std::sync::mpsc;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::{EspNvsPartition, NvsDefault}};
use log::*;

const NUM_LEDS: u8 = 23;
const LED_PIN: u32 = 15;

fn main() -> ! {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Setup logger for cooperation with log crate.
    // esp_idf_svc::log::EspLogger::initialize_default();
    // info!("Logger is connected");

    // Message channel 
    let (info_sender, info_reciever) = mpsc::channel();

    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspNvsPartition::<NvsDefault>::take().unwrap();

    let peripherals = Peripherals::take().unwrap();
    #[allow(unused_mut)]
    let mut wifi = wifi::wifi(peripherals.modem, sysloop.clone(), nvs).unwrap();
    let ip_info = wifi.sta_netif().get_ip_info().unwrap();
    println!("connected on ip {}", ip_info.ip);

    #[allow(unused)]
    let httpd = http::httpd(info_sender).unwrap();

    info!("Test logging");
    lights::run_lights(info_reciever)
}