use crate::{PatternType, SemanticGraph};
use std::collections::HashMap;

pub struct PatternRecognizer {
    patterns: HashMap<String, Vec<PatternType>>,
}

impl PatternRecognizer {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        patterns.insert(
            "rust".to_string(),
            vec![PatternType::IteratorChain, PatternType::Builder, PatternType::FibonacciLike],
        );

        patterns.insert(
            "python".to_string(),
            vec![PatternType::MapReduce, PatternType::DataProcessor, PatternType::RecursiveTree],
        );

        patterns.insert(
            "javascript".to_string(),
            vec![PatternType::WebEndpoint, PatternType::MapReduce, PatternType::Cacheable],
        );

        Self { patterns }
    }

    pub fn recognize_cross_language_patterns(
        &self,
        graphs: Vec<&SemanticGraph>,
    ) -> Vec<CrossLanguagePattern> {
        let mut cross_patterns = Vec::new();

        for graph in graphs {
            // Use the patterns field to check if language is supported
            if !self.patterns.contains_key(&graph.language) {
                continue;
            }

            for (pattern_hash, node_ids) in &graph.pattern_cache {
                let pattern_type = self.hash_to_pattern(*pattern_hash);

                // Check if this pattern is common for the language
                if let Some(supported_patterns) = self.patterns.get(&graph.language) {
                    if supported_patterns.contains(&pattern_type) {
                        cross_patterns.push(CrossLanguagePattern {
                            pattern_type,
                            language: graph.language.clone(),
                            node_count: node_ids.len(),
                            semantic_hash: graph.calculate_semantic_hash(),
                        });
                    }
                }
            }
        }

        cross_patterns
    }

    pub fn suggest_optimal_language(&self, pattern: PatternType) -> Option<String> {
        // Use the patterns field to find the best language
        let mut language_scores: HashMap<String, usize> = HashMap::new();

        for (lang, patterns) in &self.patterns {
            if patterns.contains(&pattern) {
                language_scores.insert(lang.clone(), patterns.len());
            }
        }

        // Return the language with the most patterns (most mature for this pattern)
        language_scores.into_iter().max_by_key(|(_, count)| *count).map(|(lang, _)| lang)
    }

    pub fn get_supported_languages(&self) -> Vec<String> {
        self.patterns.keys().cloned().collect()
    }

    fn hash_to_pattern(&self, hash: u64) -> PatternType {
        match hash % 7 {
            0 => PatternType::FibonacciLike,
            1 => PatternType::MapReduce,
            2 => PatternType::IteratorChain,
            3 => PatternType::Builder,
            4 => PatternType::RecursiveTree,
            5 => PatternType::WebEndpoint,
            _ => PatternType::Cacheable,
        }
    }
}

#[derive(Debug)]
pub struct CrossLanguagePattern {
    pub pattern_type: PatternType,
    pub language: String,
    pub node_count: usize,
    pub semantic_hash: u64,
}
