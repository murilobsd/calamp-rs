pub struct AcknowledgeMessage {
    pub tp: u8,
    pub ack: u8,
    pub unused: u8,
    pub app_version: [u8],
}
