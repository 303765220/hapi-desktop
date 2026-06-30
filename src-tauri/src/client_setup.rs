use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml_edit::{value, DocumentMut, Item, Table};

pub const HAPI_BASE_URL: &str = "https://www.hapi666.com/api/v1";
const HAPI_PROVIDER_ID: &str = "hapi";
const DEFAULT_CODE_MODEL: &str = "gpt-5";
const DEFAULT_GEMINI_MODEL: &str = "gemini-2.5-pro";
const OPENCODE_GPT_MODELS: &[&str] = &["gpt-5.5", "gpt-5.4", "gpt-5.4-mini"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientSetupClient {
    GeminiCli,
    Codex,
    Opencode,
    Openclaw,
    Hermes,
}

impl ClientSetupClient {
    fn command_name(self) -> &'static str {
        match self {
            ClientSetupClient::GeminiCli => "gemini",
            ClientSetupClient::Codex => "codex",
            ClientSetupClient::Opencode => "opencode",
            ClientSetupClient::Openclaw => "openclaw",
            ClientSetupClient::Hermes => "hermes",
        }
    }

    fn display_name(self) -> &'static str {
        match self {
            ClientSetupClient::GeminiCli => "Gemini CLI",
            ClientSetupClient::Codex => "Codex",
            ClientSetupClient::Opencode => "OpenCode",
            ClientSetupClient::Openclaw => "OpenClaw",
            ClientSetupClient::Hermes => "Hermes",
        }
    }

    fn all() -> [ClientSetupClient; 5] {
        [
            ClientSetupClient::GeminiCli,
            ClientSetupClient::Codex,
            ClientSetupClient::Opencode,
            ClientSetupClient::Openclaw,
            ClientSetupClient::Hermes,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientSetupStatus {
    client: ClientSetupClient,
    name: String,
    installed: bool,
    configured: bool,
    command: String,
    config_path: String,
    note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigureResult {
    client: ClientSetupClient,
    configured: bool,
    config_path: String,
    backup_path: Option<String>,
    note: String,
}

#[derive(Debug, Clone)]
struct ClientSetupPaths {
    home: PathBuf,
}

impl ClientSetupPaths {
    fn new(home: PathBuf) -> Self {
        Self { home }
    }

    fn from_env() -> Result<Self, String> {
        home_dir()
            .map(Self::new)
            .ok_or_else(|| "找不到用户主目录，无法定位客户端配置文件。".to_string())
    }

    fn config_path(&self, client: ClientSetupClient) -> PathBuf {
        match client {
            ClientSetupClient::GeminiCli => self.home.join(".gemini").join(".env"),
            ClientSetupClient::Codex => self.home.join(".codex").join("config.toml"),
            ClientSetupClient::Opencode => self
                .home
                .join(".config")
                .join("opencode")
                .join("opencode.json"),
            ClientSetupClient::Openclaw => self.home.join(".openclaw").join("openclaw.json"),
            ClientSetupClient::Hermes => hermes_config_path(&self.home),
        }
    }

    fn install_dir(&self, client: ClientSetupClient) -> PathBuf {
        match client {
            ClientSetupClient::GeminiCli => self.home.join(".gemini"),
            ClientSetupClient::Codex => self.home.join(".codex"),
            ClientSetupClient::Opencode => self.home.join(".config").join("opencode"),
            ClientSetupClient::Openclaw => self.home.join(".openclaw"),
            ClientSetupClient::Hermes => hermes_config_dir(&self.home),
        }
    }

    fn codex_auth_path(&self) -> PathBuf {
        self.home.join(".codex").join("auth.json")
    }
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from))
}

fn hermes_config_dir(home: &Path) -> PathBuf {
    if let Some(raw) = std::env::var_os("HERMES_HOME") {
        let value = raw.to_string_lossy();
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    if cfg!(target_os = "windows") {
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join("AppData").join("Local"))
            .join("hermes")
    } else {
        home.join(".hermes")
    }
}

fn hermes_config_path(home: &Path) -> PathBuf {
    hermes_config_dir(home).join("config.yaml")
}

fn command_exists(command: &str) -> bool {
    let checker = if cfg!(target_os = "windows") {
        ("where", command)
    } else {
        ("command", command)
    };
    if cfg!(target_os = "windows") {
        Command::new(checker.0)
            .arg(checker.1)
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    } else {
        Command::new("sh")
            .args(["-lc", &format!("command -v {command}")])
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    }
}

fn has_hapi_config(client: ClientSetupClient, paths: &ClientSetupPaths) -> bool {
    match client {
        ClientSetupClient::Codex => {
            let config = paths.config_path(client);
            fs::read_to_string(config)
                .map(|text| text.contains("[model_providers.hapi]") || text.contains("model_provider = \"hapi\""))
                .unwrap_or(false)
        }
        ClientSetupClient::GeminiCli => fs::read_to_string(paths.config_path(client))
            .map(|text| text.contains(HAPI_BASE_URL) && text.contains("GEMINI_API_KEY="))
            .unwrap_or(false),
        ClientSetupClient::Opencode => fs::read_to_string(paths.config_path(client))
            .map(|text| text.contains("\"hapi\"") && text.contains(HAPI_BASE_URL))
            .unwrap_or(false),
        ClientSetupClient::Openclaw => fs::read_to_string(paths.config_path(client))
            .map(|text| text.contains("hapi") && text.contains(HAPI_BASE_URL))
            .unwrap_or(false),
        ClientSetupClient::Hermes => fs::read_to_string(paths.config_path(client))
            .map(|text| text.contains("name: hapi") && text.contains(HAPI_BASE_URL))
            .unwrap_or(false),
    }
}

fn client_installed(client: ClientSetupClient, paths: &ClientSetupPaths) -> bool {
    command_exists(client.command_name()) || paths.install_dir(client).exists()
}

pub fn client_setup_status() -> Result<Vec<ClientSetupStatus>, String> {
    let paths = ClientSetupPaths::from_env()?;
    Ok(ClientSetupClient::all()
        .into_iter()
        .map(|client| {
            let command = client.command_name().to_string();
            let dir = paths.install_dir(client);
            let config_path = paths.config_path(client);
            let installed = command_exists(&command) || dir.exists();
            let configured = has_hapi_config(client, &paths);
            let note = if installed {
                "已检测到客户端，可写入 Hapi 配置。".to_string()
            } else {
                "未检测到客户端，请先安装后再配置。".to_string()
            };
            ClientSetupStatus {
                client,
                name: client.display_name().to_string(),
                installed,
                configured,
                command,
                config_path: config_path.to_string_lossy().into_owned(),
                note,
            }
        })
        .collect())
}

pub fn configure_client(
    client: ClientSetupClient,
    api_key: String,
    key_platform: Option<String>,
) -> Result<ClientConfigureResult, String> {
    let paths = ClientSetupPaths::from_env()?;
    configure_client_with_paths(client, &api_key, key_platform.as_deref(), &paths)
}

pub fn clear_client_config(client: ClientSetupClient) -> Result<ClientConfigureResult, String> {
    let paths = ClientSetupPaths::from_env()?;
    clear_client_config_with_paths(client, &paths)
}

fn configure_client_with_paths(
    client: ClientSetupClient,
    api_key: &str,
    key_platform: Option<&str>,
    paths: &ClientSetupPaths,
) -> Result<ClientConfigureResult, String> {
    let trimmed_key = api_key.trim();
    if trimmed_key.is_empty() {
        return Err("请选择有效的 Hapi API Key。".to_string());
    }
    if !client_installed(client, paths) {
        return Err(format!("未检测到 {}，请先安装后再配置。", client.display_name()));
    }

    let config_path = paths.config_path(client);
    let backup_path = backup_existing_file(&config_path)?;
    match client {
        ClientSetupClient::Codex => {
            configure_codex(paths, trimmed_key)?;
        }
        ClientSetupClient::GeminiCli => {
            write_text_file(&config_path, &build_gemini_env_text(read_optional_text(&config_path)?, trimmed_key)?)?;
        }
        ClientSetupClient::Opencode => {
            write_json_file(&config_path, &build_opencode_config(read_optional_text(&config_path)?, trimmed_key, key_platform)?)?;
        }
        ClientSetupClient::Openclaw => {
            write_json_file(&config_path, &build_openclaw_config(read_optional_text(&config_path)?, trimmed_key)?)?;
        }
        ClientSetupClient::Hermes => {
            write_text_file(&config_path, &build_hermes_yaml_text(read_optional_text(&config_path)?, trimmed_key)?)?;
        }
    }

    Ok(ClientConfigureResult {
        client,
        configured: true,
        config_path: config_path.to_string_lossy().into_owned(),
        backup_path: backup_path.map(|path| path.to_string_lossy().into_owned()),
        note: format!("已写入 {} 的 Hapi 配置。", client.display_name()),
    })
}

fn clear_client_config_with_paths(
    client: ClientSetupClient,
    paths: &ClientSetupPaths,
) -> Result<ClientConfigureResult, String> {
    let config_path = paths.config_path(client);
    let backup_path = backup_existing_file(&config_path)?;
    match client {
        ClientSetupClient::Codex => {
            clear_codex(paths)?;
        }
        ClientSetupClient::GeminiCli => {
            write_text_file(&config_path, &clear_gemini_env_text(read_optional_text(&config_path)?)?)?;
        }
        ClientSetupClient::Opencode => {
            write_json_file(&config_path, &clear_opencode_config(read_optional_text(&config_path)?)?)?;
        }
        ClientSetupClient::Openclaw => {
            write_json_file(&config_path, &clear_openclaw_config(read_optional_text(&config_path)?)?)?;
        }
        ClientSetupClient::Hermes => {
            write_text_file(&config_path, &clear_hermes_yaml_text(read_optional_text(&config_path)?)?)?;
        }
    }

    Ok(ClientConfigureResult {
        client,
        configured: false,
        config_path: config_path.to_string_lossy().into_owned(),
        backup_path: backup_path.map(|path| path.to_string_lossy().into_owned()),
        note: format!("已清除 {} 的 Hapi 配置。", client.display_name()),
    })
}

fn configure_codex(paths: &ClientSetupPaths, api_key: &str) -> Result<(), String> {
    let auth_path = paths.codex_auth_path();
    let config_path = paths.config_path(ClientSetupClient::Codex);
    let old_auth = read_optional_bytes(&auth_path)?;
    let old_config = read_optional_bytes(&config_path)?;

    let auth_value = build_codex_auth(read_optional_text(&auth_path)?, api_key)?;
    if let Err(err) = write_json_file(&auth_path, &auth_value) {
        return Err(err);
    }

    let config_text = match String::from_utf8(old_config.clone().unwrap_or_default()) {
        Ok(text) => build_codex_config_text(&text)?,
        Err(err) => {
            restore_optional_file(&auth_path, old_auth)?;
            return Err(format!("读取 Codex config.toml 失败: {err}"));
        }
    };

    if let Err(err) = write_text_file(&config_path, &config_text) {
        restore_optional_file(&auth_path, old_auth)?;
        restore_optional_file(&config_path, old_config)?;
        return Err(err);
    }

    Ok(())
}

fn clear_codex(paths: &ClientSetupPaths) -> Result<(), String> {
    let auth_path = paths.codex_auth_path();
    let config_path = paths.config_path(ClientSetupClient::Codex);
    let old_auth = read_optional_bytes(&auth_path)?;
    let old_config = read_optional_bytes(&config_path)?;
    let _ = backup_existing_file(&auth_path)?;

    let auth_value = clear_codex_auth(read_optional_text(&auth_path)?)?;
    if let Err(err) = write_json_file(&auth_path, &auth_value) {
        return Err(err);
    }

    let config_text = match String::from_utf8(old_config.clone().unwrap_or_default()) {
        Ok(text) => clear_codex_config_text(&text)?,
        Err(err) => {
            restore_optional_file(&auth_path, old_auth)?;
            return Err(format!("读取 Codex config.toml 失败: {err}"));
        }
    };

    if let Err(err) = write_text_file(&config_path, &config_text) {
        restore_optional_file(&auth_path, old_auth)?;
        restore_optional_file(&config_path, old_config)?;
        return Err(err);
    }

    Ok(())
}

fn build_codex_auth(existing: Option<String>, api_key: &str) -> Result<Value, String> {
    let mut value = match existing {
        Some(text) if !text.trim().is_empty() => serde_json::from_str::<Value>(&text)
            .map_err(|err| format!("解析 Codex auth.json 失败: {err}"))?,
        _ => json!({}),
    };

    if !value.is_object() {
        value = json!({});
    }
    value["OPENAI_API_KEY"] = Value::String(api_key.to_string());
    Ok(value)
}

fn clear_codex_auth(existing: Option<String>) -> Result<Value, String> {
    let mut value = match existing {
        Some(text) if !text.trim().is_empty() => serde_json::from_str::<Value>(&text)
            .map_err(|err| format!("解析 Codex auth.json 失败: {err}"))?,
        _ => json!({}),
    };

    if let Some(object) = value.as_object_mut() {
        object.remove("OPENAI_API_KEY");
    }
    Ok(value)
}

fn build_codex_config_text(existing: &str) -> Result<String, String> {
    let mut doc = if existing.trim().is_empty() {
        DocumentMut::new()
    } else {
        existing
            .parse::<DocumentMut>()
            .map_err(|err| format!("解析 Codex config.toml 失败: {err}"))?
    };

    doc["model_provider"] = value(HAPI_PROVIDER_ID);
    doc["model"] = value(DEFAULT_CODE_MODEL);
    doc["model_reasoning_effort"] = value("high");
    doc["disable_response_storage"] = value(true);

    if !doc["model_providers"].is_table() {
        doc["model_providers"] = Item::Table(Table::new());
    }
    let providers = doc["model_providers"]
        .as_table_mut()
        .ok_or_else(|| "Codex model_providers 不是 TOML 表。".to_string())?;
    providers[HAPI_PROVIDER_ID] = Item::Table(build_codex_hapi_provider_table());

    Ok(doc.to_string())
}

fn clear_codex_config_text(existing: &str) -> Result<String, String> {
    if existing.trim().is_empty() {
        return Ok(String::new());
    }
    let mut doc = existing
        .parse::<DocumentMut>()
        .map_err(|err| format!("解析 Codex config.toml 失败: {err}"))?;

    if doc.get("model_provider").and_then(Item::as_str) == Some(HAPI_PROVIDER_ID) {
        doc.remove("model_provider");
        if doc.get("model").and_then(Item::as_str) == Some(DEFAULT_CODE_MODEL) {
            doc.remove("model");
        }
        if doc.get("model_reasoning_effort").and_then(Item::as_str) == Some("high") {
            doc.remove("model_reasoning_effort");
        }
        if doc
            .get("disable_response_storage")
            .and_then(Item::as_bool)
            == Some(true)
        {
            doc.remove("disable_response_storage");
        }
    }

    if let Some(providers) = doc["model_providers"].as_table_mut() {
        providers.remove(HAPI_PROVIDER_ID);
    }

    Ok(doc.to_string())
}

fn build_codex_hapi_provider_table() -> Table {
    let mut table = Table::new();
    table["name"] = value("Hapi");
    table["base_url"] = value(HAPI_BASE_URL);
    table["wire_api"] = value("responses");
    table["requires_openai_auth"] = value(true);
    table
}

fn build_gemini_env_text(existing: Option<String>, api_key: &str) -> Result<String, String> {
    let mut lines = Vec::new();
    let mut seen_base = false;
    let mut seen_key = false;
    let mut seen_model = false;

    if let Some(existing) = existing {
        for line in existing.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("GOOGLE_GEMINI_BASE_URL=") {
                lines.push(format!("GOOGLE_GEMINI_BASE_URL={HAPI_BASE_URL}"));
                seen_base = true;
            } else if trimmed.starts_with("GEMINI_API_KEY=") {
                lines.push(format!("GEMINI_API_KEY={api_key}"));
                seen_key = true;
            } else if trimmed.starts_with("GEMINI_MODEL=") {
                lines.push(format!("GEMINI_MODEL={DEFAULT_GEMINI_MODEL}"));
                seen_model = true;
            } else {
                lines.push(line.to_string());
            }
        }
    }

    if !seen_base {
        lines.push(format!("GOOGLE_GEMINI_BASE_URL={HAPI_BASE_URL}"));
    }
    if !seen_key {
        lines.push(format!("GEMINI_API_KEY={api_key}"));
    }
    if !seen_model {
        lines.push(format!("GEMINI_MODEL={DEFAULT_GEMINI_MODEL}"));
    }

    Ok(format!("{}\n", lines.join("\n")))
}

