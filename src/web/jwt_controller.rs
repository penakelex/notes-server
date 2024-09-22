use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::settings::Jwt;
use crate::error::{Error, Result, SessionError};

#[derive(Clone)]
pub struct JWTController {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    header: Header,
    validation: Validation,
}

impl JWTController {
    pub fn new(jwt: &Jwt) -> Self {
        let algorithm = Algorithm::HS512;
        Self {
            encoding_key: EncodingKey::from_secret(jwt.secret.as_ref()),
            decoding_key: DecodingKey::from_secret(jwt.secret.as_ref()),
            header: Header::new(algorithm),
            validation: Validation::new(algorithm),
        }
    }
}

impl JWTController {
    pub fn generate_token(&self, token_claims: &TokenClaims) -> Result<String> {
        encode(&self.header, token_claims, &self.encoding_key)
            .map_err(|_| Error::Sessions(SessionError::SessionInvalid))
    }

    pub fn get_claims_from_token(&self, token: &str) -> Result<TokenClaims> {
        let token_data = decode::<TokenClaims>(
            token,
            &self.decoding_key,
            &self.validation
        ).map_err(|_| Error::Sessions(SessionError::ValidityCheckFail))?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: u32, //user id
    pub exp: usize,
}