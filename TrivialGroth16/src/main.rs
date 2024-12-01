fn dot_product(s: &[u8], a: &[u8]) -> u32 {
    s.iter()
     .zip(a.iter())
     .map(|(s_bit, a_bit)| (*s_bit & *a_bit) as u32)
     .fold(0, |acc, bit| acc + bit)
}

fn round_to_nearest_2_power_28(c: u32) -> u32 {
    let divisor = 1u32 << 28;
    ((c + divisor / 2) / divisor) * divisor
}

fn compute_c(s: &[u8], a: &[u8], m: &[u8], e: u32) -> u32 {
    // Ensure vectors are 32 bits long
    assert_eq!(s.len(), 32, "s must be 32 bits");
    assert_eq!(a.len(), 32, "a must be 32 bits");
    assert_eq!(m.len(), 4, "m must be 4 bits");

    // Compute dot product of a and s
    let dot_prod = dot_product(s, a);
    println!("dot_prod: {}",dot_prod);

    // Convert m to a 32-bit value, shifted left by 28 bits
    let m_shifted = (m.iter()
        .fold(0u32, |acc, &bit| (acc << 1) | bit as u32)) << 28;

    println!("shifted m : {}",m_shifted);

    // Add dot product, shifted m, and noise

    println!("pre_noise : {}",m_shifted + dot_prod);

    let c = dot_prod + m_shifted + e;

    println!("final : {}",c);

    c
}

fn decimal_to_4bit_binary(decimal: u32) -> Vec<u8> {
    // Take the lowest 4 bits
    let lowest_4_bits = decimal & 0b1111;
    
    // Convert to binary vector
    (0..4)
        .map(|i| ((lowest_4_bits >> (3 - i)) & 1) as u8)
        .collect()
}

fn main() {
    // Example usage with predefined vectors and noise
    let s: Vec<u8> = vec![
        1,0,1,1,1,0,1,0,1,1,1,0,1,0,1,1,
        1,0,1,1,1,0,1,1,1,0,0,0,0,0,1,1
    ];
    let a: Vec<u8> = vec![
        1,1,1,0,0,1,0,1,0,1,0,0,0,0,0,0,
        0,1,0,0,0,1,0,0,0,1,1,1,1,0,0,1
    ];
    let m: Vec<u8> = vec![1, 0, 1, 0];
    let e: u32 = 57; // Noise value provided by you

    let c = compute_c(&s, &a, &m, e);

    println!("s: {:?}", s);
    println!("a: {:?}", a);
    println!("m: {:?}", m);
    println!("e: {}", e);
    println!("c: {}", c);
    
    // Compute dot product
    let dot_prod = dot_product(&s, &a);

    // Compute and round d
    let d: u32 = round_to_nearest_2_power_28((c - dot_prod));

    println!("decrypted : {}", d);

    let e: u32 = d / (1u32 << 28);

    println!("shifted decrypted : {},",e);

    let f: Vec<u8> = decimal_to_4bit_binary(e);

    println!("binary vector conversion : {:?}",f);

}