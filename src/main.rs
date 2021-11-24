use std::io;
use std::io::*;

fn draw_board(grid: &mut [i32]) {
	println!("-------");
	let mut i = 1;
	for n in grid.iter() {
		let square = if *n == 0 {String::from(" ")} else if *n == 1 {String::from("O")} else {String::from("X")};
		print!("|{}", square);
		if i % 3 == 0 {
			println!("|\n-------");
		}
		i += 1;
	}
}

fn did_win(grid: &mut [i32]) -> bool {

	for i in 0..3 {
		/* all vertical possibilities */
		if grid[i] == grid[i+3] && grid[i+3] == grid[i+6] && grid[i] != 0 {
			return true;
		}
	}

	for i in 0..3 {
		/* all horizontal possiblilities*/
		if grid[i*3] == grid[i*3+1] && grid[i*3+1] == grid[i*3+2] && grid[i*3] != 0{
			return true;
		}
	}

	/* diagonals */
	// left to right
	if grid[0] == grid[4] && grid[4] == grid[8] && grid[0] != 0 {
		return true;
	}
	// right to left
	if grid[2] == grid[4] && grid[4] == grid[6] && grid[2] != 0 {
		return true;
	}

	return false;
}

fn did_draw(grid: &mut [i32]) -> bool {
	for n in grid.iter() {
		if *n == 0 {
			return false;
		}
	}
	return true;
}

fn get_turn(turn: bool) -> String {
	return if turn {String::from("O")} else {String::from("X")};
}

fn main() {
    let mut turn = false;
	let mut grid: [i32; 9] = [
		0, 0, 0,
		0, 0, 0,
		0, 0, 0
	];

	// game loop
	loop {
		draw_board(&mut grid);
		if did_win(&mut grid) {
			println!("{} won the game!", get_turn(turn));
			break;
		}
		if did_draw(&mut grid) {
			println!("Draw!");
			break;
		}

		turn = !turn;
		println!("its {}'s turn!", get_turn(turn));
		let mut input = String::new();
		print!("enter a number 1-9: ");
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut input).expect("error: could not read user input");
		let input = input.trim();
		if input == "q" {
			break;
		}
		
		let mut number: usize = input.parse().expect("invalid number!");
		if number >= 1 && number <= 9 {
			number -= 1;
			if grid[number] == 0 {
				grid[number] = if turn { 1 } else { 2 };
			}
			else {
				turn = !turn; // invert turn 
				continue;
			}
		}
		else {
			print!("invalid number!");
			continue;
		}
	}
}
