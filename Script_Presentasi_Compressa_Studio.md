# Script Presentasi Compressa Studio

## Cara memakai script ini

Script ini tidak harus dihafal kata per kata. Bagian **Script bicara** bisa dipakai kalau ingin latihan alur presentasi. Bagian **Keypoints** dipakai sebagai pegangan kalau lupa kalimat, jadi cukup ingat poinnya lalu jelaskan dengan bahasa sendiri.

Durasi ideal presentasi: sekitar 8 sampai 12 menit, tergantung seberapa lama demo kompresi berjalan.

Catatan saat demo: jangan menyebut angka hasil kompresi secara asal. Pakai angka yang muncul di panel **Statistics**, seperti original size, output size, space saved, compression ratio, dan elapsed time.

---

## 1. Pembuka

### Script bicara

Selamat pagi/siang Pak/Bu. Pada kesempatan ini saya akan mempresentasikan aplikasi yang saya buat, namanya **Compressa Studio**.

Aplikasi ini adalah aplikasi desktop untuk kompresi file. Fokusnya bukan hanya mengecilkan ukuran file, tapi juga memperlihatkan konsep kompresi data yang sudah dipelajari di materi, seperti kompresi lossless, kompresi lossy, entropy coding, dictionary coding, sampai kompresi media seperti gambar, PDF, dan video.

Jadi ide utamanya adalah: user memilih file, aplikasi mendeteksi jenis filenya, lalu aplikasi memilih jalur kompresi yang sesuai. Setelah proses selesai, aplikasi menampilkan hasilnya secara langsung, seperti ukuran awal, ukuran hasil, rasio kompresi, waktu proses, lokasi output, dan checksum.

### Keypoints

- Nama aplikasi: Compressa Studio.
- Aplikasi desktop untuk kompresi file.
- Tujuan: menghubungkan teori kompresi data dengan praktik langsung.
- Alur besar: pilih file, deteksi tipe file, kompresi sesuai kategori, tampilkan hasil.

---

## 2. Kenapa Saya Membuat Aplikasi Ini

### Script bicara

Alasan saya membuat aplikasi ini karena materi kompresi data sebenarnya cukup luas, tapi kalau hanya dijelaskan lewat teori, kadang sulit dibayangkan hasil nyatanya.

Misalnya, kita sering mendengar istilah lossless dan lossy. Secara teori lossless berarti data bisa dikembalikan persis seperti semula, sedangkan lossy berarti ada sebagian detail yang dibuang supaya ukuran bisa lebih kecil. Tapi perbedaannya akan lebih mudah dipahami kalau langsung dicoba dengan file asli.

Dari situ saya membuat aplikasi ini sebagai media praktik. Aplikasi ini bisa dipakai untuk melihat bagaimana file gambar, video, PDF, dan file generic diproses dengan pendekatan yang berbeda. Jadi bukan satu algoritma dipaksakan untuk semua file, tapi tiap jenis file diarahkan ke pipeline yang lebih cocok.

Selain itu, aplikasi ini juga saya buat karena masalah ukuran file memang sering ditemui. File PDF materi kuliah bisa besar, video bisa sangat besar, gambar juga bisa memakan banyak ruang, dan file dokumen tertentu kadang perlu dikompresi tanpa kehilangan data. Jadi aplikasi ini dibuat untuk kebutuhan praktis sekaligus untuk pembelajaran.

### Keypoints

- Teori kompresi lebih mudah dipahami kalau dipraktikkan.
- Menunjukkan perbedaan lossless dan lossy secara langsung.
- Setiap jenis file butuh pendekatan kompresi berbeda.
- Masalah file besar sering ditemui di kehidupan sehari-hari.
- Aplikasi ini berfungsi sebagai alat bantu praktik dan pembelajaran.

---

## 3. Saya Membuat Apa dan Dengan Apa

### Script bicara

Secara teknis, aplikasi ini saya buat sebagai aplikasi desktop.

