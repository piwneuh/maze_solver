//Filip Pinjuh E254-2022
use std::fs;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub enum Dir {
	// Legacy - to be deleted
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

fn parse(file_path: &str, rows: usize, columns: usize) -> Vec<Vec<Node>>{

 	let maze_raw = fs::read_to_string(file_path)
		.expect("Error while reading file!");

    let mut maze_matrix: Vec<Vec<Node>> = vec![vec![Node { ..Default::default()};rows]; columns];

	for (index, row) in maze_raw.lines().enumerate(){
		let mut west: u32 = 0;
		let mut east: u32 = 0;
		let mut north: u32 = 0;
		let mut south: u32 = 0;

		let mut key: u32 = 0;
		let mut exit: u32 = 0;

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
		
		//Check for exit
		if row.chars().nth(12) == Some('1') && row.chars().nth(13) == Some('1'){
			exit = 1;
		};

		let node = Node{ west: west, east:east, north:north, south:south, key:key, exit:exit};
		maze_matrix[index / 9][index % 9] = node;
	}

	maze_matrix
}

fn solve(maze_raw: Vec<Vec<Node>>, start_x: usize, start_y: usize) -> Option<Vec<(usize, usize)>> {
	let maze = maze_raw; 
	let path = Vec::new();
	let visited = vec![vec![false; maze[0].len()]; maze.len()];
	let mut que = VecDeque::new();

	let key_inventory: i32 = 0;

	que.push_back((start_x, start_y, maze, path, visited, key_inventory));
	while let Some((x, y, maze, curr_path, visited, mut key_inventory)) = que.pop_back() {
		let mut node = maze[x][y];

		if node.exit == 1 {
			// Sanity yet again
			println!("Maze solved !");
			return Some(curr_path);
		}

		// key_inventory should be larger for more complex mazes
		if node.key == 1 && key_inventory < 2{
			// Pick up key
			key_inventory += 1;
			println!("Key picked up {} in inventory ! ({}, {})", key_inventory, x, y);

			// Remove key
			node.key = 0;

			let mut new_maze = maze.clone();
			new_maze[x][y] = node;
			
			let new_path = curr_path.clone();
			let mut new_visited = visited.clone();

			// Legacy Magic
			if x != 0 { new_visited[x-1][y] = false; }
			if x != 5 { new_visited[x+1][y] = false; }
			if y != 0 { new_visited[x][y-1] = false; }
			if y != 8 { new_visited[x][y+1] = false; }

			// True Magic
			// let mut _prev_x: usize = 0;
			// let mut _prev_y: usize = 0;
			// (_prev_x, _prev_y) = curr_path[curr_path.len()-1];
			// new_visited[_prev_x][_prev_y] = false;

			que.push_back((x, y, new_maze, new_path, new_visited, key_inventory));
		}

		if node.north == 0 && !visited[x - 1][y] {
			let mut new_path = curr_path.clone();
			new_path.push((x, y));

			let mut new_visited = visited.clone();
			new_visited[x][y] = true;

			let new_maze = maze.clone();
			que.push_back((x - 1, y, new_maze, new_path, new_visited, key_inventory));
		}

		if node.south == 0 && !visited[x + 1][y] {
			let mut new_path = curr_path.clone();
			new_path.push((x, y));

			let mut new_visited = visited.clone();
			new_visited[x][y] = true;

			let new_maze = maze.clone();
			que.push_back((x + 1, y, new_maze, new_path, new_visited, key_inventory));
		}

		if node.west == 0 && !visited[x][y - 1] {
			let mut new_path = curr_path.clone();
			new_path.push((x, y));

			let mut new_visited = visited.clone();
			new_visited[x][y] = true;

			let new_maze = maze.clone();
			que.push_back((x, y - 1, new_maze, new_path, new_visited, key_inventory));
		}

		if node.east == 0 && !visited[x][y + 1] {
			let mut new_path = curr_path.clone();
			new_path.push((x, y));

			let mut new_visited = visited.clone();
			new_visited[x][y] = true;

			let new_maze = maze.clone();
			que.push_back((x, y + 1, new_maze, new_path, new_visited, key_inventory));
		}
	
		// Open doors
		if node.north == 2 && key_inventory > 0 && !visited[x - 1][y] {
			// Remove door
			key_inventory -= 1;
			node.north = 0;
			println!("Opened door, {} in inventory ! ({}, {})", key_inventory, x, y);
			
			let mut new_maze = maze.clone();
			new_maze[x][y] = node;

			let mut new_path = curr_path.clone();
			new_path.push((x, y));

			let mut new_visited = visited.clone();
			new_visited[x][y] = true;
			que.push_back((x - 1, y, new_maze, new_path, new_visited, key_inventory));
		}

		if node.south == 2 && key_inventory > 0 && !visited[x + 1][y]{
			key_inventory -= 1;
			node.south = 0;
			println!("Opened door, {} in inventory ! ({}, {})", key_inventory, x, y);
			
			let mut new_maze = maze.clone();
			new_maze[x][y] = node;

			let mut new_path = curr_path.clone();
			new_path.push((x, y));

			let mut new_visited = visited.clone();
			new_visited[x][y] = true;
			que.push_back((x + 1, y, new_maze, new_path, new_visited, key_inventory));
		}

		if node.west == 2 && key_inventory > 0 && !visited[x][y - 1]{
			key_inventory -= 1;
			node.west = 0;
			println!("Opened door, {} in inventory ! ({}, {})", key_inventory, x, y);
			
			let mut new_maze = maze.clone();
			new_maze[x][y] = node;

			let mut new_path = curr_path.clone();
			new_path.push((x, y));

			let mut new_visited = visited.clone();
			new_visited[x][y] = true;
			que.push_back((x, y - 1, new_maze, new_path, new_visited, key_inventory));
		}

		if node.east == 2 && key_inventory > 0 && !visited[x][y + 1]{
			key_inventory -= 1;
			node.east = 0;
			println!("Opened door, {} in inventory ! ({}, {})", key_inventory, x, y);
			
			let mut new_maze = maze.clone();
			new_maze[x][y] = node;

			let mut new_path = curr_path.clone();
			new_path.push((x, y));

			let mut new_visited = visited.clone();
			new_visited[x][y] = true;
			que.push_back((x, y + 1, new_maze, new_path, new_visited, key_inventory));
		}
	}

	println!("No viable options !");
	None
}

fn main() {
	// Path to maze
	let file_path="./src/maze.txt";

	// Set maze size
	let rows = 9;
	let columns = 6;

	// Set maze start
	let start_x = 0;
	let start_y = 0;
	
	// Parsing
	let maze: Vec<Vec<Node>> = parse(file_path, rows, columns);

	// Maze sanity print
	// for row in &maze {
    //     for node in row {
    //         print!("{:#?} ", node);
    //     }
    //     println!("_____________");
    // }
	
	// Solving
	let final_path = solve(maze, start_x, start_y);

	for field in final_path{
		print!("{:?}", field);
		println!();
	}

}
