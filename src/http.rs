use crate::message::CustomProgramValues;

use super::message;
// Wi-fi
use embedded_svc::http::server::Method;
use embedded_svc::io::Write;
use esp_idf_svc::http::server::EspHttpServer;
use minijinja::{context, Environment};
use smart_leds::RGB8;
use std::sync::mpsc::Sender;
use std::sync::Arc;

pub fn init_http_server(info_sender: Sender<message::Message>) -> anyhow::Result<EspHttpServer> {
    let mut server = EspHttpServer::new(&Default::default())?;

    let env = {
        let mut env = Environment::new();

        env.add_template("layout.html", include_str!("templates/layout.html"))?;
        Arc::new(env)
    };

    {
        let env = env.clone();
        server.fn_handler("/", Method::Get, move |req| {
            let tpl = env.get_template("layout.html")?;
            let op = tpl.render(context!())?;
            let mut res = req.into_ok_response()?;
            res.write_all(op.as_bytes())?;
            return Ok(());
        })?;
    }
    {
        let info_sender = info_sender.clone();
        server.fn_handler("/rotate", Method::Get, move |req| {
            match info_sender.send(message::Message::Rotate) {
                Ok(_ok) => {
                    req.into_ok_response()?
                        .write_all("Knallbra, neste lysfarge kommer".as_bytes())?;
                }
                Err(err) => {
                    req.into_response(500, Some("Internal error"), &[])?
                        .write_all(format!("Error: {}", { err }).as_bytes())?;
                }
            }
            Ok(())
        })?;
    }
    {
        let info_sender = info_sender.clone();
        server.fn_handler("/off", Method::Get, move |req| {
            match info_sender.send(message::Message::SetProgram(-2)) {
                Ok(_ok) => {
                    req.into_ok_response()?
                        .write_all("God natt :)".as_bytes())?;
                }
                Err(err) => {
                    req.into_response(500, Some("Internal error"), &[])?
                        .write_all(format!("Error: {}", { err }).as_bytes())?;
                }
            }
            Ok(())
        })?;
    }
    {
        let info_sender = info_sender.clone();
        server.fn_handler("/program", Method::Post, move |mut req| {
            let mut body: [u8; 100] = [0; 100];
            let len_read = req.read(&mut body)?;
            let body_str = String::from_utf8_lossy(&body[0..len_read]);
            println!("Body: {}", body_str);
            let program: i32 = match body_str.parse() {
                Ok(num) => num,
                Err(err) => {
                    // If number cant be parsed respond with error
                    req.into_response(400, Some("Bad input"), &[])?
                        .write_all(format!("Invalid input: {}", { err }).as_bytes())?;
                    return Ok(());
                }
            };
            match info_sender.send(message::Message::SetProgram(program)) {
                Ok(_ok) => {
                    req.into_ok_response()?.write_all(&body)?;
                }
                Err(err) => {
                    req.into_response(500, Some("Internal error"), &[])?
                        .write_all(format!("Error: {}", { err }).as_bytes())?;
                }
            }
            Ok(())
        })?;
    }
    {
        let info_sender = info_sender.clone();
        server.fn_handler("/custom", Method::Post, move |mut req| {
            let mut body: [u8; 14] = [0; 14]; // Let buffer be too long to detect bad inputs
            let len_read = req.read(&mut body)?;
            if len_read != 13 {
                println!("Wrong length input");
                req.into_response(400, Some("Bad input"), &[])?
                    .write_all(format!("Error: Bad input").as_bytes())?;
                return Ok(());
            }
            let program = CustomProgramValues {
                time_1_light_1: RGB8 {
                    r: body[0],
                    g: body[1],
                    b: body[2],
                },
                time_1_light_2: RGB8 {
                    r: body[3],
                    g: body[4],
                    b: body[5],
                },
                time_2_light_1: RGB8 {
                    r: body[6],
                    g: body[7],
                    b: body[8],
                },
                time_2_light_2: RGB8 {
                    r: body[9],
                    g: body[10],
                    b: body[11],
                },
                num_tenth_seconds_blink: body[12],
            };

            match info_sender.send(message::Message::CustomProgram(program)) {
                Ok(_ok) => {
                    req.into_ok_response()?.write_all(&body)?;
                }
                Err(err) => {
                    req.into_response(500, Some("Internal error"), &[])?
                        .write_all(format!("Error: {}", { err }).as_bytes())?;
                }
            }
            Ok(())
        })?;
    }

    Ok(server)
}
