use crate::player::Player;
use macroquad::prelude::*;
pub mod player;

#[macroquad::main("Mining Game")]
async fn main() {
    let mut player = Player {
        x_pos: screen_width() / 2.0,
        y_pos: screen_height() / 2.0,
    };

    // main game loop
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        player.handle_input();
        player.draw();

        next_frame().await
    }
}