Untuk tampilannya, saya menggunakan **React** dan **TypeScript**. React dipakai untuk membuat tampilan yang interaktif, sedangkan TypeScript membantu supaya struktur data dan state aplikasi lebih jelas.

Untuk desktop runtime-nya, saya menggunakan **Tauri**. Jadi tampilannya memang berbasis web, tapi aplikasinya berjalan sebagai aplikasi desktop. Bagian proses file dan kompresinya dikerjakan di backend menggunakan **Rust**.

Saya memilih Rust karena Rust cocok untuk operasi file dan pengolahan data byte-level. Kompresi itu banyak berhubungan dengan membaca byte, menulis byte, menghitung ukuran, dan memastikan data hasilnya benar, jadi Rust cukup sesuai untuk kebutuhan ini.

Untuk video, aplikasi memakai **FFmpeg** karena video codec seperti H.264 dan audio AAC memang kompleks dan FFmpeg sudah menjadi tool standar. Untuk PDF, aplikasi memakai **Ghostscript** karena Ghostscript kuat untuk optimasi PDF, terutama untuk downsampling gambar dan optimasi struktur PDF.

Untuk generic lossless compression, aplikasi mendukung beberapa algoritma. Ada **Zstd** untuk kompresi praktis yang cepat dan efisien. Lalu ada algoritma edukatif seperti **RLE, Shannon, Shannon-Fano, Huffman, LZ77, LZ78, LZW, dan Arithmetic Coding**. Beberapa algoritma ini dibuat supaya konsep dari materi kuliah bisa terlihat langsung di aplikasi.

### Keypoints

- Frontend: React + TypeScript.
- Desktop app: Tauri.
- Backend dan proses kompresi: Rust.
- Video: FFmpeg.
- PDF: Ghostscript.
- Generic lossless: Zstd, RLE, Shannon, Shannon-Fano, Huffman, LZ77, LZ78, LZW, Arithmetic.

---

## 4. Alur Kerja Aplikasi

### Script bicara

Alur kerja aplikasi ini dibuat sederhana supaya user tidak perlu memilih banyak hal dari awal.

Pertama, user memilih file. File bisa dipilih lewat tombol **Choose File**, lewat menu **Quick Start**, atau drag and drop di upload modal.

Kedua, aplikasi membaca informasi file. Di sini aplikasi mengambil nama file, path, ekstensi, ukuran, kategori file, dan SHA-256. SHA-256 ini berguna untuk identitas file, terutama ketika membuktikan bahwa hasil decompression lossless benar-benar sama dengan file aslinya.

Ketiga, aplikasi menentukan kategori file. Kalau gambar, diarahkan ke image compression. Kalau video, diarahkan ke video compression. Kalau PDF, diarahkan ke PDF compression. Selain itu masuk ke generic lossless compression.

Keempat, user bisa mengatur preset atau algoritma sesuai kategori. Setelah itu user menekan tombol **Compress**.

Kelima, setelah selesai, aplikasi menampilkan hasil di panel **Statistics** dan mencatat prosesnya di **Logs**.

### Keypoints

- Pilih file.
- Aplikasi baca metadata dan SHA-256.
- File dideteksi otomatis.
- Setting menyesuaikan kategori file.
- Klik Compress.
- Hasil muncul di Statistics dan Logs.

---

## 5. Fitur Utama Secara Ringkas

### Script bicara

Fitur utama aplikasi ini ada beberapa.

Pertama, **auto detect file type**. Aplikasi bisa mengenali apakah file termasuk image, video, PDF, atau generic.

Kedua, **image compression**. Untuk gambar, aplikasi bisa mengatur kualitas, resize percentage, dan pilihan encoder JPEG. Kompresi gambar ini bersifat lossy, jadi tujuannya mengecilkan ukuran dengan tetap menjaga visual tetap layak.