fn clear_gemini_env_text(existing: Option<String>) -> Result<String, String> {
    let Some(existing) = existing else {
        return Ok(String::new());
    };
    let mut lines = Vec::new();
    for line in existing.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("GOOGLE_GEMINI_BASE_URL=")
            || trimmed.starts_with("GEMINI_API_KEY=")
            || trimmed.starts_with("GEMINI_MODEL=")
        {
            continue;
        }
        lines.push(line.to_string());
    }
    if lines.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("{}\n", lines.join("\n")))
    }
}

fn build_opencode_config(
    existing: Option<String>,
    api_key: &str,
    key_platform: Option<&str>,
) -> Result<Value, String> {
    let mut config = parse_json_or_default(existing, json!({ "$schema": "https://opencode.ai/config.json" }))?;
    ensure_object(&mut config);
    if config.get("provider").and_then(Value::as_object).is_none() {
        config["provider"] = json!({});
    }
    let models = opencode_models_for_platform(key_platform);
    config["provider"][HAPI_PROVIDER_ID] = json!({
        "npm": "@ai-sdk/openai-compatible",
        "name": "Hapi",
        "options": {
            "baseURL": HAPI_BASE_URL,
            "apiKey": api_key
        },
        "models": models
    });
    Ok(config)
}

fn opencode_models_for_platform(key_platform: Option<&str>) -> Value {
    let mut models = Map::new();
    if key_platform == Some("openai") {
        for model in OPENCODE_GPT_MODELS {
            models.insert((*model).to_string(), json!({ "name": model }));
        }
    } else {
        models.insert(
            DEFAULT_CODE_MODEL.to_string(),
            json!({ "name": DEFAULT_CODE_MODEL }),
        );
    }
    Value::Object(models)
}

