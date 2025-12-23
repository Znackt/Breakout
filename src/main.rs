use macroquad::prelude::*;

#[macroquad::main("BreakOut")]

async fn main() {
    loop {
        clear_background(WHITE);
        next_frame().await;
    }
}
