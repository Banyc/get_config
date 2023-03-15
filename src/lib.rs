pub mod toml;

fn config_path(default: &str) -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| default.to_string())
}
