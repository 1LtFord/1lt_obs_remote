pub enum OBSMatchError {
    OBSOpcodeNotFound,
    OBSWebSocketCloseCodeNotFound,
    OBSRequestBatchExecutionTypeNotFound,
    OBSRequestStatusNotFound,
    OBSEventSubscriptionNotFound,
}

pub enum OBSOpcode {
    Hello,
    Identify,
    Identifyed,
    Reidentify,
    Event,
    Request,
    RequestResponse,
    RequestBatch,
    RequestBatchResponse
}

pub fn obs_opcode_match_value(obs_op_code: &OBSOpcode) -> String {
    match obs_op_code {
        OBSOpcode::Hello => "0",
        OBSOpcode::Identify => "1",
        OBSOpcode::Identifyed => "2",
        OBSOpcode::Reidentify => "3",
        OBSOpcode::Event => "5",
        OBSOpcode::Request => "6",
        OBSOpcode::RequestResponse => "7",
        OBSOpcode::RequestBatch => "8",
        OBSOpcode::RequestBatchResponse => "9"
    }.to_string()
}

pub fn obs_opcode_match_enum(obs_op_code: &String) -> Result<OBSOpcode, OBSMatchError> {
    Ok(match &obs_op_code[..] {
        "0" => OBSOpcode::Hello,
        "1" => OBSOpcode::Identify,
        "2" => OBSOpcode::Identifyed,
        "3" => OBSOpcode::Reidentify,
        "5" => OBSOpcode::Event,
        "6" => OBSOpcode::Request,
        "7" => OBSOpcode::RequestResponse,
        "8" => OBSOpcode::RequestBatch,
        "9" => OBSOpcode::RequestBatchResponse,
        _ => return Err(OBSMatchError::OBSOpcodeNotFound)
    })
}

pub enum OBSWebSocketCloseCode {
    DontClose,
    UnknownReason,
    MessageDecodeError,
    MissingDataField,
    InvalidDataFieldType,
    InvalidDataFieldValue,
    UnknownOpCode,
    NotIdentified,
    AlreadyIdentified,
    AuthenticationFailed,
    UnsupportedRpcVersion,
    SessionInvalidated,
    UnsupportedFeature
}

pub fn obs_websocket_close_code_match_value(obs_websocket_close_code: OBSWebSocketCloseCode) -> String {
    match obs_websocket_close_code {
        OBSWebSocketCloseCode::DontClose => "0",
        OBSWebSocketCloseCode::UnknownReason => "4000",
        OBSWebSocketCloseCode::MessageDecodeError => "4002",
        OBSWebSocketCloseCode::MissingDataField => "4003",
        OBSWebSocketCloseCode::InvalidDataFieldType => "4004",
        OBSWebSocketCloseCode::InvalidDataFieldValue => "4005",
        OBSWebSocketCloseCode::UnknownOpCode => "4006",
        OBSWebSocketCloseCode::NotIdentified => "4007",
        OBSWebSocketCloseCode::AlreadyIdentified => "4008",
        OBSWebSocketCloseCode::AuthenticationFailed => "4009",
        OBSWebSocketCloseCode::UnsupportedRpcVersion => "4010",
        OBSWebSocketCloseCode::SessionInvalidated => "4011",
        OBSWebSocketCloseCode::UnsupportedFeature => "4012"
    }.to_string()
}

pub fn obs_websocket_close_code_match_enum(obs_websocket_close_code: &String) -> Result<OBSWebSocketCloseCode, OBSMatchError> {
    Ok(match &obs_websocket_close_code[..] {
        "0" => OBSWebSocketCloseCode::DontClose,
        "4000" => OBSWebSocketCloseCode::UnknownReason,
        "4002" => OBSWebSocketCloseCode::MessageDecodeError,
        "4003" => OBSWebSocketCloseCode::MissingDataField,
        "4004" => OBSWebSocketCloseCode::InvalidDataFieldType,
        "4005" => OBSWebSocketCloseCode::InvalidDataFieldValue,
        "4006" => OBSWebSocketCloseCode::UnknownOpCode,
        "4007" => OBSWebSocketCloseCode::NotIdentified,
        "4008" => OBSWebSocketCloseCode::AlreadyIdentified,
        "4009" => OBSWebSocketCloseCode::AuthenticationFailed,
        "4010" => OBSWebSocketCloseCode::UnsupportedRpcVersion,
        "4011" => OBSWebSocketCloseCode::SessionInvalidated,
        "4012" => OBSWebSocketCloseCode::UnsupportedFeature,
        _ => return Err(OBSMatchError::OBSWebSocketCloseCodeNotFound)
    })
}

pub enum OBSRequestBatchExecutionType {
    None,
    SerialRealtime,
    SerialFrame,
    Parallel
}