fn clear_opencode_config(existing: Option<String>) -> Result<Value, String> {
    let mut config = parse_json_or_default(existing, json!({ "$schema": "https://opencode.ai/config.json" }))?;
    if let Some(providers) = config.get_mut("provider").and_then(Value::as_object_mut) {
        providers.remove(HAPI_PROVIDER_ID);
    }
    Ok(config)
}

fn build_openclaw_config(existing: Option<String>, api_key: &str) -> Result<Value, String> {
    let mut config = parse_json_or_default(existing, json!({ "models": { "mode": "merge", "providers": {} } }))?;
    ensure_object(&mut config);
    if config.get("models").and_then(Value::as_object).is_none() {
        config["models"] = json!({});
    }
    if config["models"].get("providers").and_then(Value::as_object).is_none() {
        config["models"]["providers"] = json!({});
    }
    if config["models"].get("mode").is_none() {
        config["models"]["mode"] = Value::String("merge".to_string());
    }
    config["models"]["providers"][HAPI_PROVIDER_ID] = json!({
        "baseUrl": HAPI_BASE_URL,
        "apiKey": api_key,
        "api": "openai",
        "models": [
            {
                "id": DEFAULT_CODE_MODEL,
                "name": DEFAULT_CODE_MODEL
            }
        ]
    });
    Ok(config)
}

