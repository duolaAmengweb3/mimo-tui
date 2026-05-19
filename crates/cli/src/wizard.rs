//! First-run setup wizard.

use std::io::Write;

use anyhow::{Context, Result};
use crossterm::style::{Color, Stylize};
use mimo_tui_anthropic_client::{Client, MessagesRequest};
use mimo_tui_core::{
    auth::Auth,
    config::Config,
    paths,
    region::{best_region, measure_all, RegionConfig},
};

pub async fn run() -> Result<()> {
    println!();
    println!(
        "  {}  {}  {}",
        "∞".with(Color::Magenta).bold(),
        "mimo-tui".with(Color::White).bold(),
        "首次配置 · first-run setup".with(Color::DarkGrey),
    );
    println!();

    // Step 1: API key.
    println!("{} {}", "Step 1/3".with(Color::Cyan).bold(), "API Key");
    println!("  去 https://platform.xiaomimimo.com 注册 → 控制台 → API Keys → Create");
    println!("  Sign up at platform.xiaomimimo.com → Console → API Keys → Create");
    println!();

    let api_key = if let Ok(env_key) = std::env::var("MIMO_API_KEY") {
        if !env_key.is_empty() {
            println!("  (using MIMO_API_KEY from environment)");
            env_key
        } else {
            prompt_api_key()?
        }
    } else {
        prompt_api_key()?
    };

    if !api_key.starts_with("tp-") {
        eprintln!(
            "  {} api key doesn't start with 'tp-' — proceeding anyway",
            "warning".with(Color::Yellow)
        );
    }

    // Step 2: Region.
    println!();
    println!(
        "{} {}",
        "Step 2/3".with(Color::Cyan).bold(),
        "Region · 集群"
    );
    println!("  Measuring latency...");
    let measurements = measure_all().await;
    for m in &measurements {
        let ms =
            m.ms.map(|x| format!("{:>4} ms", x))
                .unwrap_or_else(|| "  fail".to_string());
        println!("    {}  ·  {}", m.region.label(), ms);
    }
    let best = best_region(&measurements);
    println!(
        "  {} {} ({})",
        "→ recommended:".with(Color::Green),
        best.label().with(Color::Green).bold(),
        "press Enter to accept, or type cn|sgp|ams".with(Color::DarkGrey),
    );
    let region = read_region(best)?;

    // Step 3: Model.
    println!();
    println!(
        "{} {}",
        "Step 3/3".with(Color::Cyan).bold(),
        "Default model"
    );
    println!("  1. mimo-v2.5-pro    · trillion-MoE 编程主力 (recommended)");
    println!("  2. mimo-v2.5        · lightweight");
    println!("  3. mimo-v2-flash    · fast tier");
    println!("  4. mimo-v2-omni     · multimodal (vision)");
    let model = read_model_choice()?;

    // Validate via a tiny API call.
    println!();
    println!("  validating key against {} ...", region.label());
    let client = Client::new(api_key.clone(), region.to_client_region());
    match client
        .messages(MessagesRequest::new(&model, 20).user("Reply with: OK"))
        .await
    {
        Ok(_) => println!("  {} key works", "✓".with(Color::Green)),
        Err(mimo_tui_anthropic_client::AnthropicError::Unauthorized) => {
            eprintln!(
                "  {} key was rejected (401). save anyway?",
                "✗".with(Color::Red)
            );
            print!("    [y/N] ");
            std::io::stdout().flush()?;
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            if !line.trim().eq_ignore_ascii_case("y") {
                anyhow::bail!("aborted");
            }
        }
        Err(e) => {
            eprintln!("  {} {}", "warning:".with(Color::Yellow), e);
        }
    }

    // Save.
    paths::ensure_layout()?;
    Auth::new(api_key).save().context("save auth")?;
    let mut cfg = Config::load().unwrap_or_default();
    cfg.region = region;
    cfg.model = model;
    cfg.save().context("save config")?;

    println!();
    println!("  {} setup complete", "✓".with(Color::Green).bold(),);
    println!("    key   → {}", paths::auth_file()?.display());
    println!("    config → {}", paths::config_file()?.display());
    println!();
    println!("  run `{}` to start", "mimo".with(Color::Cyan).bold());
    println!();
    Ok(())
}

fn prompt_api_key() -> Result<String> {
    print!("  API key (tp-...): ");
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let key = line.trim().to_string();
    if key.is_empty() {
        anyhow::bail!("empty key");
    }
    Ok(key)
}

fn read_region(default: RegionConfig) -> Result<RegionConfig> {
    print!("  region [{}]: ", default.label().to_lowercase());
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    match line.trim().to_lowercase().as_str() {
        "" => Ok(default),
        "cn" => Ok(RegionConfig::Cn),
        "sgp" => Ok(RegionConfig::Sgp),
        "ams" => Ok(RegionConfig::Ams),
        other => anyhow::bail!("unknown region '{}'", other),
    }
}

fn read_model_choice() -> Result<String> {
    print!("  choose [1-4, default 1]: ");
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    Ok(match line.trim() {
        "" | "1" => "mimo-v2.5-pro",
        "2" => "mimo-v2.5",
        "3" => "mimo-v2-flash",
        "4" => "mimo-v2-omni",
        other => anyhow::bail!("invalid choice '{}'", other),
    }
    .to_string())
}