pub fn obs_request_batch_execution_type_match_value(obs_request_batch_execution_type: OBSRequestBatchExecutionType) -> String {
    match obs_request_batch_execution_type {
        OBSRequestBatchExecutionType::None => "-1",
        OBSRequestBatchExecutionType::SerialRealtime => "0",
        OBSRequestBatchExecutionType::SerialFrame => "1",
        OBSRequestBatchExecutionType::Parallel => "2"
    }.to_string()
}

pub fn obs_request_batch_execution_type_match_enum(obs_request_batch_execution_type: &String) -> Result<OBSRequestBatchExecutionType, OBSMatchError> {
    Ok(match &obs_request_batch_execution_type[..] {
        "-1" => OBSRequestBatchExecutionType::None,
        "0" => OBSRequestBatchExecutionType::SerialRealtime,
        "1" => OBSRequestBatchExecutionType::SerialFrame,
        "2" => OBSRequestBatchExecutionType::Parallel,
        _ => return Err(OBSMatchError::OBSRequestBatchExecutionTypeNotFound)
    })
}

pub enum OBSRequestStatus {
    Unknown,
    NoError,
    Success,
    MissingRequestType,
    UnknownRequestType,
    GenericError,
    UnsupportedRequestBatchExecutionType,
    NotReady,
    MissingRequestField,
    MissingRequestData,
    InvalidRequestField,
    InvalidRequestFieldType,
    RequestFieldOutOfRange,
    RequestFieldEmpty,
    TooManyRequestFields,
    OutputRunning,
    OutputNotRunning,
    OutputPaused,
    OutputNotPaused,
    OutputDisabled,
    StudioModeActive,
    StudioModeNotActive,
    ResourceNotFound,
    ResourceAlreadyExists,
    InvalidResourceType,
    NotEnoughResources,
    InvalidResourceState,
    InvalidInputKind,
    ResourceNotConfigurable,
    InvalidFilterKind,
    ResourceCreationFailed,
    ResourceActionFailed,
    RequestProcessingFailed,
    CannotAct
}

pub fn obs_request_status_match_value(obs_request_status: OBSRequestStatus) -> String {
    match obs_request_status {
        OBSRequestStatus::Unknown => "0",
        OBSRequestStatus::NoError => "10",
        OBSRequestStatus::Success => "100",
        OBSRequestStatus::MissingRequestType => "203",
        OBSRequestStatus::UnknownRequestType => "204",
        OBSRequestStatus::GenericError => "205",
        OBSRequestStatus::UnsupportedRequestBatchExecutionType => "206",
        OBSRequestStatus::NotReady => "207",
        OBSRequestStatus::MissingRequestField => "300",
        OBSRequestStatus::MissingRequestData => "301",
        OBSRequestStatus::InvalidRequestField => "400",
        OBSRequestStatus::InvalidRequestFieldType => "401",
        OBSRequestStatus::RequestFieldOutOfRange => "402",
        OBSRequestStatus::RequestFieldEmpty => "403",
        OBSRequestStatus::TooManyRequestFields => "404",
        OBSRequestStatus::OutputRunning => "500",
        OBSRequestStatus::OutputNotRunning => "501",
        OBSRequestStatus::OutputPaused => "502",
        OBSRequestStatus::OutputNotPaused => "503",
        OBSRequestStatus::OutputDisabled => "504",
        OBSRequestStatus::StudioModeActive => "505",
        OBSRequestStatus::StudioModeNotActive => "506",
        OBSRequestStatus::ResourceNotFound => "600",
        OBSRequestStatus::ResourceAlreadyExists => "601",
        OBSRequestStatus::InvalidResourceType => "602",
        OBSRequestStatus::NotEnoughResources => "603",
        OBSRequestStatus::InvalidResourceState => "604",
        OBSRequestStatus::InvalidInputKind => "605",
        OBSRequestStatus::ResourceNotConfigurable => "606",
        OBSRequestStatus::InvalidFilterKind => "607",
        OBSRequestStatus::ResourceCreationFailed => "700",
        OBSRequestStatus::ResourceActionFailed => "701",
        OBSRequestStatus::RequestProcessingFailed => "702",
        OBSRequestStatus::CannotAct => "703"
    }.to_string()
}

