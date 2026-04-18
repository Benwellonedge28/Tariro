//! A basic Command-Line Interface (CLI) for interacting with the S.O.R.A.E. (Self-Optimizing Resource Allocation Engine).
//! This binary allows users or other systems to submit resource requests and observe the AGI's resource management behavior.

use clap::{Parser, Subcommand};
use anyhow::Result;
use uuid::Uuid;
use std::collections::HashMap;

use tariro_ide_idpte::agi_core::AgiCoreEngine;
use tariro_ide_idpte::resource_management::resource_models::{ResourceType, ResourceRequest, AllocationPolicy};

#[derive(Parser, Debug)]
#[command(author, version, about = "S.O.R.A.E. CLI - Interact with Tariro's Resource Allocation Engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Request CPU resources
    RequestCpu {
        #[arg(short, long, help = "Amount of CPU cores/units requested")]
        amount: f64,
        #[arg(short, long, default_value_t = 100, help = "Urgency of the request (0-255, higher is more urgent)")]
        urgency: u8,
        #[arg(long, help = "Minimum CPU required")]
        min_amount: Option<f64>,
        #[arg(long, help = "Task ID to associate with this request")]
        task_id: Option<String>,
    },
    /// Request Memory resources
    RequestMemory {
        #[arg(short, long, help = "Amount of Memory requested in GB")]
        amount: f64,
        #[arg(short, long, default_value_t = 100, help = "Urgency of the request (0-255, higher is more urgent)")]
        urgency: u8,
        #[arg(long, help = "Minimum Memory required in GB")]
        min_amount: Option<f64>,
        #[arg(long, help = "Task ID to associate with this request")]
        task_id: Option<String>,
    },
    /// Request GPU resources
    RequestGpu {
        #[arg(short, long, help = "Amount of GPU units requested (e.g., 0.5 for half a GPU)")]
        amount: f64,
        #[arg(short, long, default_value_t = 100, help = "Urgency of the request (0-255, higher is more urgent)")]
        urgency: u8,
        #[arg(long, help = "Minimum GPU required")]
        min_amount: Option<f64>,
        #[arg(long, help = "Task ID to associate with this request")]
        task_id: Option<String>,
    },
    // Add commands for other resource types like NetworkIo, Storage, AgiComputeUnit
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init(); // Initialize logging

    let agi_core = AgiCoreEngine::new().await?;
    let cli = Cli::parse();

    let mut requested_resources = HashMap::new();
    let mut min_required_resources = HashMap::new();
    let mut policies = vec![];
    let mut urgency = 0;
    let task_id = Uuid::new_v4().to_string();

    match &cli.command {
        Commands::RequestCpu { amount, urgency: u, min_amount, task_id: tid } => {
            requested_resources.insert(ResourceType::Cpu, *amount);
            if let Some(min) = min_amount { min_required_resources.insert(ResourceType::Cpu, *min); }
            urgency = *u;
            if let Some(id) = tid { task_id.clone_from(id); }
            println!("CLI: Requesting {} CPU for task {}", amount, task_id);
        },
        Commands::RequestMemory { amount, urgency: u, min_amount, task_id: tid } => {
            requested_resources.insert(ResourceType::Memory, *amount);
            if let Some(min) = min_amount { min_required_resources.insert(ResourceType::Memory, *min); }
            urgency = *u;
            if let Some(id) = tid { task_id.clone_from(id); }
            println!("CLI: Requesting {} GB Memory for task {}", amount, task_id);
        },
        Commands::RequestGpu { amount, urgency: u, min_amount, task_id: tid } => {
            requested_resources.insert(ResourceType::Gpu, *amount);
            if let Some(min) = min_amount { min_required_resources.insert(ResourceType::Gpu, *min); }
            urgency = *u;
            if let Some(id) = tid { task_id.clone_from(id); }
            println!("CLI: Requesting {} GPU units for task {}", amount, task_id);
        },
    }

    let resource_request = ResourceRequest {
        task_id,
        requested_resources,
        min_required_resources,
        policies,
        urgency,
    };

    // Send the resource request to the S.O.R.A.E. orchestrator
    agi_core.sorae_request_sender.send(resource_request).await?;

    println!("Resource request sent to S.O.R.A.E. Orchestrator.");
    println!("S.O.R.A.E. will continue to run in the background. Check AGI logs for allocation results.");

    // The main function needs to keep running to allow the background S.O.R.A.E. task to operate
    // In a real CLI, you might want to wait for a specific response or run indefinitely
    // For now, we'll keep it alive briefly for demonstration purposes
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    Ok(())
}
