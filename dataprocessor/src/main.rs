mod sigtest;

fn main() {
    let data1 = vec![3.0, 3.1, 100.0, 2.9, 3.3];
    let data2 = vec![3.0, 3.1, 3.2, 2.9, 3.3];

    let sig = sigtest::significant_difference(&data1, &data2, 0.05);
    println!("Signifikanter Unterschied? {}", sig);
}
