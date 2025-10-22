use serde::{Deserialize, Serialize};
use colored::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LanguageTask {
    pub language: String,
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<String>,
    pub timeout_seconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiLanguageWorkflow {
    pub name: String,
    pub tasks: Vec<LanguageTask>,
    pub concurrent: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub task_name: String,
    pub language: String,
    pub success: bool,
    pub output: String,
    pub execution_time: u128,
    pub exit_code: Option<i32>,
}

pub struct MultiLanguageOrchestrator;

impl MultiLanguageOrchestrator {
    pub async fn execute_workflow(workflow: MultiLanguageWorkflow) -> Vec<ExecutionResult> {
        println!("{} {}", "🚀 Executing Multi-Language Workflow:".bright_green().bold(), workflow.name.bright_cyan());
        
        let mut results = Vec::new();

        if workflow.concurrent {
            // Execute all tasks concurrently (mock implementation)
            let mut handles = Vec::new();
            
            for task in workflow.tasks {
                let handle = tokio::spawn(async move {
                    Self::execute_task_mock(task).await
                });
                handles.push(handle);
            }

            for handle in handles {
                if let Ok(result) = handle.await {
                    results.push(result);
                }
            }
        } else {
            // Execute tasks sequentially (mock implementation)
            for task in workflow.tasks {
                let result = Self::execute_task_mock(task).await;
                results.push(result);
            }
        }

        Self::generate_workflow_insights(&results);
        results
    }

    async fn execute_task_mock(task: LanguageTask) -> ExecutionResult {
        let language = task.language.clone(); // Clone for use in output
        println!("{} {} {}", "▶️  Executing".bright_blue(), language.bright_yellow(), "task".bright_blue());
        
        // Mock execution - simulate some work
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Create output before moving language
        let output = format!("Mock output from {} task", language);
        let task_name = format!("{}_task", language);

        ExecutionResult {
            task_name,
            language,
            success: true,
            output,
            execution_time: 500, // milliseconds
            exit_code: Some(0),
        }
    }

    pub async fn compile_multiple_languages(projects: Vec<&str>) -> HashMap<String, ExecutionResult> {
        println!("{}", "🔨 Concurrent Multi-Language Compilation".bright_magenta().bold());
        
        let mut compilation_tasks = Vec::new();

        for project in projects {
            if project.ends_with(".rs") || project.contains("Cargo.toml") {
                compilation_tasks.push(LanguageTask {
                    language: "Rust".to_string(),
                    command: "cargo".to_string(),
                    args: vec!["build", "--release"].into_iter().map(String::from).collect(),
                    working_dir: Some(".".to_string()),
                    timeout_seconds: Some(300),
                });
            } else if project.ends_with(".py") {
                compilation_tasks.push(LanguageTask {
                    language: "Python".to_string(),
                    command: "python".to_string(),
                    args: vec!["-m", "py_compile", project].into_iter().map(String::from).collect(),
                    working_dir: None,
                    timeout_seconds: Some(30),
                });
            } else if project.contains("package.json") {
                compilation_tasks.push(LanguageTask {
                    language: "Node.js".to_string(),
                    command: "npm".to_string(),
                    args: vec!["run", "build"].into_iter().map(String::from).collect(),
                    working_dir: Some(".".to_string()),
                    timeout_seconds: Some(120),
                });
            }
        }

        let workflow = MultiLanguageWorkflow {
            name: "Multi-Language Build".to_string(),
            tasks: compilation_tasks,
            concurrent: true,
        };

        let results = Self::execute_workflow(workflow).await;
        
        let mut result_map = HashMap::new();
        for result in results {
            result_map.insert(result.language.clone(), result);
        }

        result_map
    }

    fn generate_workflow_insights(results: &[ExecutionResult]) {
        println!("\n{}", "📊 Workflow Insights".bright_cyan().bold());
        println!("{}", "─".repeat(40).bright_cyan());

        let total_time: u128 = results.iter().map(|r| r.execution_time).sum();
        let successful_tasks: Vec<&ExecutionResult> = results.iter().filter(|r| r.success).collect();
        let failed_tasks: Vec<&ExecutionResult> = results.iter().filter(|r| !r.success).collect();

        println!("✅ Successful tasks: {}", successful_tasks.len().to_string().bright_green());
        println!("❌ Failed tasks: {}", failed_tasks.len().to_string().bright_red());
        println!("⏱️  Total execution time: {}ms", total_time.to_string().bright_yellow());

        // Find fastest and slowest tasks
        if let Some(fastest) = results.iter().min_by_key(|r| r.execution_time) {
            println!("⚡ Fastest: {} ({}ms)", fastest.language.bright_green(), fastest.execution_time.to_string().bright_green());
        }

        if let Some(slowest) = results.iter().max_by_key(|r| r.execution_time) {
            println!("🐌 Slowest: {} ({}ms)", slowest.language.bright_red(), slowest.execution_time.to_string().bright_red());
        }

        // Language-specific insights
        let mut language_stats: HashMap<&str, (u128, usize)> = HashMap::new();
        for result in results {
            let entry = language_stats.entry(&result.language).or_insert((0, 0));
            entry.0 += result.execution_time;
            entry.1 += 1;
        }

        println!("\n{}", "🌐 Language Performance".bright_blue().bold());
        for (lang, (total_time, count)) in language_stats {
            let avg_time = total_time / count as u128;
            println!("   {}: avg {}ms ({} tasks)", lang.bright_yellow(), avg_time.to_string().bright_white(), count.to_string().bright_white());
        }
    }
}
