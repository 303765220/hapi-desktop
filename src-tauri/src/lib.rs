use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;

mod client_setup;

const SAVED_CREDENTIALS_FILE: &str = "saved-credentials.json";
const CODEX_MIRROR_BASE_URL: &str = "https://codexapp.agentsmirror.com";
const CODEX_MAC_ARM64_APPCAST_URL: &str = "https://codexapp.agentsmirror.com/latest/appcast.xml";
const CODEX_MAC_X64_APPCAST_URL: &str = "https://codexapp.agentsmirror.com/latest/appcast-x64.xml";
const CODEX_WINDOWS_MANIFEST_URL: &str = "https://codexapp.agentsmirror.com/latest/manifest";
const CODEX_WINDOWS_CHECKSUMS_URL: &str = "https://codexapp.agentsmirror.com/latest/checksums";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedCredentials {
    email: String,
    password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexInstallStatus {
    installed: bool,
    platform: String,
    architecture: String,
    install_url: String,
    staged_path: Option<String>,
    installed_path: Option<String>,
    version: Option<String>,
    verified: bool,
    note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WindowsReleasePlan {
    version: String,
    package_moniker: String,
    package_identity: Option<String>,
    architecture: Option<String>,
    content_length: Option<u64>,
    package_url: String,
    sha256: String,
}

fn credentials_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|dir| dir.join(SAVED_CREDENTIALS_FILE))
        .map_err(|err| format!("resolve app data directory: {err}"))
}

fn load_saved_credentials_from_path(path: &Path) -> Result<Option<SavedCredentials>, String> {
    if !path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(path).map_err(|err| format!("read saved credentials: {err}"))?;
    serde_json::from_str::<SavedCredentials>(&raw)
        .map(Some)
        .map_err(|err| format!("parse saved credentials: {err}"))
}

fn save_saved_credentials_to_path(
    path: &Path,
    credentials: &SavedCredentials,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("create credentials directory: {err}"))?;
    }

    let raw = serde_json::to_string(credentials)
        .map_err(|err| format!("serialize saved credentials: {err}"))?;
    fs::write(path, raw).map_err(|err| format!("save credentials: {err}"))
}

fn clear_saved_credentials_at_path(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(format!("clear saved credentials: {err}")),
    }
}

fn host_platform() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else {
        "unsupported".to_string()
    }
}

fn host_architecture() -> String {
    match std::env::consts::ARCH {
        "x86_64" => "x64".to_string(),
        "aarch64" => "arm64".to_string(),
        other => other.to_string(),
    }
}

fn codex_install_url(platform: &str, architecture: &str) -> String {
    match platform {
        "windows" => match architecture {
            "arm64" => format!("{CODEX_MIRROR_BASE_URL}/latest/win-arm64"),
            "x64" => format!("{CODEX_MIRROR_BASE_URL}/latest/win-x64"),
            _ => format!("{CODEX_MIRROR_BASE_URL}/latest/win"),
        },
        "macos" => match architecture {
            "x64" => format!("{CODEX_MIRROR_BASE_URL}/latest/mac-intel"),
            _ => format!("{CODEX_MIRROR_BASE_URL}/latest/mac-arm64"),
        },
        _ => CODEX_MIRROR_BASE_URL.to_string(),
    }
}

fn codex_installed(platform: &str) -> bool {
    match platform {
        "macos" => macos_installed_codex().is_some(),
        "windows" => windows_installed_codex().is_some(),
        _ => false,
    }
}

fn macos_installed_codex() -> Option<(String, u64)> {
    if !cfg!(target_os = "macos") {
        return None;
    }
    codex_mac_engine::sys::installed_codex_build()
}

fn windows_portable_root() -> PathBuf {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("Programs")
        .join("Codex")
}

fn windows_installed_codex() -> Option<codex_win_engine::InstalledWindowsCodex> {
    if !cfg!(target_os = "windows") {
        return None;
    }
    codex_win_engine::detect_installed_codex(&windows_portable_root())
}

fn macos_appcast_url(architecture: &str) -> &'static str {
    match architecture {
        "x64" | "x86_64" => CODEX_MAC_X64_APPCAST_URL,
        _ => CODEX_MAC_ARM64_APPCAST_URL,
    }
}

