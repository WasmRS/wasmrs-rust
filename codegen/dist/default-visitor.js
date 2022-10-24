import { RustBasic } from "@apexlang/codegen/rust";
import { ServiceVisitor } from "./visitors/service-visitor.js";
import { ProviderVisitor } from "./visitors/provider-visitor.js";
import { constantCase } from "./utils/index.js";
import { utils } from "@apexlang/codegen/rust";
export class DefaultVisitor extends RustBasic {
  constructor() {
    super(...arguments);
    this.namespace = "";
    this.exports = [];
    this.imports = [];
  }
  visitNamespace(context) {
    const { namespace } = context;
    this.namespace = namespace.name;
  }
  visitContextBefore(context) {
    super.visitContextBefore(context);
    this.append(`
use wasmrs_guest::FutureExt;

use wasmrs_guest::*;

#[no_mangle]
extern "C" fn __wasmrs_init(
    guest_buffer_size: u32,
    host_buffer_size: u32,
    max_host_frame_len: u32,
) {
    #[cfg(all(target= "wasm32-wasi", debug_assertions))]
    env_logger::builder()
        .default_format()
        .filter(None, tracing::log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .init();
    init_exports();
    init_imports();
    wasmrs_guest::init(guest_buffer_size, host_buffer_size, max_host_frame_len);
}`);
  }
  visitContextAfter(context) {
    super.visitContextAfter(context);
    const imports = this.imports.map(([iface, op]) => {
      const importConstant = constantCase(`${iface}_${op}`);
      return `
wasmrs_guest::add_import(
  u32::from_be_bytes(${importConstant}_INDEX_BYTES),OperationType::RequestResponse,"${this.namespace}.${iface}","${op}",
);`;
    });
    const exports = this.exports.map(([iface, op]) => {
      return `
wasmrs_guest::register_request_response(
  "${this.namespace}.${iface}","${op}",${utils.rustifyCaps(
        `${iface}Component`
      )}::${utils.rustify(op)}_wrapper,
);`;
    });
    this.write(`
pub(crate) fn init_imports() {
  ${imports.join("\n")}
}
pub(crate) fn init_exports() {
  ${exports.join("\n")}
}
    `);
  }
  visitInterface(context) {
    if (context.interface.annotation("service")) {
      const visitor = new ServiceVisitor(context);
      this.exports.push(...visitor.exports);
      this.append(visitor.buffer());
    } else if (context.interface.annotation("provider")) {
      const visitor = new ProviderVisitor(context, this.imports.length);
      this.imports.push(...visitor.imports);
      this.append(visitor.buffer());
    } else {
      super.visitInterface(context);
    }
  }
}
//# sourceMappingURL=default-visitor.js.map
