use anyhow::Result;
use client::Client;
use feature_flags::FeatureFlagAppExt;
use gpui::{App, AppContext};
use smol::stream::StreamExt;
use std::sync::Arc;
use util::ResultExt;

mod hang_detection;

pub fn init(client: Arc<Client>, cx: &mut App) {
    hang_detection::start(client.clone(), cx);

    cx.on_flags_ready({
        move |flags_ready, cx| {
            if flags_ready.is_staff {
                let client = client.clone();
                cx.background_spawn(async move {
                    upload_build_timings(client).await.warn_on_err();
                })
                .detach();
            }
        }
    })
    .detach();
}

#[allow(dead_code)]
pub async fn upload_previous_minidumps(_client: Arc<Client>) -> anyhow::Result<()> {
    Ok(())
}

#[allow(dead_code)]
async fn upload_minidump(_client: Arc<Client>, _endpoint: &str, _minidump: Vec<u8>, _metadata: &str) -> Result<()> {
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct BuildTiming {
    started_at: chrono::DateTime<chrono::Utc>,
    duration_ms: f32,
    first_crate: String,
    target: String,
    blocked_ms: f32,
    command: String,
}

async fn upload_build_timings(_client: Arc<Client>) -> Result<()> {
    let build_timings_dir = paths::data_dir().join("build_timings");

    if !build_timings_dir.exists() {
        return Ok(());
    }

    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    use sysinfo::{MemoryRefreshKind, RefreshKind, System};
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
    );
    let ram_size_gb = (system.total_memory() as f64) / (1024.0 * 1024.0 * 1024.0);

    let mut entries = smol::fs::read_dir(&build_timings_dir).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();

        if path.extension() != Some(std::ffi::OsStr::new("json")) {
            continue;
        }

        let contents = match smol::fs::read_to_string(&path).await {
            Ok(contents) => contents,
            Err(err) => {
                log::warn!("Failed to read build timing file {:?}: {}", path, err);
                continue;
            }
        };

        let timing: BuildTiming = match serde_json::from_str(&contents) {
            Ok(timing) => timing,
            Err(err) => {
                log::warn!("Failed to parse build timing file {:?}: {}", path, err);
                continue;
            }
        };

        telemetry::event!(
            "Build Timing: Cargo Build",
            started_at = timing.started_at.to_rfc3339(),
            duration_ms = timing.duration_ms,
            first_crate = timing.first_crate,
            target = timing.target,
            blocked_ms = timing.blocked_ms,
            command = timing.command,
            cpu_count = cpu_count,
            ram_size_gb = ram_size_gb
        );

        if let Err(err) = smol::fs::remove_file(&path).await {
            log::warn!("Failed to delete build timing file {:?}: {}", path, err);
        }
    }

    Ok(())
}
