use crate::{op_list_request, send_frame};

#[allow(unsafe_code)]
#[no_mangle]
extern "C" fn __wasmrs_op_list_request() {
  op_list_request();
}

#[allow(unsafe_code)]
#[no_mangle]
extern "C" fn __wasmrs_send(read_until: u32) {
  send_frame(read_until);
}

#[allow(unsafe_code)]
#[no_mangle]
extern "C" fn __wasmrs_v1() {
  /* no-op */
}
