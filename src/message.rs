use smart_leds::RGB8;

pub const NUM_STATES: i32 = 3;
pub enum Message {
    Rotate,
    SetProgram(i32),
    CustomProgram(CustomProgramValues)
}

#[derive(Default, Clone)]
pub struct CustomProgramValues {
    pub time_1_light_1: RGB8,
    pub time_1_light_2: RGB8,
    pub time_2_light_1: RGB8,
    pub time_2_light_2: RGB8,
    pub num_tenth_seconds_blink: u8,
}

