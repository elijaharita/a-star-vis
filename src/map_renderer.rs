extern crate nalgebra;

use crate::map::{Map, Tile};
use crate::path_finder::PathFinder;

use nalgebra::Vector2;

impl Tile {
    fn as_str(self) -> &'static str {
        match self {
            Tile::Empty => "  ",
            Tile::Solid => "██",
        }
    }
}

pub fn render_to_string(map: &Map, path_finders: Vec<&PathFinder>) -> String {
    let mut string = String::new();

    for y in 0..map.size().y {
        for x in 0..map.size().x {
            let mut tile_str = map.get_tile(&Vector2::new(x, y)).as_str();

            let pos = Vector2::new(x, y);

            for path_finder in &path_finders {
                if pos == path_finder.start() {
                    tile_str = "@ ";
                    continue;
                } else if pos == path_finder.end() {
                    tile_str = "X ";
                } else if let Some(node) = path_finder.get_node(&pos) {
                    tile_str = if node.open() {
                        ". "
                    } else {
                        "::"
                    };
                }
            }

            string += tile_str;
        }
        string += "\n";
    }

    string
}
