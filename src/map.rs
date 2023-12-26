use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// The type of tiles on the map canvas
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

// map_idx calcuates the current tile index with x and y
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    // Map constructor
    pub fn new() -> Self {
        // Initilize all tiles with TileType of floor
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
}
