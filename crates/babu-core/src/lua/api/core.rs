// Core Lua API: log, sleep, print, etc.

use mlua::{Lua, MultiValue, Table};

pub fn register(lua: &Lua, babu: &Table) -> mlua::Result<()> {
    // @ babu.log(level, message)
    // log something
    let log_fn = lua.create_function(|_, (level, message): (String, String)| {
        match level.to_lowercase().as_str() {
            "debug" => log::debug!("[Lua] {}", message),
            "info" => log::info!("[Lua] {}", message),
            "warn" => log::warn!("[Lua] {}", message),
            "error" => log::error!("[Lua] {}", message),
            _ => log::info!("[Lua] {}", message),
        }
        Ok(())
    })?;
    babu.set("log", log_fn)?;

    // @ babu.print(...)
    // simple print
    let print_fn = lua.create_function(|_, args: MultiValue| {
        let parts: Vec<String> = args.iter().map(|v| format!("{:?}", v)).collect();
        log::info!("[Lua] {}", parts.join(" "));
        Ok(())
    })?;
    babu.set("print", print_fn)?;

    // @ babu.sleep(ms)
    // ..zZZ
    let sleep_fn = lua.create_function(|_, ms: u64| {
        std::thread::sleep(std::time::Duration::from_millis(ms));
        Ok(())
    })?;
    babu.set("sleep", sleep_fn)?;

    // @ babu.speak(text)
    // @TODO: update when TTS will be implemented
    let speak_fn = lua.create_function(|_, text: String| {
        log::info!("[Lua] SPEAK: {}", text);
        // pass
        Ok(())
    })?;
    babu.set("speak", speak_fn)?;

    Ok(())
}
