use num::bigint::ToBigInt;
use num_bigint::BigInt;

fn main() {
    let a = 3.to_bigint().unwrap();
    let x = num::pow(a, 247);
    let xs = serde_json::to_string(&x).unwrap();
    let xd: BigInt = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    println!("3**247 is {} and serializes to {}", x, xs);
}
