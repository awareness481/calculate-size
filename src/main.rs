use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let bytes_vec = std::fs::read(filename)?;
    let bytes = bytes_vec.len() as u128;

    dbg!(&bytes);
    println!("{}", calculate_size(bytes));
    Ok(())
}
fn calculate_size(bytes: u128) -> String {
    const KB: u128 = 1024;
    const MB: u128 = u128::pow(KB, 2);
    const GB: u128 = MB * KB;
    const TB: u128 = GB * KB;

    if bytes < KB {
        return format!("{} B", bytes);
    }
    if bytes < MB {
        return format!("{},{} KB", bytes / KB, remainder(bytes, KB));
    }
    if bytes < GB {
        return format!("{},{} MB", bytes / MB, remainder(bytes, MB) / KB);
    }
    if bytes < TB {
        return format!("{},{} GB", bytes / GB, remainder(bytes, GB) / MB);
    }

    format!("{},{} TB", bytes / TB, remainder(bytes, TB) / GB)
}

fn remainder(bytes: u128, divisor: u128) -> u128 {
    bytes % divisor
}

#[cfg(test)]
mod tests {
    #[test]
    fn bytes() {
        let mut bytes = super::calculate_size(1024);
        assert_eq!(bytes, "1,0 KB");

        bytes = super::calculate_size(1023);
        assert_eq!(bytes, "1023 B");
    }

    #[test]
    fn kilobytes() {
        let mut bytes = super::calculate_size(1024 * 1024);
        assert_eq!(bytes, "1,0 MB");

        bytes = super::calculate_size(1024 * 1023);
        assert_eq!(bytes, "1023,0 KB");
    }

    #[test]
    fn megabytes() {
        let mut bytes = super::calculate_size(1024 * 1024 * 1024);
        assert_eq!(bytes, "1,0 GB");

        bytes = super::calculate_size(1024 * 1024 * 1023);
        assert_eq!(bytes, "1023,0 MB");
    }

    #[test]
    fn gigabytes() {
        let mut bytes = super::calculate_size(1024 * 1024 * 1024 * 1024);
        assert_eq!(bytes, "1,0 TB");

        bytes = super::calculate_size(1024 * 1024 * 1024 * 1023);
        assert_eq!(bytes, "1023,0 GB");
    }

    #[test]
    fn terabytes() {
        let bytes = super::calculate_size(1024 * 1024 * 1024 * 1024 * 1023);
        assert_eq!(bytes, "1023,0 TB");
    }
}
