# 📓 Catatan Harian

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=flat&logo=rust)
![Status](https://img.shields.io/badge/Status-Completed-success)
![License](https://img.shields.io/badge/License-MIT-blue)

Program Rust sederhana yang mensimulasikan satu sesi menulis catatan harian ke terminal — lengkap dengan metadata jam, lokasi, tanggal, dan isi catatan. Project ini dibangun sebagai latihan fundamental Rust dengan fokus pada **function**, **return value**, dan **ownership saat passing parameter**.

---

## 📋 Tentang Project

Terinspirasi dari kebiasaan menulis catatan harian di kertas — selalu menyertakan jam, lokasi, dan tanggal sebagai identitas catatan. Program ini merepresentasikan satu sesi menulis tersebut dalam bentuk output terminal yang terformat rapi.

### 🎯 Tujuan Pembelajaran
Project ini dirancang untuk melatih pemahaman materi dasar Rust:
- ✅ **Function dengan parameter dan return value** (`-> String`)
- ✅ **Expression vs Statement** — return value tanpa keyword `return`
- ✅ **Ownership saat passing parameter** ke function
- ✅ **`format!` macro** untuk menggabungkan berbagai tipe data menjadi `String`
- ✅ **Perbedaan `&str` dan `String`** dalam konteks parameter function
- ✅ Separation of concern — logika di function, presentasi di `main`

---

## 🚀 Fitur

- **Metadata Lengkap**: Mencatat jam, lokasi, tanggal, bulan, dan tahun secara terstruktur.
- **Header Terformat**: Menggabungkan semua metadata menjadi satu baris header yang rapi.
- **Tiga Function Terpisah**: Setiap function punya tanggung jawab tunggal — format waktu, format tanggal, dan format header.
- **Output Bersih**: Laporan catatan ditampilkan terformat ke terminal.

---

## 🛠️ Tech Stack

| Komponen | Keterangan |
|----------|------------|
| **Bahasa** | Rust |
| **Tipe Data** | `String`, `&str`, `u8`, `u16` |
| **Konsep Utama** | Function, Return Value, Ownership, Borrowing |
| **Macro** | `format!`, `println!` |

---

## 📥 Instalasi & Penggunaan

### Prasyarat
Pastikan Rust sudah terinstal di sistem Anda:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone Repository
```bash
git clone https://github.com/KyraWillow/catatan_harian.git
cd catatan_harian
```

### Jalankan Program
```bash
cargo run
```

### Build Release
```bash
cargo build --release
```

---

## 📊 Contoh Output

```text
============================================
[3 April 2026 | 6:13 AM WIB | Boteng, menganti Gresik]
============================================
Lorem Ipsum is simply dummy text of the printing and
typesetting industry...
============================================
```

---

## 🧠 Poin Teknis Penting

### 1. Expression sebagai Return Value
Semua function mengembalikan nilai menggunakan **expression** tanpa keyword `return` — ini cara idiomatik di Rust:
```rust
fn format_waktu(jam: &str) -> String {
    format!("{jam} WIB") // tanpa `;` = expression = return value
}
```

### 2. `format!` vs `println!`
Keduanya mirip secara sintaks, tapi berbeda tujuan:
```rust
println!("{jam} WIB"); // mencetak ke terminal, tidak menghasilkan nilai
format!("{jam} WIB");  // menghasilkan String, tidak mencetak apapun
```

### 3. Ownership saat Passing Parameter
`String` kehilangan ownership saat dikirim ke function — berbeda dengan `&str` yang hanya meminjam:
```rust
fn format_header(tanggal: String, waktu: String, lokasi: String) -> String {
    format!("[{tanggal} | {waktu} | {lokasi}]")
}
```

### 4. Separation of Concern
Logika penggabungan string sepenuhnya ada di dalam function — `main` hanya memanggil dan mencetak:
```rust
fn main() {
    let waktu_format: String = format_waktu(jam);
    let tanggal_format: String = format_tanggal(tanggal, bulan, tahun);
    let header: String = format_header(tanggal_format, waktu_format, lokasi);
    println!("{header}");
}
```

---

## 📁 Struktur Project

```
catatan_harian/
├── src/
│   └── main.rs          # Source code utama
├── Cargo.toml           # Manifest file
└── README.md            # Dokumentasi project
```

---

## 👤 Penulis

**KyraWillow**
- GitHub: [@KyraWillow](https://github.com/KyraWillow)
- Repository: [catatan_harian](https://github.com/KyraWillow/catatan_harian)

---

## 📄 Lisensi

Project ini dilisensikan di bawah [MIT License](LICENSE).

---

⭐ *Jika project ini membantu pembelajaran Anda, jangan lupa berikan star!*