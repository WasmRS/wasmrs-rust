#[link(wasm_import_module = "wasmrs")]
extern "C" {
  #[link_name = "__init_buffers"]
  pub(crate) fn _host_wasmrs_init(guest_buffer_ptr: usize, host_buffer_ptr: usize);
  #[link_name = "__send"]
  pub(crate) fn _host_wasmrs_send(size: usize);
  #[link_name = "__op_list"]
  pub(crate) fn _host_op_list(ptr: usize, len: usize);
}
