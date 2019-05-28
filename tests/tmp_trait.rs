use simple_pgp::traits::{PublicKeyAlgorithm, SignatureAlgorithm};

pub struct Ed25519;

impl SignatureAlgorithm for Ed25519 {
    type SignKey = [u8; 32];
    type VerifyKey = [u8; 32];
    const SIGNATURE_LENGTH: usize = 64;

    fn sign(_plain: &[u8], _sign_key: &Self::SignKey) -> Vec<u8> {
        vec![]
    }

    fn verify(_plain: &[u8], _sign: &[u8], _verify_key: &Self::VerifyKey) -> bool {
        true
    }
}

pub struct RSA;

impl PublicKeyAlgorithm for RSA {
    type PublicKey = [u8; 32];
    type SecretKey = [u8; 32];

    fn encrypt(_plain: &[u8], _public_key: &Self::PublicKey) -> Vec<u8> {
        vec![]
    }

    fn decrypt(_cipher: &[u8], _secret_key: &Self::SecretKey) -> Vec<u8> {
        vec![]
    }
}
