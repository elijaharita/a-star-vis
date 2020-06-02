extern crate nalgebra;
extern crate rand;

mod map;
mod map_renderer;
mod path_finder;

use map::Map;
use nalgebra::Vector2;
use path_finder::PathFinder;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let map = Rc::new(RefCell::new(Map::generate(Vector2::new(32, 32))));
    let mut path_finder = PathFinder::new(
        map.borrow().rand_empty_location(),
        map.borrow().rand_empty_location(),
        map.clone()
    );

    // Loop the iteration until a Some value is returned
    loop {
        if let Some((res, iterations)) = path_finder.iterate() {
            println!("{}",  map_renderer::render_to_string(&map.borrow(), vec![&path_finder]));
            if res {
                println!("Path found in {} iterations!", iterations);
            } else {
                println!("Could not find a path after {} iterations", iterations);
            }
            break;
        }
    }
}