fn windows_package_url_for_arch(architecture: Option<&str>) -> String {
    match architecture.map(|arch| arch.trim().to_ascii_lowercase()) {
        Some(arch) if arch == "arm64" || arch == "aarch64" => {
            format!("{CODEX_MIRROR_BASE_URL}/latest/win-arm64")
        }
        Some(arch) if arch == "x64" || arch == "x86_64" || arch == "amd64" => {
            format!("{CODEX_MIRROR_BASE_URL}/latest/win-x64")
        }
        _ => format!("{CODEX_MIRROR_BASE_URL}/latest/win"),
    }
}

fn parse_windows_release_plan_for_arch(
    manifest_text: &str,
    checksums_text: &str,
    architecture: &str,
) -> Result<WindowsReleasePlan, String> {
    let release =
        codex_win_engine::manifest::parse_manifest_for_arch(manifest_text, Some(architecture))
            .map_err(|err| format!("parse Windows Codex manifest: {err}"))?;
    let package_url = windows_package_url_for_arch(release.download_architecture.as_deref());
    let sha256 =
        codex_win_engine::find_msix_sha256(checksums_text, &release.package_moniker)
            .map_err(|err| format!("bind Windows Codex checksum: {err}"))?;

    Ok(WindowsReleasePlan {
        version: release.version,
        package_moniker: release.package_moniker,
        package_identity: release.package_identity,
        architecture: release.architecture,
        content_length: release.content_length,
        package_url,
        sha256,
    })
}

fn fetch_windows_release_plan(architecture: &str) -> Result<WindowsReleasePlan, String> {
    let network = codex_win_engine::NetworkConfig::system();
    let manifest = codex_win_engine::fetch_text_with_network(CODEX_WINDOWS_MANIFEST_URL, &network)
        .map_err(|err| format!("fetch Windows Codex manifest: {err}"))?;
    let checksums = codex_win_engine::fetch_text_with_network(CODEX_WINDOWS_CHECKSUMS_URL, &network)
        .map_err(|err| format!("fetch Windows Codex checksums: {err}"))?;
    parse_windows_release_plan_for_arch(&manifest, &checksums, architecture)
}

fn build_codex_install_status() -> CodexInstallStatus {
    let platform = host_platform();
    let architecture = host_architecture();
    let installed = codex_installed(&platform);
    let install_url = codex_install_url(&platform, &architecture);
    let installed_path = match platform.as_str() {
        "macos" => macos_installed_codex().map(|(path, _)| path),
        "windows" => windows_installed_codex().map(|installed| installed.path),
        _ => None,
    };
    let note = if installed {
        "已检测到 Codex 桌面端。".to_string()
    } else if platform == "unsupported" {
        "当前系统暂不支持自动安装引导。".to_string()
    } else if platform == "windows" {
        "将读取 Windows 安装清单并校验后执行 MSIX 安装。".to_string()
    } else if platform == "macos" {
        "将读取 macOS 安装清单并校验后安装 Codex.app。".to_string()
    } else {
        "将安装当前系统对应的 Codex。".to_string()
    };

    CodexInstallStatus {
        installed,
        platform,
        architecture,
        install_url,
        staged_path: None,
        installed_path,
        version: None,
        verified: false,
        note,
    }
}

