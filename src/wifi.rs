use embedded_svc::{
    ipv4,
    wifi::{AccessPointConfiguration, ClientConfiguration, Configuration},
};
use esp_idf_hal::peripheral;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    netif::EspNetif,
    nvs::{EspNvsPartition, NvsDefault},
    ping,
    wifi::{BlockingWifi, EspWifi},
};
use std::{net::Ipv4Addr, time::Duration};

const SSID: &str = dotenv!("RUST_ESP32_STD_DEMO_WIFI_SSID");
const PASS: &str = dotenv!("RUST_ESP32_STD_DEMO_WIFI_PASS");

pub fn wifi(
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
    nvs: EspNvsPartition<NvsDefault>,
) -> anyhow::Result<Box<EspWifi<'static>>> {
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs)).unwrap();

    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop).unwrap();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

    log::info!("Starting wifi...");

    wifi.start().unwrap();

    log::info!("Wifi created, about to scan");

    let ap_infos = wifi.scan().unwrap();

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        log::info!(
            "Found configured access point {} on channel {}",
            SSID,
            ours.channel
        );
        Some(ours.channel)
    } else {
        log::info!(
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
    ))
    .unwrap();

    wifi.connect().unwrap();

    wifi.wait_netif_up().unwrap();

    let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();

    log::info!("Wifi DHCP info: {:?}", ip_info);

    ping(ip_info.subnet.gateway).unwrap();

    Ok(Box::new(esp_wifi))
}

fn ping(ip: ipv4::Ipv4Addr) -> anyhow::Result<()> {
    log::info!("About to do some pings for {:?}", ip);

    let ping_summary = ping::EspPing::default()
        .ping(ip, &Default::default())
        .unwrap();
    if ping_summary.transmitted != ping_summary.received {
        anyhow::bail!("Pinging IP {} resulted in timeouts", ip);
    }

    log::info!("Pinging done");

    Ok(())
}
