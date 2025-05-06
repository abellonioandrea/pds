use std::collections::{HashMap, HashSet};
use std::ops::Deref;

struct Albero {
    switch: HashMap<String, bool>,
    luci: HashMap<String, bool>,
    father: HashMap<String, String>, //figlio - padre
    children: HashMap<String, HashSet<String>>, //padre - n figli
}

impl Albero {
    // nota: aggiustare mutabilità dove necessario gestire errori in caso
    // di collisioni, valori mancanti

    // aggiungi un nodo figlio del nodo father
    pub fn add(&mut self, father: &str, node: &str) {
        self.switch.insert(node.to_string(), false);
        self.luci.insert(node.to_string(), false);
        self.father.insert(node.to_string(), father.to_string());
        self.children
            .entry(father.to_string())
            .or_insert_with(HashSet::new)
            .insert(node.to_string());
    }

    // togli un nodo e tutti gli eventuali rami collegati
    pub fn remove(&mut self, node: &str) {
        if let Some(children) = self.children.remove(node) {
            for child in children {
                self.remove(&child);
            }
        }
        if let Some(parent) = self.father.remove(node) {
            if let Some(parent_children) = self.children.get_mut(&parent) {
                parent_children.remove(node);
            }
        }
        self.switch.remove(node);
        self.luci.remove(node);
    }

    // commuta l’interruttore del nodo (che può essere on off) e restituisci il nuovo valore
    pub fn toggle(&mut self, node: &str) -> bool {
        *self.switch.get_mut(node).unwrap() = !self.switch.get(node).unwrap();
        *self.switch.get(node).unwrap()
    }

    // restituisci se la luce è accesa e spenta
    pub fn peek(&self, node: &str) -> bool {
        *self.luci.get(node).unwrap()
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

fn main() {}