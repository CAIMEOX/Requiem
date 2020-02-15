extern crate crypto;
use self::crypto::digest::Digest;
use crypto::curve25519;
use crypto::{aes, sha2, symmetriccipher};
use ecdsa::curve::nistp384::FixedSignature;
use ecdsa::signature::digest::generic_array::GenericArray;
use ecdsa::SecretKey;

static MinecraftWSEncryptSubprotocol: String = "com.microsoft.minecraft.wsencrypt";

//encryptionSession is a session unique to a player, that handles encryption between the server and the client
//enableencryption <PublicKey> <Salt>
struct encryptionSession {
    salt: &mut [u8; 16],
    private_key: SecretKey,
}

impl encryptionSession {
    fn compute_ivs(&mut self) {
        let mut hasher = sha2::Sha256::new();
        hasher.input_str();
    }
}
