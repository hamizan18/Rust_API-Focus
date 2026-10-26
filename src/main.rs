fn main() {
    let suhu = 30;

    // if sebagai expression
    let kategori = if suhu > 28 { "panas" } else { "sejuk" };
    println!("Cuaca {}", kategori);

    // for dengan range
    for i in 1..=3 {
        println!("Putaran {}", i);
    }

    // while
    let mut hitung = 3;
    while hitung > 0 {
        println!("{}...", hitung);
        hitung -= 1;
    }

    // match: wajib mencakup semua kemungkinan
    let nilai = 95;
    let grade = match nilai {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'D', // `_` = semua sisanya
    };
    println!("Grade: {}", grade);
}