pub enum ServiceType {
    /// Unacknowledged Request
    Unacknowledged,
    /// Acknowledged Request
    Acknowledged,
    /// Response to an Acknowledged Request
    ResponseToAnAcknowledged,
}

pub enum Action {
    ReadRequest,
    WriteRequest,
    ReadReport,
    WriteReport,
    UpdateBegin,
    UpdateEnd,
}
