extern crate petgraph;

use petgraph::graph::{NodeIndex, Graph};
use petgraph::visit::EdgeRef;

#[derive(Debug)]
pub struct Trie {
    graph: Graph<bool, char, petgraph::Directed, u32>,
    root: NodeIndex<u32>,
    size: usize,
}

impl Trie {
    pub fn new() -> Trie {
        let mut graph = Graph::new();
        let root = graph.add_node(false);
        Trie { graph: graph, root: root, size: 0 }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self.root;

        for letter in word.chars() {
            let mut found = false;
            for edge in self.graph.edges(node) {
                if *edge.weight() == letter {
                    found = true;
                    node = edge.target();
                    break;
                }
            }
            if !found {
                let new_node = self.graph.add_node(false);
                self.graph.update_edge(node, new_node, letter);
                node = new_node;
            }
        }

        let mut terminal_node = self.graph.node_weight_mut(node).expect("node to exist");
        if !*terminal_node {
            *terminal_node = true;
            self.size += 1;
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = self.root;

        for letter in word.chars() {
            let mut found = false;
            for edge in self.graph.edges(node) {
                if *edge.weight() == letter {
                    found = true;
                    node = edge.target();
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        return *self.graph.node_weight(node).unwrap();
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_insert_makes_expected_number_of_nodes() {
        let mut t = Trie::new();
        t.insert("cat");
        // one for root, one each for c, ca, cat
        assert_eq!(t.graph.node_count(), 4);
        // one for each letter
        assert_eq!(t.graph.edge_count(), 3);

        t.insert("car");
        t.insert("cab");
        // one more each for last letter of car and cab
        assert_eq!(t.graph.node_count(), 6);
        // one more each for last letter of car and cab
        assert_eq!(t.graph.edge_count(), 5);

        t.insert("pizza");
        // one more for each letter in pizza
        assert_eq!(t.graph.node_count(), 11);
        // one more for each letter in pizza
        assert_eq!(t.graph.edge_count(), 10);
    }

    #[test]
    fn test_contains_works() {
        let mut t = Trie::new();
        t.insert("za");
        t.insert("jo");

        assert!(t.contains("za"), "contains za");
        assert!(t.contains("jo"), "contains jo");

        for first in 0..26 {
            for second in 0..26 {
                let a = 'a' as u8;
                let word = format!("{}{}", (a + first) as char, (a + second) as char);
                if word == "za" || word == "jo" {
                    break;
                }
                assert!(!t.contains(&word), format!("does not contain {}", word));
            }
        }
    }

    #[test]
    fn test_contains_only_finds_terminals() {
        let mut t = Trie::new();
        t.insert("suffix");
        assert!(!t.contains("suf"), "does not contain partial word");
    }

    #[test]
    fn test_tracks_size() {
        let mut t = Trie::new();
        assert_eq!(t.size(), 0, "empty to start with");
        t.insert("a");
        assert_eq!(t.size(), 1, "contains a");
        t.insert("b");
        assert_eq!(t.size(), 2, "contains a and b");
        t.insert("at");
        assert_eq!(t.size(), 3, "contains a, b, and at");
        t.insert("a");
        assert_eq!(t.size(), 3, "didn't increment for duplicate a");
    }

    #[test]
    fn does_not_contain_empty_string() {
        let mut t = Trie::new();
        assert!(!t.contains(""), "does not contain empty word");
        t.insert("word");
        assert!(!t.contains(""), "does not contain empty word");
    }
}
