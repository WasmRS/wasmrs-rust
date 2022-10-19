use bytes::{BufMut, Bytes, BytesMut};

use crate::util::{from_u16_bytes, from_u32_bytes};

/*
WasmRS operations list header

 0                   1                   2
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|       |   |       |                           |
+-------+---+-------+---------------------------+
|"\0wrs"| v | # ops |    operations...          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

v (u16): Version
# Ops (u32): The number of operations to parse.

Operations

 0                   1
 0 1 2 3 4 5 6 7 8 9 0 ...
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| | |       |   |                   |   |                     |
+-+-+-------+---+-------------------+---+---------------------+
|A|B| Index | N |  Namespace[0..N]  | O |  OpLen[N+2..N+2+O]  |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
A (u8): Operation type
B (u8): Direction (import/export)
Index (u32): The operation index to use when calling
N (u16): The length of the Namespace buffer
Namespace (u8[]): The Namespace String as UTF-8 bytes
O (u16): The length of the Operation buffer
Operation (u8[]): The Operation String as UTF-8 bytes
*/

static WASMRS_MAGIC: [u8; 4] = [0x00, 0x77, 0x72, 0x73];

#[derive(Debug, Copy, Clone)]
pub enum OperationType {
    RequestResponse,
    RequestFnF,
    RequestStream,
    RequestChannel,
}

impl From<u8> for OperationType {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::RequestResponse,
            2 => Self::RequestFnF,
            3 => Self::RequestStream,
            4 => Self::RequestChannel,
            _ => unreachable!("Bad Operation Type {}", v),
        }
    }
}

impl From<OperationType> for u8 {
    fn from(op: OperationType) -> Self {
        match op {
            OperationType::RequestResponse => 1,
            OperationType::RequestFnF => 2,
            OperationType::RequestStream => 3,
            OperationType::RequestChannel => 4,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    Magic,
    Version,
    Utf8String,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::Magic => "Bad magic bytes",
            Error::Version => "Bad version",
            Error::Utf8String => "Could not convert bytes to UTF-8 String",
        })
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Self::Utf8String
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    index: u32,
    kind: OperationType,
    namespace: String,
    operation: String,
}

#[derive(Debug, Default, Clone)]
pub struct OperationList {
    imports: Vec<Operation>,
    exports: Vec<Operation>,
}

impl OperationList {
    #[must_use]
    pub fn get_import(&self, namespace: &str, operation: &str) -> Option<u32> {
        Self::get_op(&self.imports, namespace, operation)
    }

    #[must_use]
    pub fn get_export(&self, namespace: &str, operation: &str) -> Option<u32> {
        Self::get_op(&self.exports, namespace, operation)
    }

    fn get_op(list: &[Operation], namespace: &str, operation: &str) -> Option<u32> {
        list.iter()
            .find(|op| op.namespace == namespace && op.operation == operation)
            .map(|op| op.index)
    }

    pub fn add_export(
        &mut self,
        index: u32,
        kind: OperationType,
        namespace: impl AsRef<str>,
        operation: impl AsRef<str>,
    ) {
        Self::add_op(&mut self.exports, index, kind, namespace, operation);
    }
    pub fn add_import(
        &mut self,
        index: u32,
        kind: OperationType,
        namespace: impl AsRef<str>,
        operation: impl AsRef<str>,
    ) {
        Self::add_op(&mut self.imports, index, kind, namespace, operation);
    }
    pub fn add_op(
        list: &mut Vec<Operation>,
        index: u32,
        kind: OperationType,
        namespace: impl AsRef<str>,
        operation: impl AsRef<str>,
    ) {
        list.push(Operation {
            index,
            kind,
            namespace: namespace.as_ref().to_owned(),
            operation: operation.as_ref().to_owned(),
        });
    }

    #[must_use]
    pub fn encode(&self) -> Bytes {
        let mut buff = BytesMut::new();
        let num_ops: u32 = (self.imports.len() + self.exports.len()) as u32;
        let version = 1u16;
        buff.put(WASMRS_MAGIC.as_slice());
        buff.put(version.to_be_bytes().as_slice());
        buff.put(num_ops.to_be_bytes().as_slice());
        for op in &self.exports {
            buff.put(Self::encode_op(op, 1));
        }
        for op in &self.imports {
            buff.put(Self::encode_op(op, 2));
        }
        buff.freeze()
    }

    fn encode_op(op: &Operation, dir: u8) -> Bytes {
        let mut buff = BytesMut::new();

        let kind: u8 = op.kind.into();
        buff.put([kind].as_slice());
        buff.put([dir].as_slice());
        buff.put(op.index.to_be_bytes().as_slice());
        buff.put((op.namespace.len() as u16).to_be_bytes().as_slice());
        buff.put(op.namespace.as_bytes());
        buff.put((op.operation.len() as u16).to_be_bytes().as_slice());
        buff.put(op.operation.as_bytes());
        buff.put(0_u16.to_be_bytes().as_slice());
        buff.freeze()
    }

    pub fn decode(mut buf: Bytes) -> Result<Self, Error> {
        let magic = buf.split_to(4);
        if magic != WASMRS_MAGIC.as_slice() {
            return Err(Error::Magic);
        }
        let version = from_u16_bytes(&buf.split_to(2));
        match version {
            1 => Self::decode_v1(buf),
            _ => Err(Error::Version),
        }
    }

    fn decode_v1(mut buf: Bytes) -> Result<Self, Error> {
        let num_ops = from_u32_bytes(&buf.split_to(4));
        let mut imports = Vec::new();
        let mut exports = Vec::new();
        for _ in 0..num_ops {
            let kind = buf.split_to(1)[0];
            let kind: OperationType = kind.into();
            let dir = buf.split_to(1)[0];
            let index = from_u32_bytes(&buf.split_to(4));
            let ns_len = from_u16_bytes(&buf.split_to(2));
            let namespace = String::from_utf8(buf.split_to(ns_len as _).to_vec())?;
            let op_len = from_u16_bytes(&buf.split_to(2));
            let operation = String::from_utf8(buf.split_to(op_len as _).to_vec())?;
            let reserved_len = from_u16_bytes(&buf.split_to(2));
            let op = Operation {
                index,
                kind,
                namespace,
                operation,
            };
            if dir == 1 {
                exports.push(op);
            } else {
                imports.push(op);
            }
        }
        Ok(Self { imports, exports })
    }
}
