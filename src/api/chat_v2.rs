use serde::{Deserialize, Serialize};
use serde_json::Value;

/// V2 Chat request using the messages-based API.
///
/// The v2 Chat API uses a `messages` list instead of separate `message`, `preamble`,
/// and `chat_history` parameters.
#[derive(Serialize, Debug, Clone)]
pub struct ChatV2Request {
    /// The name of a compatible Cohere model (required).
    pub model: String,
    /// A list of chat messages in chronological order.
    pub messages: Vec<ChatV2Message>,
    /// Whether to stream the response. Defaults to false for non-stream requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// A list of tools (functions) available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolV2>>,
    /// When true, tool calls will strictly follow tool definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_tools: Option<bool>,
    /// A list of relevant documents that the model can cite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentV2>>,
    /// Options for controlling citation generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_options: Option<CitationOptions>,
    /// Configuration for forcing the model output to adhere to a specified format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormatV2>,
    /// Safety mode. Defaults to CONTEXTUAL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_mode: Option<SafetyMode>,
    /// The maximum number of output tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    /// A list of up to 5 strings that the model will use to stop generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    /// A non-negative float that tunes the degree of randomness in generation. Defaults to 0.3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// If specified, the backend will attempt deterministic sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Used to reduce repetitiveness of generated tokens. 0.0 to 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// Used to reduce repetitiveness of generated tokens. 0.0 to 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// Ensures only the top k most likely tokens are considered. 0 disables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<u64>,
    /// Ensures only the most likely tokens with total probability mass of p are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,
    /// When true, log probabilities of generated tokens will be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Controls whether the model is forced to use a tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Configuration for reasoning features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,
}

/// A message in the v2 chat conversation.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "role")]
pub enum ChatV2Message {
    /// System message (replaces preamble in v1).
    #[serde(rename = "system")]
    System { content: MessageContent },
    /// User message.
    #[serde(rename = "user")]
    User { content: MessageContent },
    /// Assistant message (model response).
    #[serde(rename = "assistant")]
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<MessageContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCallV2>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_plan: Option<String>,
    },
    /// Tool result message.
    #[serde(rename = "tool")]
    Tool {
        tool_call_id: String,
        content: ToolMessageContent,
    },
}

/// Message content can be a simple string or a list of content blocks.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text string.
    Text(String),
    /// List of content blocks (text, images, etc.).
    Blocks(Vec<ContentBlock>),
}

/// A content block within a message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ContentBlock {
    /// Text content block.
    #[serde(rename = "text")]
    Text { text: String },
    /// Image URL content block.
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

/// Image URL with optional detail level.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageUrl {
    /// URL of the image. Can be base64 data URI or web URL.
    pub url: String,
    /// Controls detail level: "auto", "low", or "high".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Tool message content can be a string or a list of tool content blocks.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ToolMessageContent {
    /// Simple text string.
    Text(String),
    /// List of tool content blocks.
    Blocks(Vec<ToolContentBlock>),
}

/// A content block within a tool result.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ToolContentBlock {
    /// Text content.
    #[serde(rename = "text")]
    Text { text: String },
    /// Document content.
    #[serde(rename = "document")]
    Document { document: DocumentV2 },
}

/// A v2-style tool definition.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolV2 {
    /// Must be "function".
    #[serde(rename = "type")]
    pub tool_type: String,
    /// The function definition.
    pub function: ToolV2Function,
}

/// Function definition within a tool.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolV2Function {
    /// The name of the function.
    pub name: String,
    /// The description of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters of the function as a JSON schema.
    pub parameters: Value,
}

/// A tool call made by the model.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolCallV2 {
    /// Unique ID for this tool call.
    pub id: String,
    /// Must be "function".
    #[serde(rename = "type")]
    pub call_type: String,
    /// The function call details.
    pub function: ToolCallV2Function,
}

/// Function call details within a tool call.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolCallV2Function {
    /// The name of the function to call.
    pub name: String,
    /// The arguments as a JSON string.
    pub arguments: String,
}

/// A document for RAG in v2 format.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DocumentV2 {
    /// The document data. Can be an object with arbitrary fields.
    pub data: Value,
    /// Optional unique identifier for this document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// V2 Chat response (non-streaming).
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ChatV2Response {
    /// Unique identifier for the generated reply.
    pub id: String,
    /// The reason the chat request finished.
    pub finish_reason: String,
    /// The assistant's message response.
    pub message: AssistantMessageResponse,
    /// Token usage information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// The assistant message in a v2 response.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AssistantMessageResponse {
    /// Always "assistant".
    pub role: String,
    /// The content blocks of the response.
    #[serde(default)]
    pub content: Vec<AssistantContentBlock>,
    /// Tool calls requested by the model.
    #[serde(default)]
    pub tool_calls: Vec<ToolCallV2>,
    /// A chain-of-thought plan for tool use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_plan: Option<String>,
    /// Citations for the response.
    #[serde(default)]
    pub citations: Vec<Citation>,
}

/// A content block in the assistant's response.
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum AssistantContentBlock {
    /// Text content.
    #[serde(rename = "text")]
    Text { text: String },
    /// Thinking content (reasoning).
    #[serde(rename = "thinking")]
    Thinking { thinking: String },
}

/// A citation in the response.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Citation {
    /// Start index of the cited text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// End index of the cited text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    /// The cited text snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Sources for this citation.
    #[serde(default)]
    pub sources: Vec<CitationSource>,
}

