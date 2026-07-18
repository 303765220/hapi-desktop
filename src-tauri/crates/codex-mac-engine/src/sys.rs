//! Thin macOS IO helpers for the read-only slice.
//!
//! NOTE: network fetch currently shells out to `curl` and version reading to
//! `PlistBuddy`. These are placeholders for the scaffold — the production
//! Tauri backend will inject a proper HTTP client adapter and may read the
//! plist with the `plist` crate. Keeping IO behind these functions means the
//! pure parsing/planning logic stays trivially testable.

use std::path::Path;
use std::process::Command;

use crate::limits::MAX_TEXT_BYTES;
use crate::network::NetworkConfig;
use crate::EngineError;

const CURL: &str = "/usr/bin/curl";
const LIPO: &str = "/usr/bin/lipo";
// Codex 系列 App 的稳定身份锚点。当前值来自 OpenAI macOS bundle；
// 改成仅按名称判断会把 ChatGPT Classic 误识别为 Codex，删除则会让
// ChatGPT.app 改名后的检测失去可信依据。用 identity_gate_* 单测锁定。
pub const CODEX_BUNDLE_ID: &str = "com.openai.codex";
// Codex 改名/合并前后可能出现的 bundle 名称。Codex.app 放前面是为了
// 同一安装根下优先保留旧安装器的显式命名；删掉 ChatGPT.app 会漏检新形态，
// 增加其他名称前必须先验证 bundle id 和官方包形态。
const CANDIDATE_BUNDLE_NAMES: [&str; 2] = ["Codex.app", "ChatGPT.app"];

