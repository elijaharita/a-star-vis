extern crate nalgebra;
extern crate rand;

mod map;
mod map_renderer;
mod path_finder;

use map::Map;
use nalgebra::Vector2;
use path_finder::PathFinder;
use rand::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{Write};

fn main() {
    let map = Rc::new(RefCell::new(Map::generate(Vector2::new(32, 32))));
    let mut path_finder = PathFinder::new(
        Vector2::new(
            rand::thread_rng().gen_range(0, map.borrow().size().x),
            rand::thread_rng().gen_range(0, map.borrow().size().x),
        ),
        Vector2::new(
            rand::thread_rng().gen_range(0, map.borrow().size().x),
            rand::thread_rng().gen_range(0, map.borrow().size().x),
        ),
        map.clone()
    );

    loop {
        if let Some(res) = path_finder.iterate() {
            if res {
                println!("Path found!");
                break;
            } else {
                println!("Could not find a path");
                break;
            }
        } 
        std::io::stdout().flush().unwrap();
    }
    println!("{}",  map_renderer::render_to_string(&map.borrow(), vec![&path_finder]));
}
