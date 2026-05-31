use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use seqdiff::ratio;
use serde::{Deserialize, Serialize};

mod structs;
pub use structs::*;

use crate::{config, i18n, APP_CONFIG_DIR, APP_DIR};

#[cfg(feature = "lua")]
use crate::lua::{self, CommandContext, SandboxLevel};

#[derive(Deserialize)]
struct LegacyYamlCommandsList {
    #[serde(default)]
    list: Vec<LegacyYamlCommandItem>,
}

#[derive(Deserialize)]
struct LegacyYamlCommandItem {
    command: LegacyYamlCommandAction,
    #[serde(default)]
    voice: LegacyYamlVoice,
    #[serde(default)]
    phrases: Vec<String>,
}

#[derive(Deserialize)]
struct LegacyYamlCommandAction {
    action: String,
    #[serde(default)]
    exe_path: String,
    #[serde(default)]
    exe_args: Vec<String>,
    #[serde(default)]
    cli_cmd: String,
    #[serde(default)]
    cli_args: Vec<String>,
}

#[derive(Default, Deserialize)]
struct LegacyYamlVoice {
    #[serde(default)]
    sounds: Vec<String>,
}

impl LegacyYamlCommandItem {
    fn into_command(self, pack_name: &str, index: usize) -> JCommand {
        let mut phrases = HashMap::new();
        phrases.insert("ru".to_string(), self.phrases);

        let mut sounds = HashMap::new();
        sounds.insert("ru".to_string(), self.voice.sounds);

        JCommand {
            id: format!("{}_{}", pack_name.replace('-', "_"), index),
            cmd_type: self.command.action,
            description: String::new(),
            exe_path: self.command.exe_path,
            exe_args: self.command.exe_args,
            cli_cmd: self.command.cli_cmd,
            cli_args: self.command.cli_args,
            script: String::new(),
            sandbox: String::new(),
            timeout: 10_000,
            sounds,
            phrases,
            slots: HashMap::new(),
            sounds_cache: Default::default(),
            phrases_cache: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct LearnedCommandsFile {
    #[serde(default)]
    aliases: Vec<LearnedCommandAlias>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LearnedCommandAlias {
    pub phrase: String,
    pub command_id: String,
    pub lang: String,
    #[serde(default)]
    pub created_at_unix: u64,
}

pub fn parse_commands() -> Result<Vec<JCommandsList>, String> {
    let mut commands: Vec<JCommandsList> = Vec::new();

    let commands_path = APP_DIR.join(config::COMMANDS_PATH);
    let cmd_dirs = fs::read_dir(&commands_path).map_err(|e| {
        format!(
            "Error reading commands directory {:?}: {}",
            commands_path, e
        )
    })?;

    for entry in cmd_dirs.flatten() {
        let cmd_path = entry.path();

        if let Some(command_list) = parse_command_pack(&cmd_path) {
            commands.push(command_list);
        }
    }

    if commands.is_empty() {
        Err("No commands found".into())
    } else {
        let command_count: usize = commands.iter().map(|pack| pack.commands.len()).sum();
        info!(
            "Loaded {} command pack(s), {} command(s)",
            commands.len(),
            command_count
        );
        Ok(commands)
    }
}

fn parse_command_pack(cmd_path: &Path) -> Option<JCommandsList> {
    let toml_file = cmd_path.join("command.toml");
    if toml_file.exists() {
        return parse_toml_command_pack(cmd_path, &toml_file);
    }

    let yaml_file = cmd_path.join("command.yaml");
    if yaml_file.exists() {
        return parse_legacy_yaml_command_pack(cmd_path, &yaml_file);
    }

    None
}

fn parse_toml_command_pack(cmd_path: &Path, toml_file: &Path) -> Option<JCommandsList> {
    let content = match fs::read_to_string(toml_file) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to read {}: {}", toml_file.display(), e);
            return None;
        }
    };

    let file: JCommandsList = match toml::from_str(&content) {
        Ok(f) => f,
        Err(e) => {
            warn!("Failed to parse {}: {}", toml_file.display(), e);
            return None;
        }
    };

    Some(JCommandsList {
        path: cmd_path.to_path_buf(),
        commands: file.commands,
    })
}

fn parse_legacy_yaml_command_pack(cmd_path: &Path, yaml_file: &Path) -> Option<JCommandsList> {
    let content = match fs::read_to_string(yaml_file) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to read {}: {}", yaml_file.display(), e);
            return None;
        }
    };

