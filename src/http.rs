use crate::message::CustomProgramValues;

use super::message;
// Wi-fi
use embedded_svc::http::server::Method;
use embedded_svc::io::{Write, Read};
use smart_leds::RGB8;
use std::sync::mpsc;

pub fn httpd(info_sender: mpsc::Sender<message::Message>) -> anyhow::Result<esp_idf_svc::http::server::EspHttpServer> {
    let mut server = esp_idf_svc::http::server::EspHttpServer::new(&Default::default())?;
    let info_sender_rotate = info_sender.clone();
    let info_sender_off = info_sender.clone();
    let info_sender_program = info_sender.clone();
    let info_sender_custom = info_sender.clone();

    server
        .fn_handler("/", Method::Get, |req| {
            req.into_ok_response()?
                .write_all("Lysa er oppe og nikker :)".as_bytes())?;
            Ok(())
        })?
        .fn_handler("/rotate", Method::Get, move |req| {
            match info_sender_rotate.send(message::Message::Rotate) {
                Ok(_ok) => { 
                    req.into_ok_response()?
                        .write_all("Knallbra, neste lysfarge kommer".as_bytes())?; 
                },
                Err(err) => {
                    req.into_response(500, Some("Internal error"), &[])?
                        .write_all(format!("Error: {}", {err}).as_bytes())?;
                }
            }
            Ok(())
        })?
        .fn_handler("/off", Method::Get, move |req| {
            match info_sender_off.send(message::Message::SetProgram(-2)) {
                Ok(_ok) => { 
                    req.into_ok_response()?
                        .write_all("God natt :)".as_bytes())?; 
                },
                Err(err) => {
                    req.into_response(500, Some("Internal error"), &[])?
                        .write_all(format!("Error: {}", {err}).as_bytes())?;
                }
            }
            Ok(())
        })?
        .fn_handler("/program", Method::Post, move |mut req| {
            let mut body: [u8; 100] = [0; 100];
            let len_read = req.read(&mut body)?;
            let body_str = String::from_utf8_lossy(& body[0..len_read]);
            println!("Body: {}", body_str);
            let program: i32 = match body_str.parse() {
                Ok(num) => num,
                Err(err) => { // If number cant be parsed respond with error
                    req.into_response(400, Some("Bad input"), &[])?
                        .write_all(format!("Invalid input: {}", {err}).as_bytes())?;
                return Ok(());
            }
            };
            match info_sender_program.send(message::Message::SetProgram(program)) {
                Ok(_ok) => { 
                    req.into_ok_response()?
                        .write_all(&body)?; 
                },
                Err(err) => {
                    req.into_response(500, Some("Internal error"), &[])?
                        .write_all(format!("Error: {}", {err}).as_bytes())?;
                }
            }
            Ok(())
        })?
        .fn_handler("/custom", Method::Post, move |mut req| {
            let mut body: [u8; 14] = [0; 14]; // Let buffer be too long to detect bad inputs
            let len_read = req.read(&mut body)?;
            if len_read != 13 {
                println!("Wrong length input");
                req.into_response(400, Some("Bad input"), &[])?
                .write_all(format!("Error: Bad input").as_bytes())?;
                return Ok(());
            }
            let program = CustomProgramValues {
                time_1_light_1: RGB8{r: body[0], g: body[1], b: body[2]},
                time_1_light_2: RGB8{r: body[3], g: body[4], b: body[5]},
                time_2_light_1: RGB8{r: body[6], g: body[7], b: body[8]},
                time_2_light_2: RGB8{r: body[9], g: body[10], b: body[11]},
                num_tenth_seconds_blink: body[12],
            };
            
            match info_sender_custom.send(message::Message::CustomProgram(program)) {
                Ok(_ok) => { 
                    req.into_ok_response()?
                        .write_all(&body)?; 
                },
                Err(err) => {
                    req.into_response(500, Some("Internal error"), &[])?
                        .write_all(format!("Error: {}", {err}).as_bytes())?;
                }
            }
            Ok(())
        })?;


    Ok(server)
}
