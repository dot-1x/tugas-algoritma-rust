use chrono::prelude::*;

#[derive(Debug)]
struct Data {
    nama: String,
    harga: i32,
    ditambahkan: DateTime<Local>,
    stok: i32,
    tipe: String,
}

type TData = Vec<Data>;

fn print_data(data: &Data) {
    println!("nama: {}", data.nama);
    println!("harga: {}", data.harga);
    println!("stok: {}", data.stok);
    println!("tipe: {}", data.tipe);
    println!("ditambahkan: {}", data.ditambahkan);
}

fn lihat_data(datas: &TData) {
    if datas.len() <= 0 {
        println!("Data kosong!");
        return;
    }
    println!("Data Tersedia:");
    for (idx, data) in datas.iter().enumerate() {
        println!("{:=^20}", idx + 1);
        print_data(data);
        println!("====================");
    }
}

fn tambah_data(datas: &mut TData) {
    let nama = input("Nama barang: ");
    let harga = input("Harga barang: ").parse().unwrap();
    let stok = input("Stok barang: ").parse().unwrap();
    let tipe = input("Tipe barang: ");
    datas.push(Data {
        nama,
        harga,
        stok,
        tipe,
        ditambahkan: Local::now(),
    });
    println!("Data berhasil ditambahkan!");
}

fn edit_data(datas: &mut TData) {
    lihat_data(datas);
    let pilihan: i32 = input("Pilih data yang ingin diedit: ").parse().unwrap();
    if pilihan < 1 || pilihan > datas.len() as i32 {
        println!("mohon masukkan pilihan yang valid!");
        return;
    }
    let data = &mut datas[pilihan as usize - 1];
    print_data(data);
    let actions = vec!["Nama", "Harga", "Stok", "Tipe"];
    loop {
        for (idx, action) in actions.iter().enumerate() {
            println!("{}. {}", idx + 1, action);
        }
        let aksi = input("Pilih aksi: ").parse().unwrap();
        match aksi {
            1 => {
                let nama = input("edit nama barang: ");
                data.nama = nama;
            }
            2 => {
                let harga = input("edit harga barang: ").parse().unwrap();
                data.harga = harga;
            }
            3 => {
                let stok = input("edit stok barang: ").parse().unwrap();
                data.stok = stok;
            }
            4 => {
                let tipe = input("edit tipe barang: ");
                data.tipe = tipe;
            }
            _ => {
                println!("Input tidak valid!");
            }
        }
        let lagi = input("edit lagi?(y/n): ");
        if lagi.to_lowercase() != "y" {
            break;
        }
    }
    println!("Data berhasil dirubah!");
}

fn hapus_data(datas: &mut TData) {
    lihat_data(datas);
    let pilihan: usize = input("Pilih data yang ingin dihapus: ").parse().unwrap();
    if pilihan < 1 || pilihan > datas.len() {
        println!("mohon masukkan pilihan yang valid!");
        return;
    }
    datas.remove(pilihan - 1);
    println!("Data berhasil dihapus!");
}

fn main() {
    let actions = vec!["Lihat data", "Tambah data", "Edit data", "Hapus data"];
    let mut datas: TData = vec![];
    loop {
        println!("SELAMAT DATANG DI APLIKASI CLI PENGELOLA BARANG!");
        for (num, name) in actions.iter().enumerate() {
            println!("{}. {}", num + 1, name);
        }
        let pilihan: String = input("Pilih aksi: ");
        match pilihan.parse::<u32>() {
            Ok(1) => lihat_data(&datas),
            Ok(2) => tambah_data(&mut datas),
            Ok(3) => edit_data(&mut datas),
            Ok(4) => hapus_data(&mut datas),
            _ => {
                return;
            }
        }
        input("tekan apa saja untuk kembali ke menu");
        clear();
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn clear() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        std::process::Command::new("sh")
            .args(&["-c", "clear"])
            .status()
            .unwrap();
    }
}
