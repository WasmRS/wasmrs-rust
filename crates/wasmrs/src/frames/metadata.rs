use bytes::{BufMut, Bytes, BytesMut};

use crate::Metadata;

impl Metadata {
    pub fn new(namespace: impl AsRef<str>, operation: impl AsRef<str>) -> Metadata {
        Metadata {
            namespace: namespace.as_ref().to_owned(),
            operation: operation.as_ref().to_owned(),
            instance: Bytes::new(),
        }
    }
    #[must_use]
    pub fn encode(self) -> Bytes {
        let len = self.namespace.len() + self.operation.len() + self.instance.len() + 2 + 2 + 2;
        let mut bytes = BytesMut::with_capacity(len);
        bytes.put((self.namespace.len() as u16).to_be_bytes().as_slice());
        bytes.put(self.namespace.into_bytes().as_slice());
        bytes.put((self.operation.len() as u16).to_be_bytes().as_slice());
        bytes.put(self.operation.into_bytes().as_slice());
        bytes.put((self.instance.len() as u16).to_be_bytes().as_slice());
        bytes.put(self.instance);

        debug_assert_eq!(
            bytes.len(),
            len,
            "encoded metadata is not the correct length."
        );
        bytes.freeze()
    }
}
