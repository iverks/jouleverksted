use embedded_svc::{wifi::{Configuration, ClientConfiguration, AccessPointConfiguration, Wifi}, ipv4};
use esp_idf_hal::peripheral;
use esp_idf_sys::EspError;
use std::{net::Ipv4Addr, time::Duration};
use esp_idf_svc::{wifi::{EspWifi, WifiWait}, eventloop::EspSystemEventLoop, netif::{EspNetifWait, EspNetif}, ping, nvs::{EspNvsPartition, NvsDefault}};

const SSID: &str = dotenv!("RUST_ESP32_STD_DEMO_WIFI_SSID");
const PASS: &str = dotenv!("RUST_ESP32_STD_DEMO_WIFI_PASS");

pub fn wifi(
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
    nvs: EspNvsPartition<NvsDefault>,
) -> Result<Box<EspWifi<'static>>, EspError> {
    let mut wifi = Box::new(EspWifi::new(modem, sysloop.clone(), Some(nvs)).unwrap());

    println!("Wifi created, about to scan");

    let ap_infos = wifi.scan().unwrap();

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        println!(
            "Found configured access point {} on channel {}",
            SSID, ours.channel
        );
        Some(ours.channel)
    } else {
        println!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
        None
    };

    wifi.set_configuration(&Configuration::Mixed(
        ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            channel,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    )).unwrap();

    wifi.start().unwrap();

    println!("Starting wifi...");

    if !WifiWait::new(&sysloop).unwrap()
        .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
    {
        panic!("Wifi did not start");
    }

    println!("Connecting wifi...");

    wifi.connect().unwrap();

    if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?.wait_with_timeout(
        Duration::from_secs(20),
        || {
            wifi.is_connected().unwrap()
                && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
        },
    ) {
        panic!("Wifi did not connect or did not receive a DHCP lease");
    }

    let ip_info = wifi.sta_netif().get_ip_info().unwrap();

    println!("Wifi DHCP info: {:?}", ip_info);

    ping(ip_info.subnet.gateway).unwrap();

    Ok(wifi)
}

fn ping(ip: ipv4::Ipv4Addr) -> Result<(), ()> {
    println!("About to do some pings for {:?}", ip);

    let ping_summary = ping::EspPing::default().ping(ip, &Default::default()).unwrap();
    if ping_summary.transmitted != ping_summary.received {
        panic!("Pinging IP {} resulted in timeouts", ip);
    }

    println!("Pinging done");

    Ok(())
}