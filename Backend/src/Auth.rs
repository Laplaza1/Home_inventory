use std::{format, print, println};
use dotenv::dotenv;
use std::env;
use reqwest::{Method, Request, Url};
use serde_json::json;
use reqwest::Client;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use log::*;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject – the MongoDB ObjectId of the user .
    pub sub: String,
    
    /// Optional ObjectId for _id
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub id: Option<ObjectId>, 
    /// Login username.
    pub username: String,

    /// Household / home the user belongs to .
    pub home: String,

    /// Optional contact details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    

    pub status: Option<i64>,


    pub title:Option<String>,
    /// Access level struct.
    pub access: String,

    /// Issuer of the token.
    pub iss: String,

    /// Issued-at time 
    pub iat: i64,

    /// Expiration time 
    pub exp: i64,

    /// Not-before time 
    pub nbf: i64,
}


#[derive(Debug)]
pub enum JwtError {
    MissingSecret,
    Encoding(jsonwebtoken::errors::Error),
    Decoding(jsonwebtoken::errors::Error),
    InvalidClaims(String),
}

impl std::fmt::Display for JwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtError::MissingSecret => write!(f, "JWT_SECRET environment variable is not set"),
            JwtError::Encoding(e) => write!(f, "jwt encoding error: {}", e),
            JwtError::Decoding(e) => write!(f, "jwt decoding error: {}", e),
            JwtError::InvalidClaims(msg) => write!(f, "Invalid claims: {}", msg),
        }
    }
}

impl std::error::Error for JwtError {}
pub fn create_jwt(
    id:String ,
    status:Option<i64> ,
    title:Option<String>,
    username: &str,
    home: &str,
    email: Option<String>,
    phone_number: Option<String>,
    access: &str,
    ttl_hours: i64,
) -> Result<String, JwtError> {
    dotenv().ok(); 

    let secret = env::var("JWT_SECRET").map_err(|_| JwtError::MissingSecret)?;

    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(ttl_hours)).timestamp();
    let nbf = iat;

    let claims = Claims {
        sub: id.to_string(),
        username: username.to_string(),
        home: home.to_string(),
        email,
        phone_number,
        access: access.to_string(),
        iss: "home-inventory".to_string(),
        iat,
        exp,
        nbf,
        status,
        title,
    };

    let header = Header::new(Algorithm::HS256);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(JwtError::Encoding)
}


pub fn validate_jwt(token: &str) -> Result<Claims, JwtError> {
    dotenv().ok();

    let secret = env::var("JWT_SECRET").map_err(|_| JwtError::MissingSecret)?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&["home-inventory"]);
    

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(JwtError::Decoding)?;

    
    if token_data.claims.sub.is_empty() {
        return Err(JwtError::InvalidClaims("sub (user id) is empty".into()));
    }
    if !["Creator", "Admin", "User"].contains(&token_data.claims.access.as_str()) {
        return Err(JwtError::InvalidClaims(format!(
            "unknown access level: {}",
            token_data.claims.access
        )));
    }
    if token_data.claims.exp < Utc::now().timestamp(){
        return Err(JwtError::InvalidClaims("sub (user id) is empty".into()));
    }

    Ok(token_data.claims)
}

pub async fn test_jwt_() {
    

    
    if env::var("JWT_SECRET").is_err() {
        error!("JWT_SECRET was not set – using a secret");
    }

    match create_jwt(
        "676abc1234567890abcdef01".to_string(),
        Some(1),
        Some("Master of home".to_string()) ,         
        "john.doe",
        "Main House",
        Some("john.doe@example.com".into()),
        Some("+15551234567".into()),
        "User",
        24,  // in hours
        
        
    ) {
        Ok(token) => {
            info!("Created jwt:\n{}", token);

            match validate_jwt(&token) {
                Ok(claims) => {
                    info!("Validated claims:\n{:#?}", claims);
                    info!(
                        "Token is valid for user '{}' belonging to home '{}'",
                        claims.username, claims.home
                    );
                }
                Err(e) => error!("Validation failed: {}", e),
            }
        }
        Err(e) => error!("Failed to create jwt: {}", e),
    }

    
}