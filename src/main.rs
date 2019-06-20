use ego_tree;
use ego_tree::NodeMut;
use ego_tree::NodeRef;
use ego_tree::NodeId;
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

#[derive(Debug)]
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
// fn find_good_node(root: NodeMut<McNode>)  {
//     match root {
//         // None => {
//         //     println!("done traversing a level");
//         //     // return root;
//         // }
//     mut node => {
//             if node.has_children(){
//                 let mut tmp = node.first_child().unwrap();
//                 // let mut val = tmp.value();

//                 let mut sib = tmp.next_sibling().unwrap();
//                 println!("{:#?}", sib.value().score);

//                 // let mut sib2 = sib.next_sibling().unwrap();
//                 // println!("{:#?}", sib2.value().score);

//                 let mut sib2 = sib.next_sibling();
//                 // match sib2 {
//                 //     Some(mut sib) => {println!("{:#?}", sib.value().score);},
//                 //     _ => {}
//                 // }

//                 while let Some(mut s) = sib2 {
//                     // println!("{:#?}", s.value().score);
//                     println!("junk");
//                     sib2 = Some(s.next_sibling().unwrap());
//                 }
//                 // if let sib2 = Some(1)  {
//                 // println!("{:#?}", sib2.unwrap().value().score);
//                 // }
//                 // println!("{:#?}", sib3.value().score);

//                 // let vals = sib2.value();
//                 // println!("{}", vals.score);
                
//                 // val.num_plays = 9;

//                 // let mut tmp2 = tmp.unwrap();
//                 // let mut tmp3 = tmp2.next_sibling();
//                 // let tmp2 = tmp.next_siblings();
//                 // let tmp3 = node.next_sibling();

//                 // for mut x in tmp {
//                     // println!("{:#?}-----------------------------",val);
//                     // println!("{:#?}",tmp3);
//                     // println!("{:#?}",tmp3);
//                 // }
//             }
//             return;
//             // return node;
//             // find_good_node(node.)
//         }
//         _ => {}
//         // return;
//     }

// }

fn find_good_node(root: &NodeRef<McNode>) -> (Vec<NodeId>)  {
    match root {
        node => {
            if node.has_children() {
                let kids = node.children();
                let non_visited = kids.filter(|k| k.value().num_plays == 0);
                // if non_visited.is_some {
				for x in non_visited {
					return vec![x.id()];
                    // return (non_visited.choose(&mut rand::thread_rng()),path);
                }

                //do UCB
                let kids = node.children();
				let path = find_good_node(&kids.choose(&mut rand::thread_rng()).unwrap());
				
				return [path, vec![node.id()]].concat()
                
                // }
                // match non_visited.choose(&mut rand::thread_rng()) 

            }
			vec![node.id()]
        }
    }
}
fn main() {
    // let mut board: Board = [['.'; 3]; 3];
    let board: Board = [
		['.', '.', '.'],
		['.', 'x', '.'],
		['.', '.', '.']
	];
    // let board3: Board = [
	// 	['x', '.', '.'],
	// 	['.', '.', '.'],
	// 	['.', '.', '.']
	// ];
    //     let board4: Board = [
	// 	['.', '.', '.'],
	// 	['.', '.', '.'],
	// 	['.', '.', 'x']
	// ];

    let mc_root_node = McNode::new(board,board.current_player());
    // let mut mcChildNode = McNode::new(board2);
    // let mut mcChildNode2 = McNode::new(board3);
    // let mut mcChildNode3 = McNode::new(board4);

    let mut tree = ego_tree::Tree::new(mc_root_node);
    let mut root = tree.root_mut();
    // let mut a = root.append(mcChildNode2);
    // let mut b = a.append(mcChildNode3);
    // root.append(mcChildNode);
	let legal = board.legal_plays();
	
	for _ in 0..7 {
		root.append(McNode::new( board.next_state( legal.choose(&mut rand::thread_rng()).unwrap()),board.current_player()));
	}
	// let passed_root = tree.root();
    // println!("this is OG tree {:?}", tree);
    // let k = tree.values();

    // for nod in k {
    //     println!("{:#?}", nod.state)
    // }

    // for t in 0..100 {

    // let mut path: Vec<u16> = Vec::new();

	let mut i = 0;

	// Monte carlo loop
	while i < 1000
	 {

		//Selection
		let mut path = find_good_node(&tree.root());
		// println!("path: {:#?}", path);
		
		let mut expand_node = tree.get_mut(path[0]).unwrap();
		// path.push(expand_node);
		let legal_plays = expand_node.value().state.legal_plays();
		if legal_plays.len() == 0 {
			continue;
		}

		// expansion
		let mut ids = vec![];
		for _ in 0..3{
			let simulation_root_node_coords = legal_plays.choose(&mut rand::thread_rng()).unwrap();
			let index = legal_plays.iter().position(|x| *x == *simulation_root_node_coords).unwrap();
			legal_plays.remove(index);
		//we deref expand node values twice, once for legal plays the other to get the board, maybe can do once
			let new_board = expand_node.value().state.next_state(simulation_root_node_coords);

			let mc_child_node4 = McNode::new(new_board, new_board.current_player());
			let mut_id = expand_node.append(mc_child_node4);
			ids.push((mut_id, simulation_root_node_coords));

		}
		
		let tchoice = ids.choose(&mut rand::thread_rng()).unwrap();
		let choice = tchoice.clone();
		path.push(choice.0.id());
		// println!("this is the treeeeeeee {:?}",tree );
		// new_board.display();
		let won = gamesim(*choice);
		println!("{:?} player WON",won );

		
		for node_id in path {
			let mut mut_ref = tree.get_mut(node_id).unwrap();
			if won == mut_ref.value().player {
				mut_ref.value().num_wins += 1;
			}
			mut_ref.value().num_plays += 1;
		}
		

		i += 1;
	}
	// let tmp = tree.get_mut(path[0]).unwrap();
	println!("this is the treeeeeee {:?}", tree);

	fn gamesim(choice: (NodeMut<McNode>,Coords)) -> i8 {
		let start = choice.0.value().state;
		if start.if_over(){
			return 0;
		}
		let mut board = start.clone();
		let mut winner = start.winner(&choice.1);
		// let mut play = (0,0);
		while winner == 0 {
			if board.if_over(){
				return 0;
			}
			//if no legal plays then its a tie, so you dont haev to check for "if over"
			let plays = board.legal_plays();
			let play = plays.choose(&mut rand::thread_rng()).unwrap();
			//optimize this next state to maybe mutate the copy you give it instead of cloning 
			board = board.next_state(play);

			winner = board.winner(play);
			

		}
		return winner;
	}
	// println!("node to expand {:?}",expand_node );

	// println!("simulation_root_node_coords {:?}",simulation_root_node_coords );
	    // let a:i32 = 0;
    // let b = 1;

    // if a.is_some() {
    //     println!("1");
    // }
    // }
    
    // println!("{:#?}",tree);
    // println!("{:#?}",k);
	let best = tree.root().children();
	let best2 = best.clone();
	// for k in best {
	// 	println!("this is the values {:?}",k.value() );
	// }
	let turn = best.map(|node| (node.value().num_plays, node.value().num_wins));
	let turn2 = best2.map(|node| ((node.value().num_wins as f32 / node.value().num_plays as f32) * 1000.0) as i64) ;
	// let fin = turn2.max().unwrap();
	// println!("fin {:?} ",fin);
	for x in turn2 {
		println!("this {:?}", x);
	}
	for x in turn {
		println!("this {:?}", x);
	}
	// println!("fin {:?}", fin);
	// let fin = turn.max();

}