Ketiga, **video compression**. Untuk video, aplikasi memakai FFmpeg untuk re-encode ke H.264 dan AAC. Di sini preset kualitas mempengaruhi CRF, speed, dan bitrate audio.

Keempat, **PDF compression**. Untuk PDF, aplikasi memakai Ghostscript dengan preset Screen, Ebook, dan Print.

Kelima, **generic lossless compression**. Ini untuk file seperti TXT, CSV, JSON, DOCX, PPTX, XLSX, BIN, dan file generic lain. Karena lossless, hasilnya bisa didekompres kembali menjadi file asli.

Keenam, **decompression untuk output generic**. Misalnya file `.zst`, `.rle`, `.shnc`, `.sfc`, `.huff`, `.lz77`, `.lz78`, `.lzw`, atau `.arith` bisa dipilih lagi lalu didekompres.

Ketujuh, **statistics dan checksum**. Aplikasi menampilkan ukuran sebelum dan sesudah, rasio, waktu proses, output path, SHA-256, dan integrity check jika tersedia.

### Keypoints

- Auto detect file.
- Image compression.
- Video compression.
- PDF compression.
- Generic lossless compression.
- Decompression untuk output lossless.
- Statistik hasil dan SHA-256.
- Log proses.

---

## 6. Penjelasan Tampilan Aplikasi

### Script bicara

Sekarang saya jelaskan bagian tampilan aplikasinya.

Di bagian atas ada **topbar**. Di sebelah kiri ada nama aplikasi **Compressa Studio**, dan di sebelah kanan ada status aplikasi, misalnya Idle, Compressing, Decompressing, Success, atau Error. Di kanan juga ada tombol **Logs** untuk melihat riwayat proses.

Di bawah topbar ada **status strip**. Bagian ini menampilkan pesan kondisi aplikasi, misalnya file sudah dipilih, proses kompresi sedang berjalan, atau kompresi berhasil.

Di tampilan awal ada bagian **Quick Start**. Di sini user bisa memilih mode awal, yaitu Auto Detect, Image, Video, PDF, atau Generic. Kalau memilih Auto Detect, aplikasi akan otomatis menentukan pipeline berdasarkan file yang dipilih.

Panel **File Selection** menampilkan detail file yang dipilih. Di sini ada nama file, path, ekstensi, kategori yang terdeteksi, ukuran asli, dan SHA-256. Kalau kategori yang terdeteksi dirasa kurang tepat, ada opsi override mode, tapi tetap ada pengecekan supaya file tidak diproses dengan pipeline yang berbahaya atau tidak sesuai.

Panel **Compression Mode** menampilkan setting sesuai kategori file. Jadi isi panelnya berubah tergantung file yang dipilih. Kalau gambar, muncul preset kualitas, resize percentage, dan JPEG encoder. Kalau PDF, muncul PDF preset. Kalau generic, muncul pilihan algoritma.

Panel **Actions** berisi tombol utama: Choose File, Compress, Decompress, dan Clear.

Panel **Statistics** muncul setelah proses berhasil. Panel ini penting karena di sinilah hasil kompresi bisa dibuktikan dengan angka.

Terakhir ada **Algorithm Explanation**. Panel ini menjelaskan konsep algoritma yang sedang digunakan. Jadi aplikasi ini tidak hanya memproses file, tapi juga memberi penjelasan singkat konsep kompresinya.

### Keypoints

- Topbar: nama aplikasi, status, logs.
- Status strip: pesan kondisi aplikasi.
- Quick Start: Auto Detect, Image, Video, PDF, Generic.
- File Selection: metadata file dan SHA-256.
- Compression Mode: setting sesuai kategori.
- Actions: Choose File, Compress, Decompress, Clear.
- Statistics: hasil kompresi/dekompresi.
- Algorithm Explanation: penjelasan konsep algoritma.

---

## 7. Penjelasan Menu Dropdown dan Opsi Interaktif

### Script bicara

