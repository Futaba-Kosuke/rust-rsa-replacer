extern crate rsa;
extern crate rand;
extern crate rand_chacha;

use rsa::{PublicKey, RsaPublicKey, PaddingScheme};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use base64::{encode};

fn main() {
    // 標準入力からテキストを読み込む
    let input = "";

    // 固定されたシード値を使用してChaCha20Rngを生成する
    let seed = [0u8; 32]; // 32バイトのシード値（固定された値）
    let mut rng = ChaCha20Rng::from_seed(seed);

    // RSA鍵を生成する
    let bits = 2048;
    let private_key = rsa::RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // テキストを暗号化する
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted_data = public_key.encrypt(&mut rng, padding, &input.as_bytes()).expect("Failed to encrypt");

    // 暗号化されたデータを表示する
    let encoded_encrypted_data = encode(&encrypted_data);
    println!("Encrypted data: {:?}", encoded_encrypted_data.get(1..20).unwrap());
}
