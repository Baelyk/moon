extern crate piston_window;

use piston_window::draw_state::Blend;
use piston_window::*; // TODO: Let's try and limit this down to what we need
use std::path::Path;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 500;
const BACK_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Moon", [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let alpha_state: DrawState = DrawState {
        scissor: None,
        stencil: None,
        blend: Some(Blend::Alpha),
    };
    let mut character_x: f64 = 0.0;
    let mut character_y: f64 = 0.0;
    let mut character = Image::new().rect([character_x, character_y, 11.0, 26.0]);
    let texture = Texture::from_path(
        &mut window.factory,
        Path::new("assets/sprites/character.png"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);

            character.draw(&texture, &alpha_state, c.transform, g);
            // image(&texture, c.transform, g);
        });
        if let Some(Button::Keyboard(Key::Up)) = event.press_args() {
            character_y -= 1.0;
            character = Image::new().rect([character_x, character_y, 11.0, 26.0]);
        }
        if let Some(Button::Keyboard(Key::Left)) = event.press_args() {
            character_x -= 1.0;
            character = Image::new().rect([character_x, character_y, 11.0, 26.0]);
        }
        if let Some(Button::Keyboard(Key::Down)) = event.press_args() {
            character_y += 1.0;
            character = Image::new().rect([character_x, character_y, 11.0, 26.0]);
        }
        if let Some(Button::Keyboard(Key::Right)) = event.press_args() {
            character_x += 1.0;
            character = Image::new().rect([character_x, character_y, 11.0, 26.0]);
        }
    }
}
