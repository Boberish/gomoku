use ego_tree;
use ego_tree::NodeMut;
use ego_tree::NodeRef;
use rand::seq::IteratorRandom;

pub type Board = [[char;3];3];
pub type Coords = (i8, i8);

#[derive(Debug)]
pub struct McNode {
    state: Board,
    num_plays: u16,
    num_wins: u16,
    player: u8,
    score: u32
}

impl McNode {
    fn new(state: Board) -> McNode {
        McNode {
            state: state,
            num_plays : 0,
            num_wins : 0,
            player: 1,
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

fn find_good_node(root: NodeRef<McNode>)  {
    match root {
        mut node => {
            if node.has_children(){
                let kids = node.children();
                let non_visited = kids.filter(|k| k.value().num_plays != 0);
                println!("dhdkh{:#?}", non_visited.size_hint());
                for x in non_visited {
                    println!("ff{:#?}",x );
                }

                // if Some(non_visited) {
                // println!("this is it {:#?}",non_visited);//.value().choose(&mut rand::thread_rng()).unwrap());
                // }
                // if non_visited.is_some() {
                //     println!("junk");
                // }
                // match non_visited {
                //     Some(n) => return n.choose(&mut rand::thread_rng()),
                //     _=> find_good_node( kids.choose(&mut rand::thread_rng()).unwrap() )
                // }
                // let cho = non_visited.choose(&mut rand::thread_rng());
                // // for x in non_visited {
                // if cho.is_some() {
                //     println!("{:#?}", cho);
                // }
                // match non_visited.choose(&mut rand::thread_rng()) 

            }
        }
    }
}
fn main() {
    let mut board: Board = [['.'; 3]; 3];
    let board2: Board = [
		['.', '.', '.'],
		['.', 'x', '.'],
		['.', '.', '.']
	];
    let board3: Board = [
		['x', '.', '.'],
		['.', '.', '.'],
		['.', '.', '.']
	];
        let board4: Board = [
		['.', '.', '.'],
		['.', '.', '.'],
		['.', '.', 'x']
	];
    let mut mcRootNode = McNode::new(board);
    let mut mcChildNode = McNode::new(board2);
    let mut mcChildNode2 = McNode::new(board3);
    let mut mcChildNode3 = McNode::new(board4);

    let mut tree = ego_tree::Tree::new(mcRootNode);
    let mut root = tree.root_mut();
    root.append(mcChildNode2);
    root.append(mcChildNode3);
    root.append(mcChildNode);
    let passed_root = tree.root();
    
    // let k = tree.values();

    // for nod in k {
    //     println!("{:#?}", nod.state)
    // }

    // for t in 0..100 {
     find_good_node(passed_root);
    // let a:i32 = 0;
    // let b = 1;

    // if a.is_some() {
    //     println!("1");
    // }
    // }
    
    // println!("{:#?}",tree);
    // println!("{:#?}",k);
}
