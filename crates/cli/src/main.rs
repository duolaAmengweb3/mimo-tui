//! mimo-tui CLI 入口
//!
//! 详细使用文档：https://mimo-tui.pages.dev/docs

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "mimo",
    version,
    about = "MiMo 模型的原创开源终端 AI 编程 agent",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// 启动时跳过欢迎屏
    #[arg(long)]
    no_banner: bool,

    /// 调试日志
    #[arg(long, env = "MIMO_DEBUG")]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// 首次配置向导（API key / 集群 / 模型）
    Init,
    /// 鉴权管理
    Auth {
        #[command(subcommand)]
        cmd: AuthCmd,
    },
    /// 非交互模式：把 prompt 跑一次后退出
    Print {
        prompt: String,
    },
}

#[derive(Subcommand)]
enum AuthCmd {
    Login,
    Logout,
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let level = if cli.debug { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(level)
        .with_target(false)
        .init();

    match cli.command {
        Some(Commands::Init) => {
            todo!("init wizard")
        }
        Some(Commands::Auth { cmd: _ }) => {
            todo!("auth subcommand")
        }
        Some(Commands::Print { prompt: _ }) => {
            todo!("non-interactive print mode")
        }
        None => {
            // 主入口：进入 TUI
            mimo_tui_tui::run().await?;
        }
    }

    Ok(())
}
