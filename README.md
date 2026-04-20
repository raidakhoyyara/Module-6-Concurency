# Rust Web Server — Reflection Notes

## Milestone 1 — Handle Connection & Check Response
Saya mengimplementasikan web server single-threaded sederhana menggunakan Rust yang menerima koneksi TCP di `127.0.0.1:7878`. Saya menggunakan `TcpListener::bind()` untuk mengikat server ke alamat tersebut, lalu `incoming()` untuk mengiterasi koneksi masuk secara sekuensial dalam sebuah loop.

Untuk membaca request, saya membungkus `TcpStream` dengan `BufReader` agar bisa membaca baris demi baris secara efisien. HTTP request di-parse dengan mengumpulkan baris-baris tersebut hingga menemukan blank line, yang menandai akhir dari headers sesuai protokol HTTP/1.1.

- Browser sering mengirim beberapa koneksi sekaligus untuk favicon, retry, dll
- "Refused to connect" berarti tidak ada proses yang listen di port tersebut, sedangkan "didn't send any data" berarti koneksi berhasil tapi server belum mengirim response yang memang terjadi di milestone ini karena belum ada logic untuk menulis response



---

## Milestone 2 — Returning HTML
![Commit 2 screen capture](assets/milestone2.png)
Saya memodifikasi fungsi `handle_connection` agar server bisa mengirimkan HTTP response yang valid dan bisa di-render browser. Saya menggunakan `fs::read_to_string()` untuk membaca file `hello.html` dari disk, lalu membuat response string menggunakan `format!()`.

Response yang saya buat mengikuti struktur HTTP/1.1:
- **Status line**: `HTTP/1.1 200 OK`
- **Header**: `Content-Length: {length}`
- **Blank line + Body**: `\r\n\r\n` sebagai pemisah antara header dan body HTML

Saya menggunakan `stream.write_all(response.as_bytes())` untuk mengirim response sebagai raw bytes ke TCP stream. Yang menarik, server tidak perlu memahami CSS atau JavaScript sama sekali, server hanya mengirim raw HTML, dan browser yang bertanggung jawab untuk parsing dan mengambil resource eksternal seperti Google Fonts.
