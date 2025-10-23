use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use ethers::providers::{Provider, Http, Middleware};
use ethers::types::BlockNumber;
use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled};
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Parser)]
#[command(name = "mordor-cli")]
#[command(about = "CLI tool for monitoring Mordor testnet", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "http://localhost:8545")]
    rpc_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Get current blockchain status
    Status,
    
    /// Get detailed block information
    Block {
        /// Block number (or 'latest')
        #[arg(default_value = "latest")]
        number: String,
    },
    
    /// Monitor blockchain in real-time
    Monitor {
        /// Refresh interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,
    },
    
    /// Get Prometheus metrics
    Metrics {
        /// Service to query (fork-monitor or gas-estimator)
        #[arg(short, long, default_value = "fork-monitor")]
        service: String,
        
        /// Prometheus endpoint
        #[arg(short, long, default_value = "http://localhost:9090")]
        endpoint: String,
    },
    
    /// Check all containers health
    Health,
    
    /// Get gas price recommendations
    Gas,
}

#[derive(Tabled)]
struct StatusRow {
    metric: String,
    value: String,
}

#[derive(Tabled)]
struct BlockInfo {
    field: String,
    value: String,
}

#[derive(Deserialize)]
struct PrometheusResponse {
    status: String,
    data: PrometheusData,
}

#[derive(Deserialize)]
struct PrometheusData {
    #[serde(rename = "resultType")]
    result_type: String,
    result: Vec<PrometheusResult>,
}

#[derive(Deserialize)]
struct PrometheusResult {
    metric: serde_json::Value,
    value: Vec<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            status_command(&cli.rpc_url).await?;
        }
        Commands::Block { number } => {
            block_command(&cli.rpc_url, &number).await?;
        }
        Commands::Monitor { interval } => {
            monitor_command(&cli.rpc_url, interval).await?;
        }
        Commands::Metrics { service, endpoint } => {
            metrics_command(&service, &endpoint).await?;
        }
        Commands::Health => {
            health_command().await?;
        }
        Commands::Gas => {
            gas_command(&cli.rpc_url).await?;
        }
    }

    Ok(())
}

async fn status_command(rpc_url: &str) -> Result<()> {
    println!("{}", "Mordor Testnet Status".bright_blue().bold());
    println!("{}", "=".repeat(50).bright_blue());
    
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    // Get basic info
    let block_number = provider.get_block_number().await?;
    let syncing = provider.syncing().await?;
    let gas_price = provider.get_gas_price().await?;
    let chain_id = provider.get_chainid().await?;
    
    let mut rows = vec![
        StatusRow {
            metric: "Chain ID".to_string(),
            value: chain_id.to_string(),
        },
        StatusRow {
            metric: "Current Block".to_string(),
            value: block_number.to_string(),
        },
        StatusRow {
            metric: "Syncing".to_string(),
            value: if syncing.is_syncing() { 
                "Yes".red().to_string() 
            } else { 
                "No".green().to_string() 
            },
        },
        StatusRow {
            metric: "Gas Price".to_string(),
            value: format!("{} wei ({:.2} Gwei)", gas_price, gas_price.as_u128() as f64 / 1e9),
        },
    ];
    
    // Get latest block
    if let Some(block) = provider.get_block(block_number).await? {
        let timestamp = block.timestamp.as_u64();
        let datetime = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap(),
            Utc
        );
        
        rows.push(StatusRow {
            metric: "Latest Block Time".to_string(),
            value: datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        });
        
        rows.push(StatusRow {
            metric: "Transactions".to_string(),
            value: block.transactions.len().to_string(),
        });
        
        rows.push(StatusRow {
            metric: "Gas Used".to_string(),
            value: format!(
                "{} / {} ({:.2}%)",
                block.gas_used,
                block.gas_limit,
                (block.gas_used.as_u64() as f64 / block.gas_limit.as_u64() as f64) * 100.0
            ),
        });
    }
    
    let table = Table::new(rows).to_string();
    println!("\n{}", table);
    
    Ok(())
}

