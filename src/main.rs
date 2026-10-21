fn main() {
    let umur = "25";
    let umur:u32 = umur.parse().unwrap();

    println!("Tahun depan: {}", umur +1);
}
