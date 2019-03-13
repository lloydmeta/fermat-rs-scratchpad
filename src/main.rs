extern crate fernet;

use fernet::*;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let key = Fernet::generate_key();
    let fernet = Fernet::new(&key).unwrap();

    let my_message = "my top secret message";
    println!("my_message: [{}]", my_message);

    let encrypted_text = fernet.encrypt(my_message.as_bytes());
    println!("encrypted_text: [{}]", encrypted_text);

    let decrypted_bytes = fernet
        .decrypt(&encrypted_text)
        // Usually not necessary but the fermat crate's DecryptionError doesn't implement Error lol
        .map_err(|e| format!("{:?}", e))?;
    let decrypted_text = String::from_utf8(decrypted_bytes)?;

    Ok(println!("decrypted_text: [{}]", decrypted_text))
}
