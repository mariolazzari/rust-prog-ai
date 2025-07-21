enum FileSize {
    Bytes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(b) => format!("{} bytes", b),
            FileSize::KiloBytes(kb) => format!("{:.2} kb", *kb as f64 / 1000.0),
            FileSize::MegaBytes(mb) => format!("{:.2} Mb", *mb as f64 / 1000.0),
            FileSize::GigaBytes(gb) => format!("{:.2} Gb", *gb as f64 / 1000.0),
        }
    }
}

fn main() {
    let size = 123400000001234;
    let file_size = match size {
        0..=999 => FileSize::Bytes(size),
        1_000..=999_999 => FileSize::KiloBytes(size / 1_000),
        1_000_000..=999_999_999 => FileSize::MegaBytes(size / 1_000_000),
        _ => FileSize::GigaBytes(size / 1_000_000_000),
    };

    println!("File size: {}", file_size.format_size());
}
