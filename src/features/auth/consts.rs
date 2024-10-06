use std::collections::HashSet;

use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use lazy_static::lazy_static;

lazy_static! {
    /// The root user is a static, omnipotent, DB-less user with maximum privilegies
    /// Root user is the one with uid = 0, just like UNIX systems do
    ///
    /// Additionally, the root user MUST have both is_active and is_staff set to true,
    /// for consistency sake.
    pub static ref ROOT_USER: super::schemas::AppUser = super::schemas::AppUser {
        id: 0,
        username: "root".into(),
        display_name: "root@contigosanmarcos.com".into(),
        password_hash: "".into(),
        is_active: true,
        is_staff: true,
        last_login: None,
        updated_at: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    pub static ref JWT_ENCODING_KEY: EncodingKey = EncodingKey::from_secret(
        crate::env::ROOT_SECRET.as_ref(),
    );

    pub static ref JWT_DECODING_KEY: DecodingKey = DecodingKey::from_secret(
        crate::env::ROOT_SECRET.as_ref(),
    );

    pub static ref JWT_VALIDATION: Validation = {
        let mut validation = Validation::default();
        validation.required_spec_claims = HashSet::new();
        validation
    };
}
