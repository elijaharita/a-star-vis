extern crate rand;

use rand::prelude::*;

#[derive(Clone, Copy)]
enum TileType {
    Empty,
    Solid,
}

impl TileType {
    pub fn get_char(self) -> char {
        match self {
            TileType::Empty => ' ',
            TileType::Solid => '#',
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<TileType>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Self {
            width,
            height,
            tiles: vec![TileType::Empty; width * height]
        };

        for x in 0..width {
            map.set_tile(x, 0, TileType::Solid);
            map.set_tile(x, height - 1, TileType::Solid);
        }
        for y in 0..height {
            map.set_tile(0, y, TileType::Solid);
            map.set_tile(width - 1, y, TileType::Solid);
        }

        map
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileType {
        self.tiles[x + y * self.width]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[x + y * self.width] = tile
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{} ", self.get_tile(x, y).get_char())
            }
            println!();
        }
    }
}

fn main() {
    let mut map = Map::new(32, 32);

    for _ in 0..25 {
        map.set_tile(
            rand::thread_rng().gen_range(0, map.width),
            rand::thread_rng().gen_range(0, map.height),
            TileType::Solid,
        );
    }

    map.print();
}
