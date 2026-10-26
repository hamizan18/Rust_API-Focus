fn main() {
    // Tuple: panjang tetap, tipe boleh berbeda
    let titik: (i32, f64, char) = (10, 2.5, 'x');
    let (x, y, z) = titik;
    
    println!("{} {} {} | {}", x, y, z, titik.0);

    // Array: panjang tetap, tipe sama
    let nilai: [i32; 3] = [80, 90, 100];
    println!("Pertama: {}, panjang: {}", nilai[0], nilai.len());
}
