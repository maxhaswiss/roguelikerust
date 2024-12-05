use doryen_rs::{App, AppOptions, Console, Input};

const SCREEN_WIDTH: u32 = 80;
const SCREEN_HEIGHT: u32 = 50;
const FONT_PATH: &str = "./arial10x10.png";

struct Gameboard {
    fps: u32,
}

impl doryen_rs::App for Gameboard {
    fn initialize(&mut self, _console: &mut dyn Console) {}
    fn update(&mut self, _console: &mut dyn Console, _input: &Input) {}
    
}