fn clear_openclaw_config(existing: Option<String>) -> Result<Value, String> {
    let mut config = parse_json_or_default(existing, json!({ "models": { "mode": "merge", "providers": {} } }))?;
    if let Some(providers) = config
        .get_mut("models")
        .and_then(|models| models.get_mut("providers"))
        .and_then(Value::as_object_mut)
    {
        providers.remove(HAPI_PROVIDER_ID);
    }
    Ok(config)
}

fn build_hermes_yaml_text(existing: Option<String>, api_key: &str) -> Result<String, String> {
    let mut config = match existing {
        Some(text) if !text.trim().is_empty() => serde_yaml::from_str::<serde_yaml::Value>(&text)
            .map_err(|err| format!("解析 Hermes config.yaml 失败: {err}"))?,
        _ => serde_yaml::Value::Mapping(serde_yaml::Mapping::new()),
    };

    if !matches!(config, serde_yaml::Value::Mapping(_)) {
        config = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
    }

    let mapping = config
        .as_mapping_mut()
        .ok_or_else(|| "Hermes config.yaml 顶层不是 YAML 对象。".to_string())?;
    let key = serde_yaml::Value::String("custom_providers".to_string());
    let entry = serde_yaml::to_value(json!({
        "name": HAPI_PROVIDER_ID,
        "base_url": HAPI_BASE_URL,
        "api_key": api_key,
        "model": DEFAULT_CODE_MODEL,
        "models": {
            DEFAULT_CODE_MODEL: {
                "context_length": 200000
            }
        }
    }))
    .map_err(|err| format!("生成 Hermes provider 失败: {err}"))?;

    let providers = mapping
        .entry(key)
        .or_insert_with(|| serde_yaml::Value::Sequence(Vec::new()));
    if !matches!(providers, serde_yaml::Value::Sequence(_)) {
        *providers = serde_yaml::Value::Sequence(Vec::new());
    }
    let sequence = providers
        .as_sequence_mut()
        .ok_or_else(|| "Hermes custom_providers 不是数组。".to_string())?;
    sequence.retain(|item| item.get("name").and_then(serde_yaml::Value::as_str) != Some(HAPI_PROVIDER_ID));
    sequence.push(entry);

    serde_yaml::to_string(&config).map_err(|err| format!("序列化 Hermes config.yaml 失败: {err}"))
}