    let file: LegacyYamlCommandsList = match serde_yaml::from_str(&content) {
        Ok(f) => f,
        Err(e) => {
            warn!("Failed to parse {}: {}", yaml_file.display(), e);
            return None;
        }
    };

    let pack_name = cmd_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("legacy_command");

    let commands = file
        .list
        .into_iter()
        .enumerate()
        .map(|(index, item)| item.into_command(pack_name, index + 1))
        .collect();

    Some(JCommandsList {
        path: cmd_path.to_path_buf(),
        commands,
    })
}

pub fn commands_hash(commands: &[JCommandsList]) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();

    let lang = i18n::get_language();
    hasher.update(lang.as_bytes());
    hasher.update(b"|");

    // collect all command ids and phrases for current language, sorted
    let mut all_data: Vec<(&str, _)> = commands
        .iter()
        .flat_map(|ac| {
            ac.commands
                .iter()
                .map(|c| (c.id.as_str(), c.get_phrases(&lang)))
        })
        .collect();
    all_data.sort_by_key(|(id, _)| *id);

    for (id, phrases) in all_data {
        hasher.update(id.as_bytes());
        for phrase in phrases.iter() {
            hasher.update(phrase.as_bytes());
        }
    }

    format!("{:x}", hasher.finalize())
}

pub fn fetch_command<'a>(
    phrase: &str,
    commands: &'a [JCommandsList],
) -> Option<(&'a PathBuf, &'a JCommand)> {
    let lang = i18n::get_language();

    let phrase = normalize_phrase(phrase);
    if phrase.is_empty() {
        return None;
    }

    if let Some(command) = fetch_learned_alias_command(&phrase, commands) {
        return Some(command);
    }

    let phrase_chars: Vec<char> = phrase.chars().collect();
    let phrase_words: Vec<&str> = phrase.split_whitespace().collect();

    let mut result: Option<(&PathBuf, &JCommand)> = None;
    let mut best_score = config::CMD_RATIO_THRESHOLD;

    for cmd_list in commands {
        for cmd in &cmd_list.commands {
            let cmd_phrases = cmd.get_phrases(&lang);

            for cmd_phrase in cmd_phrases.iter() {
                let cmd_phrase_lower = cmd_phrase.trim().to_lowercase();
                let cmd_phrase_chars: Vec<char> = cmd_phrase_lower.chars().collect();

                // character-level similarity
                let char_ratio = ratio(&phrase_chars, &cmd_phrase_chars);

                // word-level similarity
                let cmd_words: Vec<&str> = cmd_phrase_lower.split_whitespace().collect();
                let word_score = word_overlap_score(&phrase_words, &cmd_words);

                // combined score
                let fuzzy_score = (char_ratio * 0.6) + (word_score * 0.4);
                let template_score = template_phrase_score(&phrase_words, &cmd_words);
                let score = fuzzy_score.max(template_score);

                // early exit on perfect match
                if score >= 99.0 {
                    debug!("Perfect match: '{}' -> '{}'", phrase, cmd_phrase_lower);
                    return Some((&cmd_list.path, cmd));
                }

                if score > best_score {
                    best_score = score;
                    result = Some((&cmd_list.path, cmd));
                }
            }
        }
    }

    if let Some((_, cmd)) = result {
        info!(
            "Fuzzy match: '{}' -> cmd '{}' (score: {:.1}%)",
            phrase, cmd.id, best_score
        );
    } else {
        debug!("No match for '{}' (best: {:.1}%)", phrase, best_score);
    }

    result
}

pub fn fetch_learned_alias_command<'a>(
    phrase: &str,
    commands: &'a [JCommandsList],
) -> Option<(&'a PathBuf, &'a JCommand)> {
    let lang = i18n::get_language();
    let phrase = normalize_phrase(phrase);
    if phrase.is_empty() {
        return None;
    }

    fetch_learned_command(&phrase, &lang, commands)
}

pub fn learn_alias(phrase: &str, command_id: &str, lang: &str) -> Result<(), String> {
    let phrase = normalize_phrase(phrase);
    if phrase.is_empty() {
        return Err("Cannot learn an empty phrase".to_string());
    }

    if command_id.trim().is_empty() {
        return Err("Cannot learn an alias for an empty command id".to_string());
    }

    let mut learned = load_learned_commands();
    if let Some(existing) = learned
        .aliases
        .iter_mut()
        .find(|alias| alias.lang == lang && normalize_phrase(&alias.phrase) == phrase)
    {
        existing.command_id = command_id.to_string();
        existing.created_at_unix = current_unix_timestamp();
    } else {
        learned.aliases.push(LearnedCommandAlias {
            phrase,
            command_id: command_id.to_string(),
            lang: lang.to_string(),
            created_at_unix: current_unix_timestamp(),
        });
    }

    save_learned_commands(&learned)
}

