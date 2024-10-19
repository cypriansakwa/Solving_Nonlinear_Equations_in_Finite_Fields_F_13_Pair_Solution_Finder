fn main() {
    let p = 13; // Prime field modulus
    for x in 0..p {
        let x2 = (x * x) % p; // Compute x^2 mod p
        for y in 0..p {
            let y2 = (y * y) % p; // Compute y^2 mod p
            let lhs = (x2 + y2) % p; // x^2 + y^2 mod p
            let rhs = (1 + 7 * x2 * y2) % p; // 1 + 7 * x^2 * y^2 mod p
            if lhs == rhs {
                println!("Solution: x = {}, y = {}", x, y);
            }
        }
    }
}

