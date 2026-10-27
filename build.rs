//! Computes the server version at compile time and exposes it as the
//! `APP_VERSION` env var for `env!`.
//!
//! The version is `YYYYMMDD.N.0`: the UTC date of the last commit on main
//! that changed one of `VERSIONED_PATHS`, with N counting the earlier such
//! commits from that same day. It is `0.0.0` until any of those paths has
//! been committed. Setting `APP_VERSION` in the build environment overrides
//! this.

use std::{path::Path, process::Command};

/// Paths whose changes bump the version. Merges that touch nothing here
/// leave `/version` unchanged. Add new courses files (e.g. data) here.
const VERSIONED_PATHS: &[&str] = &["src/routes/courses.rs", "src/routes/courses"];

fn main() {
    println!("cargo:rerun-if-env-changed=APP_VERSION");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
    if Path::new(".git/packed-refs").exists() {
        println!("cargo:rerun-if-changed=.git/packed-refs");
    }

    let version = std::env::var("APP_VERSION")
        .ok()
        // The Dockerfile sets it to "" when no build arg is given.
        .filter(|v| !v.is_empty())
        .or_else(version_from_git)
        .unwrap_or_else(|| {
            println!("cargo:warning=Could not read git history; using version 0.0.0");
            "0.0.0".to_string()
        });

    println!("cargo:rustc-env=APP_VERSION={version}");
}

/// Builds `YYYYMMDD.N.0` from the commits on main's first-parent history
/// that touched `VERSIONED_PATHS`, newest first.
fn version_from_git() -> Option<String> {
    let output = Command::new("git")
        .args([
            "log",
            "--first-parent",
            "--format=%cd",
            "--date=format-local:%Y%m%d",
            "HEAD",
            "--",
        ])
        .args(VERSIONED_PATHS)
        .env("TZ", "UTC")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let mut dates = stdout.lines();
    let Some(date) = dates.next() else {
        // None of the versioned paths has been committed yet.
        return Some("0.0.0".to_string());
    };
    let earlier_same_day = dates.take_while(|d| *d == date).count();

    Some(format!("{date}.{earlier_same_day}.0"))
}
