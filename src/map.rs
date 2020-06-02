extern crate nalgebra;
extern crate rand;
extern crate noise;

use nalgebra::Vector2;
use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Solid,
}

pub struct Map {
    size: Vector2<i32>,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn generate(size: Vector2<i32>) -> Self {
        let mut map = Self {
            size,
            tiles: vec![Tile::Empty; (size.x * size.y) as usize],
        };


        // Obstacles

        let noise = OpenSimplex::new().set_seed(rand::thread_rng().next_u32());

        for x in 0..size.x {
            for y in 0..size.y {
                let pos = Vector2::new(x, y);

                // Set the tile if it's not already set and it's not the origin
                // or target
                const SCALE: f64 = 5.0;
                if noise.get([x as f64 / SCALE, y as f64 / SCALE]) > 0.1 {
                    map.set_tile(&pos, Tile::Solid);
                }
            }
        }

        map
    }

    pub fn size(&self) -> Vector2<i32> {
        self.size
    }

    pub fn get_tile(&self, pos: &Vector2<i32>) -> Tile {
        self.tiles[self.pos_index(pos)]
    }

    pub fn set_tile(&mut self, pos: &Vector2<i32>, tile: Tile) {
        let index = self.pos_index(pos);
        self.tiles[index] = tile;
    }

    fn pos_index(&self, pos: &Vector2<i32>) -> usize {
        (pos.x + pos.y * self.size.x) as usize
    }
}