async fn block_command(rpc_url: &str, number: &str) -> Result<()> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    let block_id = if number == "latest" {
        BlockNumber::Latest
    } else {
        BlockNumber::Number(number.parse::<u64>()?.into())
    };
    
    let block = provider.get_block_with_txs(block_id).await?
        .ok_or_else(|| anyhow::anyhow!("Block not found"))?;
    
    println!("{}", format!("Block #{}", block.number.unwrap()).bright_blue().bold());
    println!("{}", "=".repeat(50).bright_blue());
    
    let timestamp = block.timestamp.as_u64();
    let datetime = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap(),
        Utc
    );
    
    let rows = vec![
        BlockInfo {
            field: "Hash".to_string(),
            value: format!("{:?}", block.hash.unwrap()),
        },
        BlockInfo {
            field: "Parent Hash".to_string(),
            value: format!("{:?}", block.parent_hash),
        },
        BlockInfo {
            field: "Timestamp".to_string(),
            value: datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        },
        BlockInfo {
            field: "Miner".to_string(),
            value: format!("{:?}", block.author.unwrap_or_default()),
        },
        BlockInfo {
            field: "Difficulty".to_string(),
            value: block.difficulty.to_string(),
        },
        BlockInfo {
            field: "Gas Limit".to_string(),
            value: block.gas_limit.to_string(),
        },
        BlockInfo {
            field: "Gas Used".to_string(),
            value: format!(
                "{} ({:.2}%)",
                block.gas_used,
                (block.gas_used.as_u64() as f64 / block.gas_limit.as_u64() as f64) * 100.0
            ),
        },
        BlockInfo {
            field: "Transactions".to_string(),
            value: block.transactions.len().to_string(),
        },
        BlockInfo {
            field: "Size".to_string(),
            value: format!("{} bytes", block.size.unwrap_or_default()),
        },
    ];
    
    let table = Table::new(rows).to_string();
    println!("\n{}", table);
    
    if !block.transactions.is_empty() {
        println!("\n{}", "Transactions:".bright_yellow().bold());
        for (i, tx) in block.transactions.iter().take(10).enumerate() {
            println!(
                "  {}. {} -> {} ({} gas @ {} wei)",
                i + 1,
                format!("{:?}", tx.from).bright_cyan(),
                format!("{:?}", tx.to.unwrap_or_default()).bright_green(),
                tx.gas,
                tx.gas_price.unwrap_or_default()
            );
        }
        if block.transactions.len() > 10 {
            println!("  ... and {} more", block.transactions.len() - 10);
        }
    }
    
    Ok(())
}

async fn monitor_command(rpc_url: &str, interval: u64) -> Result<()> {
    use tokio::time::{sleep, Duration};
    
    println!("{}", "Monitoring Mordor Testnet (Ctrl+C to stop)".bright_blue().bold());
    println!("{}", "=".repeat(70).bright_blue());
    
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let mut last_block = 0u64;
    
    loop {
        let block_number = provider.get_block_number().await?;
        
        if block_number.as_u64() != last_block {
            if let Some(block) = provider.get_block(block_number).await? {
                let timestamp = block.timestamp.as_u64();
                let datetime = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap(),
                    Utc
                );
                
                let block_time = if last_block > 0 {
                    format!("(+{:.1}s)", (timestamp as i64 - last_block as i64).abs())
                } else {
                    String::new()
                };
                
                println!(
                    "{} Block {} {} | Txs: {} | Gas: {}/{} ({:.1}%) | Difficulty: {}",
                    datetime.format("%H:%M:%S").to_string().bright_black(),
                    block_number.to_string().bright_yellow(),
                    block_time.bright_black(),
                    block.transactions.len().to_string().bright_cyan(),
                    block.gas_used.to_string().bright_green(),
                    block.gas_limit,
                    (block.gas_used.as_u64() as f64 / block.gas_limit.as_u64() as f64) * 100.0,
                    block.difficulty
                );
                
                last_block = timestamp;
            }
        }
        
        sleep(Duration::from_secs(interval)).await;
    }
}

async fn metrics_command(service: &str, endpoint: &str) -> Result<()> {
    let port = match service {
        "fork-monitor" => 9090,
        "gas-estimator" => 9091,
        _ => return Err(anyhow::anyhow!("Unknown service. Use 'fork-monitor' or 'gas-estimator'")),
    };
    
    let url = format!("http://localhost:{}/metrics", port);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let text = response.text().await?;
    
    println!("{}", format!("Metrics from {}", service).bright_blue().bold());
    println!("{}", "=".repeat(70).bright_blue());
    
    // Parse and display key metrics
    for line in text.lines() {
        if line.starts_with("etc_mordor_") && !line.starts_with("#") {
            if let Some((metric, value)) = line.split_once(' ') {
                let metric_name = metric
                    .strip_prefix("etc_mordor_")
                    .unwrap_or(metric)
                    .replace('_', " ");
                println!("  {}: {}", metric_name.bright_cyan(), value.bright_yellow());
            }
        }
    }
    
    Ok(())
}

