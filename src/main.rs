#[macro_use] extern crate ego_tree;
use ego_tree::NodeMut;
use ego_tree::NodeRef;
use ego_tree::NodeId;
use ego_tree::Tree;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

pub trait TBoard {
    fn legal_plays(&self) -> Vec<Coords>;
	fn start(&self) -> Board;
	fn next_state(&self, play: &Coords) -> Board;
	fn winner(&self, c: &Coords) -> i8;
	fn current_player(&self) -> i8;
	fn display(&self);
	fn if_over(&self) -> bool;
	// fn winnergomo()
}

impl TBoard for Board {
    fn legal_plays(&self) -> Vec<Coords> {
        let mut moves: Vec<(i8,i8)> = Vec::new();
		for i in 0..3 {
            for j in 0..3 {
                if self[i][j] == '.' {
                    moves.push((i as i8, j as i8));
                }
            }
        }
        moves
    }

	fn if_over(&self) -> bool {
		for i in 0..3 {
			for j in 0..3 {
				if self[i][j] == '.' {
					return false;
                }
            }
        }
		true
	}
	

	fn next_state(&self, play: &Coords) -> Board {
		let mut new = self.clone();
		let player = new.current_player();
		if player == 1 {
			new[play.0 as usize][play.1 as usize] = 'x';
		} else {
			new[play.0 as usize][play.1 as usize] = 'o';
		}
		new
	}
	
	fn start(&self) -> Board {
		*self
	}

	fn winner(&self, coords: &Coords) -> i8 {
		// let board = self.board;
		// println!("this is the one ime lidk {}","1");
		// Horizontal and verical check
		// println!("------------------------");
		// self.display();
		// println!("------------------------");
		for i in 0..3 {
			if self[i][0] == self[i][1] && self[i][0] == self[i][2] && self[i][0] != '.' {
				return if self[i][0] == 'x' { 1 } else { 2 };
			}
			if self[0][i] == self[1][i] && self[0][i] == self[2][i] && self[0][i] != '.' {
				return if self[0][i] == 'x' { 1 } else { 2 };
			}
		}

		//Diagonal check
		if self[0][0] == self[1][1] && self[0][0] == self[2][2] && self[0][0] != '.' {
			return if self[0][0] == 'x' { 1 } else { 2 };
		}
		if self[0][2] == self[1][1] && self[0][2] == self[2][0] && self[0][2] != '.' {
			return if self[0][2] == 'x' { 1 } else { 2 };
		}

		// No one is winning yet
		0
	}
	

	// fn winnergomo(&self, c: &Coords) -> i8 {
	// 	// self.display();
	// 	let player: char = self[c.0 as usize][c.1 as usize];
	// 	// let mut c: Coords = coords.clone();
	// 	// let d: Coords = (c.0 as usize,c.1 as usize)
	// 	let mut count: i8 = 0;
	// 	//check right
	// 	for i in 1..5 {
	// 		if c.1 + i <= 18 {
	// 			if count == 5 {
	// 				return if player == 'x' { 1 } else { 2 };
	// 			}
	// 			if self[c.0 as usize][(c.1 + i) as usize] != player {
	// 				break;
	// 			}
	// 			count += 1;
	// 		}
	// 	}
	// 	// check left
	// 	for i in 1..5 {
	// 		if c.1 - i >= 0 {
	// 			if count == 5 {
	// 				return if player == 'x' { 1 } else { 2 };
	// 			}
	// 			if self[c.0 as usize][(c.1 - i) as usize] != player {
	// 				break;
	// 			}
	// 			count += 1;
	// 		}
	// 	}
	// 	count = 0;
	// 	//check down
	// 	for i in 1..5 {
	// 		if c.0 + i <= 18  {
	// 			if count == 5 {
	// 				return if player == 'x' { 1 } else { 2 };
	// 			}
	// 			if self[(c.0 + i) as usize][c.1 as usize] != player {
	// 				break;
	// 			}
	// 			count += 1;
	// 		}
	// 	}
	// 	// check up
	// 	for i in 1..5 {
	// 		if c.0 - i >= 0 {
	// 			if count == 5 {
	// 				return if player == 'x' { 1 } else { 2 };
	// 			}
	// 			if self[(c.0 - i) as usize][c.1 as usize] != player {
	// 				break;
	// 			}
	// 			count += 1;
	// 		}
	// 	}
	// 	count = 0;
	// 	//check down right
	// 	for i in 1..5 {
	// 		if c.1 + i <= 18 && c.0 + i <= 18 {
	// 			if count == 5 {
	// 				return if player == 'x' { 1 } else { 2 };
	// 			}
	// 			if self[(c.0 + i) as usize][(c.1 + i) as usize] != player {
	// 				break;
	// 			}
	// 			count += 1;
	// 		}
	// 	}
	// 	// up left
	// 	for i in 1..5 {
	// 		if c.1 - i >= 0 && c.0 - i >= 0 {
	// 			if count == 5 {
	// 				return if player == 'x' { 1 } else { 2 };
	// 			}
	// 			if self[(c.0 - i) as usize][(c.1 - i) as usize] != player {
	// 				break;
	// 			}
	// 			count += 1;
	// 		}
	// 	}
	// 	count = 0;
	// 	//check down left
	// 	for i in 1..5 {
	// 		if c.1 - i >= 0 && c.0 + i <= 18 {
	// 			if count == 5 {
	// 				return if player == 'x' { 1 } else { 2 };
	// 			}
	// 			if self[(c.0 + i) as usize][(c.1 - i) as usize] != player {
	// 				break;
	// 			}
	// 			count += 1;
	// 		}
	// 	}
	// 	// up right
	// 	for i in 1..5 {
	// 		if c.1 + i <= 18 && c.0 - i >= 0 {
	// 			if count == 5 {
	// 				return if player == 'x' { 1 } else { 2 };
	// 			}
	// 			if self[(c.0 - i) as usize][(c.1 + i) as usize] != player {
	// 				break;
	// 			}
	// 			count += 1;
	// 		}
	// 	}
	// 	0
	// }

