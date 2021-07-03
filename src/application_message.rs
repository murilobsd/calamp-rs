pub struct ApplicationMessage {
    pub AppMsgType: AppMsgType,
    pub AppMsgLength: i16,
    pub AppendixE: AppMsg,
}

pub struct VBusDiagnosticsReport {
    pub BlockItem: std::vec::Vec<DTCBlockItem>,
}

pub struct TimeSync<'a> {
    pub data: &'a [u8],
}

pub struct DTCBlockItem<'a> {
    pub tp: DTClockType<'a>,
    pub length: i16,
    pub data: &'a [u8],
}

pub struct DTClockType<'a> {
    pub source_bus: SourceBus,
    pub report_type: ReportType,
    pub bus_specific_flags: int,
    pub zero: int,
}

pub enum AppMsgType {
    IPRequest = 10,
    IPReport = 11,
    TimeSync = 50,
    Services = 80,
    SVRMessaging = 81,
    DownloadIDReport = 100,
    DownloadAuthorization = 101,
    DownloadRequest = 102,
    DownloadUpdate = 103,
    DownloadComplete = 104,
    DownloadHTTPLMUFW = 105,
    DownloadHTTPFile = 106,
    OTADownload = 107,
    ATCommand = 110,
    VersionReport = 111,
    GPSStatusReport = 112,
    MessageStatisticsReport = 113,
    StateReport = 115,
    GeoZoneActionMessage = 116,
    GeoZoneUpdateMessage = 117,
    ProbeIDReport = 118,
    CaptureReport = 120,
    MotionLogReport = 122,
    CompressedMotionLogReport = 123,
    VBusDataReport = 130,
    VehicleIDReport = 131,
    VBusDTCReport = 132,
    VBusVINDecodeLookup = 133,
    SquarellCommandMessage = 134,
    SquarellStatusMessage = 135,
    VBusRegisterDeviceMessage = 136,
    VBusFreezeFrame = 137,
    VBusDiagnosticsReport = 139,
    VBusRemoteOBD = 140,
    VBusGroupDataReport = 142,
    VBusManagementOutbound = 145,
    VBusManagementInbound = 148,
}

pub enum SourceBus {
    J1939,
    J1708,
    Obdii,
}

pub enum ReportType {
    All,
    Unreported,
}