Di aplikasi ini ada beberapa dropdown dan menu interaktif.

Pertama, ada **Logs dropdown** di kanan atas. Isinya adalah operation history. Di situ aplikasi mencatat proses yang terjadi, misalnya aplikasi dimulai, file dipilih, kompresi dimulai, kompresi berhasil, atau error jika ada masalah.

Kedua, ada menu **Quick Start**. Ini bukan dropdown, tapi berupa pilihan mode cepat. Opsinya adalah Auto Detect, Image, Video, PDF, dan Generic.

Ketiga, di panel **File Selection** ada opsi **Active Mode**. Kalau file sudah terdeteksi, aplikasi menampilkan mode aktif. Jika user ingin mengganti mode, ada pilihan override ke image, video, PDF, atau generic. Ini berguna misalnya untuk file yang ingin diuji sebagai generic lossless.

Keempat, di panel **Compression Mode** ada dropdown **Preset**. Untuk image dan video, opsinya adalah **High quality**, **Balanced**, dan **Small size**. High quality menjaga kualitas lebih tinggi, Balanced menjadi pilihan tengah, dan Small size fokus mengecilkan ukuran.

Untuk PDF, dropdown preset berisi **Screen**, **Ebook**, dan **Print**. Screen biasanya paling kecil karena cocok untuk tampilan layar, Ebook menengah, dan Print lebih menjaga kualitas untuk kebutuhan cetak.

Untuk gambar, ada juga dropdown **JPEG Encoder**. Opsinya **Standard** dan **Native**. Standard memakai encoder standar dari library image, sedangkan Native adalah encoder JPEG dari Rust yang dibuat sendiri dari konsep seperti color conversion, DCT, quantization, dan Huffman coding.

Untuk generic, ada dropdown **Algorithm Choice**. Opsinya adalah Zstd, RLE, Shannon, Shannon-Fano, Huffman, LZ77, LZ78, LZW, dan Arithmetic Coding. Kalau memilih Zstd, akan muncul setting tambahan **Zstd Level** dari 1 sampai 19. Semakin tinggi level, biasanya proses bisa lebih lama tapi kompresinya bisa lebih agresif.

### Keypoints

- Logs dropdown: riwayat proses.
- Quick Start: Auto Detect, Image, Video, PDF, Generic.
- Active Mode override: image, video, PDF, generic.
- Image/video preset: High quality, Balanced, Small size.
- PDF preset: Screen, Ebook, Print.
- Image JPEG Encoder: Standard, Native.
- Generic Algorithm Choice: Zstd, RLE, Shannon, Shannon-Fano, Huffman, LZ77, LZ78, LZW, Arithmetic.
- Zstd Level: 1 sampai 19.

---

## 8. Demo Uji Coba Fitur

Bagian ini dipakai saat presentasi sambil menjalankan aplikasi. Gunakan file asli yang tersedia atau file contoh yang sudah disiapkan.

---

### 8.1 Demo Kompresi Foto atau Gambar

### Langkah demo

1. Klik **Image** di Quick Start, atau klik **Auto Detect**.
2. Pilih file gambar, misalnya JPG, PNG, atau WebP.
3. Tunjukkan panel **File Selection**.
4. Tunjukkan kategori terdeteksi sebagai image.
5. Atur preset, misalnya **Balanced** atau **Small size**.
6. Jika ingin terlihat jelas, turunkan **Resize Percentage**, misalnya 70% atau 80%.
7. Jelaskan pilihan **JPEG Encoder**.
8. Klik **Compress**.
9. Setelah selesai, buka panel **Statistics** dan tunjukkan hasilnya.

### Script bicara saat demo

Sekarang saya coba fitur kompresi gambar.

Saya pilih file gambar terlebih dahulu. Setelah dipilih, aplikasi langsung membaca informasi file. Di panel ini terlihat nama file, lokasi file, ekstensi, kategori yang terdeteksi, ukuran asli, dan SHA-256.