	fn current_player(&self) -> i8 {
		let mut count: u32 = 0;
		for i in 0..3 {
            for j in 0..3 {
                if self[i][j] != '.' {
					count +=1;
                }
            }
        }
		if count % 2 == 0 {
			1
		} else {
			2
		}
	}

	fn display(&self) {
		for i in 0..3 {
			for j in 0..3 {
				print!("{} ", self[i][j]);
			}
			println!("");
		}
		println!("");
	}
}

pub type Board = [[char;3];3];
pub type Coords = (i8, i8);

pub struct McNode {
    state: Board,
    num_plays: u16,
    num_wins: u16,
    player: i8,
    score: u32,
}

impl McNode {
    fn new(state: Board, current_player: i8) -> McNode {
        McNode {
            state: state,
            num_plays : 0,
            num_wins : 0,
            player: current_player,
            score : 0
        }
    }

    
}

// Mock: not doing anything
// Update all nodes (num_visited, num_win)
fn back_propagation (mut node: NodeMut<McNode>, win: i8) {
	match node.parent() {
		None => (),
		Some(parent) => back_propagation(parent, win) 
	}
}

// Mock, always retuning the node
fn selection (node: NodeMut<McNode>) -> NodeMut<McNode> {
	node
}

// Explore first next play until the game is over
// or there is no plays left to select
fn simulation (state: &Board) -> i8 {

	// coords: Mock
	let winner = state.winner(&(0,0));
	if winner != 0 {
		return winner;
	}

	// Mock: always use first possible play
	let legal_plays = state.legal_plays();
	if legal_plays.is_empty() {
		return winner;
	}
	simulation(&state.next_state(&legal_plays[0]))
}

// Mock, always retuning the node
fn expand (node: NodeMut<McNode>) -> NodeMut<McNode> {
	node	
}

fn monte_carlo(mut tree: Tree<McNode>) -> Coords {
	let n_run = 50;

	for i in 0..=n_run {
		let selected_node = selection(tree.root_mut());
		let mut expansion_node = expand(selected_node);
		let winner = simulation(&expansion_node.value().state);
		let win = if winner == 2 { 0 } else { 1 }; // Pat is worth a victory for now
		println!("winner: {}", winner);
		back_propagation(expansion_node, win);
	}

	// Mock return
	(0, 0)
}

fn main() {
    let board: Board = [
		['.', '.', '.'],
		['.', 'x', '.'],
		['.', '.', '.']
	];

	let tree = tree!(McNode::new(board, 1));
	monte_carlo(tree);
}
