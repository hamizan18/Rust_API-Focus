fn main() {
    let mut jumlah_buku = 3;
    let harga_per_buku: f64 = 45000.0;
    jumlah_buku = jumlah_buku + 1;
    let total = jumlah_buku as f64 * harga_per_buku;
    println!("Total: Rp{}", total);
}