fn text_from_curl(url: &str, output: std::process::Output) -> Result<String, EngineError> {
    if !output.status.success() {
        // Keep the exit code in the message so the app-layer classifier can tell
        // a connect / timeout / TLS failure apart (mirrors the Windows engine).
        return Err(EngineError::Io(format!(
            "curl failed for {url} exit={}: stderr='{}'",
            output
                .status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "signal".to_string()),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    if output.stdout.len() > MAX_TEXT_BYTES as usize {
        return Err(EngineError::Io(format!(
            "text response exceeded {MAX_TEXT_BYTES} bytes"
        )));
    }
    String::from_utf8(output.stdout).map_err(|e| EngineError::Io(e.to_string()))
}

/// Fetch a small text resource (the appcast) over HTTPS via system `curl`.
pub fn fetch_text(url: &str) -> Result<String, EngineError> {
    fetch_text_with_network(url, &NetworkConfig::system())
}

pub fn fetch_text_with_network(url: &str, network: &NetworkConfig) -> Result<String, EngineError> {
    let max_text = MAX_TEXT_BYTES.to_string();
    let mut command = Command::new(CURL);
    network.apply_to_command(&mut command);
    let output = command
        .args([
            "-fsSL",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "--connect-timeout",
            "20",
            "--max-time",
            "60",
            "--max-filesize",
            &max_text,
            url,
        ])
        .output()
        .map_err(|e| EngineError::Io(format!("spawn curl: {e}")))?;

    text_from_curl(url, output)
}

/// Like `fetch_text` but with a caller-set total timeout. Used to probe a
/// possibly-unreachable source (e.g. OpenAI's official appcast for users behind
/// a block) without stalling on the default long connect timeout.
pub fn fetch_text_timeout(url: &str, max_secs: u64) -> Result<String, EngineError> {
    fetch_text_timeout_with_network(url, max_secs, &NetworkConfig::system())
}

pub fn fetch_text_timeout_with_network(
    url: &str,
    max_secs: u64,
    network: &NetworkConfig,
) -> Result<String, EngineError> {
    let max_text = MAX_TEXT_BYTES.to_string();
    let mut command = Command::new(CURL);
    network.apply_to_command(&mut command);
    let output = command
        .args([
            "-fsSL",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "--connect-timeout",
            "5",
            "--max-time",
            &max_secs.to_string(),
            "--max-filesize",
            &max_text,
            url,
        ])
        .output()
        .map_err(|e| EngineError::Io(format!("spawn curl: {e}")))?;

    text_from_curl(url, output)
}

/// Locate an installed Codex-lineage app and read its `CFBundleVersion` (build number).
///
/// Returns `(app_path, build)` for the first candidate found, or `None`.
pub fn installed_codex_build() -> Option<(String, u64)> {
    candidate_app_paths()
        .into_iter()
        .find_map(|app| installed_codex_build_at_path(&app))
}

pub fn installed_codex_build_at_path(app: &str) -> Option<(String, u64)> {
    if read_bundle_identifier(app).as_deref() != Some(CODEX_BUNDLE_ID) {
        return None;
    }
    read_bundle_build(app).map(|build| (app.to_string(), build))
}

fn candidate_app_paths() -> Vec<String> {
    let mut roots = vec!["/Applications".to_string()];
    if let Ok(home) = std::env::var("HOME") {
        roots.push(format!("{home}/Applications"));
    }
    roots
        .into_iter()
        .flat_map(|root| {
            CANDIDATE_BUNDLE_NAMES
                .iter()
                .map(move |name| format!("{root}/{name}"))
        })
        .collect()
}

/// Best-effort architecture of an installed Codex.app, read from its Mach-O
/// executable via `lipo`. Returns the host arch when the bundle is universal,
/// otherwise the bundle's single arch (e.g. an Intel/Rosetta install on Apple
/// Silicon reports `x86_64`). Values match `lipo` naming: `arm64` / `x86_64`.
pub fn app_arch(app: &str) -> Option<String> {
    let plist = format!("{app}/Contents/Info.plist");
    let exe = Command::new("/usr/libexec/PlistBuddy")
        .args(["-c", "Print :CFBundleExecutable", &plist])
        .output()
        .ok()?;
    if !exe.status.success() {
        return None;
    }
    let exe_name = String::from_utf8_lossy(&exe.stdout).trim().to_string();
    if exe_name.is_empty() {
        return None;
    }
    let output = Command::new(LIPO)
        .args(["-archs", &format!("{app}/Contents/MacOS/{exe_name}")])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let archs: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    if archs.is_empty() {
        return None;
    }
    let host = if std::env::consts::ARCH == "aarch64" {
        "arm64"
    } else {
        "x86_64"
    };
    if archs.iter().any(|a| a == host) {
        Some(host.to_string())
    } else {
        Some(archs[0].clone())
    }
}

fn read_bundle_build(app: &str) -> Option<u64> {
    let plist = format!("{app}/Contents/Info.plist");
    if !Path::new(&plist).exists() {
        return None;
    }
    let output = Command::new("/usr/libexec/PlistBuddy")
        .args(["-c", "Print :CFBundleVersion", &plist])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8_lossy(&output.stdout).trim().parse().ok()
}

/// Read a bundle's `CFBundleIdentifier`, which is the product identity used to
/// distinguish the Codex lineage from the regular ChatGPT desktop app.
pub fn read_bundle_identifier(app: &str) -> Option<String> {
    let plist = format!("{app}/Contents/Info.plist");
    if !Path::new(&plist).exists() {
        return None;
    }
    let output = Command::new("/usr/libexec/PlistBuddy")
        .args(["-c", "Print :CFBundleIdentifier", &plist])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if id.is_empty() {
        None
    } else {
        Some(id)
    }
}

/// Read the human-facing version string (`CFBundleShortVersionString`, e.g.
/// `26.602.40724`) of an installed bundle. This is what we show the user; the
/// build number (`CFBundleVersion`) is what Sparkle compares. Returns `None` if
/// the key is missing.
pub fn read_bundle_short_version(app: &str) -> Option<String> {
    let plist = format!("{app}/Contents/Info.plist");
    if !Path::new(&plist).exists() {
        return None;
    }
    let output = Command::new("/usr/libexec/PlistBuddy")
        .args(["-c", "Print :CFBundleShortVersionString", &plist])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if v.is_empty() {
        None
    } else {
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn write_fake_app(root: &Path, name: &str, bundle_id: &str, build: u64) -> String {
        let app = root.join(name);
        let contents = app.join("Contents");
        fs::create_dir_all(&contents).unwrap();
        fs::write(
            contents.join("Info.plist"),
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleIdentifier</key>
  <string>{bundle_id}</string>
  <key>CFBundleVersion</key>
  <string>{build}</string>
</dict>
</plist>
"#
            ),
        )
        .unwrap();
        app.to_string_lossy().into_owned()
    }

    #[test]
    fn identity_gate_accepts_codex_lineage_under_chatgpt_bundle_name() {
        let root = std::env::temp_dir().join(format!("codex-sys-chatgpt-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);

        let renamed = write_fake_app(&root, "ChatGPT.app", CODEX_BUNDLE_ID, 5059);

        assert_eq!(
            installed_codex_build_at_path(&renamed),
            Some((renamed.clone(), 5059))
        );
        assert_eq!(
            read_bundle_identifier(&renamed).as_deref(),
            Some(CODEX_BUNDLE_ID)
        );

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn identity_gate_rejects_chatgpt_classic_and_impostors() {
        let root = std::env::temp_dir().join(format!("codex-sys-classic-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);

        let classic = write_fake_app(&root, "ChatGPT.app", "com.openai.chat", 42);
        let impostor = write_fake_app(&root, "Codex.app", "com.example.fake", 7);

        assert_eq!(installed_codex_build_at_path(&classic), None);
        assert_eq!(installed_codex_build_at_path(&impostor), None);

        let _ = fs::remove_dir_all(&root);
    }
}
