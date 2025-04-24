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

fn main() {
    println!("Hello, world!");
}
