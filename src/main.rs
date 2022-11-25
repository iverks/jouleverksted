#[macro_use]
extern crate dotenv_codegen;

mod http;
mod wifi;

use std::sync::{Arc, Mutex, Condvar};

use esp_idf_hal::{delay::FreeRtos, prelude::Peripherals};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::{EspNvsPartition, NvsDefault}};
// use smart_leds::{hsv::{hsv2rgb, Hsv}, RGB};
use smart_leds::RGB;
use smart_leds_trait::SmartLedsWrite;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;



fn main() -> ! {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspNvsPartition::<NvsDefault>::take().unwrap();

    let peripherals = Peripherals::take().unwrap();
    #[allow(unused_mut)]
    let mut wifi = wifi::wifi(peripherals.modem, sysloop.clone(), nvs).unwrap();
    let ip_info = wifi.sta_netif().get_ip_info().unwrap();
    println!("connected on ip {}", ip_info.subnet.gateway);

    let mutex = Arc::new((Mutex::new(None), Condvar::new()));
    #[allow(unused)]
    let httpd = http::httpd(mutex.clone()).unwrap();

    const NUM_LEDS: u8 = 23;
    const LED_PIN: u32 = 15;

    let mut ws2812 = Ws2812Esp32Rmt::new(0, LED_PIN).unwrap();

    loop {
        let pixels = (0..NUM_LEDS).map(|n| { 
            let odd = n%2;
            return RGB { r: 255, g: 255*odd, b: 255*odd};
        }); 
        ws2812.write(pixels).unwrap();
        FreeRtos::delay_ms(500);

        let pixels = (0..NUM_LEDS).map(|n| { 
            let odd = 1-n%2;
            return RGB { r: 255, g: 255*odd, b: 255*odd};
        }); 
        ws2812.write(pixels).unwrap();
        FreeRtos::delay_ms(500);
    }
}