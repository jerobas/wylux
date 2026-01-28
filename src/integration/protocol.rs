use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Call,
    Response,
    Event,
}

impl Message {
    pub fn new_call(id: String, function: String, args: serde_json::Value) -> Self {
        Self {
            id,
            msg_type: MessageType::Call,
            function: Some(function),
            args: Some(args),
            result: None,
            error: None,
            event: None,
            data: None,
        }
    }
    
    pub fn new_response(id: String, result: serde_json::Value) -> Self {
        Self {
            id,
            msg_type: MessageType::Response,
            function: None,
            args: None,
            result: Some(result),
            error: None,
            event: None,
            data: None,
        }
    }
    
    pub fn new_error_response(id: String, error: String) -> Self {
        Self {
            id,
            msg_type: MessageType::Response,
            function: None,
            args: None,
            result: None,
            error: Some(error),
            event: None,
            data: None,
        }
    }
    
    pub fn new_event(event: String, data: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            msg_type: MessageType::Event,
            function: None,
            args: None,
            result: None,
            error: None,
            event: Some(event),
            data: Some(data),
        }
    }
}