fn clear_hermes_yaml_text(existing: Option<String>) -> Result<String, String> {
    let Some(existing) = existing else {
        return Ok(String::new());
    };
    let mut config = if existing.trim().is_empty() {
        serde_yaml::Value::Mapping(serde_yaml::Mapping::new())
    } else {
        serde_yaml::from_str::<serde_yaml::Value>(&existing)
            .map_err(|err| format!("解析 Hermes config.yaml 失败: {err}"))?
    };

    if let Some(sequence) = config
        .get_mut("custom_providers")
        .and_then(serde_yaml::Value::as_sequence_mut)
    {
        sequence.retain(|item| item.get("name").and_then(serde_yaml::Value::as_str) != Some(HAPI_PROVIDER_ID));
    }

    serde_yaml::to_string(&config).map_err(|err| format!("序列化 Hermes config.yaml 失败: {err}"))
}

fn parse_json_or_default(existing: Option<String>, default_value: Value) -> Result<Value, String> {
    match existing {
        Some(text) if !text.trim().is_empty() => serde_json::from_str::<Value>(&text)
            .map_err(|err| format!("解析 JSON 配置失败: {err}")),
        _ => Ok(default_value),
    }
}

fn ensure_object(value: &mut Value) {
    if !value.is_object() {
        *value = json!({});
    }
}

