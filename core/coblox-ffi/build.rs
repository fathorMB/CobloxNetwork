fn main() {
    uniffi::generate_scaffolding("src/coblox_ffi.udl")
        .expect("UniFFI scaffolding generation must succeed");
}
