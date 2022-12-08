use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum TreeNode {
    Node { children: HashMap<String, TreeNode> },
    Leaf { value: u32 },
}

impl TreeNode {
    pub fn insert(&mut self, path: &str, value: Option<u32>) {
        match self {
            TreeNode::Leaf {
                value: current_value,
            } => {
                if path.matches("/").count() != 0 {
                    panic!("This is a leaf!");
                }

                if let Some(value) = value {
                    *current_value = value;
                }
            }
            TreeNode::Node { children } => {
                let path_parts: Vec<&str> = path.split("/").collect();

                match path_parts.as_slice() {
                    ["", next_child] | [next_child] => {
                        let new_node = match value {
                            Some(leaf_value) => TreeNode::Leaf { value: leaf_value },
                            None => TreeNode::Node {
                                children: HashMap::new(),
                            },
                        };
                        children.insert(next_child.to_string(), new_node);
                    }
                    ["", next_child, rest @ ..] | [next_child, rest @ ..] => {
                        let child = children
                            .get_mut(*next_child)
                            .expect("Child with that name does not exist!");

                        child.insert(&rest.join("/"), value);
                    }
                    _ => panic!("Unknown!"),
                }
            }
        }
    }

    fn get_total_size(&self) -> u32 {
        let mut total_size = 0;

        match self {
            TreeNode::Leaf { value } => total_size += value,
            TreeNode::Node { children } => {
                for child in children.values() {
                    let child_size = child.get_total_size();
                    total_size += child_size;
                }
            }
        }

        total_size
    }

    pub fn get_all_directory_sizes(&self) -> Vec<u32> {
        let mut result = Vec::new();

        match self {
            TreeNode::Leaf { value: _ } => {}
            TreeNode::Node { children } => {
                let mut current_directory_size = 0;

                for child in children.values() {
                    match child {
                        TreeNode::Leaf { value } => {
                            current_directory_size += *value;
                        }
                        TreeNode::Node { children: _ } => {
                            let mut children_directories = child.get_all_directory_sizes();

                            current_directory_size += child.get_total_size();

                            result.append(&mut children_directories);
                        }
                    }
                }

                result.push(current_directory_size);
            }
        }

        result
    }

    #[cfg(test)]
    pub fn children(&self) -> Option<Vec<&TreeNode>> {
        match self {
            TreeNode::Node { children } => Some(children.values().collect::<Vec<&TreeNode>>()),
            TreeNode::Leaf { value: _value } => None,
        }
    }
}

#[derive(Debug)]
pub struct GeneralTree {
    root: Option<Box<TreeNode>>,
}

impl GeneralTree {
    pub fn new() -> Self {
        Self {
            root: Some(Box::new(TreeNode::Node {
                children: HashMap::new(),
            })),
        }
    }

    pub fn insert(&mut self, path: &str, value: Option<u32>) {
        match &mut self.root {
            Some(node) => node.insert(path, value),
            None => {
                if path == "/" {
                    let new_node = match value {
                        Some(leaf_value) => TreeNode::Leaf { value: leaf_value },
                        None => TreeNode::Node {
                            children: HashMap::new(),
                        },
                    };

                    self.root = Some(Box::new(new_node));
                } else {
                    panic!("Path was not '/' but there's no root node!");
                }
            }
        }
    }

    pub fn get_all_directory_sizes(&self) -> Vec<u32> {
        match &self.root {
            Some(node) => node.get_all_directory_sizes(),
            None => Vec::new(),
        }
    }

    pub fn get_total_used_space(&self) -> u32 {
        match &self.root {
            Some(node) => node.get_total_size(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf_insert_works() {
        let mut new_node = TreeNode::Leaf { value: 1 };

        new_node.insert("", Some(2));

        assert_eq!(new_node, TreeNode::Leaf { value: 2 });
    }

    #[test]
    fn leaf_insert_with_none_does_not_work() {
        let mut new_node = TreeNode::Leaf { value: 1 };

        new_node.insert("", None);

        assert_eq!(new_node, TreeNode::Leaf { value: 1 });
    }

    #[should_panic]
    #[test]
    fn leaf_insert_panics_if_path_wrong() {
        let mut new_node = TreeNode::Leaf { value: 1 };

        new_node.insert("/", None);
    }

    #[test]
    fn node_insert_works() {
        let mut new_node = TreeNode::Node {
            children: HashMap::new(),
        };

        new_node.insert("/a", Some(3));
        new_node.insert("/b", None);
        new_node.insert("/b/c", Some(4));

        assert_eq!(
            new_node.children().unwrap(),
            vec![
                &TreeNode::Leaf { value: 3 },
                &TreeNode::Node {
                    children: HashMap::from([("c".to_string(), TreeNode::Leaf { value: 4 })])
                }
            ]
        );
    }
}