async fn health_command() -> Result<()> {
    println!("{}", "Checking Container Health".bright_blue().bold());
    println!("{}", "=".repeat(50).bright_blue());
    
    let services = vec![
        ("Mordor Node RPC", "http://localhost:8545", "eth_blockNumber"),
        ("Fork Monitor", "http://localhost:9090/health", ""),
        ("Gas Estimator", "http://localhost:9091/health", ""),
        ("Prometheus", "http://localhost:9092/-/healthy", ""),
        ("Grafana", "http://localhost:3000/api/health", ""),
    ];
    
    let client = reqwest::Client::new();
    
    for (name, url, _method) in services {
        print!("  {} ... ", name);
        match client.get(url).timeout(std::time::Duration::from_secs(5)).send().await {
            Ok(response) if response.status().is_success() => {
                println!("{}", "✓ OK".bright_green().bold());
            }
            Ok(response) => {
                println!("{}", format!("✗ ERROR ({})", response.status()).bright_red().bold());
            }
            Err(e) => {
                println!("{}", format!("✗ UNREACHABLE ({})", e).bright_red().bold());
            }
        }
    }
    
    Ok(())
}

async fn gas_command(rpc_url: &str) -> Result<()> {
    println!("{}", "Gas Price Recommendations".bright_blue().bold());
    println!("{}", "=".repeat(50).bright_blue());
    
    // Query gas estimator metrics
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:9091/metrics").send().await?;
    let text = response.text().await?;
    
    let mut metrics = std::collections::HashMap::new();
    
    for line in text.lines() {
        if let Some((metric, value)) = line.split_once(' ') {
            if let Ok(val) = value.parse::<f64>() {
                metrics.insert(metric.to_string(), val);
            }
        }
    }
    
    let slow = metrics.get("etc_mordor_gas_price_min_wei").copied().unwrap_or(0.0);
    let standard = metrics.get("etc_mordor_gas_price_median_wei").copied().unwrap_or(0.0);
    let fast = metrics.get("etc_mordor_gas_price_p75_wei").copied().unwrap_or(0.0);
    let instant = metrics.get("etc_mordor_gas_price_max_wei").copied().unwrap_or(0.0);
    
    let gwei = |wei: f64| wei / 1e9;
    
    println!("\n  {}: {} wei ({:.2} Gwei)", "Slow".bright_yellow(), slow as u64, gwei(slow));
    println!("  {}: {} wei ({:.2} Gwei)", "Standard".bright_cyan(), standard as u64, gwei(standard));
    println!("  {}: {} wei ({:.2} Gwei)", "Fast".bright_green(), fast as u64, gwei(fast));
    println!("  {}: {} wei ({:.2} Gwei)", "Instant".bright_magenta(), instant as u64, gwei(instant));
    
    let utilization = metrics.get("etc_mordor_gas_utilization_percent").copied().unwrap_or(0.0);
    println!("\n  Network Utilization: {:.2}%", utilization);
    
    Ok(())
}
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use ethers::providers::{Provider, Http, Middleware};
use ethers::types::BlockNumber;
use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled};
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Parser)]
#[command(name = "mordor-cli")]
#[command(about = "CLI tool for monitoring Mordor testnet", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "http://localhost:8545")]
    rpc_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Get current blockchain status
    Status,
    
    /// Get detailed block information
    Block {
        /// Block number (or 'latest')
        #[arg(default_value = "latest")]
        number: String,
    },
    
    /// Monitor blockchain in real-time
    Monitor {
        /// Refresh interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,
    },
    
    /// Get Prometheus metrics
    Metrics {
        /// Service to query (fork-monitor or gas-estimator)
        #[arg(short, long, default_value = "fork-monitor")]
        service: String,
        
        /// Prometheus endpoint
        #[arg(short, long, default_value = "http://localhost:9090")]
        endpoint: String,
    },
    
    /// Check all containers health
    Health,
    
    /// Get gas price recommendations
    Gas,
}

#[derive(Tabled)]
struct StatusRow {
    metric: String,
    value: String,
}

#[derive(Tabled)]
struct BlockInfo {
    field: String,
    value: String,
}

#[derive(Deserialize)]
struct PrometheusResponse {
    status: String,
    data: PrometheusData,
}

#[derive(Deserialize)]
struct PrometheusData {
    #[serde(rename = "resultType")]
    result_type: String,
    result: Vec<PrometheusResult>,
}