Untuk gambar, aplikasi menyediakan preset kualitas. Di sini saya gunakan preset **Balanced** supaya hasilnya masih terlihat layak, tapi ukuran tetap bisa turun. Saya juga bisa mengubah resize percentage. Kalau resize diturunkan, jumlah pixel ikut berkurang, sehingga ukuran file biasanya ikut turun.

Di bagian encoder, ada pilihan Standard dan Native. Standard memakai encoder dari library, sedangkan Native adalah encoder JPEG yang dibuat dari konsep dasar kompresi gambar, seperti DCT, quantization, dan Huffman coding.

Sekarang saya klik **Compress**.

Setelah selesai, panel **Statistics** muncul. Di sini saya bisa menunjukkan ukuran awal, ukuran hasil, saved size, compression ratio, space saved, elapsed time, dan output path. Jadi hasil kompresinya tidak hanya terlihat dari file output, tapi juga bisa dibuktikan dengan angka.

Untuk gambar, kompresinya bersifat lossy. Artinya, tujuan utamanya bukan mengembalikan file persis seperti semula, tapi mengurangi ukuran dengan membuang detail visual yang tidak terlalu terlihat.

### Keypoints

- Gambar masuk kategori image.
- Setting utama: preset, resize percentage, JPEG encoder.
- Image compression bersifat lossy.
- Output bisa JPG atau WebP tergantung kondisi gambar.
- Tunjukkan Statistics: original size, output size, space saved, elapsed time.
- Kalau ukuran tidak turun banyak, jelaskan bahwa file awal mungkin sudah terkompresi.

---

### 8.2 Demo Kompresi PDF

### Langkah demo

1. Klik **PDF** atau **Auto Detect**.
2. Pilih file PDF, misalnya `Materi/MPEG.pdf` jika ingin memakai file dari folder materi.
3. Tunjukkan kategori terdeteksi sebagai PDF.
4. Tunjukkan dropdown **Preset**: Screen, Ebook, Print.
5. Pilih **Ebook** untuk hasil yang seimbang.
6. Pastikan panel Ghostscript menunjukkan tersedia.
7. Klik **Compress**.
8. Setelah selesai, tunjukkan **Statistics** dan output path.

### Script bicara saat demo

Berikutnya saya coba kompresi PDF.

Saya pilih file PDF. Setelah file dipilih, aplikasi mendeteksi kategorinya sebagai PDF. Untuk PDF, aplikasi memakai Ghostscript. Karena itu di panel setting ada informasi apakah Ghostscript tersedia atau belum.

Preset PDF yang tersedia ada tiga: Screen, Ebook, dan Print. Screen biasanya menghasilkan ukuran paling kecil karena ditujukan untuk tampilan layar. Ebook berada di tengah, sedangkan Print lebih menjaga kualitas untuk kebutuhan cetak.

Di demo ini saya pilih **Ebook** supaya seimbang antara ukuran dan kualitas.

Sekarang saya klik **Compress**.

Setelah selesai, saya lihat panel **Statistics**. Di sini terlihat method yang dipakai, yaitu PDF compression dengan Ghostscript, ukuran awal, ukuran hasil, space saved, waktu proses, dan output path. Output-nya tetap file PDF, biasanya dengan nama yang mengandung `_compressed`.

Untuk PDF, proses ini tidak sama seperti generic lossless. PDF compression lebih fokus pada optimasi struktur PDF, downsampling gambar di dalam PDF, dan pengurangan metadata atau objek yang tidak perlu. Jadi tombol Decompress tidak dipakai untuk mengembalikan PDF ke file awal.

### Keypoints

- PDF memakai Ghostscript.
- Preset: Screen, Ebook, Print.
- Screen kecil, Ebook seimbang, Print lebih menjaga kualitas.
- Output tetap PDF.
- Tunjukkan Statistics.
- PDF compression bukan lossless round-trip melalui tombol Decompress.