fn read_optional_text(path: &Path) -> Result<Option<String>, String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(format!("读取 {} 失败: {err}", path.display())),
    }
}

fn read_optional_bytes(path: &Path) -> Result<Option<Vec<u8>>, String> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(format!("读取 {} 失败: {err}", path.display())),
    }
}

fn write_text_file(path: &Path, text: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("创建 {} 失败: {err}", parent.display()))?;
    }
    fs::write(path, text).map_err(|err| format!("写入 {} 失败: {err}", path.display()))
}

fn write_json_file(path: &Path, value: &Value) -> Result<(), String> {
    let text = serde_json::to_string_pretty(value)
        .map_err(|err| format!("序列化 JSON 配置失败: {err}"))?;
    write_text_file(path, &format!("{text}\n"))
}

fn backup_existing_file(path: &Path) -> Result<Option<PathBuf>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let backup = path.with_extension(format!(
        "{}hapi.bak",
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| format!("{ext}."))
            .unwrap_or_default()
    ));
    fs::copy(path, &backup).map_err(|err| format!("备份 {} 失败: {err}", path.display()))?;
    Ok(Some(backup))
}

fn restore_optional_file(path: &Path, content: Option<Vec<u8>>) -> Result<(), String> {
    match content {
        Some(bytes) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|err| format!("创建 {} 失败: {err}", parent.display()))?;
            }
            fs::write(path, bytes).map_err(|err| format!("回滚 {} 失败: {err}", path.display()))
        }
        None => match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(format!("回滚删除 {} 失败: {err}", path.display())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codex_auth_merge_preserves_login_state() {
        let existing = r#"{
          "OPENAI_API_KEY": "old-key",
          "tokens": { "access_token": "keep-me" },
          "last_refresh": "2026-06-30"
        }"#;

        let merged = build_codex_auth(Some(existing.to_string()), "new-hapi-key").unwrap();

        assert_eq!(merged["OPENAI_API_KEY"], "new-hapi-key");
        assert_eq!(merged["tokens"]["access_token"], "keep-me");
        assert_eq!(merged["last_refresh"], "2026-06-30");
    }

    #[test]
    fn codex_config_preserves_existing_provider_and_adds_hapi() {
        let existing = r#"model_provider = "openai"
approval_policy = "never"

[model_providers.openai]
name = "OpenAI"
base_url = "https://api.openai.com/v1"
"#;

        let text = build_codex_config_text(existing).unwrap();

        assert!(text.contains("approval_policy = \"never\""));
        assert!(text.contains("[model_providers.openai]"));
        assert!(text.contains("[model_providers.hapi]"));
        assert!(text.contains("base_url = \"https://www.hapi666.com/api/v1\""));
        assert!(text.contains("model_provider = \"hapi\""));
    }

    #[test]
    fn gemini_env_updates_hapi_values_without_dropping_user_lines() {
        let existing = "OTHER=value\nGEMINI_API_KEY=old\n";

        let text = build_gemini_env_text(Some(existing.to_string()), "new-key").unwrap();

        assert!(text.contains("OTHER=value"));
        assert!(text.contains("GEMINI_API_KEY=new-key"));
        assert!(text.contains("GOOGLE_GEMINI_BASE_URL=https://www.hapi666.com/api/v1"));
        assert!(text.contains("GEMINI_MODEL=gemini-2.5-pro"));
    }

    #[test]
    fn opencode_config_adds_hapi_provider() {
        let config = build_opencode_config(Some(r#"{ "theme": "dark" }"#.to_string()), "key", None).unwrap();

        assert_eq!(config["theme"], "dark");
        assert_eq!(config["provider"]["hapi"]["npm"], "@ai-sdk/openai-compatible");
        assert_eq!(config["provider"]["hapi"]["options"]["baseURL"], HAPI_BASE_URL);
        assert_eq!(config["provider"]["hapi"]["options"]["apiKey"], "key");
    }

    #[test]
    fn opencode_config_adds_gpt_models_for_openai_key() {
        let config = build_opencode_config(None, "key", Some("openai")).unwrap();

        assert_eq!(config["provider"]["hapi"]["models"]["gpt-5.5"]["name"], "gpt-5.5");
        assert_eq!(config["provider"]["hapi"]["models"]["gpt-5.4"]["name"], "gpt-5.4");
        assert_eq!(config["provider"]["hapi"]["models"]["gpt-5.4-mini"]["name"], "gpt-5.4-mini");
    }

    #[test]
    fn openclaw_config_adds_hapi_provider() {
        let config = build_openclaw_config(Some(r#"{ "models": { "mode": "merge" } }"#.to_string()), "key").unwrap();

        assert_eq!(config["models"]["mode"], "merge");
        assert_eq!(config["models"]["providers"]["hapi"]["baseUrl"], HAPI_BASE_URL);
        assert_eq!(config["models"]["providers"]["hapi"]["apiKey"], "key");
    }

    #[test]
    fn hermes_yaml_upserts_hapi_custom_provider() {
        let existing = r#"agent:
  max_turns: 50
custom_providers:
- name: hapi
  base_url: https://old.example.com
  api_key: old
"#;

        let text = build_hermes_yaml_text(Some(existing.to_string()), "key").unwrap();

        assert!(text.contains("agent:"));
        assert!(text.contains("max_turns: 50"));
        assert!(text.contains("name: hapi"));
        assert!(text.contains("base_url: https://www.hapi666.com/api/v1"));
        assert!(text.contains("api_key: key"));
        assert!(!text.contains("https://old.example.com"));
    }

    #[test]
    fn clear_config_removes_hapi_provider_without_dropping_other_entries() {
        let opencode = clear_opencode_config(Some(r#"{
          "provider": {
            "hapi": { "name": "Hapi" },
            "other": { "name": "Other" }
          }
        }"#.to_string())).unwrap();
        assert!(opencode["provider"].get("hapi").is_none());
        assert_eq!(opencode["provider"]["other"]["name"], "Other");

        let openclaw = clear_openclaw_config(Some(r#"{
          "models": {
            "providers": {
              "hapi": { "baseUrl": "https://www.hapi666.com/api/v1" },
              "other": { "baseUrl": "https://example.com" }
            }
          }
        }"#.to_string())).unwrap();
        assert!(openclaw["models"]["providers"].get("hapi").is_none());
        assert_eq!(openclaw["models"]["providers"]["other"]["baseUrl"], "https://example.com");
    }

    #[test]
    fn clear_codex_removes_hapi_provider_and_preserves_login_state() {
        let auth = clear_codex_auth(Some(r#"{
          "OPENAI_API_KEY": "hapi-key",
          "tokens": { "access_token": "keep-me" }
        }"#.to_string())).unwrap();
        assert!(auth.get("OPENAI_API_KEY").is_none());
        assert_eq!(auth["tokens"]["access_token"], "keep-me");

        let config = clear_codex_config_text(r#"model_provider = "hapi"
model = "gpt-5"
approval_policy = "never"

[model_providers.hapi]
name = "Hapi"

[model_providers.other]
name = "Other"
"#).unwrap();
        assert!(!config.contains("[model_providers.hapi]"));
        assert!(config.contains("[model_providers.other]"));
        assert!(config.contains("approval_policy = \"never\""));
        assert!(!config.contains("model_provider = \"hapi\""));
    }

    #[test]
    fn configure_refuses_when_client_is_not_installed() {
        let home = std::env::temp_dir().join(format!(
            "hapi-client-setup-missing-{}",
            std::process::id()
        ));
        let paths = ClientSetupPaths::new(home);

        let err = configure_client_with_paths(ClientSetupClient::Openclaw, "key", None, &paths).unwrap_err();

        assert!(err.contains("未检测到 OpenClaw"));
    }
}
