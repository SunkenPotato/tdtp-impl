mod sigtest;
mod i2b;
use i2b::BaselineBinner;
mod max_quantile;

fn main() {
    let data1 = vec![3.0, 3.1, 100.0, 2.9, 3.3];
    let data2 = vec![3.0, 3.1, 3.2, 2.9, 3.3];

    let sig = sigtest::significant_difference(&data1, &data2, 0.05);
    println!("Signifikanter Unterschied? {}", sig);

    // Beispiel-Baseline
    let baseline = vec![3.0, 3.1, 2.9, 3.2, 3.0];

    // BaselineBinner initialisieren
    let binner = BaselineBinner::new(baseline, 3); // num_bits = 3

    // Testwert
    let x = 3.05;

    // Berechne Bin als Bits
    if let Some(bits) = binner.bin_as_bits(x) {
        println!("Bin als Bits für {}: {:?}", x, bits);
    } else {
        println!("Fehler bei der Berechnung des Bins");
    }
    let max_quan = max_quantile::max_safe_quantile_bins
}
