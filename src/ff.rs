fn find_good_node(root: NodeRef<McNode>, path: Vec<u16>) -> (NodeRef<McNode>, Vec<u16>)  {
    match root {
        mut node => {
            if node.has_children(){
                let kids = node.children();
                let non_visited = kids.filter(|k| k.value().num_plays == 0);
                if non_visited.is_some {
                    return (non_visited.choose(&mut rand::thread_rng()),path);
                }
                //do UCB
                find_good_node(kids.choose(&mut rand::thread_rng()), path)
                
                // }
                // match non_visited.choose(&mut rand::thread_rng()) 

            }
            return (node,path)
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
    let mut path: Vec<u16> = Vec::new();
    let (node, path) = find_good_node(passed_root, path);
	println!("node: {:#?}", node)
	println!("path: {:#?}", path)
    // let a:i32 = 0;
    // let b = 1;

    // if a.is_some() {
    //     println!("1");
    // }
    // }
    
    // println!("{:#?}",tree);
    // println!("{:#?}",k);
}
