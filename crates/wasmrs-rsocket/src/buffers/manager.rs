use std::collections::HashMap;

use wasmrs_ringbuffer::{ReadOnlyRingBuffer, RingBuffer, VecRingBuffer};

use crate::{
    frames::FrameCodec,
    generated::{FragmentedPayload, Frame, FrameHeader, FrameType, Payload},
    util::from_u32_bytes,
};

use super::Stream;

#[derive(Default)]
pub(super) struct Manager {
    streams: HashMap<u32, Stream>,
    fragments: HashMap<u32, FragmentedPayload>,

    // circular buffers
    local_buffer: VecRingBuffer<u8>,
    foreign_buffer: VecRingBuffer<u8>,
    foreign_frame_size: u32,
}

impl Manager {
    pub(super) fn init(&mut self, local_capacity: u32, foreign_capacity: u32, frame_size: u32) {
        self.local_buffer.resize(local_capacity as usize, || 0);
        self.foreign_buffer.resize(foreign_capacity as usize, || 0);
        self.foreign_frame_size = frame_size;
    }
    pub(super) fn get_pointers(&self) -> (u32, u32) {
        (
            self.local_buffer.as_ptr() as usize as _,
            self.foreign_buffer.as_ptr() as usize as _,
        )
    }

    fn write_local<A: IntoIterator<Item = u8>>(&mut self, position: usize, iter: A) {
        self.local_buffer.write_at(position, iter);
    }
    fn remove_stream(&mut self, id: u32) {
        self.streams.remove(&id);
    }
    fn handle_frame(
        &mut self,
        header: FrameHeader,
        buffer: Vec<u8>,
    ) -> Result<(), crate::frames::Error> {
        let stream_id = header.stream_id();
        match header.frame_type() {
            FrameType::Reserved => todo!(),
            FrameType::Setup => todo!(),
            FrameType::RequestResponse => todo!(),
            FrameType::RequestFnf => todo!(),
            FrameType::RequestStream => todo!(),
            FrameType::RequestChannel => todo!(),
            FrameType::RequestN => todo!(),
            FrameType::Cancel => {
                // TODO: trigger ONCOMPLETE
                self.remove_stream(header.stream_id())
            }
            FrameType::Payload => {
                let mut payload = Payload::decode(buffer)?;
                let fragment = if let Some(mut fragment) = self.fragments.remove(&stream_id) {
                    fragment.data.append(&mut payload.data);
                    fragment.metadata.append(&mut payload.metadata);
                    fragment
                } else {
                    FragmentedPayload {
                        initial_n: 0,
                        metadata: payload.metadata,
                        data: payload.data,
                        frame_type: FrameType::Payload,
                    }
                };

                if payload.follows {
                    self.fragments.insert(stream_id, fragment);
                } else {
                    match &fragment.frame_type {
                        FrameType::Payload => {
                            if payload.next {
                                // TODO: trigger ONNEXT
                            }
                        }
                        FrameType::RequestResponse => self.handle_request_response(
                            stream_id,
                            fragment.data,
                            fragment.metadata,
                        ),
                        FrameType::RequestStream => self.handle_request_stream(
                            stream_id,
                            fragment.data,
                            fragment.metadata,
                            fragment.initial_n,
                        ),
                        FrameType::RequestChannel => self.handle_request_stream(
                            stream_id,
                            fragment.data,
                            fragment.metadata,
                            fragment.initial_n,
                        ),
                        _ => todo!(), // Maybe not todo?,
                    }
                    if payload.complete {
                        // TODO: trigger ONCOMPLETE
                        todo!();
                    }
                }
            }
            FrameType::Err => {
                let error = crate::generated::ErrorFrame::decode(buffer)?;
                // TODO: trigger ONERROR
                // TODO: trigger ONCOMPLETE
                self.remove_stream(stream_id);
            }
            FrameType::Ext => todo!(),
            _ => todo!(), // Maybe not todo?,
        };

        Ok(())
    }

    fn handle_request_response(&mut self, stream_id: u32, data: Vec<u8>, metadata: Vec<u8>) {
        todo!();
    }
    fn handle_request_stream(
        &mut self,
        stream_id: u32,
        data: Vec<u8>,
        metadata: Vec<u8>,
        initial_n: u32,
    ) {
        todo!();
    }
    fn handle_request_channel(
        &mut self,
        stream_id: u32,
        data: Vec<u8>,
        metadata: Vec<u8>,
        initial_n: u32,
    ) {
        todo!();
    }

    pub(super) fn send(&mut self, next_pos: u32) {
        self.local_buffer.update_read_pos(next_pos as usize);
        let len_bytes: Vec<_> = self.local_buffer.read(4).collect();
        println!("len bytes: {:?}", len_bytes);
        let len = from_u32_bytes(&len_bytes);
        println!("len: {}", len);
        let mut frame_data: Vec<_> = self.local_buffer.read(len as usize).collect();

        let header = FrameHeader::from_bytes(frame_data[0..Frame::LEN_HEADER].to_vec());
        let stream_id = header.stream_id();
        let buffer = frame_data.drain(Frame::LEN_HEADER..).collect();
        println!("header:{}", header);
        println!("buffer:{:?}", buffer);
        if let Err(e) = self.handle_frame(header, buffer) {
            let frame = Frame::ErrorFrame(Box::new(crate::generated::ErrorFrame {
                stream_id,
                code: 99,
                data: "Error decoding frame".to_owned(),
            }));
            // TODO: trigger ONERROR
            // TODO: trigger ONCOMPLETE
            self.remove_stream(stream_id);
        }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::buffers::manager::Manager;

    #[test]
    #[ignore]
    fn test_send() -> Result<()> {
        static BYTES: &[u8] = include_bytes!("../../testdata/frame.payload.bin");
        let mut manager = Manager::default();
        manager.init(1024, 1024, 16);
        let len_bytes = (BYTES.len() as u32).to_be_bytes();
        manager.write_local(1, len_bytes);
        manager.write_local(5, BYTES.into());
        manager.send(1);
        // super::guest::send(next_pos)

        Ok(())
    }
}
