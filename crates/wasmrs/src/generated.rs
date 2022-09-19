pub type FrameFlags = u16;
pub type FrameTypeId = u8;

/// Six (6) bytes reserved for FrameHeader information.
#[derive()]
pub(crate) struct FrameHeader {
    pub(crate) header: Vec<u8>,
}

#[derive()]
pub(crate) struct FragmentedPayload {
    pub(crate) frame_type: FrameType,
    pub(crate) initial_n: u32,
    pub(crate) metadata: Vec<u8>,
    pub(crate) data: Vec<u8>,
}
/// A Payload frame.
#[derive()]
pub(crate) struct Payload {
    /// The stream ID this frame belongs to.
    pub(crate) stream_id: u32,
    /// Any metadata associated with the Payload as raw bytes.
    pub(crate) metadata: Vec<u8>,
    /// The actual payload data as raw bytes.
    pub(crate) data: Vec<u8>,
    /// Whether this payload is broken up into multiple frames.
    pub(crate) follows: bool,
    /// Whether or not this frame is the last frame in a stream.
    pub(crate) complete: bool,
    /// TODO
    pub(crate) next: bool,
}

#[derive()]
pub(crate) struct Cancel {
    /// The stream ID this frame belongs to.
    pub(crate) stream_id: u32,
}

#[derive()]
pub(crate) struct ErrorFrame {
    /// The stream ID this frame belongs to.
    pub(crate) stream_id: u32,
    pub(crate) code: u32,
    pub(crate) data: String,
}

#[derive()]
pub(crate) struct RequestN {
    /// The stream ID this frame belongs to.
    pub(crate) stream_id: u32,
    pub(crate) n: u32,
}

#[derive()]
pub(crate) struct RequestResponse {
    /// The stream ID this frame belongs to.
    pub(crate) stream_id: u32,
    /// Any metadata associated with the Payload as raw bytes.
    pub(crate) metadata: Vec<u8>,
    /// The actual payload data as raw bytes.
    pub(crate) data: Vec<u8>,
    /// Whether this payload is broken up into multiple frames.
    pub(crate) follows: bool,
    /// Whether or not this frame is the last frame in a stream.
    pub(crate) complete: bool,
    /// TODO
    pub(crate) initial_n: u32,
}

#[derive()]
pub(crate) struct FireAndForget {
    /// The stream ID this frame belongs to.
    pub(crate) stream_id: u32,
    /// Any metadata associated with the Payload as raw bytes.
    pub(crate) metadata: Vec<u8>,
    /// The actual payload data as raw bytes.
    pub(crate) data: Vec<u8>,
    /// Whether this payload is broken up into multiple frames.
    pub(crate) follows: bool,
    /// Whether or not this frame is the last frame in a stream.
    pub(crate) complete: bool,
    /// TODO
    pub(crate) initial_n: u32,
}

#[derive()]
pub(crate) struct RequestStream {
    /// The stream ID this frame belongs to.
    pub(crate) stream_id: u32,
    /// Any metadata associated with the Payload as raw bytes.
    pub(crate) metadata: Vec<u8>,
    /// The actual payload data as raw bytes.
    pub(crate) data: Vec<u8>,
    /// Whether this payload is broken up into multiple frames.
    pub(crate) follows: bool,
    /// Whether or not this frame is the last frame in a stream.
    pub(crate) complete: bool,
    /// TODO
    pub(crate) initial_n: u32,
}

#[derive()]
pub(crate) struct RequestChannel {
    /// The stream ID this frame belongs to.
    pub(crate) stream_id: u32,
    /// Any metadata associated with the Payload as raw bytes.
    pub(crate) metadata: Vec<u8>,
    /// The actual payload data as raw bytes.
    pub(crate) data: Vec<u8>,
    /// Whether this payload is broken up into multiple frames.
    pub(crate) follows: bool,
    /// Whether or not this frame is the last frame in a stream.
    pub(crate) complete: bool,
    /// TODO
    pub(crate) initial_n: u32,
}
#[derive()]
pub(crate) enum Frame {
    Payload(Box<Payload>),
    Cancel(Box<Cancel>),
    ErrorFrame(Box<ErrorFrame>),
    RequestN(Box<RequestN>),
    RequestResponse(Box<RequestResponse>),
    FireAndForget(Box<FireAndForget>),
    RequestStream(Box<RequestStream>),
    RequestChannel(Box<RequestChannel>),
}

