//Filip Pinjuh E254-2022
use std::fs;

#[derive(Clone, Copy, Debug)]
pub enum Dir {
	Up,
  	Down,
  	Left,
  	Right
}

#[derive(Clone, Copy, Debug)]
struct Node {
    west: u32,
	east: u32,
	north: u32,
	south: u32,

	key: u32,
	exit: u32,
}

impl Default for Node {
	fn default() -> Self {
		Node {
			// 0 = free, 1 = wall, 2 = door
			west: 0,
			east: 0,
			north: 0,
			south: 0,
			// 0 = no, 1 = yes
			key: 0,
			exit: 0,
		}
	}
}

fn parse(){

  	// Path to maze
	let file_path="./src/maze.txt";

	// Set maze size
	let rows = 9;
    let columns = 54;

 	let maze_raw = fs::read_to_string(file_path)
		.expect("Error while reading file!");

    let mut maze_matrix: Vec<Node> = vec![Node { ..Default::default()}; columns];

	for (row_index, row) in maze_raw.lines().enumerate(){
		let mut west: u32 = 0;
		let mut east: u32 = 0;
		let mut north: u32 = 0;
		let mut south: u32 = 0;

		let mut key: u32 = 0;
		let mut end: u32 = 0;

		// Check for walls
		if row.chars().nth(0) == Some('0'){
			west = 1;
		}

		if row.chars().nth(1) == Some('0'){
			east = 1;
		}; 

		if row.chars().nth(2) == Some('0'){
			north = 1;
		}; 

		if row.chars().nth(3) == Some('0'){
			south = 1;
		}; 
		
		// Check for doors 
		if row.chars().nth(5) == Some('1'){
			west = 2;
		}

		if row.chars().nth(6) == Some('1'){
			east = 2;
		}; 

		if row.chars().nth(7) == Some('1'){
			north = 2;
		}; 

		if row.chars().nth(8) == Some('1'){
			south = 2;
		}; 

		// Check for keys
		if row.chars().nth(10) == Some('1') && row.chars().nth(11) == Some('1'){
			key = 1;
		}; 

		//Check for end
		if row.chars().nth(12) == Some('1') && row.chars().nth(13) == Some('1'){
			end = 1;
		};
		let node = Node{ west: west, east:east, north:north, south:south, key:key, exit:end};
		maze_matrix[row_index] = node;
	}

	// Maze sanity print
	for row in &maze_matrix {
		print!("{:#?} ", row);
		println!();
	}
}

fn main() {
	parse();
}
