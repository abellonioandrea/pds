use std::{collections::HashMap, process::Child};

#[derive(Debug)]
pub enum Error {
    Duplicate,
    NotFound,
    Forbidden,
}

// Tree nodes have nodes which contain a state (on/off) and they are linked with 
// a father/child structure. 
// Theredore we can represent node states with a hashmap, where the key is the node name
// and the value is the state of the node (on/off).

// A second hashmap is needed to represent the tree structure, where the key is the parent name
// and the value is a vector of children names.

// In order to make complexity O(1) for the add and remove operations, 
// we can keep track of the parent of each node and therefore we need
// a third hashmap child_name -> parent_name


pub struct Albero {
    // node id -> switch value (we could store any state of the node, not just a boolean)
    nodes: HashMap<String, bool>,

    // Tree structure: parent id -> children ids, we just need an hash map
    children: HashMap<String, Vec<String>>,

    // back link from child id to parent id
    fathers: HashMap<String, String>,
}

impl Albero {
    pub fn new() -> Albero {
        // init with the special node root already present
        Albero {
            nodes: HashMap::from([
                ("root".to_string(), false),
            ]),
            children: HashMap::from([
                ("root".to_string(), Vec::new()),
            ]),
            // root has root as parent
            fathers: HashMap::from([
                ("root".to_string(), "root".to_string()),
            ]),
        }
    }

    pub fn add(&mut self, parent: &str, node: &str) -> Result<(), Error> {

        // can't add if the parent doesn't exist
        if !self.nodes.contains_key(parent) {
            return Err(Error::NotFound);
        }
        // if node is already present, return error
        if self.nodes.contains_key(node) {
            return Err(Error::Duplicate);
        }


        self.children.entry(parent.to_string())
            // if the parent already has children, push the new node to the list
            .and_modify(|children| children.push(node.to_string()))
            // if the parent hans't any children yet, we need to create a new vector
            .or_insert_with(|| vec![node.to_string()]);

        // insert te back link
        self.fathers.insert(node.to_string(), parent.to_string());
        // finally inser the node
        self.nodes.insert(node.to_string(), false);
        Ok(())
    }


    // recursive solution
    pub fn remove(&mut self, node: &str) -> Result<(), Error> {
        if node == "root" {
            // can't remove root
            return Err(Error::Forbidden);
        }

        if !self.nodes.contains_key(node) {
            // can't remove if the node is not present
            return Err(Error::NotFound);
        }

        // we must remove all the children first

        // we need to extract child names from the children hashmap before deleting,
        //  in order to break the reference cycle, 
        // otherwise we would have a borrow checker error while trying
        // to call remove() recusively with a mutable reference to self

        // children is a Vec<String> which has no refernce to anything in self anymore
        let children = match self.children.get(node) {
            Some(children) => children.clone(),
            None => vec![],
        };

        // now we can safely get a mutable reference to self call rmeove recursively
        for child in children {
            self.remove(&child);
        }

        // once here all the chindre in the structure are removed

        // remove the reference to the deling node from the parent
        if let Some(parent) = self.fathers.get(node) {
            if let Some(children) = self.children.get_mut(parent) {
                // remove the node from the parent's children
                children.retain(|c| *c != node);
            }
        }

        // finally remove the node from the three hashmaps
        self.children.remove(node);
        self.fathers.remove(node);
        self.nodes.remove(node);
        Ok(())
    }


    // here we use an iterative solution
    pub fn remove_iterative(&mut self, node: &str) -> Result<(), Error> {
        if node == "root" {
            // can't remove root
            return Err(Error::Forbidden);
        }

        // if the node is not present, return error
        if !self.nodes.contains_key(node) {
            return Err(Error::NotFound);
        }

        // get all the children that will be deleted
        let mut deleting = Vec::new();
        let mut visiting = vec![node];

        while !visiting.is_empty() {
            let node = visiting.pop().unwrap();
            if let Some(children) = self.children.get(node) {
                for c in children {
                    visiting.push(c);
                }
            }
            deleting.push(node.to_string());
        }

        // now delete all the nodes
        for node in deleting {
            if let Some(parent) = self.fathers.get(&node) {
                if let Some(children) = self.children.get_mut(parent) {
                    children.retain(|c| *c != node);
                }
            }
            self.fathers.remove(&node);
            self.nodes.remove(&node);
        }
        Ok(())
    }

