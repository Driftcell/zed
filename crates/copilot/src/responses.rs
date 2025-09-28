#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

// StreamingEvent enum moved to bottom after type definitions.

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseCreatedEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub response: ResponseData,
    pub sequence_number: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseData {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub status: String,
    pub error: Option<Value>,
    pub incomplete_details: Option<Value>,
    pub instructions: Option<String>,
    pub max_output_tokens: Option<u32>,
    pub model: String,
    pub output: Vec<Value>, // Placeholder; will become typed output items.
    pub parallel_tool_calls: bool,
    pub previous_response_id: Option<String>,
    pub reasoning: Option<Reasoning>,
    pub store: bool,
    pub temperature: Option<f32>,
    pub text: Option<TextConfigWrapper>,
    pub tool_choice: Value, // Could be "auto" | object; keep generic until schema fixed.
    pub tools: Vec<Value>,  // Placeholder for tool definitions.
    pub top_p: Option<f32>,
    pub truncation: Option<String>,
    pub usage: Option<Value>,
    pub user: Option<String>,
    pub metadata: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reasoning {
    pub effort: Option<String>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextConfigWrapper {
    pub format: TextFormat,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextFormat {
    #[serde(rename = "type")]
    pub type_field: String,
}

macro_rules! placeholder_event {
    ($name:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            #[serde(rename = "type")]
            pub event_type: String,
            pub sequence_number: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub response: Option<ResponseData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub data: Option<Value>,
            #[serde(flatten)]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub extra: Option<Map<String, Value>>,
        }
    };
}

placeholder_event!(ResponseInProgressEvent);
placeholder_event!(ResponseFailedEvent);
placeholder_event!(ResponseCompletedEvent);
placeholder_event!(ResponseOutputItemAdded);
placeholder_event!(ResponseOutputItemDone);
placeholder_event!(ResponseContentPartAdded);
placeholder_event!(ResponseContentPartDone);
placeholder_event!(ResponseOutputTextDelta);
placeholder_event!(ResponseOutputTextAnnotationAdded);
placeholder_event!(ResponseTextDone);
placeholder_event!(ResponseRefusalDelta);
placeholder_event!(ResponseRefusalDone);
placeholder_event!(ResponseFunctionCallArgumentsDelta);
placeholder_event!(ResponseFunctionCallArgumentsDone);
placeholder_event!(ResponseFileSearchCallInProgress);
placeholder_event!(ResponseFileSearchCallSearching);
placeholder_event!(ResponseFileSearchCallCompleted);
placeholder_event!(ResponseCodeInterpreterInProgress);
placeholder_event!(ResponseCodeInterpreterCallCodeDelta);
placeholder_event!(ResponseCodeInterpreterCallCodeDone);
placeholder_event!(ResponseCodeInterpreterCallInterpreting);
placeholder_event!(ResponseCodeInterpreterCallCompleted);

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ErrorEvent {
    #[serde(rename = "type")]
    pub event_type: String,

    pub sequence_number: u64,

    pub error: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamingEvent {
    ResponseCreated(ResponseCreatedEvent),
    ResponseInProgress(ResponseInProgressEvent),
    ResponseFailed(ResponseFailedEvent),
    ResponseCompleted(ResponseCompletedEvent),
    ResponseOutputItemAdded(ResponseOutputItemAdded),
    ResponseOutputItemDone(ResponseOutputItemDone),
    ResponseContentPartAdded(ResponseContentPartAdded),
    ResponseContentPartDone(ResponseContentPartDone),
    ResponseOutputTextDelta(ResponseOutputTextDelta),
    ResponseOutputTextAnnotationAdded(ResponseOutputTextAnnotationAdded),
    ResponseTextDone(ResponseTextDone),
    ResponseRefusalDelta(ResponseRefusalDelta),
    ResponseRefusalDone(ResponseRefusalDone),
    ResponseFunctionCallArgumentsDelta(ResponseFunctionCallArgumentsDelta),
    ResponseFunctionCallArgumentsDone(ResponseFunctionCallArgumentsDone),
    ResponseFileSearchCallInProgress(ResponseFileSearchCallInProgress),
    ResponseFileSearchCallSearching(ResponseFileSearchCallSearching),
    ResponseFileSearchCallCompleted(ResponseFileSearchCallCompleted),
    ResponseCodeInterpreterInProgress(ResponseCodeInterpreterInProgress),
    ResponseCodeInterpreterCallCodeDelta(ResponseCodeInterpreterCallCodeDelta),
    ResponseCodeInterpreterCallCodeDone(ResponseCodeInterpreterCallCodeDone),
    ResponseCodeInterpreterCallInterpreting(ResponseCodeInterpreterCallInterpreting),
    ResponseCodeInterpreterCallCompleted(ResponseCodeInterpreterCallCompleted),
    Error(ErrorEvent),
}