/// A source for a citation.
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum CitationSource {
    /// Document source.
    #[serde(rename = "document")]
    Document {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        document: Option<Value>,
    },
    /// Tool source.
    #[serde(rename = "tool")]
    Tool {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_output: Option<Value>,
    },
}

/// Token usage information.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Usage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billed_units: Option<BilledUnits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<TokenUsage>,
}

/// Billed token units.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BilledUnits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<f64>,
}

/// Token counts.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TokenUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<f64>,
}

/// A v2 chat stream event.
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ChatV2StreamEvent {
    /// Stream started, contains message ID.
    #[serde(rename = "message-start")]
    MessageStart {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        delta: Option<Value>,
    },
    /// Content delta with text chunk.
    #[serde(rename = "content-start")]
    ContentStart {
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        delta: Option<Value>,
    },
    /// Content delta with text chunk.
    #[serde(rename = "content-delta")]
    ContentDelta {
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        delta: Option<ContentDelta>,
    },
    /// Content block finished.
    #[serde(rename = "content-end")]
    ContentEnd {
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
    },
    /// Tool call started.
    #[serde(rename = "tool-call-start")]
    ToolCallStart {
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        delta: Option<Value>,
    },
    /// Tool call argument delta.
    #[serde(rename = "tool-call-delta")]
    ToolCallDelta {
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        delta: Option<Value>,
    },
    /// Tool call finished.
    #[serde(rename = "tool-call-end")]
    ToolCallEnd {
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
    },
    /// Tool plan delta (reasoning for tool use).
    #[serde(rename = "tool-plan-delta")]
    ToolPlanDelta {
        #[serde(skip_serializing_if = "Option::is_none")]
        delta: Option<Value>,
    },
    /// Citation start.
    #[serde(rename = "citation-start")]
    CitationStart {
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        delta: Option<Value>,
    },
    /// Citation end.
    #[serde(rename = "citation-end")]
    CitationEnd {
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
    },
    /// Message complete.
    #[serde(rename = "message-end")]
    MessageEnd {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        delta: Option<MessageEndDelta>,
    },
}

/// Delta content in a content-delta event.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ContentDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ContentDeltaMessage>,
}

/// Message content in a content delta.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ContentDeltaMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentDeltaText>,
}

/// Text content in a content delta.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ContentDeltaText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Delta in a message-end event.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MessageEndDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// Citation options for v2 chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CitationOptions {
    pub mode: CitationMode,
}

/// Citation mode.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CitationMode {
    Enabled,
    Disabled,
    Fast,
    Accurate,
    Off,
}

/// Response format configuration.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ResponseFormatV2 {
    /// Free-form text response.
    #[serde(rename = "text")]
    Text,
    /// JSON object response with optional schema.
    #[serde(rename = "json_object")]
    JsonObject {
        #[serde(skip_serializing_if = "Option::is_none")]
        json_schema: Option<Value>,
    },
}

/// Safety mode configuration.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SafetyMode {
    Contextual,
    Strict,
    Off,
}

/// Tool choice configuration.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ToolChoice {
    Required,
    None,
}

/// Thinking/reasoning configuration.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Thinking {
    /// "enabled" or "disabled".
    #[serde(rename = "type")]
    pub thinking_type: String,
    /// Maximum number of tokens for thinking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_budget: Option<u64>,
}

impl ChatV2Message {
    /// Create a system message.
    pub fn system(content: impl Into<String>) -> Self {
        ChatV2Message::System {
            content: MessageContent::Text(content.into()),
        }
    }

    /// Create a user message.
    pub fn user(content: impl Into<String>) -> Self {
        ChatV2Message::User {
            content: MessageContent::Text(content.into()),
        }
    }

    /// Create an assistant message.
    pub fn assistant(content: impl Into<String>) -> Self {
        ChatV2Message::Assistant {
            content: Some(MessageContent::Text(content.into())),
            tool_calls: None,
            tool_plan: None,
        }
    }

    /// Create a tool result message.
    pub fn tool_result(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        ChatV2Message::Tool {
            tool_call_id: tool_call_id.into(),
            content: ToolMessageContent::Text(content.into()),
        }
    }
}

impl ChatV2Request {
    /// Create a simple chat request with a single user message.
    pub fn new(model: impl Into<String>, message: impl Into<String>) -> Self {
        ChatV2Request {
            model: model.into(),
            messages: vec![ChatV2Message::user(message)],
            stream: None,
            tools: None,
            strict_tools: None,
            documents: None,
            citation_options: None,
            response_format: None,
            safety_mode: None,
            max_tokens: None,
            stop_sequences: None,
            temperature: None,
            seed: None,
            frequency_penalty: None,
            presence_penalty: None,
            k: None,
            p: None,
            logprobs: None,
            tool_choice: None,
            thinking: None,
        }
    }
}

impl AssistantMessageResponse {
    /// Get the text content of the first content block, if any.
    pub fn text(&self) -> Option<&str> {
        self.content.first().and_then(|block| match block {
            AssistantContentBlock::Text { text } => Some(text.as_str()),
            _ => None,
        })
    }
}

impl ChatV2StreamEvent {
    /// Extract text content from a content-delta event, if present.
    pub fn text(&self) -> Option<&str> {
        match self {
            ChatV2StreamEvent::ContentDelta { delta, .. } => delta
                .as_ref()?
                .message
                .as_ref()?
                .content
                .as_ref()?
                .text
                .as_deref(),
            _ => None,
        }
    }
}
