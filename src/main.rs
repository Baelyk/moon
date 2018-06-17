extern crate piston_window;

/// Module for entities (e.g. items, player, enemies)
mod entity;
mod vector;

use entity::Direction;
use entity::Player;
use piston_window::draw_state::Blend;
use piston_window::*; // TODO: Let's try and limit this down to what we need
use std::path::Path;
use vector::Vec2u;

/// The width of the game window
const WINDOW_WIDTH: usize = 832;
/// The height of the game window
const WINDOW_HEIGHT: usize = 512;
/// The default background color, white
const BACK_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
/// The dimensions of tiles (squares)
const TILE_SIZE: usize = 32;

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
    let mut character = Player::new();
    let texture = Texture::from_path(
        &mut window.factory,
        Path::join(Path::new("assets/sprites/"), &character.texture),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);

            character.image.draw(&texture, &alpha_state, c.transform, g);
            // image(&texture, c.transform, g);
        });
        if let Some(Button::Keyboard(Key::Up)) = event.press_args() {
            character.move_in(Direction::Up);
        }
        if let Some(Button::Keyboard(Key::Left)) = event.press_args() {
            character.move_in(Direction::Left);
        }
        if let Some(Button::Keyboard(Key::Down)) = event.press_args() {
            character.move_in(Direction::Down);
        }
        if let Some(Button::Keyboard(Key::Right)) = event.press_args() {
            character.move_in(Direction::Right);
        }
    }
}
