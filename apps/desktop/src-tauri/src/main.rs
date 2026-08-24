#[tauri::command]
fn core_version() -> &'static str {
    coblox_core::core_version()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![core_version])
        .run(tauri::generate_context!())
        .expect("failed to run Coblox desktop shell");
}

#[cfg(test)]
mod tests {
    #[test]
    fn desktop_command_reads_the_shared_core_version() {
        println!("Coblox desktop core version: {}", super::core_version());
        assert_eq!(super::core_version(), coblox_core::core_version());
    }
}
