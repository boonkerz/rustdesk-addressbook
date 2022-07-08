use argon2::Config;
use rand::Rng;

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    match argon2::hash_encoded(password.as_bytes(), &salt, &config) {
        Ok(password) => Ok(password),
        Err(_) => Err("not generated".to_string())
    }
}

pub fn verify_password(hash: String, password: String) -> Result<bool, bool> {
    match argon2::verify_encoded(hash.as_str(), password.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(false)
    }
    
}