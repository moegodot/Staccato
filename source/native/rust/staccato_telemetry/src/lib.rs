use std::path::Path;
use tracing::Level;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, prelude::*, Registry};

/// 注意：返回的 WorkerGuard 必须在 main 函数中保留，否则日志会在后台线程停止前被丢弃
pub struct TelemetryGuard {
    _file_guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}

pub fn initialize(log_directory: Option<&Path>) -> eyre::Result<TelemetryGuard> {
    // 1. 将传统的 log 记录重定向到 tracing
    LogTracer::init()?;

    // 2. 创建控制台输出层
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(true)
        .with_thread_ids(true)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO); // 控制台只看 INFO 以上

    // 3. 创建文件输出层（如果有目录）
    let mut file_guard = None;
    let file_layer = if let Some(dir) = log_directory {
        let file_appender = tracing_appender::rolling::hourly(dir, "staccato.log");

        // 使用非阻塞写入，防止磁盘 I/O 阻塞游戏主线程
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        file_guard = Some(guard);

        let layer = fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false) // 文件通常不需要 ANSI 颜色字符
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE); // 文件记录最详细的 TRACE

        Some(layer)
    } else {
        None
    };

    // 4. 将所有层注册到 Registry 并初始化
    // Registry::default() 是所有层的容器
    Registry::default()
        .with(console_layer)   // 添加控制台层
        .with(file_layer)      // 添加文件层 (Option 实现了 Layer 特性)
        .init();               // 正式设为全局默认 Subscriber

    Ok(TelemetryGuard { _file_guard: file_guard })
}