#[derive()]
pub(crate) enum FrameType {
    Reserved,
    Setup,
    Lease,
    Keepalive,
    RequestResponse,
    RequestFnf,
    RequestStream,
    RequestChannel,
    RequestN,
    Cancel,
    Payload,
    Err,
    MetadataPush,
    Resume,
    ResumeOk,
    Ext,
}
impl std::fmt::Display for FrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Reserved => "RESERVED",
                Self::Setup => "SETUP",
                Self::Lease => "LEASE",
                Self::Keepalive => "KEEPALIVE",
                Self::RequestResponse => "REQUEST_RESPONSE",
                Self::RequestFnf => "REQUEST_FNF",
                Self::RequestStream => "REQUEST_STREAM",
                Self::RequestChannel => "REQUEST_CHANNEL",
                Self::RequestN => "REQUEST_N",
                Self::Cancel => "CANCEL",
                Self::Payload => "PAYLOAD",
                Self::Err => "ERROR",
                Self::MetadataPush => "METADATA_PUSH",
                Self::Resume => "RESUME",
                Self::ResumeOk => "RESUME_OK",
                Self::Ext => "EXT",
            }
        )
    }
}
impl std::convert::TryFrom<u32> for FrameType {
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn try_from(index: u32) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(Self::Reserved),
            1 => Ok(Self::Setup),
            2 => Ok(Self::Lease),
            3 => Ok(Self::Keepalive),
            4 => Ok(Self::RequestResponse),
            5 => Ok(Self::RequestFnf),
            6 => Ok(Self::RequestStream),
            7 => Ok(Self::RequestChannel),
            8 => Ok(Self::RequestN),
            9 => Ok(Self::Cancel),
            10 => Ok(Self::Payload),
            11 => Ok(Self::Err),
            12 => Ok(Self::MetadataPush),
            13 => Ok(Self::Resume),
            14 => Ok(Self::ResumeOk),
            63 => Ok(Self::Ext),
            _ => Err(format!("{} is not a valid index for FrameType", index).into()),
        }
    }
}
impl Into<u32> for FrameType {
    fn into(self) -> u32 {
        match self {
            Self::Reserved => unreachable!(),
            Self::Setup => 1,
            Self::Lease => 2,
            Self::Keepalive => 3,
            Self::RequestResponse => 4,
            Self::RequestFnf => 5,
            Self::RequestStream => 6,
            Self::RequestChannel => 7,
            Self::RequestN => 8,
            Self::Cancel => 9,
            Self::Payload => 10,
            Self::Err => 11,
            Self::MetadataPush => 12,
            Self::Resume => 13,
            Self::ResumeOk => 14,
            Self::Ext => 63,
        }
    }
}

#[derive()]
pub(crate) enum FrameFlag {
    Metadata,
    Follows,
    Complete,
    Next,
    Ignore,
}
impl std::fmt::Display for FrameFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Metadata => "M",
                Self::Follows => "FRS",
                Self::Complete => "CL",
                Self::Next => "N",
                Self::Ignore => "I",
            }
        )
    }
}
impl std::convert::TryFrom<u32> for FrameFlag {
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn try_from(index: u32) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(Self::Metadata),
            1 => Ok(Self::Follows),
            2 => Ok(Self::Complete),
            3 => Ok(Self::Next),
            4 => Ok(Self::Ignore),
            _ => Err(format!("{} is not a valid index for FrameFlag", index).into()),
        }
    }
}
impl Into<u32> for FrameFlag {
    fn into(self) -> u32 {
        match self {
            Self::Metadata => unreachable!(),
            Self::Follows => 1,
            Self::Complete => 2,
            Self::Next => 3,
            Self::Ignore => 4,
        }
    }
}
