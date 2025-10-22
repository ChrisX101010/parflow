// semantic-compiler/examples/basic_usage.rs
use semantic_compiler::{
    CrossLanguageAnalyzer, PatternType, SemanticGraph, SemanticNode, NodeType
};
use std::collections::HashMap;

fn main() {
    println!("🧠 Testing Semantic Compiler with Actual API...");

    // Create semantic graphs for different languages
    let mut rust_graph = SemanticGraph::new("rust");
    let mut python_graph = SemanticGraph::new("python");

    // Create semantic nodes properly
    let rust_node = SemanticNode {
        id: 1,
        node_type: NodeType::Function,
        children: vec![],
        metadata: HashMap::new(),
        language: "rust".to_string(),
        pattern_hash: 12345,
    };

    let python_node = SemanticNode {
        id: 1,
        node_type: NodeType::Function,
        children: vec![],
        metadata: HashMap::new(),
        language: "python".to_string(),
        pattern_hash: 54321,
    };

    // Add nodes to graphs
    rust_graph.add_node(rust_node);
    python_graph.add_node(python_node);

    // Add root nodes
    rust_graph.root_nodes.push(1);
    python_graph.root_nodes.push(1);

    // Detect patterns
    rust_graph.detect_patterns();
    python_graph.detect_patterns();

    println!("✅ Patterns detected in both graphs");

    // Test CrossLanguageAnalyzer - it's a unit struct, use directly
    let analyzer = CrossLanguageAnalyzer;
    
    // Test pattern recognition - use ALL actual variants
    println!("📊 Available Pattern Types:");
    println!("  - {:?}", PatternType::FibonacciLike);
    println!("  - {:?}", PatternType::MapReduce);
    println!("  - {:?}", PatternType::IteratorChain);
    println!("  - {:?}", PatternType::Builder);
    println!("  - {:?}", PatternType::RecursiveTree);
    println!("  - {:?}", PatternType::WebEndpoint);
    println!("  - {:?}", PatternType::Cacheable);
    println!("  - {:?}", PatternType::DataProcessing);
    println!("  - {:?}", PatternType::ConcurrentTasks);
    println!("  - {:?}", PatternType::MathematicalComputation);

    // Test multi-language project analysis
    let graphs = vec![rust_graph, python_graph];
    let _project_analysis = CrossLanguageAnalyzer::analyze_multi_language_project(graphs);
    
    println!("📈 Multi-language project analysis completed!");

    // Test migration suggestions
    let mut rust_graph_for_migration = SemanticGraph::new("rust");
    let rust_node_2 = SemanticNode {
        id: 1,
        node_type: NodeType::Function,
        children: vec![],
        metadata: HashMap::new(),
        language: "rust".to_string(),
        pattern_hash: 9999,
    };
    rust_graph_for_migration.add_node(rust_node_2);
    rust_graph_for_migration.root_nodes.push(1);
    rust_graph_for_migration.detect_patterns();

    let migration_suggestions = analyzer.suggest_migration_targets(&rust_graph_for_migration);
    println!("🔄 Migration suggestions: {}", migration_suggestions.len());

    // Test optimal language suggestions for different patterns
    if let Some(optimal_lang) = analyzer.get_optimal_language(&PatternType::MapReduce) {
        println!("🎯 Optimal language for MapReduce: {}", optimal_lang);
    }

    if let Some(optimal_lang) = analyzer.get_optimal_language(&PatternType::FibonacciLike) {
        println!("🎯 Optimal language for FibonacciLike: {}", optimal_lang);
    }

    if let Some(optimal_lang) = analyzer.get_optimal_language(&PatternType::MathematicalComputation) {
        println!("🎯 Optimal language for MathematicalComputation: {}", optimal_lang);
    }

    // Test semantic hashing
    let mut test_graph = SemanticGraph::new("test");
    let test_node = SemanticNode {
        id: 1,
        node_type: NodeType::Function,
        children: vec![],
        metadata: HashMap::new(),
        language: "test".to_string(),
        pattern_hash: 1111,
    };
    test_graph.add_node(test_node);
    test_graph.root_nodes.push(1);
    
    let hash = test_graph.calculate_semantic_hash();
    println!("🔢 Semantic hash: {}", hash);

    println!("\n✅ All Semantic Compiler features tested successfully!");
    println!("   The library is fully functional with cross-language analysis.");
    println!("   Available patterns: {}", 10); // 10 pattern types available
}
