use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticNode {
    pub id: u64,
    pub node_type: NodeType,
    pub children: Vec<u64>,
    pub metadata: HashMap<String, String>,
    pub language: String,
    pub pattern_hash: u64,
}

// Implement Hash manually for SemanticNode
impl std::hash::Hash for SemanticNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.node_type.hash(state);
        self.children.hash(state);
        // For metadata, we'll hash a sorted representation
        let mut sorted_metadata: Vec<_> = self.metadata.iter().collect();
        sorted_metadata.sort_by_key(|(k, _)| *k);
        for (k, v) in sorted_metadata {
            k.hash(state);
            v.hash(state);
        }
        self.language.hash(state);
        self.pattern_hash.hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum NodeType {
    Function,
    Variable,
    ControlFlow,
    Arithmetic,
    Comparison,
    Loop,
    Pattern(PatternType),
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum PatternType {
    FibonacciLike,
    MapReduce,
    Builder,
    IteratorChain,
    RecursiveTree,
    Cacheable,
    DataProcessor,
    WebEndpoint,
    DatabaseQuery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticGraph {
    pub nodes: HashMap<u64, SemanticNode>,
    pub root_nodes: Vec<u64>,
    pub pattern_cache: HashMap<u64, Vec<u64>>,
    pub language: String,
}

impl SemanticGraph {
    pub fn new(language: &str) -> Self {
        Self {
            nodes: HashMap::new(),
            root_nodes: Vec::new(),
            pattern_cache: HashMap::new(),
            language: language.to_string(),
        }
    }

    pub fn add_node(&mut self, node: SemanticNode) -> u64 {
        let id = node.id;
        self.nodes.insert(id, node);
        id
    }

    pub fn detect_patterns(&mut self) {
        let node_ids: Vec<u64> = self.nodes.keys().cloned().collect();

        for node_id in node_ids {
            if let Some(node) = self.nodes.get(&node_id) {
                if let Some(pattern) = self.analyze_node_pattern(node) {
                    self.pattern_cache.entry(pattern as u64).or_insert_with(Vec::new).push(node_id);
                }
            }
        }
    }

    fn analyze_node_pattern(&self, node: &SemanticNode) -> Option<PatternType> {
        match &node.node_type {
            NodeType::Function if node.children.len() >= 2 => {
                if self.has_recursive_structure(node.id, &mut vec![]) {
                    Some(PatternType::FibonacciLike)
                } else if node.metadata.get("operation") == Some(&"map_reduce".to_string()) {
                    Some(PatternType::MapReduce)
                } else {
                    None
                }
            }
            NodeType::ControlFlow if node.children.len() > 3 => Some(PatternType::MapReduce),
            NodeType::Pattern(pattern_type) => Some(pattern_type.clone()),
            _ => None,
        }
    }

    fn has_recursive_structure(&self, node_id: u64, visited: &mut Vec<u64>) -> bool {
        if visited.contains(&node_id) {
            return true;
        }
        visited.push(node_id);

        if let Some(node) = self.nodes.get(&node_id) {
            for &child_id in &node.children {
                if self.has_recursive_structure(child_id, visited) {
                    return true;
                }
            }
        }
        false
    }

    pub fn calculate_semantic_hash(&self) -> u64 {
        use blake3::Hasher;
        let mut hasher = Hasher::new();

        for node_id in &self.root_nodes {
            if let Some(node) = self.nodes.get(node_id) {
                hasher.update(&node.id.to_le_bytes());
                hasher.update(node.language.as_bytes());
                self.hash_node_tree(&mut hasher, *node_id);
            }
        }

        let hash = hasher.finalize();
        u64::from_le_bytes(hash.as_bytes()[0..8].try_into().unwrap())
    }

    fn hash_node_tree(&self, hasher: &mut blake3::Hasher, node_id: u64) {
        if let Some(node) = self.nodes.get(&node_id) {
            hasher.update(&node.pattern_hash.to_le_bytes());
            for &child_id in &node.children {
                self.hash_node_tree(hasher, child_id);
            }
        }
    }
}