---

### 8.3 Demo Kompresi Video

### Langkah demo

1. Klik **Video** atau **Auto Detect**.
2. Pilih file video, misalnya MP4, MOV, MKV, atau AVI.
3. Tunjukkan kategori terdeteksi sebagai video.
4. Tunjukkan preset **High quality**, **Balanced**, dan **Small size**.
5. Pastikan panel FFmpeg menunjukkan tersedia.
6. Pilih **Balanced** untuk demo.
7. Klik **Compress**.
8. Tunggu proses selesai.
9. Tunjukkan hasil di **Statistics**.

### Script bicara saat demo

Selanjutnya saya coba kompresi video.

Video biasanya ukurannya lebih besar karena berisi banyak frame dan audio. Untuk video, aplikasi memakai FFmpeg dan melakukan re-encode ke H.264 untuk video dan AAC untuk audio.

Di sini preset kualitas mempengaruhi parameter kompresi. High quality menjaga kualitas lebih tinggi, Balanced menjadi pilihan tengah, dan Small size lebih agresif mengecilkan ukuran.

Saya pilih **Balanced**. Di panel ini juga terlihat status FFmpeg. Kalau FFmpeg tidak tersedia, aplikasi akan memberi tahu bahwa tool-nya belum ditemukan. Jadi aplikasi tidak langsung gagal tanpa penjelasan.

Sekarang saya klik **Compress**.

Proses video bisa lebih lama dibanding gambar atau PDF, karena video harus diproses frame demi frame. Setelah selesai, saya bisa melihat hasilnya di panel **Statistics**. Di sini terlihat ukuran awal, ukuran output, rasio kompresi, waktu proses, dan output path.

Konsep pentingnya, video compression memanfaatkan dua jenis redundansi. Ada spatial redundancy di dalam frame, mirip seperti gambar. Ada juga temporal redundancy antar frame, karena banyak frame video yang mirip satu sama lain.

### Keypoints

- Video memakai FFmpeg.
- Output MP4 dengan H.264 dan AAC.
- Preset: High quality, Balanced, Small size.
- Video bisa lebih lama karena memproses frame.
- Konsep: spatial redundancy dan temporal redundancy.
- Tunjukkan Statistics setelah selesai.

---

### 8.4 Demo Opsional: Generic Lossless dan Decompression

Bagian ini opsional, tapi bagus untuk menunjukkan bahwa tombol **Decompress** benar-benar bekerja untuk output lossless.

### Langkah demo

1. Klik **Generic** atau **Auto Detect**.
2. Pilih file generic, misalnya TXT, CSV, JSON, DOCX, PPTX, XLSX, atau file dari folder `Materi`.
3. Pilih algoritma **Zstd dictionary/statistical coding**.
4. Biarkan **Zstd Level** di 6.
5. Klik **Compress**.
6. Setelah output `.zst` dibuat, aplikasi akan memilih output tersebut.
7. Karena file `.zst` dikenali sebagai output lossless, tombol **Decompress** aktif.
8. Klik **Decompress**.
9. Tunjukkan **Integrity Check** di Statistics.

### Script bicara saat demo

Saya tambahkan satu demo singkat untuk generic lossless, karena ini bagian yang membedakan lossless dan lossy.

Saya pilih file generic, misalnya dokumen atau PPTX. File seperti ini masuk ke generic lossless compression. Di sini saya pilih algoritma Zstd dengan level default 6.

Sekarang saya klik **Compress**. Output yang dibuat berekstensi `.zst`, dan aplikasi juga membuat metadata untuk menyimpan informasi file asli, termasuk SHA-256.

Setelah itu saya klik **Decompress**. Karena ini lossless, hasil decompression seharusnya sama persis dengan file awal. Di panel **Statistics**, bagian **Integrity Check** akan menunjukkan apakah checksum-nya cocok.