pub fn obs_request_status_match_enum(obs_request_status: String) -> Result<OBSRequestStatus, OBSMatchError> {
    Ok(match &obs_request_status[..] {
        "0" => OBSRequestStatus::Unknown,
        "10" => OBSRequestStatus::NoError,
        "100" => OBSRequestStatus::Success,
        "203" => OBSRequestStatus::MissingRequestType,
        "204" => OBSRequestStatus::UnknownRequestType,
        "205" => OBSRequestStatus::GenericError,
        "206" => OBSRequestStatus::UnsupportedRequestBatchExecutionType,
        "207" => OBSRequestStatus::NotReady,
        "300" => OBSRequestStatus::MissingRequestField,
        "301" => OBSRequestStatus::MissingRequestData,
        "400" => OBSRequestStatus::InvalidRequestField,
        "401" => OBSRequestStatus::InvalidRequestFieldType,
        "402" => OBSRequestStatus::RequestFieldOutOfRange,
        "403" => OBSRequestStatus::RequestFieldEmpty,
        "404" => OBSRequestStatus::TooManyRequestFields,
        "500" => OBSRequestStatus::OutputRunning,
        "501" => OBSRequestStatus::OutputNotRunning,
        "502" => OBSRequestStatus::OutputPaused,
        "503" => OBSRequestStatus::OutputNotPaused,
        "504" => OBSRequestStatus::OutputDisabled,
        "505" => OBSRequestStatus::StudioModeActive,
        "506" => OBSRequestStatus::StudioModeNotActive,
        "600" => OBSRequestStatus::ResourceNotFound,
        "601" => OBSRequestStatus::ResourceAlreadyExists,
        "602" => OBSRequestStatus::InvalidResourceType,
        "603" => OBSRequestStatus::NotEnoughResources,
        "604" => OBSRequestStatus::InvalidResourceState,
        "605" => OBSRequestStatus::InvalidInputKind,
        "606" => OBSRequestStatus::ResourceNotConfigurable,
        "607" => OBSRequestStatus::InvalidFilterKind,
        "700" => OBSRequestStatus::ResourceCreationFailed,
        "701" => OBSRequestStatus::ResourceActionFailed,
        "702" => OBSRequestStatus::RequestProcessingFailed,
        "703" => OBSRequestStatus::CannotAct,
        _ => return Err(OBSMatchError::OBSRequestStatusNotFound)
    })
}

pub enum OBSEventSubscription {
    None,
    General,
    Config,
    Scenes,
    Inputs,
    Transitions,
    Filters,
    Outputs,
    SceneItems,
    MediaInputs,
    Vendors,
    Ui,
    All,
    InputVolumeMeters,
    InputActiveStateChanged,
    InputShowStateChanged,
    SceneItemTransformChanged
}

pub fn obs_event_subscription_match_value(obs_event_subscription: OBSEventSubscription) -> String {
    match obs_event_subscription {
        OBSEventSubscription::None => "0",
        OBSEventSubscription::General => "(1 << 0)",
        OBSEventSubscription::Config => "(1 << 1)",
        OBSEventSubscription::Scenes => "(1 << 2)",
        OBSEventSubscription::Inputs => "(1 << 3)",
        OBSEventSubscription::Transitions => "(1 << 4)",
        OBSEventSubscription::Filters => "(1 << 5)",
        OBSEventSubscription::Outputs => "(1 << 6)",
        OBSEventSubscription::SceneItems => "(1 << 7)",
        OBSEventSubscription::MediaInputs => "(1 << 8)",
        OBSEventSubscription::Vendors => "(1 << 9)",
        OBSEventSubscription::Ui => "(1 << 10)",
        OBSEventSubscription::All => "(General | Config | Scenes | Inputs | Transitions | Filters | Outputs | SceneItems | MediaInputs | Vendors | Ui)",
        OBSEventSubscription::InputVolumeMeters => "(1 << 16)",
        OBSEventSubscription::InputActiveStateChanged => "(1 << 17)",
        OBSEventSubscription::InputShowStateChanged => "(1 << 18)",
        OBSEventSubscription::SceneItemTransformChanged => "(1 << 19)"
    }.to_string()
}

pub fn obs_event_subscription_match_enum(obs_event_subscription: &String) -> Result<OBSEventSubscription, OBSMatchError> {
    Ok(match &obs_event_subscription[..] {
        "0" => OBSEventSubscription::None,
        "(1 << 0)" => OBSEventSubscription::General,
        "(1 << 1)" => OBSEventSubscription::Config,
        "(1 << 2)" => OBSEventSubscription::Scenes,
        "(1 << 3)" => OBSEventSubscription::Inputs,
        "(1 << 4)" => OBSEventSubscription::Transitions,
        "(1 << 5)" => OBSEventSubscription::Filters,
        "(1 << 6)" => OBSEventSubscription::Outputs,
        "(1 << 7)" => OBSEventSubscription::SceneItems,
        "(1 << 8)" => OBSEventSubscription::MediaInputs,
        "(1 << 9)" => OBSEventSubscription::Vendors,
        "(1 << 10)" => OBSEventSubscription::Ui,
        "(General | Config | Scenes | Inputs | Transitions | Filters | Outputs | SceneItems | MediaInputs | Vendors | Ui)" => OBSEventSubscription::All,
        "(1 << 16)" => OBSEventSubscription::InputVolumeMeters,
        "(1 << 17)" => OBSEventSubscription::InputActiveStateChanged,
        "(1 << 18)" => OBSEventSubscription::InputShowStateChanged,
        "(1 << 19)" => OBSEventSubscription::SceneItemTransformChanged,
        _ => return Err(OBSMatchError::OBSEventSubscriptionNotFound)
    })
}