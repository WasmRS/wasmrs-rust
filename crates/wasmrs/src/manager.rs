use crate::{fragmentation::Joiner, frames::RSocketFlags, Frame};

pub struct Manager {
    joiners: SafeMap<u32, Joiner>,
}

impl Manager {
    fn join_frame(&self, input: Frame) -> Option<Frame> {
        let (is_follow, is_payload) = input.is_followable_or_payload();
        if !is_follow {
            return Some(input);
        }
        let sid = input.stream_id();
        if input.get_flag().flag_follows() {
            self.joiners
                .entry(sid)
                .or_insert_with(Joiner::new)
                .push(input);
            return None;
        }

        if !is_payload {
            return Some(input);
        }

        match joiners.remove(&sid) {
            None => Some(input),
            Some((_, mut joiner)) => {
                joiner.push(input);
                let flag = joiner.get_flag();
                let first = joiner.first();
                match &first {
                    Frame::RequestResponse(_) => {
                        let pa: Payload = joiner.into();
                        Some(Frame::new_request_response(sid, pa, flag, 0))
                    }
                    Frame::RequestStream(b) => {
                        let n = b.0.initial_n;
                        let pa: Payload = joiner.into();

                        Some(Frame::new_request_stream(sid, pa, flag, n))
                    }
                    Frame::RequestFnF(_) => {
                        let pa: Payload = joiner.into();
                        Some(Frame::new_request_stream(sid, pa, flag, 0))
                    }
                    Frame::RequestChannel(b) => {
                        let n = b.0.initial_n;
                        let pa: Payload = joiner.into();
                        Some(Frame::new_request_channel(sid, pa, flag, n))
                    }
                    Frame::PayloadFrame(b) => {
                        let pa: Payload = joiner.into();
                        Some(Frame::new_payload(sid, pa, flag))
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
