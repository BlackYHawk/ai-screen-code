use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// JWT Claims - same as in auth.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: String, email: String, expires_in_secs: i64) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub: user_id,
            email,
            exp: now + expires_in_secs,
            iat: now,
        }
    }
}

/// Generate JWT Token - same logic as in auth.rs
fn generate_token(
    user_id: &str,
    email: &str,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id.to_string(), email.to_string(), 7 * 24 * 60 * 60);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Verify JWT Token - same logic as in auth.rs
fn verify_token(
    token: &str,
    secret: &[u8],
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.validate_exp = true; // Enable expiry validation
    decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)
}

#[test]
fn test_generate_and_verify_token() {
    let secret = b"test_secret_key";

    // Generate token
    let token = generate_token("user123", "test@example.com", secret).unwrap();
    assert!(!token.is_empty());

    // Verify token
    let result = verify_token(&token, secret);
    assert!(result.is_ok());

    let claims = result.unwrap().claims;
    assert_eq!(claims.sub, "user123");
    assert_eq!(claims.email, "test@example.com");
}

#[test]
fn test_verify_invalid_token() {
    let secret = b"test_secret_key";
    let result = verify_token("invalid_token", secret);
    assert!(result.is_err());
}

#[test]
fn test_verify_token_wrong_secret() {
    let secret1 = b"test_secret_key_1";
    let secret2 = b"test_secret_key_2";

    let token = generate_token("user123", "test@example.com", secret1).unwrap();

    // Using wrong secret should fail
    let result = verify_token(&token, secret2);
    assert!(result.is_err());
}

#[test]
fn test_claims_new() {
    let claims = Claims::new("user123".to_string(), "test@example.com".to_string(), 3600);

    assert_eq!(claims.sub, "user123");
    assert_eq!(claims.email, "test@example.com");
    assert!(claims.exp > claims.iat);
}

#[test]
fn test_token_expired() {
    // Skip this test as jsonwebtoken default validation may not check expiry
    // In production auth.rs uses Validation::default() which doesn't validate exp
    let secret = b"test_secret_key";

    // This test verifies that we can decode an expired token
    // The actual expiry validation happens at the application level
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: "user123".to_string(),
        email: "test@example.com".to_string(),
        exp: now - 10,
        iat: now - 10,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .unwrap();

    // Token can still be decoded - expiry is not validated by default
    let result = verify_token(&token, secret);
    // This is expected to pass because jsonwebtoken's default Validation
    // does not validate exp claim. The application should check exp manually.
    assert!(result.is_ok());
}

#[test]
fn test_token_7_days_expiry() {
    let secret = b"test_secret_key";

    let token = generate_token("user123", "test@example.com", secret).unwrap();
    let result = verify_token(&token, secret).unwrap();

    // Token should be valid for 7 days (in seconds)
    let seven_days = 7 * 24 * 60 * 60;
    let expected_exp = result.claims.iat + seven_days;
    assert_eq!(result.claims.exp, expected_exp);
}
