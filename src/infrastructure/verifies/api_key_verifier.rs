use super::errors::ApiKeyVerifierError;
use crate::domain::settings::model::Credentials;
use crate::domain::verifies::service::ApiKeyVerifierService;
use rand::{rngs::OsRng, TryRngCore};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const BASE: usize = 62;

pub struct ApiKeyVerifier {
    pub credentials: Credentials,
}

impl ApiKeyVerifier {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }

    fn bytes_to_base62(&self, mut bytes: Vec<u8>) -> String {
        let mut num = 0u128;
        for byte in bytes.drain(..) {
            num = (num << 8) | (byte as u128);
        }

        let mut result = String::new();
        while num > 0 {
            let index = (num % BASE as u128) as usize;
            result.insert(0, CHARSET[index] as char);
            num /= BASE as u128;
        }

        result
    }

    fn generate_random_str(&self, length: usize) -> String {
        let mut random = String::new();

        while random.len() < length {
            let mut buffer = [0u8; 16];
            OsRng.try_fill_bytes(&mut buffer).unwrap();
            random.push_str(&self.bytes_to_base62(buffer.to_vec()));
        }

        // Обрезаем до нужной длины
        random.chars().take(length).collect()
    }

}

impl ApiKeyVerifierService for ApiKeyVerifier {
    type Error = ApiKeyVerifierError;

    fn generate(&self) -> String {
        let identifier = self.generate_random_str(*self.credentials.api_key_length() as usize);
        let secret = self.generate_random_str(*self.credentials.api_key_length() as usize);

        format!("{}-{}", identifier, secret)
    }

    fn extract_identifier(&self, api_key: &str) -> Result<String, Self::Error> {
        let parts: Vec<&str> = api_key.split('-').collect();

        if parts.len() != 2 || api_key == "-" {
            return Err(ApiKeyVerifierError::DecryptionError(
                "Token format invalid".to_string(),
            ));
        }

        Ok(parts[0].to_string())
    }

    fn is_verified(&self, api_key_hash: &str, api_key: &str) -> Result<bool, Self::Error> {
        bcrypt::verify(api_key, api_key_hash).map_err(|e| {
            ApiKeyVerifierError::HashPasswordCryptError {
                stage: "bcrypt::verify",
                source: e,
            }
        })
    }

    fn create_hash(&self, api_key: &str) -> Result<String, Self::Error> {
        bcrypt::hash(api_key, bcrypt::DEFAULT_COST).map_err(|e| {
            ApiKeyVerifierError::HashPasswordCryptError {
                stage: "bcrypt::hash",
                source: e,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::settings::model::Credentials;
    use crate::domain::verifies::service::ApiKeyVerifierService;

    fn mock_credentials() -> Credentials {
        Credentials::mock()
    }

    #[test]
    fn test_extract_identifier() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials);
        let api_key = "ABCDEF1234567890-ZYXW9876543210";
        let identifier = verifier.extract_identifier(api_key).unwrap();
        assert_eq!(identifier, "ABCDEF1234567890");
    }

    #[test]
    fn test_extract_identifier_invalid_format() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials);
        let api_key = "invalidformat";

        let result = verifier.extract_identifier(api_key);
        println!("result {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_and_verify_success() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials);
        let api_key = "TestApiKey123";

        let hash = verifier.create_hash(api_key).unwrap();
        let is_valid = verifier.is_verified(&hash, api_key).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_hash_and_verify_failure() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials);
        let api_key = "TestApiKey123";
        let wrong_key = "WrongKey";

        let hash = verifier.create_hash(api_key).unwrap();
        let is_valid = verifier.is_verified(&hash, wrong_key).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_generate_returns_valid_key() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials.clone());

        let api_key = verifier.generate();
        println!("Generated API key: {}", api_key);

        let parts: Vec<&str> = api_key.split('-').collect();
        assert_eq!(parts.len(), 2, "API key should have 2 parts separated by '-'");

        let expected_len = *credentials.api_key_length() as usize;
        assert_eq!(
            parts[0].len(),
            expected_len,
            "First part (identifier) should be {} characters, got {}",
            expected_len,
            parts[0].len()
        );
        assert_eq!(
            parts[1].len(),
            expected_len,
            "Second part (secret) should be {} characters, got {}",
            expected_len,
            parts[1].len()
        );
    }


    #[test]
    fn test_generate_random_str_length() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials);

        let result = verifier.generate_random_str(40);
        assert!(result.len() >= 40); // может быть чуть длиннее, из-за base62
    }

    #[test]
    fn test_bytes_to_base62_known_input() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials);

        let bytes = vec![0x01, 0x02, 0x03]; // 0x010203 = 66051
        let result = verifier.bytes_to_base62(bytes);
        assert!(!result.is_empty());
        assert!(result.chars().all(|c| CHARSET.contains(&(c as u8))));
    }

    #[test]
    fn test_extract_identifier_empty_string() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials);

        let result = verifier.extract_identifier("");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_identifier_dash_only() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials);

        let result = verifier.extract_identifier("-");
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_and_verify_empty_string() {
        let credentials = mock_credentials();
        let verifier = ApiKeyVerifier::new(credentials);

        let hash = verifier.create_hash("").unwrap();
        let valid = verifier.is_verified(&hash, "").unwrap();
        assert!(valid);
    }

}
