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

fn decimal_to_4bit_binary(decimal: u32) -> Vec<u8> {
    let lowest_4_bits = decimal & 0b1111;
    (0..4)
        .map(|i| ((lowest_4_bits >> (3 - i)) & 1) as u8)
        .collect()
}

fn encrypt<'a>(s: &[u8], a:  &'a [u8], m: &[u8], e: u32 ) -> Vec<(u32, &'a [u8])> {
    let dot_prod = dot_product(s, a);
    let m_shifted = (m.iter().fold(0u32, |acc, &bit| (acc << 1) | bit as u32)) << 28;
    let enc:u32 = dot_prod + m_shifted + e;
    let rand_key :&[u8] = a;
    let result: Vec<(u32, &[u8])> = vec![(enc, rand_key)];
    result
}

fn decrypt(enc_msg: Vec<(u32,&[u8])> , s: &[u8]) -> Vec<u8> {
    let (enc_val, a) = &enc_msg[0];
    let dot_prod = dot_product(&s, &a);
    let d: u32 = round_to_nearest_2_power_28(enc_val - dot_prod);
    let e: u32 = d / (1u32 << 28);
    let result: Vec<u8> = decimal_to_4bit_binary(e);
    result
}

fn main() {
    
    // 4 bit message vector
    let m: Vec<u8> = vec![1, 0, 1, 0];
    // 32 bit secret key
    let s: Vec<u8> = vec![1,0,1,1,1,0,1,0,1,1,1,0,1,0,1,1,1,0,1,1,1,0,1,1,1,0,0,0,0,0,1,1];
    // 32 bit random vector
    let a: Vec<u8> = vec![1,1,1,0,0,1,0,1,0,1,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,1,1,1,1,0,0,1];
    //noise value
    let e: u32 = 57;

    // Ensure vectors are of desired length
    assert_eq!(s.len(), 32, "s must be 32 bits");
    assert_eq!(a.len(), 32, "a must be 32 bits");
    assert_eq!(m.len(), 4, "m must be 4 bits");

    // print initial values
    println!("secret key: {:?}", s);
    println!("random no.: {:?}", a);
    println!("4 bit msg: {:?}", m);

    // ENCRYPTION ------> 
    println!("---------------------------------Encrypting--------------------------------------");
    let enc_msg = encrypt(&s, &a, &m, e);
    let enc_val = enc_msg[0].0;
    println!("encrypted msg  : {:?}", enc_msg);
    println!("random vector  : {:?}", a);
    println!("<s,a> + m + e  : {:?}", enc_val);
    
    // DECRYPTION ------>
    println!("---------------------------------Decrypting--------------------------------------");
    let dec_msg : Vec<u8> = decrypt(enc_msg, &s);
    println!("decrypted msg : {:?}",dec_msg);

}