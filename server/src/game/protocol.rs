use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProtocolMessage {
    PlaceToken(Token),
    DeleteToken { token_id: String },
    Movement { id: String, token_id: String, dx: i16, dy: i16 },
    Connect { username: String, host: bool },
    Disconnect { username: String },
    FailedConnection { reason: String },
}

impl ProtocolMessage {
    pub(crate) fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub(crate) fn into_msg(self) -> RawMessage {
        RawMessage::Text(self.to_string())
    }
}

type RawMessage = tokio_tungstenite::tungstenite::Message;

impl Into<RawMessage> for ProtocolMessage {
    fn into(self) -> RawMessage {
        self.into_msg()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub id: Option<String>,
    pub kind: String,
    pub x: i16,
    pub y: i16,
    pub colour: String,
    pub controller: Option<String>,
}

impl Token {
    pub fn to_msg(&self) -> ProtocolMessage {
        ProtocolMessage::PlaceToken(self.clone())
    }
}