use super::message;

use std::sync::mpsc;
use esp_idf_hal::delay::FreeRtos;
use smart_leds::RGB;
use smart_leds_trait::SmartLedsWrite;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
// use smart_leds::{hsv::{hsv2rgb, Hsv}, RGB};


pub fn run_lights(info_reciever: mpsc::Receiver<message::Message>) -> ! {
    let mut ws2812 = Ws2812Esp32Rmt::new(0, super::LED_PIN).unwrap();
    let mut state: i32 = 0;
    let mut sequence = message::CustomProgramValues{..Default::default()};

    loop {
        match info_reciever.try_recv() {
            Err(_error) => (),
            Ok(message) => match message {
                message::Message::Rotate => state = (state+1)%message::NUM_STATES,
                message::Message::SetProgram(number) => state = number,
                message::Message::CustomProgram(custom_sequence) => {
                    sequence = custom_sequence;
                    state = -1;
                }
            } 
        }

        match state {
            -1 =>custom_programme(&mut ws2812, sequence.clone()),
            0 => red_white(&mut ws2812),
            1 => blue(&mut ws2812),
            _ => red_white(&mut ws2812),
        }
    }
}

fn red_white(ws2812: &mut Ws2812Esp32Rmt) {
    let pixels = (0..super::NUM_LEDS).map(|n| { 
        let odd = n%2;
        return RGB { r: 255, g: 255*odd, b: 255*odd};
    }); 
    ws2812.write(pixels).unwrap();
    FreeRtos::delay_ms(500);

    let pixels = (0..super::NUM_LEDS).map(|n| { 
        let even = 1-n%2;
        return RGB { r: 255, g: 255*even, b: 255*even};
    }); 
    ws2812.write(pixels).unwrap();
    FreeRtos::delay_ms(500);
}

fn blue(ws2812: &mut Ws2812Esp32Rmt) {
    let pixels = (0..super::NUM_LEDS).map(|n| { 
        let odd = n%2;
        return RGB { r: 255*odd, g: 255*odd, b: 255};
    }); 
    ws2812.write(pixels).unwrap();
    FreeRtos::delay_ms(500);

    let pixels = (0..super::NUM_LEDS).map(|n| { 
        let even = 1-n%2;
        return RGB { r: 125 + 125*even, g: 255*even, b: 255};
    }); 
    ws2812.write(pixels).unwrap();
    FreeRtos::delay_ms(500);
}

fn custom_programme(ws2812: &mut Ws2812Esp32Rmt, sequence: message::CustomProgramValues) {
    let pixels = (0..super::NUM_LEDS).map(|n| { 
        let odd = n%2;
        if odd == 1 {
            return sequence.time_1_light_1;
        }
        else {
            return sequence.time_1_light_2;
        }
    }); 
    ws2812.write(pixels).unwrap();
    FreeRtos::delay_ms(500);

    let pixels = (0..super::NUM_LEDS).map(|n| { 
        let odd = n%2;
        if odd == 1 {
            return sequence.time_2_light_1;
        }
        else {
            return sequence.time_2_light_2;
        }
    }); 
    ws2812.write(pixels).unwrap();
    FreeRtos::delay_ms(500);
}