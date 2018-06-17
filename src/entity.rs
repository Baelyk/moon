extern crate piston_window;

use piston_window::Image;
use vector::Vec2u;
use TILE_SIZE;
use WINDOW_HEIGHT;
use WINDOW_WIDTH;

/// The trait for all entities (e.g. enemies, player)
// trait Entity {
//     fn set_destination(&mut self, target: Vec2d);
//     fn move(&mut self);
// }

/// Arrow key directions (Up, Left, Down, Right)
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

enum EntityType {
    Enemy,
    Player,
}

// struct Enemy {
//     /// Current location, in tile coords
//     position: Vec2d,
//     /// Size in tiles
//     size: Vec2d,
//     /// Path to texture
//     texture: String,
//     /// Location moving to, if any, in tile coords
//     destination: Option<Vec2d>,
// }

pub struct Player {
    /// Current location, in tile coords
    pub position: Vec2u,
    /// Size in tiles
    pub size: Vec2u,
    /// Path to texture
    pub texture: String,
    /// Image object
    pub image: Image,
}

impl Player {
    /// Create a new Player instance
    ///
    /// Located at (0, 0) with a single tile size
    /// Sprite: character.png
    pub fn new() -> Player {
        Player {
            position: Vec2u { x: 0, y: 0 },
            size: Vec2u { x: 1, y: 1 },
            texture: String::from("character.png"),
            image: Image::new().rect([
                0 as f64,
                0 as f64,
                (1 * TILE_SIZE) as f64,
                (1 * TILE_SIZE) as f64,
            ]),
        }
    }
    /// Move in a direction
    pub fn move_in(&mut self, direction: Direction) {
        match direction {
            Direction::Down => if self.position.y + TILE_SIZE < WINDOW_HEIGHT {
                self.position.y += TILE_SIZE;
            },
            Direction::Left => if (self.position.x as isize - TILE_SIZE as isize) >= 0 {
                self.position.x -= TILE_SIZE;
            },
            Direction::Up => if (self.position.y as isize - TILE_SIZE as isize) >= 0 {
                self.position.y -= TILE_SIZE;
            },
            Direction::Right => if self.position.x + TILE_SIZE < WINDOW_WIDTH {
                self.position.x += TILE_SIZE;
            },
        };
        self.image = Image::new().rect([
            self.position.x as f64,
            self.position.y as f64,
            (self.size.x * TILE_SIZE) as f64,
            (self.size.y * TILE_SIZE) as f64,
        ]);
    }
}

// impl Entity for Player {
//
// }

// impl Enemy {
//     fn new(entity_type: EntityType) -> Enemy {
//         Enemy {
//             position: [0.0, 0.0],
//             size: [1.0, 1.0],
//             texture: String::from("baddie.png"),
//             destination: None,
//         }
//     }
// }

// impl Entity for Enemy {
//     fn set_destination(&mut self, target: Vec2d) {
//         self.destination = Some(target);
//     }
// }
