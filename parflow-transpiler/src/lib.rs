use colored::*;
use std::collections::HashMap;

pub struct CodeTranspiler;

impl CodeTranspiler {
    pub fn python_to_rust(python_code: &str) -> String {
        println!("{}", "🔄 Transpiling Python → Rust".bright_blue().bold());
        
        let mut rust_code = String::from("// Auto-generated Rust code from Python\n");
        rust_code.push_str("fn main() {\n");

        for line in python_code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }

            let rust_line = if trimmed.starts_with("print(") && trimmed.ends_with(')') {
                let content = &trimmed[6..trimmed.len()-1]; // Remove "print(" and ")"
                format!("    println!(\"{{}}\", {});", content)
            } else if trimmed.starts_with("def ") && trimmed.ends_with(':') {
                let func_def = &trimmed[4..trimmed.len()-1]; // Remove "def " and ":"
                format!("    fn {} {{", func_def)
            } else if trimmed.starts_with("for ") && trimmed.contains(" in range(") && trimmed.ends_with(':') {
                // Simple for loop conversion
                let loop_def = &trimmed[4..trimmed.len()-1]; // Remove "for " and ":"
                format!("    for {} {{", loop_def.replace(" in range(", " in "))
            } else if trimmed.starts_with("if ") && trimmed.ends_with(':') {
                let condition = &trimmed[3..trimmed.len()-1]; // Remove "if " and ":"
                format!("    if {} {{", condition)
            } else if trimmed == "else:" {
                "    } else {".to_string()
            } else {
                format!("    {}; // TODO: Manual conversion needed", trimmed)
            };

            rust_code.push_str(&rust_line);
            rust_code.push('\n');
        }

        rust_code.push_str("}\n");
        rust_code
    }

    pub fn rust_to_typescript(rust_code: &str) -> String {
        println!("{}", "🔄 Transpiling Rust → TypeScript".bright_yellow().bold());
        
        let mut ts_code = String::from("// Auto-generated TypeScript code from Rust\n");

        for line in rust_code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }

            let ts_line = if trimmed.starts_with("fn ") && trimmed.ends_with('{') {
                let func_def = &trimmed[3..trimmed.len()-1]; // Remove "fn " and "{"
                format!("function {} {{", func_def)
            } else if trimmed.starts_with("let ") && trimmed.ends_with(';') {
                let var_def = &trimmed[4..trimmed.len()-1]; // Remove "let " and ";"
                format!("let {};", var_def)
            } else if trimmed.starts_with("println!") && trimmed.ends_with(';') {
                let content = &trimmed[9..trimmed.len()-2]; // Remove "println!(\"" and "\");"
                format!("console.log(\"{}\");", content)
            } else if trimmed.starts_with("for ") && trimmed.ends_with('{') {
                let loop_def = &trimmed[4..trimmed.len()-1]; // Remove "for " and "{"
                // Convert Rust range syntax to TypeScript
                if loop_def.contains("..") {
                    let parts: Vec<&str> = loop_def.split(" in ").collect();
                    if parts.len() == 2 {
                        let range_parts: Vec<&str> = parts[1].split("..").collect();
                        if range_parts.len() == 2 {
                            return format!("for (let {} = {}; {} < {}; {}++) {{", 
                                parts[0], range_parts[0], parts[0], range_parts[1], parts[0]);
                        }
                    }
                }
                format!("for {} {{", loop_def)
            } else {
                format!("// {}", trimmed)
            };

            ts_code.push_str(&ts_line);
            ts_code.push('\n');
        }

        ts_code
    }

    pub fn analyze_code_complexity(code: &str, _language: &str) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        
        let lines = code.lines().count();
        let non_empty_lines = code.lines().filter(|l| !l.trim().is_empty()).count();
        let comment_lines = code.lines().filter(|l| {
            let trimmed = l.trim();
            trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*")
        }).count();
        
        metrics.insert("total_lines".to_string(), lines as f64);
        metrics.insert("code_lines".to_string(), (non_empty_lines - comment_lines) as f64);
        metrics.insert("comment_density".to_string(), (comment_lines as f64 / non_empty_lines as f64) * 100.0);
        
        // Simple complexity estimation
        let complexity_indicators = vec!["if", "for", "while", "match", "fn ", "def ", "function "];
        let mut complexity_score = 0;
        
        for indicator in complexity_indicators {
            complexity_score += code.matches(indicator).count();
        }
        
        metrics.insert("complexity_score".to_string(), complexity_score as f64);
        metrics.insert("maintainability_index".to_string(), (171.0 - 5.2 * (complexity_score as f64).ln() - 0.23 * (non_empty_lines as f64) - 16.2 * (comment_lines as f64).ln()).max(0.0));
        
        metrics
    }
}
