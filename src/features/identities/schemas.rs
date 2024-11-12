use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[allow(unused)]
pub struct UserIdentity {
    pub id: i32,
    /// The user fingerprint stored in the database
    #[schema(example = "38d8035fe1f6a25c9043e18592634fa65b125ec1746d40185fac388add3c4a72")]
    pub fingerprint: String,
    /// The user ip address stored in the database. Don't let the users know we store this.
    #[serde(skip_serializing)]
    pub last_ip: IpNetwork, // sqlx "ipnetwork" feature
    /// The last known version of the app the user is using
    #[schema(example = "0.0.1")]
    pub last_version: String,
    /// The platform the user is using. It can be "android", "ios" or "web".
    pub platform: Option<String>,
    /// The number of sessions the user has had so far
    pub session_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserIdentityPayload {
    /// Fingerprints are identifiers that the client considers unique to the user.
    /// The server will SHA256 hash this value before storing it.
    ///
    /// In android, it may be the [Android ID](https://developer.android.com/reference/android/provider/Settings.Secure.html#ANDROID_ID).
    ///
    /// In iOS, it may be the
    /// [Advertising Identifier](https://developer.apple.com/documentation/adsupport/asidentifiermanager/advertisingidentifier)
    /// or [Vendor Identifier](https://developer.apple.com/documentation/uikit/uidevice/1620059-identifierforvendor)
    ///
    /// In web, it may be the [User agent](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent).
    #[schema(example = "38400000-8cf0-11bd-b23e-10b96e40000d")]
    pub fingerprint: String,
    /// The current app version the user is using
    #[schema(example = "0.0.1")]
    pub last_version: String,
    /// The platform the user is using. It can be "android", "ios" or "web".
    #[schema(example = "android")]
    pub platform: String,
}
