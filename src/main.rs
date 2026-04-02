fn main() {
    let jam: &str = "6:13 AM";
    let lokasi: String = String::from("Boteng, menganti Gresik");
    let tanggal: u8 = 3;
    let bulan: &str = "April";
    let tahun: u16 = 2026;
    let isi: String = String::from(
        "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.",
    );

    let waktu_format: String = format_waktu(jam);
    let tanggal_format: String = format_tanggal(tanggal, bulan, tahun);
    let header: String = format_header(tanggal_format, waktu_format, lokasi);
    println!("============================================");
    println!("{header}");
    println!("============================================");
    println!("{isi}");
    println!("============================================");
}

fn format_waktu(jam: &str) -> String {
    format!("{jam} WIB")
}

fn format_tanggal(tanggal: u8, bulan: &str, tahun: u16) -> String {
    format!("{tanggal} {bulan} {tahun}")
}

fn format_header(tanggal: String, waktu: String, lokasi: String) -> String {
    format!("[{tanggal} | {waktu} | {lokasi}]")
}