fn download_windows_msix(plan: &WindowsReleasePlan) -> Result<PathBuf, String> {
    let staged_path = std::env::temp_dir().join(format!("{}.msix", plan.package_moniker));
    if let Some(parent) = staged_path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("create Windows staging directory: {err}"))?;
    }

    let cached_ok = staged_path.exists()
        && codex_win_engine::sha256_file(&staged_path)
            .map(|actual| actual.eq_ignore_ascii_case(&plan.sha256))
            .unwrap_or(false);
    if !cached_ok {
        if staged_path.exists() {
            let _ = fs::remove_file(&staged_path);
        }
        codex_win_engine::download_to_with_network(
            &plan.package_url,
            &staged_path,
            &codex_win_engine::NetworkConfig::system(),
        )
        .map_err(|err| format!("download Windows Codex MSIX: {err}"))?;
    }

    let actual_size = fs::metadata(&staged_path)
        .map_err(|err| format!("read Windows Codex MSIX metadata: {err}"))?
        .len();
    if let Some(expected_size) = plan.content_length {
        if actual_size != expected_size {
            return Err(format!(
                "Windows Codex MSIX size mismatch: {actual_size} != {expected_size}"
            ));
        }
    }
    let actual_sha = codex_win_engine::sha256_file(&staged_path)
        .map_err(|err| format!("hash Windows Codex MSIX: {err}"))?;
    if !actual_sha.eq_ignore_ascii_case(&plan.sha256) {
        let _ = fs::remove_file(&staged_path);
        return Err(format!(
            "Windows Codex MSIX sha256 mismatch: {actual_sha} != {}",
            plan.sha256
        ));
    }

    Ok(staged_path)
}

fn install_windows_codex(status: &mut CodexInstallStatus) -> Result<(), String> {
    let plan = fetch_windows_release_plan(&status.architecture)?;
    if let Some(identity) = plan.package_identity.as_deref() {
        if identity != codex_win_engine::OPENAI_PACKAGE_IDENTITY {
            return Err(format!(
                "Windows Codex package identity mismatch: {identity} != {}",
                codex_win_engine::OPENAI_PACKAGE_IDENTITY
            ));
        }
    }
    let staged_path = download_windows_msix(&plan)?;

    let authenticode = codex_win_engine::verify_openai_authenticode(&staged_path)
        .map_err(|err| format!("verify Windows Codex Authenticode: {err}"))?;
    if !authenticode.is_valid_openai() {
        return Err(format!(
            "Windows Codex Authenticode verification failed: status={}, subject={}",
            authenticode.status, authenticode.subject
        ));
    }

    let identity = codex_win_engine::read_msix_identity(&staged_path)
        .map_err(|err| format!("read Windows Codex MSIX identity: {err}"))?;
    codex_win_engine::validate_codex_identity(
        &identity,
        &plan.version,
        plan.architecture.as_deref(),
    )
    .map_err(|err| format!("validate Windows Codex MSIX identity: {err}"))?;

    let report = codex_win_engine::install_msix_sideload(&staged_path)
        .map_err(|err| format!("install Windows Codex MSIX: {err}"))?;
    if !report.success {
        return Err(report.message);
    }

    status.staged_path = Some(staged_path.to_string_lossy().into_owned());
    status.installed_path = report
        .installed
        .as_ref()
        .map(|installed| installed.path.clone())
        .or_else(|| windows_installed_codex().map(|installed| installed.path));
    status.version = Some(plan.version);
    status.verified = true;
    status.installed = report.installed.is_some() || codex_installed("windows");
    status.note = "已通过 Windows manifest/checksums 校验并执行 MSIX 安装。".to_string();
    Ok(())
}

