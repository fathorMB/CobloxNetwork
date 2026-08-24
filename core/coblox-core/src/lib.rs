//! Shared, platform-neutral foundation for Coblox services and native shells.

/// Returns the semantic version exposed by every native shell.
#[must_use]
pub const fn core_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn exposes_the_package_version() {
        assert_eq!(super::core_version(), env!("CARGO_PKG_VERSION"));
    }
}