    pub fn toggle(&mut self, node: &str) -> Result<(), Error> {
        match self.nodes.get_mut(node) {
            Some(switch) => {
                *switch = !*switch;
                Ok(())
            }
            None => return Err(Error::NotFound),
        }
    }

    pub fn peek(&self, node: &str) -> Result<bool, Error> {
        // all switches to the root must be on
        let mut cnode = node;
        loop {
            match self.nodes.get(cnode) {
                Some(false) => {
                    // if any node is off, return false
                    return Ok(false);
                }
                Some(true) => {
                    // continue to the parent
                    if let Some(parent) = self.fathers.get(cnode) {
                        cnode = parent;
                    }

                    // reached the root
                    if cnode == "root" {
                        return Ok(true);
                    }
                }
                None => {
                    // the first node could be not present
                    return Err(Error::NotFound);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Albero;

    fn build_tree() -> Albero {
        let mut tree = Albero::new();
        tree.add("root", "A").unwrap();
        tree.add("A", "B").unwrap();
        tree.add("A", "C").unwrap();
        tree.add("B", "D").unwrap();
        tree.add("B", "E").unwrap();
        tree.add("C", "F").unwrap();
        tree.add("C", "G").unwrap();
        tree
    }

    #[test]
    fn test_simple_add() {
        let mut tree = Albero::new();
        tree.add("root", "A").unwrap();
        assert_eq!(tree.nodes.get("A"), Some(&false));
        assert_eq!(tree.children.get("root"), Some(&vec!["A".to_string()]));
        assert_eq!(tree.fathers.get("A"), Some(&"root".to_string()));
    }

    #[test]
    fn test_complex_add() {
        let tree = build_tree();
        assert_eq!(tree.nodes.get("G"), Some(&false));
        assert_eq!(tree.children.get("C"), Some(&vec!["F".to_string(), "G".to_string()]));
        assert_eq!(tree.fathers.get("G"), Some(&"C".to_string()));
    }

    #[test]
    fn test_simple_remove() {
        let mut tree = build_tree();
        tree.remove("A").unwrap();
        assert_eq!(tree.nodes.get("A"), None);
        assert_eq!(tree.children.get("root"), Some(&vec![]));
        assert_eq!(tree.fathers.get("A"), None);
    }

    #[test]
    fn test_complex_remove() {
        let mut tree = build_tree();
        tree.remove("B").unwrap();
        assert_eq!(tree.nodes.get("B"), None);
        assert_eq!(tree.children.get("A"), Some(&vec!["C".to_string()]));
        assert_eq!(tree.fathers.get("B"), None);
    }

    #[test]
    fn test_remove_all_children() {
        let mut tree = build_tree();
        tree.remove("A").unwrap();
        assert_eq!(tree.nodes.get("B"), None);
        assert_eq!(tree.nodes.get("C"), None);
        assert_eq!(tree.children.get("root"), Some(&vec![]));
        assert_eq!(tree.fathers.get("A"), None);
    }

    #[test]
    fn test_turn_on_just_leaf() {
        let mut tree = build_tree();
        tree.toggle("D").unwrap();
        assert_eq!(tree.nodes.get("D"), Some(&true));
        assert_eq!(tree.peek("D").unwrap(), false);
    }

    #[test]
    fn test_turn_on_path_to_leaf() {
        let mut tree = build_tree();
        tree.toggle("A").unwrap();
        tree.toggle("C").unwrap();
        tree.toggle("F").unwrap();
        assert_eq!(tree.peek("F").unwrap(), true);
    }
}