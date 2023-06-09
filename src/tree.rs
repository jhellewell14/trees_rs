use crate::node::Node;

#[derive(Debug)]
pub struct Tree {
    pub tree_vec: Vec<usize>,
    pub nodes: Vec<Node>,
    pub max_depth: usize,
}

impl<'a> Tree {
    pub fn new(tree_vec: Vec<usize>) -> Tree {
        let k = tree_vec.len();
        Tree {tree_vec,
        nodes: vec![Node::default(); 2 * k + 1],
        max_depth: 0}
    }

    pub fn get_root(&self) -> Option<&Node> {
        self.nodes.iter().find(|node| node.parent.is_none())
    }

    pub fn iter(&'a self, node: Option<&'a Node>) -> RootIter {
        RootIter{current_node: node, next_node: node, 
            tree: self, end_flag: false}
    }

    pub fn preorder(&'a self, node: Option<&'a Node>) -> Preorder {
        Preorder{current_node: node, next_node: node, 
            tree:self, return_nodes: vec![]}
    }

    pub fn postorder(&'a self, node: Option<&'a Node>) -> PostOrder {
        PostOrder{current_node: node,
            end_index: node.unwrap().index, 
            tree: self, 
            start_flag: true,}
    }

    pub fn mut_node(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }

    pub fn mut_parent(&mut self, index: usize) -> Option<&mut Node> {
        match self.nodes.get(index).unwrap().parent {
            Some(i) => self.mut_node(i),
            None => None,
        }
    }

    pub fn get_node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    pub fn get_parent(&self, index: usize) -> Option<&Node> {
        match self.nodes.get(index).unwrap().parent {
            Some(i) => self.get_node(i),
            None => None,
        }
    }

    // Returns vector of nodes in tree that are tips
    pub fn get_tips(&self) -> Vec<&Node> {
        self.nodes
        .iter()
        .filter(|n| n.tip == true)
        .collect()
    }

    // pub fn get_nodes_at_depth(&self, depth: usize) -> Vec<&Node> {
    //     self.nodes
    //     .iter()
    //     .filter(|n| n.depth == depth)
    //     .collect()
    // }

    // Depth of given node in tree
    // pub fn node_depth(&self, node: Option<&Node>) -> usize {
    //     self
    //     .iter(node)
    //     .fold(0, |acc, _node| acc + 1)
    // }

    // Find maximum node depth
    pub fn max_treedepth(&self) -> usize {
        self.nodes.iter().map(|node| node.depth).max().unwrap_or(0)
    }

    pub fn add(&mut self, index: usize, parent: Option<usize>){

        let mut dpth: usize = 0;

        match parent {
            Some(par) => {
                self.mut_node(par).unwrap().new_child(index);
                dpth = self.get_node(par).unwrap().depth + 1;
            },
            None => {},
        };
        
        self.nodes[index] = Node::new(parent, (None, None), index, dpth);
    }

    pub fn get_handedness(&self, index: usize) -> Handedness {
        let (l, _) = self.get_parent(index).unwrap().children;

        if l == Some(index) {
            Handedness::Left
        } else {
            Handedness::Right
        }
    }

    // pub fn relocate(&mut self, node_index: usize, new_parent_index: usize) {

    //     if self.get_node(node_index).is_none() {
    //         panic!("Node to move does not exist");
    //     }

    //     if self.get_node(new_parent_index).is_none() {
    //         panic!("New parent does not exist");
    //     }

    //     if self.get_parent(node_index).is_none() {
    //         panic!("Cannot move root node")
    //     }

    //     self.mut_parent(node_index).unwrap().remove_child(node_index);
    //     self.mut_node(node_index).unwrap().parent = Some(new_parent_index);
    //     self.mut_node(new_parent_index).unwrap().new_child(node_index);

    // }

    pub fn most_left_child(&'a self, node: Option<&'a Node>) -> Option<&Node> {
        let mut cur_node = node;
        let mut cur_left_child = cur_node.unwrap().children.0;

        while cur_left_child.is_some() {
            cur_node = self.get_node(cur_left_child.unwrap());
            cur_left_child = cur_node.unwrap().children.0;
        }
        // println!("current node: {:?}", cur_node);
        cur_node
    }

    pub fn swap_to_right_child(&self, index: usize) -> Option<&Node> {
        self.get_node(self.get_parent(index).unwrap().children.1.unwrap())
    }

}

#[derive(Debug)]
pub enum Handedness {
    Left,
    Right,
}


#[derive(Debug)]
pub struct RootIter<'a> {
    current_node: Option<&'a Node>,
    next_node: Option<&'a Node>,
    tree: &'a Tree,
    end_flag: bool,
}

// Traverses from a specified node up to the root of the tree
impl<'a> Iterator for RootIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Self::Item>;

        if self.end_flag {return None};

        match self.current_node.unwrap().parent {
            None => {
                output = self.tree.get_root();
                self.end_flag = true;
            },
            Some(i) => {
                output = self.current_node;
                self.next_node = self.tree.get_node(i);
            },
        };

        self.current_node = self.next_node;

        output
    }
}

#[derive(Debug)]
pub struct Preorder<'a> {
    current_node: Option<&'a Node>,
    next_node: Option<&'a Node>,
    return_nodes: Vec<Option<&'a Node>>,
    tree: &'a Tree,
}

// Traverses tree in preorder starting from specified node
impl<'a> Iterator for Preorder<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Self::Item> = self.current_node; 

        if self.current_node.is_none() {return output;}

        match self.current_node.unwrap().children {
            (Some(a), None) => {
                self.next_node = self.tree.get_node(a);
            },
            (Some(a), Some(b)) => {
                self.next_node = self.tree.get_node(a);
                self.return_nodes.push(self.tree.get_node(b));
            },
            (None, None) => {
                self.next_node = match self.return_nodes.pop() {
                    None => None,
                    Some(node) => node,
                };  
            },
            _ => {panic!("Iterator has found a node with only a right child")},
        };

        self.current_node = self.next_node;
        
        output
    }
}



// Start: go as far left as possible
// If in Left node, swap and go left
// If in Right node, go up to parent
pub struct PostOrder<'a> {
    tree: &'a Tree,
    start_flag: bool,
    current_node: Option<&'a Node>,
    end_index: usize,
}

impl<'a> Iterator for PostOrder<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {

        if self.start_flag {
            self.current_node = self.tree.most_left_child(self.current_node);
            self.start_flag = false;
        } else {
            // If we return to start node, end iterator
            if self.current_node.unwrap().index == self.end_index {
                return None;
            }

            let ind = self.current_node.unwrap().index;
            match self.tree.get_handedness(ind) {
                Handedness::Left => {
                    self.current_node = self.tree.most_left_child(self.tree.swap_to_right_child(ind));
                },
                Handedness::Right => {
                    self.current_node = self.tree.get_parent(ind);
                },
            }
        }

        self.current_node 
    }
}