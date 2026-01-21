use std::path::PathBuf;

pub fn play_sound(sound_path: &PathBuf) -> Result<(), String> {
    if !sound_path.exists() {
        return format!("Sound file not found: {:?}", sound_path);
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("afplay")
            .arg(sound_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("powershell")
            .args(&["-c", &(format!(r#"(New-Object Media.SoundPlayer '{}').PlaySync()"#, sound_path.to_string_lossy()))])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("aplay")
            .arg(sound_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn get_resource_sound_path(sound_name: &str) -> PathBuf {
    PathBuf::from(format!("../src-tauri/resources/sounds/{}", sound_name))
}

pub fn play_default_beep() -> Result<(), String> {
    // Play system default beep sound as fallback
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("afplay")
            .arg("/System/Library/Sounds/Ping.aiff")
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("powershell")
            .args(&["-c", "[console]::beep(800,500)"])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("beep")
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn play_completion_sound() -> Result<(), String> {
    let sound_path = get_resource_sound_path("complete.mp3");

    if sound_path.exists() {
        play_sound(&sound_path)
    } else {
        // Fallback to system beep if custom sound not available
        play_default_beep()
    }
}

pub fn play_tick_sound() -> Result<(), String> {
    let sound_path = get_resource_sound_path("tick.mp3");

    if sound_path.exists() {
        play_sound(&sound_path)
    } else {
        play_default_beep()
    }
}
