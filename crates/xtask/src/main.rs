use anyhow::Result;
use notify::{Event, RecursiveMode, Watcher};
use std::{
    path::Path,
    process::Command,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

fn build_wasm() -> Result<()> {
    println!("🚀 Building WASM game client...");
    let status = Command::new("wasm-pack")
        .current_dir("crates/game/client")
        .args([
            "build",
            "--target",
            "web",
            "--out-dir",
            "../../../assets/game",
            "--no-opt",
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("wasm-pack build failed");
    }
    println!("✅ WASM game client updated successfully.");
    Ok(())
}

fn trigger_topcoat_reload() -> Result<()> {
    let main_rs = Path::new("crates/web/src/main.rs");
    if main_rs.exists() {
        let file = std::fs::File::open(main_rs)?;
        file.set_modified(std::time::SystemTime::now())?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(|s| s.as_str()).unwrap_or("dev");

    match command {
        "dev" => {
            build_wasm()?;

            println!("⚡ Starting topcoat dev server...");
            let mut topcoat_child = Command::new("topcoat")
                .args(["dev", "-p", "personal-page"])
                .spawn()?;

            let (tx, rx) = channel();
            let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
                if let Ok(event) = res
                    && (event.kind.is_modify() || event.kind.is_create()) {
                        let _ = tx.send(());
                    }
            })?;

            for watch_path in ["crates/game/client", "crates/game/common"] {
                if Path::new(watch_path).exists() {
                    watcher.watch(Path::new(watch_path), RecursiveMode::Recursive)?;
                }
            }

            println!("👀 Watching crates/game/client and crates/game/common for changes...");

            let mut last_build = Instant::now();
            let debounce_duration = Duration::from_millis(500);

            while let Ok(()) = rx.recv() {
                // Drain any pending duplicate events
                while rx.try_recv().is_ok() {}

                if last_build.elapsed() >= debounce_duration {
                    last_build = Instant::now();
                    println!("🔄 Game source modified. Rebuilding WASM...");
                    if let Err(err) = build_wasm() {
                        eprintln!("❌ WASM rebuild error: {err}");
                    } else if let Err(err) = trigger_topcoat_reload() {
                        eprintln!("❌ Failed to trigger topcoat reload: {err}");
                    }
                }
            }

            let _ = topcoat_child.wait();
        }
        _ => {
            println!("Usage: cargo dev (or cargo run -p xtask -- dev)");
        }
    }

    Ok(())
}
