use std::collections::{HashMap, VecDeque};

fn main() {
    // You can optionally experiment here.

    let mut config: HashMap<&str, Option<&str>> = HashMap::new();
    config.insert("database_url", Some("postgres://localhost"));
    config.insert("timeout", None); // Tidak ada nilai

    if let Some(db_url) = config.get("database_url").and_then(|&url| url) {
        println!("Menghubungkan ke database di: {}", db_url);
    } else {
        println!("Tidak ada konfigurasi database.");
    }

    let mut job_queue: VecDeque<Option<&str>> = VecDeque::from(vec![
        Some("Job 1"),
        Some("Job 2"),
        None, // Pekerjaan gagal atau tidak ada data
        Some("Job 3"),
        None,
    ]);

    while let Some(job) = job_queue.pop_front() {
        match job {
            Some(task) => println!("Memproses {}", task),
            None => println!("Pekerjaan kosong, dilewati."),
        }
    }

    println!("Semua pekerjaan selesai!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // if-let
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }

        let kotak_hadiah = Some("Cokelat");

        if let Some(hadiah) = kotak_hadiah {
            println!("Yeay! Aku dapat {}", hadiah);
        } else {
            println!("Yah, kotaknya kosong...");
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // while-let with nested pattern matching
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);

        let mut tumpukan_kotak = vec![Some("Permen"), Some("Boneka"), None, Some("Mobil Mainan")];

        while let Some(Some(hadiah)) = tumpukan_kotak.pop() {
            println!("Aku dapat {}", hadiah);
        }
    }
}
