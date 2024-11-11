use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct WhatsappMessage {
    pub object: String,
    pub entry: Vec<WhatsappMessageEntry>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct WhatsappMessageEntry {
    pub id: String,
    pub changes: Vec<WhatsappMessageChange>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct WhatsappMessageChange {
    // Should be equal to "messages"
    pub field: String,
    pub value: WhatsappMessageValue,
}

#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct WhatsappMessageValue {
    pub messaging_product: String,
    pub metadata: WhatsappMessageMetadata,
    pub contacts: Vec<WhatsappMessageContact>,
    pub messages: Vec<WhatsappMessageItem>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct WhatsappMessageMetadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct WhatsappMessageContact {
    pub profile: WhatsappMessageContactProfile,
    pub wa_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct WhatsappMessageContactProfile {
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct WhatsappMessageItem {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    /// Can be either text, sticker, audio, video and image
    /// Missing: contacts, location, etc.
    pub r#type: WhastappMessageType,
    #[serde(
        alias = "text",
        alias = "sticker",
        alias = "audio",
        alias = "video",
        alias = "image",
        alias = "interactive"
    )]
    pub object: WhatsappMessageItemObject,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum WhastappMessageType {
    Text,
    Sticker,
    Audio,
    Video,
    Image,
    Interactive,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(untagged, rename_all = "snake_case")]
#[allow(dead_code)]
/// See <https://developers.facebook.com/docs/whatsapp/cloud-api/reference/messages/#overview>
pub enum WhatsappMessageItemObject {
    Text {
        body: String,
    },
    Sticker {
        mime_type: String,
        sha256: String,
        id: String,
        animated: bool,
    },
    Audio {
        mime_type: String,
        sha256: String,
        id: String,
        voice: bool,
    },
    Video {
        mime_type: String,
        sha256: String,
        id: String,
    },
    Image {
        mime_type: String,
        sha256: String,
        id: String,
    },
    Interactive {
        r#type: String,
        button_reply: WhatsappMessageButtonReply,
    },
}

#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct WhatsappMessageButtonReply {
    pub id: String,
    pub title: String,
}