#[derive(Deserialize)]
struct PrometheusResult {
    metric: serde_json::Value,
    value: Vec<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            status_command(&cli.rpc_url).await?;
        }
        Commands::Block { number } => {
            block_command(&cli.rpc_url, &number).await?;
        }
        Commands::Monitor { interval } => {
            monitor_command(&cli.rpc_url, interval).await?;
        }
        Commands::Metrics { service, endpoint } => {
            metrics_command(&service, &endpoint).await?;
        }
        Commands::Health => {
            health_command().await?;
        }
        Commands::Gas => {
            gas_command(&cli.rpc_url).await?;
        }
    }

    Ok(())
}

async fn status_command(rpc_url: &str) -> Result<()> {
    println!("{}", "Mordor Testnet Status".bright_blue().bold());
    println!("{}", "=".repeat(50).bright_blue());
    
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    // Get basic info
    let block_number = provider.get_block_number().await?;
    let syncing = provider.syncing().await?;
    let gas_price = provider.get_gas_price().await?;
block_command(rpc_url: &str, number: &str) -> Result<()> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    let block_id = if number == "latest" {
        BlockNumber::Latest
    } else {
        BlockNumber::Number(number.parse::<u64>()?.into())
    };
    
    let block = provider.get_block_with_txs(block_id).await?
        .ok_or_else(|| anyhow::anyhow!("Block not found"))?;
    
    println!("{}", format!("Block #{}", block.number.unwrap()).bright_blue().bold());
    println!("{}", "=".repeat(50).bright_blue());
    
    let timestamp = block.timestamp.as_u64();
    let datetime = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap(),
        Utc
    );
    
    let rows = vec![
        BlockInfo {
            field: "Hash".to_string(),
            value: format!("{:?}", block.hash.unwrap()),
        },
        BlockInfo {
            field: "Parent Hash".to_string(),
            value: format!("{:?}", block.parent_hash),
        },
        BlockInfo {
            field: "Timestamp".to_string(),
            value: datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        },
        BlockInfo {
            field: "Miner".to_string(),
            value: format!("{:?}", block.author.unwrap_or_default()),
        },
        BlockInfo {
            field: "Difficulty".to_string(),
            value: block.difficulty.to_string(),
        },
        BlockInfo {
            field: "Gas Limit".to_string(),
            value: block.gas_limit.to_string(),
        },
        BlockInfo {
            field: "Gas Used".to_string(),
            value: format!(
                "{} ({:.2}%)",
                block.gas_used,
                (block.gas_used.as_u64() as f64 / block.gas_limit.as_u64() as f64) * 100.0
            ),
        },
        BlockInfo {
            field: "Transactions".to_string(),
            value: block.transactions.len().to_string(),
        },
        BlockInfo {
            field: "Size".to_string(),
            value: format!("{} bytes", block.size.unwrap_or_default()),
        },
    ];
    
    let table = Table::new(rows).to_string();
    println!("\n{}", table);
    
    if !block.transactions.is_empty() {
        println!("\n{}", "Transactions:".bright_yellow().bold());
        for (i, tx) in block.transactions.iter().take(10).enumerate() {
            println!(
                "  {}. {} -> {} ({} gas @ {} wei)",
                i + 1,
                format!("{:?}", tx.from).bright_cyan(),
                format!("{:?}", tx.to.unwrap_or_default()).bright_green(),
                tx.gas,
                tx.gas_price.unwrap_or_default()
            );
        }
        if block.transactions.len() > 10 {
            println!("  ... and {} more", block.transactions.len() - 10);
        }
    }
    
    Ok(())
}

async fn monitor_command(rpc_url: &str, interval: u64) -> Result<()> {
    use tokio::time::{sleep, Duration};
    
    println!("{}", "Monitoring Mordor Testnet (Ctrl+C to stop)".bright_blue().bold());
    println!("{}", "=".repeat(70).bright_blue());
    
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let mut last_block = 0u64;
    
    loop {
        let block_number = provider.get_block_number().await?;
        
        if block_number.as_u64() != last_block {
            if let Some(block) = provider.get_block(block_number).await? {
                let timestamp = block.timestamp.as_u64();
                let datetime = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap(),
                    Utc
                );
                
                let block_time = if last_block > 0 {
                    format!("(+{:.1}s)", (timestamp as i64 - last_block as i64).abs())
                } else {
                    String::new()
                };
                
                println!(
                    "{} Block {} {} | Txs: {} | Gas: {}/{} ({:.1}%) | Difficulty: {}",
                    datetime.format("%H:%M:%S").to_string().bright_black(),
                    block_number.to_string().bright_yellow(),
                    block_time.bright_black(),
                    block.transactions.len().to_string().bright_cyan(),
                    block.gas_used.to_string().bright_green(),
                    block.gas_limit,
                    (block.gas_used.as_u64() as f64 / block.gas_limit.as_u64() as f64) * 100.0,
                    block.difficulty
                );
                
                last_block = timestamp;
            }
        }
        
        sleep(Duration::from_secs(interval)).await;
    }
}

