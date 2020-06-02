extern crate nalgebra;

use crate::map::{Map, Tile};

use nalgebra::Vector2;
use std::cell::RefCell;
use std::rc::Rc;

const MAX_ITERATIONS: usize = 250;

fn dist_sq(a: &Vector2<i32>, b: &Vector2<i32>) -> i32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

#[derive(Clone, Copy, PartialEq)]
pub struct Node {
    h_cost: i32,
    g_cost: i32,
    parent: Option<Vector2<i32>>,
    open: bool,
}

impl Node {
    fn f_cost(&self) -> i32 {
        self.h_cost + self.g_cost
    }

    pub fn open(&self) -> bool {
        self.open
    }
}

pub struct PathFinder {
    start: Vector2<i32>,
    end: Vector2<i32>,
    map: Rc<RefCell<Map>>,
    nodes: Vec<Option<Node>>,
    open: Vec<Vector2<i32>>,
    closed: Vec<Vector2<i32>>,
    iteration_count: usize
}

impl PathFinder {
    pub fn new(start: Vector2<i32>, end: Vector2<i32>, map: Rc<RefCell<Map>>) -> Self {
        let node_count = (map.borrow().size().x * map.borrow().size().y) as usize;
        let mut path_finder = Self {
            start,
            end,
            map,
            nodes: vec![None; node_count],
            open: Vec::new(),
            closed: Vec::new(),
            iteration_count: 0
        };

        // Add first node
        path_finder.set_node(
            &start,
            Some(Node {
                g_cost: 0,
                h_cost: 0,
                parent: None,
                open: true,
            }),
        );
        path_finder.open.push(start);

        path_finder
    }

    pub fn start(&self) -> Vector2<i32> {
        self.start
    }

    pub fn end(&self) -> Vector2<i32> {
        self.end
    }

    // Process one iteration of the simulation
    pub fn iterate(&mut self) -> Option<bool> {
        // Find open node with lowest f_cost
        let (curr_index, &curr_pos) = match self
            .open
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                self.get_node(a)
                    .unwrap()
                    .f_cost()
                    .cmp(&self.get_node(b).unwrap().f_cost())
            }) {
                Some(curr) => curr,
                None => return Some(false)
            };

        // Move that node from open to closed
        self.open.remove(curr_index);
        self.closed.push(curr_pos);
        self.set_node_open(&curr_pos, false);

        // If the path has been found, return positive
        if curr_pos == self.end {
            return Some(true);
        }

        // Check each neighbor
        for (neighbor_pos, neighbor_node) in self.neighbors(&curr_pos) {
            // Skip neighbors already in closed
            if let Some(_) = self
                .closed
                .iter()
                .enumerate()
                .find(|(_, &pos)| pos == neighbor_pos)
            {
                continue;
            }

            // Update the neighbor node if the path is shorter or it hasn't been
            // set yet

            let old_neighbor_node_opt = self.get_node(&neighbor_pos);

            if old_neighbor_node_opt.is_none()
                || neighbor_node.f_cost() > old_neighbor_node_opt.unwrap().f_cost()
            {
                self.set_node(&neighbor_pos, Some(neighbor_node));

                self.open.push(neighbor_pos);
            }
        }

        if self.iteration_count > MAX_ITERATIONS {
            return Some(false);
        }

        self.iteration_count += 1;

        None
    }

    pub fn set_node(&mut self, pos: &Vector2<i32>, node: Option<Node>) {
        let index = self.pos_index(pos);
        self.nodes[index] = node;
    }

    pub fn set_node_open(&mut self, pos: &Vector2<i32>, open: bool) {
        let index = self.pos_index(pos);
        let mut node = self.nodes[index].expect("There is no node here!");
        node.open = open;
        self.nodes[index] = Some(node);
    }

    pub fn get_node(&self, pos: &Vector2<i32>) -> Option<&Node> {
        if pos.x > self.map.borrow().size().x || pos.y > self.map.borrow().size().y {
            return None;
        }
        self.nodes[self.pos_index(pos)].as_ref()
    }

    fn pos_index(&self, pos: &Vector2<i32>) -> usize {
        (pos.x + pos.y * self.map.borrow().size().x) as usize
    }

    fn neighbors(&self, pos: &Vector2<i32>) -> Vec<(Vector2<i32>, Node)> {
        let mut neighbors = Vec::new();
        let &node = self.get_node(pos).expect("That position isn't a node!");
        for x in std::cmp::max(pos.x - 1, 0)..std::cmp::min(pos.x + 2, self.map.borrow().size().x - 1) {
            for y in std::cmp::max(pos.y - 1, 0)..std::cmp::min(pos.y + 2, self.map.borrow().size().y - 1) {
                let neighbor_pos = Vector2::new(x, y);

                // Skip self and solid blocks
                if neighbor_pos == *pos || self.map.borrow().get_tile(&neighbor_pos) == Tile::Solid
                {
                    continue;
                }

                neighbors.push((
                    neighbor_pos,
                    Node {
                        g_cost: node.g_cost + dist_sq(&pos, &neighbor_pos),
                        h_cost: dist_sq(&self.end, &neighbor_pos),
                        parent: Some(*pos),
                        open: true,
                    },
                ));
            }
        }
        
        neighbors
    }
}
