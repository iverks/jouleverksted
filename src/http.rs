// Wi-fi
use std::sync::{Arc, Mutex, Condvar};
use embedded_svc::http::server::Method;
use embedded_svc::io::Write;

pub fn httpd(
    mutex: Arc<(Mutex<Option<u32>>, Condvar)>,
) -> Result<esp_idf_svc::http::server::EspHttpServer, ()> {
    let mut server = esp_idf_svc::http::server::EspHttpServer::new(&Default::default()).unwrap();

    server
        .fn_handler("/", Method::Get, |req| {
            req.into_ok_response()?
                .write_all("Hello from Rust!".as_bytes()).unwrap();

            Ok(())
        }).unwrap()
        .fn_handler("/foo", Method::Get, |_req| {
            Result::Err("Boo, something happened!".into())
        }).unwrap()
        .fn_handler("/bar", Method::Get, |req| {
            req.into_response(403, Some("No permissions"), &[])?
                .write_all("You have no permissions to access this page".as_bytes()).unwrap();

            Ok(())
        }).unwrap()
        .fn_handler("/panic", Method::Get, |_req| {
            panic!("User requested a panic!")
        }).unwrap();

    Ok(server)
}