async fn metrics_command(service: &str, endpoint: &str) -> Result<()> {
    let port = match service {
        "fork-monitor" => 9090,
        "gas-estimator" => 9091,
        _ => return Err(anyhow::anyhow!("Unknown service. Use 'fork-monitor' or 'gas-estimator'")),
    };
    
    let url = format!("http://localhost:{}/metrics", port);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let text = response.text().await?;
    
    println!("{}", format!("Metrics from {}", service).bright_blue().bold());
    println!("{}", "=".repeat(70).bright_blue());
    
    // Parse and display key metrics
    for line in text.lines() {
        if line.starts_with("etc_mordor_") && !line.starts_with("#") {
            if let Some((metric, value)) = line.split_once(' ') {
                let metric_name = metric
                    .strip_prefix("etc_mordor_")
                    .unwrap_or(metric)
                    .replace('_', " ");
                println!("  {}: {}", metric_name.bright_cyan(), value.bright_yellow());
            }
        }
    }
    
    Ok(())
}

async fn health_command() -> Result<()> {
    println!("{}", "Checking Container Health".bright_blue().bold());
    println!("{}", "=".repeat(50).bright_blue());
    
    let services = vec![
        ("Mordor Node RPC", "http://localhost:8545", "eth_blockNumber"),
        ("Fork Monitor", "http://localhost:9090/health", ""),
        ("Gas Estimator", "http://localhost:9091/health", ""),
        ("Prometheus", "http://localhost:9092/-/healthy", ""),
        ("Grafana", "http://localhost:3000/api/health", ""),
    ];
    
    let client = reqwest::Client::new();
    
    for (name, url, _method) in services {
        print!("  {} ... ", name);
        match client.get(url).timeout(std::time::Duration::from_secs(5)).send().await {
            Ok(response) if response.status().is_success() => {
                println!("{}", "✓ OK".bright_green().bold());
            }
            Ok(response) => {
                println!("{}", format!("✗ ERROR ({})", response.status()).bright_red().bold());
            }
            Err(e) => {
                println!("{}", format!("✗ UNREACHABLE ({})", e).bright_red().bold());
            }
        }
    }
    
    Ok(())
}

async fn gas_command(rpc_url: &str) -> Result<()> {
    println!("{}", "Gas Price Recommendations".bright_blue().bold());
    println!("{}", "=".repeat(50).bright_blue());
    
    // Query gas estimator metrics
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:9091/metrics").send().await?;
    let text = response.text().await?;
    
    let mut metrics = std::collections::HashMap::new();
    
    for line in text.lines() {
        if let Some((metric, value)) = line.split_once(' ') {
            if let Ok(val) = value.parse::<f64>() {
                metrics.insert(metric.to_string(), val);
            }
        }
    }
    
    let slow = metrics.get("etc_mordor_gas_price_min_wei").copied().unwrap_or(0.0);
    let standard = metrics.get("etc_mordor_gas_price_median_wei").copied().unwrap_or(0.0);
    let fast = metrics.get("etc_mordor_gas_price_p75_wei").copied().unwrap_or(0.0);
    let instant = metrics.get("etc_mordor_gas_price_max_wei").copied().unwrap_or(0.0);
    
    let gwei = |wei: f64| wei / 1e9;
    
    println!("\n  {}: {} wei ({:.2} Gwei)", "Slow".bright_yellow(), slow as u64, gwei(slow));
    println!("  {}: {} wei ({:.2} Gwei)", "Standard".bright_cyan(), standard as u64, gwei(standard));
    println!("  {}: {} wei ({:.2} Gwei)", "Fast".bright_green(), fast as u64, gwei(fast));
    println!("  {}: {} wei ({:.2} Gwei)", "Instant".bright_magenta(), instant as u64, gwei(instant));
    
    let utilization = metrics.get("etc_mordor_gas_utilization_percent").copied().unwrap_or(0.0);
    println!("\n  Network Utilization: {:.2}%", utilization);
    
    Ok(())
}
