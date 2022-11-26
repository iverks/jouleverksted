// Wi-fi
use embedded_svc::http::server::Method;
use embedded_svc::io::Write;
use std::sync::mpsc;

pub fn httpd(info_sender: mpsc::Sender<i32>) -> Result<esp_idf_svc::http::server::EspHttpServer, ()> {
    let mut server = esp_idf_svc::http::server::EspHttpServer::new(&Default::default()).unwrap();
    let info_sender_toggle = info_sender.clone();

    server
        .fn_handler("/", Method::Get, |req| {
            req.into_ok_response()?
                .write_all("Hello from Rust!".as_bytes()).unwrap();
            Ok(())
        }).unwrap()
        .fn_handler("/toggle", Method::Get, move |req| {
            // Tanker: Send informasjon vi får fra post request :)
            // Lag en struct for det
            match info_sender_toggle.send(1) {
                Ok(_ok) => { 
                    req.into_ok_response()?
                        .write_all("Success! Set toggle message".as_bytes()).unwrap(); 
                }
                Err(err) => {
                    req.into_response(500, Some("Internal error"), &[])?
                        .write_all(format!("Error: {}", {err}).as_bytes()).unwrap();
                }
            }
            Ok(())
        })
        .unwrap();

    Ok(server)
}
