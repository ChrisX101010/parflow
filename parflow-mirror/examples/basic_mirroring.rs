use parflow_mirror::{MirroringEngine, LanguageTranslator};

#[tokio::main]
async fn main() {
    println!("🧠 Testing ParFlow Mirroring Engine...");
    
    let engine = MirroringEngine::new();
    let translator = LanguageTranslator;
    
    // Test repository analysis
    match engine.analyze_repository("./example-project").await {
        Ok(analysis) => {
            println!("📊 Repository Analysis:");
            println!("  Languages: {:?}", analysis.languages);
            println!("  Estimated improvement: {:.1}x", analysis.estimated_improvement);
            for suggestion in &analysis.mirroring_suggestions {
                println!("  💡 {} (Effort: {}, Gain: {:.1}x)", 
                    suggestion.description, 
                    suggestion.effort_estimate,
                    suggestion.estimated_performance_gain
                );
            }
        }
        Err(e) => println!("❌ Analysis failed: {}", e),
    }
    
    // Test pattern translation
    println!("\n🔄 Testing Pattern Translation:");
    let rust_fib = translator.translate_pattern(
        semantic_compiler::PatternType::FibonacciLike, 
        "python", 
        "rust"
    );
    println!("Python → Rust Fibonacci:\n{}", rust_fib);
    
    // Test codebase mirroring
    match engine.mirror_codebase("./legacy-python", "rust").await {
        Ok(result) => {
            println!("\n📁 Mirroring Results:");
            println!("  Files: {} → {}", result.original_file_count, result.mirrored_file_count);
            println!("  Performance improvement: {:.1}x", result.performance_improvement);
            for warning in &result.warnings {
                println!("  ⚠️  {}", warning);
            }
        }
        Err(e) => println!("❌ Mirroring failed: {}", e),
    }
    
    println!("✅ Mirroring Engine test completed!");
}
