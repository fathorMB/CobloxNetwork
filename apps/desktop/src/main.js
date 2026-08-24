import { invoke } from "@tauri-apps/api/core";

document.querySelector("#core-version").textContent = await invoke("core_version");
