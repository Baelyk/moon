extern crate piston_window;

/// Module for entities (e.g. items, player, enemies)
pub mod entity;

use self::entity::Entity;
use piston_window::Image;
use vector::Vec2u;

type Position = Vec2u;
type Size = Vec2u;

/// Arrow key directions (Up, Left, Down, Right)
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

type TileContents = Vec<&'static Entity>;

#[derive(Clone)]
struct Tile {
    position: Vec2u,
    texture: String,
    contents: TileContents,
}

pub struct World {
    /// Size in tiles
    size: Vec2u,
    tiles: Vec<Tile>,
}

// I think that I am hoping that the world is completely unaware of pixels and simply thinks in
// tiles, while the Engine is aware of pixels and (mostly?) sticks to thinking in pixels and leaves
// tiles to World.
pub struct Engine {
    /// Size in pixels of the world area
    world_pixels: Vec2u,
    /// Size of a tile in pixels
    tile_pixels: Vec2u,
    pub world: World,
}

impl Tile {
    fn blank() -> Self {
        Tile {
            position: Vec2u::new(0, 0),
            texture: String::new(),
            contents: vec![],
        }
    }
}

impl Engine {
    pub fn new(world_pixels: &Vec2u, tile_pixels: &Vec2u) -> Self {
        Engine {
            world_pixels: world_pixels.clone(),
            tile_pixels: tile_pixels.clone(),
            world: World {
                size: Vec2u::new(
                    world_pixels.x / tile_pixels.x,
                    world_pixels.y / tile_pixels.y,
                ),
                tiles: vec![
                    Tile::blank();
                    (world_pixels.x / tile_pixels.x) * (world_pixels.y / tile_pixels.y)
                ],
            },
        }
    }
    pub fn updateEntityImage(&self, entity: &Entity) -> Image {
        Image::new().rect([
            entity.get_position().x as f64,
            entity.get_position().y as f64,
            (entity.get_size().x * self.tile_pixels.x) as f64,
            (entity.get_size().y * self.tile_pixels.y) as f64,
        ])
    }
}
