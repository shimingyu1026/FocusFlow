use std::path::PathBuf;

pub fn play_sound(sound_path: &PathBuf) -> Result<(), String> {
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

pub fn play_completion_sound() -> Result<(), String> {
    let sound_path = get_resource_sound_path("complete.mp3");
    play_sound(&sound_path)
}

pub fn play_tick_sound() -> Result<(), String> {
    let sound_path = get_resource_sound_path("tick.mp3");
    play_sound(&sound_path)
}