Kalau tertulis checksum match, artinya file hasil restore punya SHA-256 yang sama dengan file asli. Itu membuktikan bahwa proses lossless berhasil mengembalikan data secara bit-for-bit.

### Keypoints

- Generic compression bersifat lossless.
- Contoh output: `.zst`.
- Tombol Decompress aktif untuk output lossless.
- Metadata menyimpan SHA-256 file asli.
- Integrity Check membuktikan hasil restore sama dengan original.

---

## 9. Fitur yang Berjalan di Tampilan

### Script bicara

Di tampilan aplikasi ini, fitur yang sudah berjalan antara lain:

Pertama, pemilihan file melalui dialog desktop dan drag and drop di modal.

Kedua, deteksi jenis file otomatis berdasarkan ekstensi dan juga magic bytes jika tersedia.

Ketiga, panel informasi file yang menampilkan nama file, path, ekstensi, kategori, ukuran, dan SHA-256.

Keempat, panel setting yang berubah mengikuti kategori file. Jadi setting untuk image, video, PDF, dan generic tidak disamakan.

Kelima, proses kompresi untuk image, video, PDF, dan generic.

Keenam, proses decompression untuk output generic lossless.

Ketujuh, panel statistik hasil proses.

Kedelapan, logs dropdown untuk melihat riwayat proses.

Kesembilan, panel penjelasan algoritma yang berubah sesuai kategori atau algoritma yang dipilih.

Kesepuluh, pengecekan dependency eksternal seperti FFmpeg untuk video dan Ghostscript untuk PDF.

### Keypoints

- File chooser dan drag-drop.
- Auto detect.
- File metadata.
- Dynamic settings panel.
- Compress image, video, PDF, generic.
- Decompress generic lossless output.
- Statistics.
- Logs.
- Algorithm Explanation.
- Tool availability check.

---

## 10. Batasan Aplikasi

### Script bicara

Ada beberapa batasan yang perlu saya jelaskan juga.

Pertama, untuk video aplikasi membutuhkan FFmpeg. Kalau FFmpeg belum terpasang di sistem, aplikasi akan menampilkan bahwa tool tersebut missing.

Kedua, untuk PDF aplikasi membutuhkan Ghostscript. Kalau Ghostscript belum tersedia, PDF compression tidak bisa dijalankan.

Ketiga, tidak semua file pasti menjadi lebih kecil. Kalau file awal sudah sangat terkompresi, misalnya file gambar tertentu, video tertentu, atau dokumen yang sudah berbentuk ZIP internal seperti PPTX dan DOCX, hasil kompresi bisa turun sedikit atau bahkan bertambah.

Keempat, tombol Decompress hanya untuk output generic lossless seperti `.zst`, `.rle`, `.shnc`, `.sfc`, `.huff`, `.lz77`, `.lz78`, `.lzw`, dan `.arith`. Untuk image, video, dan PDF, output-nya bukan untuk dikembalikan persis melalui tombol Decompress karena prosesnya memang lossy atau optimasi format.

### Keypoints

- Video butuh FFmpeg.
- PDF butuh Ghostscript.
- File yang sudah terkompresi mungkin tidak turun banyak.
- Decompress hanya untuk generic lossless output.
- Image, video, PDF tidak dikembalikan lewat Decompress.

---

## 11. Penutup

### Script bicara

Jadi kesimpulannya, Compressa Studio saya buat sebagai aplikasi kompresi file sekaligus media pembelajaran kompresi data.

Aplikasi ini menunjukkan bahwa kompresi tidak bisa dipukul rata untuk semua jenis file. Gambar, video, PDF, dan file generic punya karakteristik yang berbeda, sehingga pipeline kompresinya juga berbeda.

Dari sisi pembelajaran, aplikasi ini membantu melihat konsep lossless dan lossy secara langsung. Untuk lossless, hasilnya bisa diverifikasi dengan SHA-256. Untuk lossy, hasilnya dilihat dari penurunan ukuran file dan tetap mempertimbangkan kualitas visual atau kualitas dokumen.

