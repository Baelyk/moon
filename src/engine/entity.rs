extern crate piston_window;

use engine::Engine;
use engine::Position;
use engine::Size;
use engine::World;
use piston_window::Image;
use vector::Vec2u;
use Direction;
use TILE_SIZE;
use WINDOW_HEIGHT;
use WINDOW_WIDTH;

/// Type of entity
pub enum EntityType {
    Enemy,
    Player,
}

/// The trait for all entities (e.g. enemies, player)
pub trait Entity {
    fn move_in(&mut self, direction: Direction);
    fn get_position(&self) -> &Position;
    fn get_size(&self) -> &Size;
}

pub struct Player {
    /// Current location, in tile coords
    pub position: Position,
    /// Size in tiles
    pub size: Size,
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
}

impl Entity for Player {
    /// Move in a direction
    fn move_in(&mut self, direction: Direction) {
        match direction {
            Direction::Down => if self.position.y + 1 < self.engine.world_pixels.y {
                self.position.y += 1;
            },
            Direction::Left => if (self.position.x as isize - 1 as isize) >= 0 {
                self.position.x -= 1;
            },
            Direction::Up => if (self.position.y as isize - 1 as isize) >= 0 {
                self.position.y -= 1;
            },
            Direction::Right => if self.position.x + 1 < self.engine.world_pixels.x {
                self.position.x += 1;
            },
        };
        self.image = self.engine.updateEntityImage(self);
    }
    fn get_position(&self) -> &Position {
        &self.position
    }
    fn get_size(&self) -> &Size {
        &self.size
    }
}

pub struct Enemy {
    /// Current location, in tile coords
    pub position: Position,
    /// Size in tiles
    pub size: Size,
    /// Path to texture
    pub texture: String,
    /// Image object
    pub image: Image,
}

impl Enemy {
    /// Create a new Enemy instance
    ///
    /// Located at (0, 0) with a single tile size
    /// Sprite: baddie.png
    pub fn new() -> Enemy {
        Enemy {
            position: Vec2u { x: 0, y: 0 },
            size: Vec2u { x: 1, y: 1 },
            texture: String::from("baddie.png"),
            image: Image::new().rect([
                0 as f64,
                0 as f64,
                (1 * TILE_SIZE) as f64,
                (1 * TILE_SIZE) as f64,
            ]),
        }
    }
}

impl Entity for Enemy {
    /// Move in a direction
    fn move_in(&mut self, direction: Direction) {
        match direction {
            Direction::Down => if self.position.y + TILE_SIZE < self.engine.world_pixels.y {
                self.position.y += TILE_SIZE;
            },
            Direction::Left => if (self.position.x as isize - TILE_SIZE as isize) >= 0 {
                self.position.x -= TILE_SIZE;
            },
            Direction::Up => if (self.position.y as isize - TILE_SIZE as isize) >= 0 {
                self.position.y -= TILE_SIZE;
            },
            Direction::Right => if self.position.x + TILE_SIZE < self.engine.world_pixels.x {
                self.position.x += TILE_SIZE;
            },
        };
        self.image = self.engine.updateEntityImage(self);
    }
    fn get_position(&self) -> &Position {
        &self.position
    }
    fn get_size(&self) -> &Size {
        &self.size
    }
}