fn fetch_learned_command<'a>(
    phrase: &str,
    lang: &str,
    commands: &'a [JCommandsList],
) -> Option<(&'a PathBuf, &'a JCommand)> {
    let phrase_chars: Vec<char> = phrase.chars().collect();
    let mut best: Option<(String, f64)> = None;

    for alias in load_learned_commands().aliases {
        if alias.lang != lang {
            continue;
        }

        let alias_phrase = normalize_phrase(&alias.phrase);
        if alias_phrase.is_empty() {
            continue;
        }

        let score = if alias_phrase == phrase {
            100.0
        } else {
            let alias_chars: Vec<char> = alias_phrase.chars().collect();
            ratio(&phrase_chars, &alias_chars)
        };

        if score >= 88.0
            && best
                .as_ref()
                .map(|(_, best_score)| score > *best_score)
                .unwrap_or(true)
        {
            best = Some((alias.command_id, score));
        }
    }

    let (command_id, score) = best?;
    let command = get_command_by_id(commands, &command_id);
    if command.is_some() {
        info!(
            "Learned alias match: '{}' -> cmd '{}' (score: {:.1}%)",
            phrase, command_id, score
        );
    } else {
        warn!(
            "Learned alias '{}' points to missing command id '{}'",
            phrase, command_id
        );
    }

    command
}

fn learned_commands_path() -> Option<PathBuf> {
    APP_CONFIG_DIR
        .get()
        .map(|dir| dir.join(config::LEARNED_COMMANDS_FILE_NAME))
}

fn load_learned_commands() -> LearnedCommandsFile {
    let Some(path) = learned_commands_path() else {
        return LearnedCommandsFile::default();
    };

    let Ok(content) = fs::read_to_string(&path) else {
        return LearnedCommandsFile::default();
    };

    match toml::from_str(&content) {
        Ok(file) => file,
        Err(e) => {
            warn!("Failed to parse learned commands {}: {}", path.display(), e);
            LearnedCommandsFile::default()
        }
    }
}

fn save_learned_commands(learned: &LearnedCommandsFile) -> Result<(), String> {
    let path = learned_commands_path().ok_or("Config directory is not initialized")?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create learned commands directory: {}", e))?;
    }

    let content = toml::to_string_pretty(learned)
        .map_err(|e| format!("Failed to serialize learned commands: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write learned commands {}: {}", path.display(), e))
}

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn normalize_phrase(phrase: &str) -> String {
    phrase
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn template_phrase_score(input_words: &[&str], cmd_words: &[&str]) -> f64 {
    let static_words: Vec<&str> = cmd_words
        .iter()
        .copied()
        .filter(|word| !(word.starts_with('{') && word.ends_with('}')))
        .collect();

    if static_words.len() == cmd_words.len() || static_words.is_empty() || input_words.is_empty() {
        return 0.0;
    }

    let matched = static_words
        .iter()
        .filter(|static_word| {
            let static_chars: Vec<char> = static_word.chars().collect();
            input_words.iter().any(|input_word| {
                let input_chars: Vec<char> = input_word.chars().collect();
                ratio(&input_chars, &static_chars) > 75.0
            })
        })
        .count();

    let score = (matched as f64 / static_words.len() as f64) * 100.0;
    if score >= 80.0 {
        score
    } else {
        0.0
    }
}

fn word_overlap_score(input_words: &[&str], cmd_words: &[&str]) -> f64 {
    if input_words.is_empty() || cmd_words.is_empty() {
        return 0.0;
    }

    let mut matched = 0.0;

    // pre-compute cmd word chars to avoid repeated allocations
    let cmd_word_chars: Vec<Vec<char>> = cmd_words.iter().map(|w| w.chars().collect()).collect();

    for input_word in input_words {
        let input_chars: Vec<char> = input_word.chars().collect();

        let best_word_match = cmd_word_chars
            .iter()
            .map(|cw| ratio(&input_chars, cw))
            .fold(0.0_f64, f64::max);

        if best_word_match > 70.0 {
            matched += best_word_match / 100.0;
        }
    }

    let max_words = input_words.len().max(cmd_words.len()) as f64;
    (matched / max_words) * 100.0
}

pub fn execute_exe(exe: &str, args: &[String]) -> std::io::Result<Child> {
    Command::new(exe).args(args).spawn()
}

pub fn execute_cli(cmd: &str, args: &[String]) -> std::io::Result<Child> {
    debug!("Spawning: cmd /C {} {:?}", cmd, args);

    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(cmd).args(args).spawn()
    } else {
        Command::new("sh").arg("-c").arg(cmd).args(args).spawn()
    }
}

