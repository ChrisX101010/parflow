use semantic_compiler::{
    SemanticGraph, SemanticNode, NodeType, PatternType, 
    PatternRecognizer, CrossLanguageAnalyzer
};

fn main() {
    println!("🧠 Testing Semantic Compiler...");
    
    // Create semantic graphs for different languages
    let mut rust_graph = SemanticGraph::new("rust");
    let mut python_graph = SemanticGraph::new("python");
    
    // Add some sample nodes
    let rust_node = SemanticNode {
        id: 1,
        node_type: NodeType::Function,
        children: vec![2, 3],
        metadata: [("name".to_string(), "fibonacci".to_string())].into(),
        language: "rust".to_string(),
        pattern_hash: 12345,
    };
    
    let python_node = SemanticNode {
        id: 1,
        node_type: NodeType::Pattern(PatternType::MapReduce),
        children: vec![],
        metadata: [("operation".to_string(), "map_reduce".to_string())].into(),
        language: "python".to_string(),
        pattern_hash: 54321,
    };
    
    rust_graph.add_node(rust_node);
    python_graph.add_node(python_node);
    
    // Add root nodes
    rust_graph.root_nodes.push(1);
    python_graph.root_nodes.push(1);
    
    // Detect patterns
    rust_graph.detect_patterns();
    python_graph.detect_patterns();
    
    println!("Rust patterns: {:?}", rust_graph.pattern_cache);
    println!("Python patterns: {:?}", python_graph.pattern_cache);
    
    // Test cross-language analysis
    let recognizer = PatternRecognizer::new();
    let cross_patterns = recognizer.recognize_cross_language_patterns(vec![&rust_graph, &python_graph]);
    
    println!("Cross-language patterns found: {}", cross_patterns.len());
    for pattern in &cross_patterns {
        println!("  - {:?} in {}", pattern.pattern_type, pattern.language);
    }
    
    // Test migration suggestions
    let analyzer = CrossLanguageAnalyzer;
    let suggestions = analyzer.suggest_migration_targets(&rust_graph);
    
    println!("Migration suggestions: {}", suggestions.len());
    for suggestion in suggestions {
        println!("  - Move {:?} from {} to {} (estimated gain: {:.1}x)", 
            suggestion.pattern_type, 
            suggestion.current_language, 
            suggestion.suggested_language,
            suggestion.estimated_performance_gain
        );
    }
    
    // Test pattern recognizer features
    println!("Supported languages: {:?}", recognizer.get_supported_languages());
    
    if let Some(best_lang) = recognizer.suggest_optimal_language(PatternType::FibonacciLike) {
        println!("Optimal language for Fibonacci: {}", best_lang);
    }
    
    // Test semantic hashing
    println!("Rust graph semantic hash: {}", rust_graph.calculate_semantic_hash());
    println!("Python graph semantic hash: {}", python_graph.calculate_semantic_hash());
    
    println!("✅ Semantic Compiler test completed!");
}
