use serde::{Deserialize, Serialize};

pub mod semantic_graph;
pub mod pattern_recognizer;
pub mod cross_language_patterns;

pub use semantic_graph::{SemanticGraph, SemanticNode, NodeType};
pub use pattern_recognizer::PatternRecognizer;
pub use cross_language_patterns::{CrossLanguageAnalyzer, ProjectAnalysis, MigrationSuggestion};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
    FibonacciLike,
    MapReduce,
    IteratorChain,
    Builder,
    RecursiveTree,
    WebEndpoint,
    Cacheable,
    DataProcessing,
    ConcurrentTasks,
    MathematicalComputation,
    StringProcessing,
    FileIO,
    NetworkRequest,
    DatabaseQuery,
    DataProcessor,
}
