// Will return the value of g^x mod p
pub fn discrete_logarithm(g: u32, x: u32, p: u32) -> u32 {
    if x == 1 {
        return g;
    } else {
        return g.pow(x) % p;
    }
}

fn main() {
    let public_parameters: (u32, u32) = (23, 9);
    let (p, g) = public_parameters;

    let private_keys: (u32, u32) = (4, 3);
    let (a, b) = private_keys;

    // Alice pub key
    let alice_pub = discrete_logarithm(g, a, p);
    println!("Alice Public key: {}", alice_pub);

    // Bob pub key
    let bob_pub = discrete_logarithm(g, b, p);
    println!("Bob Public key: {}", bob_pub);

    //Bob generated secrete key
    let mut common_secrete_key = discrete_logarithm(alice_pub, b, p);
    println!("Bob generated secrete key: {}", common_secrete_key);

    //Alice generated secrete key
    common_secrete_key = discrete_logarithm(bob_pub, a, p);
    println!("Alice generated secrete key: {}", common_secrete_key);
}
