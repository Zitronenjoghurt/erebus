pub fn format_byte_size(bytes: usize) -> String {
    if bytes < 1_000 {
        format!("{} B", bytes)
    } else if bytes < 1_000_000 {
        format!("{} kB", bytes / 1000)
    } else if bytes < 1_000_000_000 {
        format!("{} MB", bytes / 1_000_000)
    } else {
        format!("{} GB", bytes / 1_000_000_000)
    }
}