#[cfg(target_os = "macos")]
fn dir_writable(dir: &Path) -> bool {
    let probe = dir.join(".hapi-codex-write-probe");
    match fs::File::create(&probe) {
        Ok(_) => {
            let _ = fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

#[cfg(target_os = "macos")]
fn choose_macos_install_dir() -> Result<PathBuf, String> {
    let system = PathBuf::from("/Applications");
    if dir_writable(&system) {
        return Ok(system);
    }

    let home = std::env::var("HOME").map_err(|_| "找不到用户主目录".to_string())?;
    let user_apps = PathBuf::from(home).join("Applications");
    fs::create_dir_all(&user_apps).map_err(|err| format!("create ~/Applications: {err}"))?;
    Ok(user_apps)
}

#[cfg(target_os = "macos")]
fn parse_macos_version(s: &str) -> Option<(u32, u32)> {
    let mut parts = s.trim().split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().and_then(|m| m.parse().ok()).unwrap_or(0);
    Some((major, minor))
}

#[cfg(target_os = "macos")]
fn host_macos_version() -> Option<(u32, u32)> {
    let out = std::process::Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    parse_macos_version(&String::from_utf8_lossy(&out.stdout))
}

#[cfg(target_os = "macos")]
fn require_macos_supported(required: Option<&str>) -> Result<(), String> {
    let (Some(req), Some(host)) = (required.and_then(parse_macos_version), host_macos_version())
    else {
        return Ok(());
    };
    if host >= req {
        Ok(())
    } else {
        Err(format!(
            "当前 macOS {}.{} 低于 Codex 要求的 {}.{}+",
            host.0, host.1, req.0, req.1
        ))
    }
}

#[cfg(target_os = "macos")]
fn download_and_verify_macos_full(
    url: &str,
    size: u64,
    signature: &str,
    artifact_name: &str,
) -> Result<PathBuf, String> {
    let staged_path = std::env::temp_dir().join(artifact_name);
    let cached_ok = staged_path.exists()
        && fs::metadata(&staged_path)
            .map(|metadata| metadata.len() == size)
            .unwrap_or(false);
    if !cached_ok {
        if staged_path.exists() {
            let _ = fs::remove_file(&staged_path);
        }
        codex_mac_engine::download::download_to_with_progress_bounded(
            url,
            &staged_path,
            codex_mac_engine::limits::MAX_PACKAGE_BYTES,
            &|_| {},
        )
        .map_err(|err| format!("download macOS Codex package: {err}"))?;
    }

    let actual_size = fs::metadata(&staged_path)
        .map_err(|err| format!("read macOS Codex package metadata: {err}"))?
        .len();
    if actual_size != size {
        let _ = fs::remove_file(&staged_path);
        return Err(format!(
            "macOS Codex package size mismatch: {actual_size} != {size}"
        ));
    }

    let bytes = codex_mac_engine::download::read_file(&staged_path)
        .map_err(|err| format!("read macOS Codex package: {err}"))?;
    codex_mac_engine::verify_sparkle(&bytes, signature)
        .map_err(|err| format!("verify macOS Codex Sparkle signature: {err}"))?;
    Ok(staged_path)
}

#[cfg(target_os = "macos")]
fn unpack_macos_codex_zip(zip: &Path, out_app: &Path) -> Result<(), String> {
    let work = std::env::temp_dir().join(format!(
        "hapi-codex-macos-install-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|err| format!("read system time: {err}"))?
            .as_nanos()
    ));
    let extract = work.join("extract");
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&extract).map_err(|err| format!("create macOS extract directory: {err}"))?;

    let status = std::process::Command::new("/usr/bin/ditto")
        .args(["-x", "-k"])
        .arg(zip)
        .arg(&extract)
        .status()
        .map_err(|err| format!("spawn ditto: {err}"))?;
    if !status.success() {
        let _ = fs::remove_dir_all(&work);
        return Err(format!("ditto exited with {status}"));
    }

    let app_path = find_codex_app(&extract)
        .ok_or_else(|| "macOS Codex package did not contain Codex.app".to_string())?;
    if out_app.exists() {
        fs::remove_dir_all(out_app).map_err(|err| format!("remove old staged Codex.app: {err}"))?;
    }
    fs::rename(&app_path, out_app).map_err(|err| format!("stage Codex.app: {err}"))?;
    let _ = fs::remove_dir_all(&work);
    Ok(())
}

#[cfg(target_os = "macos")]
fn find_codex_app(root: &Path) -> Option<PathBuf> {
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).ok()?.flatten() {
            let path = entry.path();
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == "Codex.app")
            {
                return Some(path);
            }
            if path.is_dir() {
                stack.push(path);
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn install_macos_codex(status: &mut CodexInstallStatus) -> Result<(), String> {
    if macos_installed_codex().is_some() {
        return Err("已检测到 Codex, 第一版安装器只处理首次安装。".to_string());
    }

    let appcast_url = macos_appcast_url(&status.architecture);
    let xml = codex_mac_engine::sys::fetch_text(appcast_url)
        .map_err(|err| format!("fetch macOS Codex appcast: {err}"))?;
    let appcast = codex_mac_engine::parse_appcast(&xml)
        .map_err(|err| format!("parse macOS Codex appcast: {err}"))?;
    let latest = appcast
        .latest()
        .ok_or_else(|| "macOS Codex appcast had no latest item".to_string())?;
    require_macos_supported(latest.minimum_system_version.as_deref())?;
    let signature = latest
        .full
        .ed_signature
        .as_deref()
        .ok_or_else(|| "macOS Codex appcast enclosure missing Sparkle signature".to_string())?;
    let artifact_name = latest
        .full
        .url
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or("Codex.zip");
    let staged_zip =
        download_and_verify_macos_full(&latest.full.url, latest.full.length, signature, artifact_name)?;

    let install_dir = choose_macos_install_dir()?;
    let install_path = install_dir.join("Codex.app");
    let staged_app = install_dir.join(format!(
        ".hapi-codex-install-{}-Codex.app",
        std::process::id()
    ));
    unpack_macos_codex_zip(&staged_zip, &staged_app)?;
    codex_mac_engine::gate_reconstructed(&staged_app)
        .map_err(|err| format!("verify macOS Codex codesign/Gatekeeper: {err}"))?;
    if install_path.exists() {
        let _ = fs::remove_dir_all(&staged_app);
        return Err(format!("{} 已存在, 请先手动确认后再更新", install_path.display()));
    }
    fs::rename(&staged_app, &install_path)
        .map_err(|err| format!("install macOS Codex.app: {err}"))?;

    let detected = macos_installed_codex();
    status.staged_path = Some(staged_zip.to_string_lossy().into_owned());
    status.installed_path = Some(install_path.to_string_lossy().into_owned());
    status.version = Some(latest.short_version.clone());
    status.verified = true;
    status.installed = detected.is_some() || install_path.exists();
    status.note = "已通过 macOS appcast 校验并安装 Codex.app。".to_string();
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn install_macos_codex(_status: &mut CodexInstallStatus) -> Result<(), String> {
    Err("macOS Codex 安装只可在 macOS 上执行".to_string())
}

fn install_codex_for_platform(status: &mut CodexInstallStatus) -> Result<(), String> {
    match status.platform.as_str() {
        "windows" => install_windows_codex(status),
        "macos" => install_macos_codex(status),
        _ => Err("当前系统暂不支持 Codex 安装".to_string()),
    }
}

#[tauri::command]
fn load_saved_credentials(app: tauri::AppHandle) -> Result<Option<SavedCredentials>, String> {
    let path = credentials_path(&app)?;
    load_saved_credentials_from_path(&path)
}

#[tauri::command]
fn save_saved_credentials(
    app: tauri::AppHandle,
    credentials: SavedCredentials,
) -> Result<(), String> {
    let path = credentials_path(&app)?;
    save_saved_credentials_to_path(&path, &credentials)
}

#[tauri::command]
fn clear_saved_credentials(app: tauri::AppHandle) -> Result<(), String> {
    let path = credentials_path(&app)?;
    clear_saved_credentials_at_path(&path)
}

#[tauri::command]
fn codex_install_status() -> Result<CodexInstallStatus, String> {
    Ok(build_codex_install_status())
}

#[tauri::command]
fn install_codex() -> Result<CodexInstallStatus, String> {
    let mut status = build_codex_install_status();
    if status.platform == "unsupported" {
        return Err("当前系统暂不支持 Codex 安装引导".to_string());
    }
    install_codex_for_platform(&mut status)?;
    Ok(status)
}

#[tauri::command]
fn client_setup_status() -> Result<Vec<client_setup::ClientSetupStatus>, String> {
    client_setup::client_setup_status()
}

#[tauri::command]
fn configure_client(
    client: client_setup::ClientSetupClient,
    api_key: String,
    key_platform: Option<String>,
) -> Result<client_setup::ClientConfigureResult, String> {
    client_setup::configure_client(client, api_key, key_platform)
}

#[tauri::command]
fn clear_client_config(
    client: client_setup::ClientSetupClient,
    api_key: Option<String>,
) -> Result<client_setup::ClientConfigureResult, String> {
    client_setup::clear_client_config(client, api_key)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            load_saved_credentials,
            save_saved_credentials,
            clear_saved_credentials,
            codex_install_status,
            install_codex,
            client_setup_status,
            configure_client,
            clear_client_config
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Hapi desktop client");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_credentials() -> SavedCredentials {
        SavedCredentials {
            email: "user@example.com".to_string(),
            password: "secret".to_string(),
        }
    }

    fn temp_credentials_path(name: &str) -> PathBuf {
        let unique = format!(
            "hapi-desktop-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock before unix epoch")
                .as_nanos()
        );
        std::env::temp_dir()
            .join(unique)
            .join(SAVED_CREDENTIALS_FILE)
    }

    #[test]
    fn load_returns_none_when_file_is_missing() {
        let path = temp_credentials_path("missing");

        assert_eq!(load_saved_credentials_from_path(&path), Ok(None));
    }

    #[test]
    fn save_then_load_round_trips_credentials() {
        let path = temp_credentials_path("round-trip");
        let credentials = test_credentials();

        save_saved_credentials_to_path(&path, &credentials).expect("save credentials");

        assert_eq!(
            load_saved_credentials_from_path(&path),
            Ok(Some(credentials))
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn clear_removes_credentials_file() {
        let path = temp_credentials_path("clear");
        save_saved_credentials_to_path(&path, &test_credentials()).expect("save credentials");

        clear_saved_credentials_at_path(&path).expect("clear credentials");

        assert_eq!(load_saved_credentials_from_path(&path), Ok(None));
    }

    #[test]
    fn codex_install_url_uses_agentsmirror_for_platform_and_arch() {
        assert_eq!(
            codex_install_url("windows", "x64"),
            "https://codexapp.agentsmirror.com/latest/win-x64"
        );
        assert_eq!(
            codex_install_url("windows", "arm64"),
            "https://codexapp.agentsmirror.com/latest/win-arm64"
        );
        assert_eq!(
            codex_install_url("macos", "arm64"),
            "https://codexapp.agentsmirror.com/latest/mac-arm64"
        );
    }

    #[test]
    fn macos_appcast_url_uses_agentsmirror_for_platform_and_arch() {
        assert_eq!(
            macos_appcast_url("arm64"),
            "https://codexapp.agentsmirror.com/latest/appcast.xml"
        );
        assert_eq!(
            macos_appcast_url("x64"),
            "https://codexapp.agentsmirror.com/latest/appcast-x64.xml"
        );
    }

    #[test]
    fn windows_release_plan_binds_manifest_checksum_and_arch_url() {
        let manifest = r#"{
          "schemaVersion": 2,
          "sources": {
            "windows": {
              "version": "26.623.5175.0",
              "packageMoniker": "OpenAI.Codex_26.623.5175.0_x64__2p2nqsd0c76g0",
              "updateManifest": {
                "packageIdentity": "OpenAI.Codex"
              },
              "architectures": {
                "x64": {
                  "downloadable": true,
                  "version": "26.623.5175.0",
                  "packageMoniker": "OpenAI.Codex_26.623.5175.0_x64__2p2nqsd0c76g0",
                  "architecture": "x64",
                  "contentLength": 671041042
                },
                "arm64": {
                  "downloadable": true,
                  "version": "26.623.5175.0",
                  "packageMoniker": "OpenAI.Codex_26.623.5175.0_arm64__2p2nqsd0c76g0",
                  "architecture": "arm64",
                  "contentLength": 669761629
                }
              }
            }
          }
        }"#;
        let checksums = "\
aaa25c0bd3658edb80e3abf0f0dab3cc2d8bb6859fc68a746a6ea2e1a21991db  OpenAI.Codex_26.623.5175.0_arm64__2p2nqsd0c76g0.Msix\n\
634ae6f5cd3adf26ed7da17d58286ff2c432ac48eae19d30cbc6a0974d2ac615  OpenAI.Codex_26.623.5175.0_x64__2p2nqsd0c76g0.Msix\n";

        let plan =
            parse_windows_release_plan_for_arch(manifest, checksums, "arm64").expect("plan");

        assert_eq!(plan.version, "26.623.5175.0");
        assert_eq!(
            plan.package_url,
            "https://codexapp.agentsmirror.com/latest/win-arm64"
        );
        assert_eq!(
            plan.sha256,
            "aaa25c0bd3658edb80e3abf0f0dab3cc2d8bb6859fc68a746a6ea2e1a21991db"
        );
        assert_eq!(plan.content_length, Some(669761629));
    }
}