pub fn execute_command(
    cmd_path: &PathBuf,
    cmd_config: &JCommand,
    phrase: Option<&str>,
    slots: Option<&HashMap<String, SlotValue>>,
) -> Result<bool, String> {
    // execute command by the type
    match cmd_config.cmd_type.as_str() {
        // BRUH
        "voice" => Ok(true),

        // LUA command
        #[cfg(feature = "lua")]
        "lua" => execute_lua_command(cmd_path, cmd_config, phrase, slots),

        // AutoHotkey command
        // @TODO: Consider adding ahk source files execution?
        "ahk" => {
            let exe_path_absolute = Path::new(&cmd_config.exe_path);
            let exe_path_local = cmd_path.join(&cmd_config.exe_path);

            let exe_path = if exe_path_absolute.exists() {
                exe_path_absolute
            } else {
                exe_path_local.as_path()
            };

            execute_exe(exe_path.to_str().unwrap(), &cmd_config.exe_args)
                .map(|_| true)
                .map_err(|e| format!("AHK process spawn error: {}", e))
        }

        // CLI command type
        // @TODO: Consider security restrictions
        "cli" => execute_cli(&cmd_config.cli_cmd, &cmd_config.cli_args)
            .map(|_| true)
            .map_err(|e| format!("CLI command error: {}", e)),

        // TERMINATOR command (T1000)
        "terminate" => {
            std::thread::sleep(Duration::from_secs(2));
            std::process::exit(0);
        }

        // STOP CHANING
        "stop_chaining" => Ok(false),

        // other
        _ => {
            error!("Command type unknown: {}", cmd_config.cmd_type);
            Err(format!("Command type unknown: {}", cmd_config.cmd_type).into())
        }
    }
}

// look up a command by its ID
pub fn get_command_by_id<'a>(
    commands: &'a [JCommandsList],
    id: &str,
) -> Option<(&'a PathBuf, &'a JCommand)> {
    for cmd_list in commands {
        for cmd in &cmd_list.commands {
            if cmd.id == id {
                return Some((&cmd_list.path, cmd));
            }
        }
    }
    None
}

pub fn list_paths(commands: &[JCommandsList]) -> Vec<&Path> {
    commands.iter().map(|x| x.path.as_path()).collect()
}

#[cfg(feature = "lua")]
fn execute_lua_command(
    cmd_path: &PathBuf,
    cmd_config: &JCommand,
    phrase: Option<&str>,
    slots: Option<&HashMap<String, SlotValue>>,
) -> Result<bool, String> {
    // get script path

    let script_name = if cmd_config.script.is_empty() {
        "script.lua"
    } else {
        &cmd_config.script
    };

    let script_path = cmd_path.join(script_name);

    if !script_path.exists() {
        return Err(format!("Lua script not found: {}", script_path.display()));
    }

    // parse sandbox level
    let sandbox = SandboxLevel::from_str(&cmd_config.sandbox);

    // create context
    let context = CommandContext {
        phrase: phrase.unwrap_or("").to_string(),
        command_id: cmd_config.id.clone(),
        command_path: cmd_path.clone(),
        language: i18n::get_language(),
        slots: slots.map(|s| s.clone()),
    };

    // get timeout
    let timeout = Duration::from_millis(cmd_config.timeout);

    info!(
        "Executing Lua command: {} (sandbox: {:?}, timeout: {:?})",
        cmd_config.id, sandbox, timeout
    );

    // execute
    match lua::execute(&script_path, context, sandbox, timeout) {
        Ok(result) => {
            info!(
                "Lua command {} completed (chain: {})",
                cmd_config.id, result.chain
            );
            Ok(result.chain)
        }
        Err(e) => {
            error!("Lua command {} failed: {}", cmd_config.id, e);
            Err(e.to_string())
        }
    }
}
