use serde::{Serialize, Deserialize};
#[cfg(feature = "server")]
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Default,Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[cfg(feature = "server")]
pub async fn get_token()->Result<String,anyhow::Error>{
    let claims = Claims::default();
    let res=encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))?;
    Ok(res)
}
