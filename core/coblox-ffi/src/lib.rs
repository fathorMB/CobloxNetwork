//! Stable foreign-function boundary for the Coblox core.

/// Returns the version of the shared Rust core.
#[must_use]
pub fn core_version() -> String {
    coblox_core::core_version().to_owned()
}

uniffi::include_scaffolding!("coblox_ffi");

#[cfg(test)]
mod tests {
    #[test]
    fn ffi_reports_the_core_version() {
        assert_eq!(super::core_version(), coblox_core::core_version());
    }
}
