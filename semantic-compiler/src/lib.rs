use serde::{Deserialize, Serialize};

pub mod cross_language_patterns;
pub mod pattern_recognizer;
pub mod semantic_graph;

pub use cross_language_patterns::{CrossLanguageAnalyzer, MigrationSuggestion, ProjectAnalysis};
pub use pattern_recognizer::PatternRecognizer;
pub use semantic_graph::{NodeType, SemanticGraph, SemanticNode};

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
