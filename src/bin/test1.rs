use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key, Error // Or `Aes128Gcm`
};

fn main()-> Result<(),Error>//Result<(),Error>
{
    // The encryption key can be generated randomly:
let key = Aes256Gcm::generate_key(OsRng);

// Transformed from a byte array:
let key: &[u8; 32] = &[42; 32];
let key: &Key<Aes256Gcm> = key.into();

// Note that you can get byte array from slice using the `TryInto` trait:
// let key: &[u8] = &[42; 32];
// let key: [u8; 32] = key.try_into()?;

// Alternatively, the key can be transformed directly from a byte slice
// (panicks on length mismatch):
let key = Key::<Aes256Gcm>::from_slice(key);

let cipher = Aes256Gcm::new(&key);
let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
println!("hello there! {ciphertext:?}");
let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;

println!("hello there! {:?}", std::str::from_utf8(&plaintext).unwrap());
assert_eq!(&plaintext, b"plaintext message");

return Ok(());
}