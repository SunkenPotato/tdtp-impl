mod sigtest;
mod i2b;
use i2b::BaselineBinner;
mod max_quantile;
///use tdtp_impl::client;

///Testanwendung für alle Hilfsfunktionen
fn main() {
    let data1 = vec![3.0, 3.1, 100.0, 2.9, 3.3];
    let data2 = vec![3.0, 3.1, 3.2, 2.9, 3.3];

    let sig = sigtest::significant_difference(&data1, &data2, 0.05);
    println!("Signifikanter Unterschied? {}", sig);

    let max_quan = max_quantile::max_safe_quantile_bins(10.5, 4.1, 0.7);

    // Beispiel-Baseline
    let baseline = vec![3.0, 3.1, 2.9, 3.2, 3.0];

    // BaselineBinner initialisieren
    let binner = BaselineBinner::new(baseline, max_quan.try_into().unwrap()); // num_bits = 3

    // Testwert
    let x = 3.05;

    // Berechne Bin als Bits
    if let Some(bits) = binner.bin_as_bits(x) {
        println!("{}: {:?}", x, bits);
    } 
}
