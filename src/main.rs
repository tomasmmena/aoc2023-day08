use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};

use std::cell::RefCell;
use std::rc::Rc;

enum Instruction {
    Right,
    Left
}

struct MapNode {
    name: String,
    left: Option<Rc<RefCell<MapNode>>>,
    right: Option<Rc<RefCell<MapNode>>>
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_of_vector(numbers: Vec<usize>) -> usize {
    numbers.iter().cloned().fold(1, |acc, x| lcm(acc, x))
}

fn main() {
    let path = env::args().nth(1).expect("Missing required parameter path!");

    let mut data = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines();

    let instructions: Vec<Instruction> = data
        .next()
        .expect("EOL when reading for instructions.")
        .expect("Could not read line!")
        .chars()
        .map(|c| {
            match c {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Invalid instruction!")
            }
        })
        .collect();

    let mut map_index: HashMap<String, Rc<RefCell<MapNode>>> = HashMap::new();
    for line in data {
        let text = line.expect("Could not read line!");
        if text == "" { continue; }

        let name = String::from(&text[0..3]);
        let left = String::from(&text[7..10]);
        let right = String::from(&text[12..15]);

        if !map_index.contains_key(&left) {  // add missing left node
            map_index.insert(
                left, 
                Rc::new(RefCell::new(
                    MapNode {
                        name: String::from(&text[7..10]),
                        left: None,
                        right: None
                    }
                ))
            );
        }

        if !map_index.contains_key(&right) {  // add missing right node
            map_index.insert(
                right, 
                Rc::new(RefCell::new(
                    MapNode {
                        name: String::from(&text[12..15]),
                        left: None,
                        right: None
                    }
                ))
            );
        }

        if map_index.contains_key(&name) {  // set reference on previously created node
            let mut node_rc = map_index.get(&name).unwrap().try_borrow_mut().unwrap();
            node_rc.left = Some(Rc::clone(map_index.get(&text[7..10]).unwrap()));
            node_rc.right = Some(Rc::clone(map_index.get(&text[12..15]).unwrap()));
        } else {
            map_index.insert(
                name.clone(), 
                Rc::new(RefCell::new(MapNode {
                    name: name.clone(),
                    left: Some(Rc::clone(map_index.get(&text[7..10]).unwrap())),
                    right: Some(Rc::clone(map_index.get(&text[12..15]).unwrap()))
                }))
            );
        }
    }

    let mut current_nodes: Vec<Rc<RefCell<MapNode>>> = map_index
        .values()
        .filter(|n| n.borrow().name.ends_with("A"))
        .map(|n| n.clone())
        .collect();
    let mut loop_len: [(usize, usize); 100] = [(0, 0); 100];
    for i in 0..current_nodes.len() {
        let mut step_count: usize = 0;
        let mut loop_end: bool = false;
        
        while !current_nodes[i].borrow().name.ends_with("Z") {
            match instructions[step_count % instructions.len()] {
                Instruction::Left => {
                    let next_node_option: Option<Rc<RefCell<MapNode>>> = {
                        if let Some(next_node_ref) = &current_nodes[i].try_borrow_mut().unwrap().left {
                            Some(next_node_ref.clone())
                        } else {
                            None
                        }
                    };
                    if let Some(next_node) = next_node_option {
                        current_nodes[i] = next_node;
                    } else {
                        println!("Dead end!");
                        break;
                    }
                },
                Instruction::Right => {
                    let next_node_option: Option<Rc<RefCell<MapNode>>> = {
                        if let Some(next_node_ref) = &current_nodes[i].try_borrow_mut().unwrap().right {
                            Some(next_node_ref.clone())
                        } else {
                            None
                        }
                    };
                    if let Some(next_node) = next_node_option {
                        current_nodes[i] = next_node;
                    } else {
                        println!("Dead end!");
                        break;
                    }
                },
            }
            step_count += 1;
        }

        loop_len[i].0 = step_count;

        while !current_nodes[i].borrow().name.ends_with("Z") && !loop_end {
            match instructions[step_count % instructions.len()] {
                Instruction::Left => {
                    let next_node_option: Option<Rc<RefCell<MapNode>>> = {
                        if let Some(next_node_ref) = &current_nodes[i].try_borrow_mut().unwrap().left {
                            Some(next_node_ref.clone())
                        } else {
                            None
                        }
                    };
                    if let Some(next_node) = next_node_option {
                        current_nodes[i] = next_node;
                    } else {
                        println!("Dead end!");
                        break;
                    }
                },
                Instruction::Right => {
                    let next_node_option: Option<Rc<RefCell<MapNode>>> = {
                        if let Some(next_node_ref) = &current_nodes[i].try_borrow_mut().unwrap().right {
                            Some(next_node_ref.clone())
                        } else {
                            None
                        }
                    };
                    if let Some(next_node) = next_node_option {
                        current_nodes[i] = next_node;
                    } else {
                        println!("Dead end!");
                        break;
                    }
                },
            }
            step_count += 1;
            loop_end = true;
        }
        loop_len[i].1 = step_count;

    }

    println!(
        "Solution is: {}",
        lcm_of_vector(  
            // this is not a very general solution, but doing this in a more general way is too expensive
            // it would seem this is the way it is intended to be solved
            loop_len.into_iter().map(|n| n.0).filter(|n| *n != 0).collect::<Vec<usize>>()
        )
    );

}