Harapan saya, aplikasi ini bisa menjadi contoh penerapan materi kompresi data dalam bentuk aplikasi nyata, bukan hanya perhitungan atau teori di slide.

Sekian presentasi dari saya. Terima kasih.

### Keypoints

- Compressa Studio = aplikasi kompresi + media pembelajaran.
- Tiap jenis file punya pipeline berbeda.
- Lossless bisa diverifikasi dengan checksum.
- Lossy mengejar ukuran lebih kecil dengan trade-off kualitas.
- Mengubah teori kompresi menjadi aplikasi nyata.

---

## 12. Cheat Sheet Keypoints Satu Halaman

Gunakan bagian ini kalau ingin presentasi tanpa membaca script panjang.

### Pembuka

- Nama aplikasi: Compressa Studio.
- Aplikasi desktop untuk kompresi file.
- Tujuan: praktik materi kompresi data.
- Alur: pilih file -> deteksi -> kompres -> tampilkan hasil.

### Kenapa dibuat

- Materi kompresi lebih mudah dipahami lewat praktik.
- Menunjukkan lossless vs lossy.
- File besar sering jadi masalah.
- Ingin membuat alat all-in-one untuk image, video, PDF, dan generic.

### Dibuat dengan apa

- React + TypeScript untuk UI.
- Tauri untuk desktop app.
- Rust untuk backend dan proses file.
- FFmpeg untuk video.
- Ghostscript untuk PDF.
- Zstd dan algoritma edukatif untuk generic.

### Fitur utama

- Auto detect file.
- Image compression.
- Video compression.
- PDF compression.
- Generic lossless compression.
- Decompression output lossless.
- Statistics dan SHA-256.
- Logs dan algorithm explanation.

### Tampilan

- Topbar: status dan logs.
- Quick Start: Auto Detect, Image, Video, PDF, Generic.
- File Selection: metadata file.
- Compression Mode: setting sesuai kategori.
- Actions: Choose File, Compress, Decompress, Clear.
- Statistics: hasil proses.
- Algorithm Explanation: konsep algoritma.

### Dropdown penting

- Logs: operation history.
- Image/video preset: High quality, Balanced, Small size.
- PDF preset: Screen, Ebook, Print.
- Image encoder: Standard, Native.
- Generic algorithm: Zstd, RLE, Shannon, Shannon-Fano, Huffman, LZ77, LZ78, LZW, Arithmetic.
- Zstd level: 1 sampai 19.

### Demo foto

- Pilih gambar.
- Tunjukkan metadata.
- Pilih Balanced atau Small size.
- Atur resize jika perlu.
- Compress.
- Tunjukkan Statistics.
- Jelaskan lossy.

### Demo PDF

- Pilih PDF.
- Tunjukkan preset Screen, Ebook, Print.
- Pilih Ebook.
- Pastikan Ghostscript tersedia.
- Compress.
- Tunjukkan Statistics dan output PDF.

### Demo video

- Pilih video.
- Pastikan FFmpeg tersedia.
- Pilih Balanced.
- Compress.
- Tunjukkan Statistics.
- Jelaskan spatial dan temporal redundancy.

### Demo generic lossless

- Pilih file generic.
- Pilih Zstd level 6.
- Compress jadi `.zst`.
- Klik Decompress.
- Tunjukkan Integrity Check checksum match.

### Batasan

- Video butuh FFmpeg.
- PDF butuh Ghostscript.
- File yang sudah terkompresi mungkin tidak mengecil.
- Decompress hanya untuk output generic lossless.

### Kalimat penutup

Compressa Studio menunjukkan bahwa kompresi data tidak hanya teori. Dengan aplikasi ini, jenis file, algoritma, hasil ukuran, dan validasi checksum bisa dilihat langsung dalam satu workflow.
