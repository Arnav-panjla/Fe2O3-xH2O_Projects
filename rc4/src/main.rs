use std::vec::Vec;

pub struct RC4 {
    state: Vec<u8>,
    i: u8,
    j: u8,
}

impl RC4 {
    pub fn new(key: &[u8]) -> RC4 {
        let mut rc4 = RC4 {
            state: (0..=255).collect(),
            i: 0,
            j: 0,
        };
        
        // Key-scheduling algorithm (KSA)
        let mut j: u8 = 0;
        for i in 0..256 {
            j = j.wrapping_add(rc4.state[i])
                 .wrapping_add(key[i % key.len()]);
            rc4.state.swap(i, j as usize);
        }
        
        rc4
    }
    
    // Pseudo-random generation algorithm (PRGA)
    fn next_byte(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.state[self.i as usize]);
        
        self.state.swap(self.i as usize, self.j as usize);
        
        let k = self.state[((self.state[self.i as usize] as u16 + 
                           self.state[self.j as usize] as u16) % 256) as usize];
        k
    }
    
    pub fn process(&mut self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .map(|&byte| byte ^ self.next_byte())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rc4_encryption() {
        let key = b"Key";
        let plaintext = b"Hello, World!";
        
        // Encrypt
        let mut rc4 = RC4::new(key);
        let ciphertext = rc4.process(plaintext);
        
        // Decrypt
        let mut rc4 = RC4::new(key);
        let decrypted = rc4.process(&ciphertext);
        
        assert_eq!(plaintext, &decrypted[..]);
    }
}

// Example usage
fn main() {
    let key = b"SecretKey123";
    let message = b"Hello, RC4 encryption!";
    
    // Encrypt
    let mut rc4 = RC4::new(key);
    let encrypted = rc4.process(message);
    println!("Encrypted: {:?}", encrypted);
    
    // Decrypt
    let mut rc4 = RC4::new(key);
    let decrypted = rc4.process(&encrypted);
    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypted));
}