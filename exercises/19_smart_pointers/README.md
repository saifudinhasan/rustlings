# Smart Pointers

In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

## Further Information

- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Using Box to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rc\<T\>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Cow Documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

# `Box<T>` di Rust

## Kapan digunakan?

- Saat ingin menyimpan data di heap tapi tetap memiliki kepemilikan tunggal (tidak bisa di-_share_).

## Kegunaan:

- Mengurangi ukuran stack.
- Membuat tipe dengan ukuran tidak tetap (seperti rekursi).
- Memindahkan data ke heap untuk kepemilikan eksklusif.

## Contoh:

```rust
let b = Box::new(5); // `b` adalah pointer ke `5` di heap
println!("{}", b); // output: 5
```

# `Rc<T>` (Reference Counted) di Rust

## Kapan digunakan?

- Saat ingin memiliki banyak referensi ke data yang sama di heap tanpa pemilik unik.

## Kegunaan:

- Digunakan saat membuat struktur data berbagi kepemilikan, misalnya dalam struktur grafik.

## Catatan:

- Hanya bisa digunakan dalam **single-threaded** karena tidak aman untuk paralelisme.

## Contoh:

```rust
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a); // Referensi tambahan ke `a`
println!("{}", a);
println!("{}", b); // output: 5
```

---

# `Arc<T>` (Atomic Reference Counted) di Rust

## Kapan digunakan?

- Saat butuh kepemilikan bersama di **multi-threading**.

## Kegunaan:

- Sama seperti `Rc<T>`, tapi aman untuk digunakan di lingkungan multi-threading karena memakai **atomic reference counting**.

## Contoh:

```rust
use std::sync::Arc;
let a = Arc::new(5);
let b = Arc::clone(&a); // Referensi tambahan, bisa diakses dari thread lain
println!("{}", b); // output: 5
```

---

# `Cow<T>` (Copy on Write) di Rust

## Kapan digunakan?

- Saat ingin **menunda alokasi** sampai benar-benar perlu mengubah data.

## Kegunaan:

- Efisien dalam penggunaan memori, karena hanya membuat salinan saat terjadi modifikasi.

## Contoh:

```rust
use std::borrow::Cow;
let s: Cow<str> = Cow::Borrowed("Hello");
let owned_s = s.to_owned(); // Sekarang jadi String yang dimiliki
println!("{}", owned_s); // output: Hello
```

# 🔥 Tabel Perbandingan `Box<T>`, `Rc<T>`, `Arc<T>`, dan `Cow<T>` di Rust

| Tipe     | Kepemilikan | Thread-Safety | Kapan digunakan?                                           |
| -------- | ----------- | ------------- | ---------------------------------------------------------- |
| `Box<T>` | Tunggal     | Aman          | Data di heap dengan kepemilikan eksklusif                  |
| `Rc<T>`  | Bersama     | Tidak aman    | Data di heap dengan referensi banyak dalam _single-thread_ |
| `Arc<T>` | Bersama     | Aman          | Data di heap dengan referensi banyak dalam _multi-thread_  |
| `Cow<T>` | Tertunda    | Aman          | Optimisasi alokasi hanya saat perlu modifikasi             |
