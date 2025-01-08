# Cryptography Projects in Rust 🔐💻 (aka Fe2O3-xH2O)

Welcome to my **Cryptographic Projects** repository! Here, I showcase various cryptographic implementations built using **Rust**, focusing on both classical and modern cryptographic techniques. The repository includes a range of cryptographic algorithms and systems, from foundational encryption techniques to advanced cryptographic protocols.

## Projects Overview 📄

### 1. Simple FHE, RSA, FHE-LWE
   This project provides simple implementations of Fully Homomorphic Encryption (FHE), RSA encryption, and FHE-LWE, demonstrating secure data processing, communication, and advanced cryptographic techniques for privacy protection.
   
### 2. **Shamir’s Secret Sharing 🗝️**
   A Rust implementation of **Shamir’s Secret Sharing** scheme, which allows a secret to be divided into multiple parts, requiring a threshold of parts to reconstruct the secret.

   - **Key Features**: Split secrets, reconstruct secrets from shares, and threshold-based sharing.
   - **Use Case**: Secure key management, distributed systems, and multi-party computation.


### 3. **Elliptic Curve Cryptography (ECC) 🔒**
   An implementation of **Elliptic Curve Cryptography (ECC)** in Rust. ECC is an advanced public-key cryptosystem that provides high security with smaller key sizes compared to RSA.

   - **Key Features**: Key generation, signature generation and verification, encryption, and decryption using elliptic curves.
   - **Use Case**: Secure communication, digital signatures, and modern encryption systems.

### 4. **BLS signature**
   Implemented BLS signature in rust, using arkworks ecosystem and using the curve bls12_381.

### 5. **Rivest Cipher 4 (RC4)**
   Software implementation of stream cipher RC4, cryptographically insecure, via exhaustive search attacks.

### 6. **ECDSA (Elliptic Curve Digital Signature Algorithm)** 
   Digital singature algorithm using Elliptic Curve secp256k1 

### 7. **Sumcheck Protocol**
   It is an interactive proof system used for efficiently verifying the sum of a polynomial over a finite field.

### 8. **Groth16**
    A simple implementation of groth16, using bls12_381 in arkworks ecosystem

    
## Some future ideas
### 1. **AES (Advanced Encryption Standard)**
   - Implementing AES encryption and decryption from scratch. 
   - You can explore both ECB (Electronic Codebook) and CBC (Cipher Block Chaining) modes of operation, as well as key expansion and padding schemes.

### 2. **HMAC (Hash-Based Message Authentication Code)**
   - Implement an HMAC construction using a secure hash function (e.g., SHA-256). 
   - This project could focus on message integrity and authentication, and you could test it by generating and verifying authentication codes.

### 3. **Zero-Knowledge Proofs (ZKPs)**
   - Implement simple zero-knowledge proofs, such as the classic "Fiat-Shamir" protocol.
   - This could be used for privacy-preserving authentication systems, or for cryptographic voting systems.


### 4. **Crypto-Agility**
   - Build a framework that allows a system to switch between different cryptographic algorithms (RSA, ECC, AES, etc.) seamlessly based on policy or performance needs. This could be useful for systems that need flexibility over time, especially in the face of evolving cryptographic research.

### 5. **TLS/SSL Protocol Implementation**
   - A project that implements the Transport Layer Security (TLS) or Secure Sockets Layer (SSL) protocol could be challenging.
   - This involves handling encryption, message integrity, certificate validation, and key exchange—ideal for understanding how modern web security works.

### 6. **Secure Multi-Party Computation (MPC)**
   - Implement a basic version of MPC, where multiple parties can compute a result without revealing their private inputs.
   - This is a powerful technique for secure collaborative computations.

### 7. **Crypto Wallet (Bitcoin or Ethereum)**
   - Create a simple cryptocurrency wallet that can generate private/public key pairs, sign transactions, and communicate with a blockchain network (Bitcoin or Ethereum).
   - This is a great way to learn more about cryptographic principles and real-world applications.

### 8. **Cryptographic Primitives Library**
   - Build a general-purpose cryptographic library in Rust that includes common primitives like hashing functions, HMAC, PBKDF2, digital signatures, and asymmetric encryption algorithms.
   - This would be a reusable toolset for anyone implementing cryptographic systems in Rust.

### 9. **Blind Signature Schemes**
   - Implement a simple blind signature scheme such as the RSA-based blind signature.
   - This can be used for anonymous authentication or anonymous voting systems.

### 10. **Secure Hash Algorithms (SHA-3, SHA-256) Optimizations**
   - Optimize cryptographic algorithms for performance, especially on specific hardware (e.g., using SIMD or parallel processing techniques).
   - This could be a great way to explore the trade-offs between security and efficiency.

### 11. **Quantum-Resistant Cryptographic Systems**
   - Research and implement some of the post-quantum cryptographic algorithms under development to counter the power of quantum computers (e.g., code-based cryptography, multivariate quadratic equations).

### 12. **Steganography**
   - Implement a simple steganography system where secret messages are hidden inside images, audio, or text files.
   - You could explore cryptographic methods used in hiding the information and adding security to the process.

### 13. **Cryptographic Protocols and Network Security**
   - Build your own cryptographic protocol for secure messaging or file sharing between peers.
   - Implement key exchange, encryption/decryption, and message verification, potentially exploring techniques like authenticated encryption.

---


### Prerequisites 🔧
Make sure you have **Rust** and **Cargo** installed. If you don't have them installed, follow the instructions at [Rust's official website](https://www.rust-lang.org/learn/get-started).


## Important Notes ⚠️

- Each project is contained within its own directory with a `Cargo.toml` for dependencies.
- Some projects may have additional dependencies or require specific configuration steps. Please refer to each project’s individual README file (if available) for more details.
- Contributions and feedback are welcome! If you'd like to improve any project or suggest a new cryptographic algorithm, feel free to submit a pull request. 🌱

---

## License 📜

This repository is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for more details.

---

### Feel free to explore, contribute, or reach out if you have any questions! 😊🔐

---
