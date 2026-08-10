use anyhow::Result;
use notify::{Event, RecursiveMode, Watcher};
use std::{
    fs::OpenOptions,
    path::Path,
    process::Command,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

fn build_wasm() -> Result<()> {
    println!("wasm-pack building...");
    let status = Command::new("wasm-pack")
        .current_dir("crates/game/client")
        .args([
            "build",
            "--target",
            "web",
            "--out-dir",
            "../../../assets/game/pkg",
            "--no-opt",
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("wasm-pack build failed!");
    }
    println!("wasm-pack finished building bevy client");
    Ok(())
}

fn trigger_topcoat_reload() -> Result<()> {
    let main_rs = Path::new("crates/web/src/main.rs");
    match OpenOptions::new().write(true).open(main_rs) {
        Ok(file) => {
            file.set_modified(std::time::SystemTime::now())?;
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            // File does not exist, ignore
        }
        Err(err) => return Err(err.into()),
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(|s| s.as_str()).unwrap_or("dev");

    match command {
        "dev" => {
            build_wasm()?;

            println!("Starting topcoat dev server...");
            let mut topcoat_child = Command::new("topcoat")
                .args(["dev", "-p", "personal-page"])
                .spawn()?;

            let (tx, rx) = channel();
            let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
                if let Ok(event) = res
                    && !event.kind.is_access()
                {
                    let _ = tx.send(());
                }
            })?;

            for watch_path in [
                "crates/game/client/src",
                "crates/game/client/Cargo.toml",
                "crates/game/common/src",
                "crates/game/common/Cargo.toml",
            ] {
                if Path::new(watch_path).exists() {
                    watcher.watch(Path::new(watch_path), RecursiveMode::Recursive)?;
                }
            }

            println!("Watching game source directories for changes...");

            let debounce_duration = Duration::from_millis(300);

            loop {
                if rx.recv().is_err() {
                    break;
                }

                let mut last_event_time = Instant::now();
                loop {
                    match rx.recv_timeout(debounce_duration) {
                        Ok(()) => {
                            last_event_time = Instant::now();
                        }
                        Err(_) => {
                            if last_event_time.elapsed() >= debounce_duration {
                                break;
                            }
                        }
                    }
                }

                while rx.try_recv().is_ok() {}

                println!("Game source modified. Rebuilding WASM...");
                match build_wasm() {
                    Ok(()) => {
                        println!("Triggering Topcoat asset re-bundle...");
                        if let Err(err) = trigger_topcoat_reload() {
                            eprintln!("Failed to trigger topcoat reload: {err}");
                        }
                    }
                    Err(err) => {
                        eprintln!("WASM rebuild error: {err}");
                    }
                }

                // Drain any events that occurred while compilation was running
                while rx.try_recv().is_ok() {}
            }

            let _ = topcoat_child.wait();
        }
        _ => {
            println!("Usage: cargo dev (or cargo run -p xtask -- dev)");
        }
    }

    Ok(())
}
