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
    const GB: u128 = u128::pow(MB, 2);
    const TB: u128 = u128::pow(GB, 2);

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
