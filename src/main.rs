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
	fn coords(&self, next_state: &Board) -> Coords;
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

	fn coords(&self, next_state: &Board) -> Coords {
		for i in 0..3 {
			for j in 0..3 {
				if self[i][j] != next_state[i][j] {
					return (i as i8, j as i8);
				}
			}
		}
		(0, 0) // should never happen
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
fn back_propagation (mut node: NodeMut<McNode>, win: u16) {
	let value = node.value();
	value.num_wins = value.num_wins + win;
	value.num_plays = value.num_wins + 1;
	match node.parent() {
		None => (),
		Some(parent) => back_propagation(parent, win) 
	}
}

// random selection
// TODO: add UCT selection
fn selection (node: NodeRef<McNode>) -> NodeId {
	if !(node.has_children()) {
		return node.id();
	}
	let kids = node.children();
	let kid = kids.choose(&mut rand::thread_rng()).unwrap();
	selection(kid)
}

// Explore first next play until the game is over
// or there is no plays left to select
fn simulation (state: &Board, last_move: &Coords) -> i8 {

	// coords: Mock
	let winner = state.winner(last_move);
	if winner != 0 {
		return winner;
	}

	// Mock: always use first possible play
	let legal_plays = state.legal_plays();
	if legal_plays.is_empty() {
		return winner;
	}
	let random_move = legal_plays.choose(&mut rand::thread_rng()).unwrap();
	simulation(&state.next_state(&random_move), &random_move)
}

// Add all possible nodes to the tree
fn expand (mut node: NodeMut<McNode>) -> NodeMut<McNode> {
	let value = node.value();
	let state = value.state;
	let player = value.player;
	let legal_plays = state.legal_plays();
	let child_player = 3 - player;
	for coords in legal_plays.into_iter() {
		let child_state = state.next_state(&coords);
		let child = McNode::new(child_state, child_player);
		node.append(child);
	}
	node
}

// mock: return first avalaible node
// TODO: avoid crash on division by 0
fn choose_best_move (root_node: NodeRef<McNode>) -> Result<Board, String> {
	let best_node = root_node.children()
	.max_by(|a, b| {
		let value_a = a.value();
		let value_b = b.value();
		println!("plays: {:?}, wins: {:?}", value_b.num_plays, value_b.num_wins);
		let score_a = (value_a.num_wins as f32 / value_a.num_plays as f32);
		let score_b = (value_b.num_wins as f32 / value_b.num_plays as f32);
		return score_a.partial_cmp(&score_b).unwrap_or(1.cmp(&1))
	});
	match best_node {
		None => Err(String::from("No avalaible nodes")),
		Some(v) => Ok(v.value().state)
	}
}

fn monte_carlo(mut tree: Tree<McNode>) -> Result<Coords, String> {
	let n_run = 50;

	for i in 0..=n_run {
		let selected_node_id = selection(tree.root());
		let mut selected_node = tree.get_mut(selected_node_id).unwrap();
		selected_node = expand(selected_node);
		let winner = simulation(&selected_node.value().state, &(0, 0)); // Not sure about the coords
		let win = if winner == 2 { 0 } else { 1 }; // Pat is worth a victory for now
		back_propagation(selected_node, win);
	}

	match choose_best_move(tree.root()) {
		Err(error_message) => Err(error_message),
		Ok(best_board) => Ok(tree.root().value().state.coords(&best_board))
	}
}

fn main() {
    let board: Board = [
		['.', '.', '.'],
		['.', '.', '.'],
		['.', '.', '.']
	];

	ai_vs_ai(board, 1)
}

fn ai_vs_ai (board: Board, player: i8) {

	let tree = tree!(McNode::new(board, player));
	match monte_carlo(tree) {
		Ok(best_move) => {
			let new_board = board.next_state(&best_move);
			new_board.display();
			let winner = new_board.winner(&best_move);
			if (winner != 0) {
				return println!("player {} wins", winner)
			}
			ai_vs_ai(new_board, 3 - player)
		},
		Err(error_message) => println!("{}", error_message)
	}
}