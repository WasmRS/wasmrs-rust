import { utils } from "@apexlang/codegen/rust";

import { Context, Interface, ObjectMap, Operation } from "@apexlang/core/model";
import { convertDescription, convertParameter } from "../utils/conversions.js";
import { constantCase } from "../utils/index.js";
import { convertType } from "../utils/types.js";

import { SourceGenerator } from "./base.js";
const { rustify, rustifyCaps, trimLines } = utils;

export class ProviderVisitor extends SourceGenerator<Interface> {
  index = 0;
  imports: [string, string][] = [];
  wrappers: string[] = [];
  types: string[] = [];

  constructor(context: Context, indexStart: number) {
    super(context.interface, context);
    this.index = indexStart;
    this.walk();
  }

  buffer(): string {
    const rootName = rustifyCaps(this.node.name);

    const module_name = `${rustify(this.node.name)}`;

    const innerSource = this.writer.string();

    const comment = convertDescription(this.node.description);

    const indexConstants = this.node.operations.map((op, i) => {
      return `static ${constantCase(
        `${rootName}_${op.name}`
      )}_INDEX_BYTES: [u8; 4] = ${this.index + i}u32.to_be_bytes();`;
    });

    return `
${indexConstants.join("\n")}
${trimLines([comment])}
pub mod ${module_name} {
  use super::*;
  ${innerSource}
}
`;
  }

  visitOperation(context: Context): void {
    const { operation } = context;

    const source = convertOperation(
      operation,
      this.node.name,
      false,
      this.config
    );
    this.imports.push([this.node.name, operation.name]);

    this.write(source);
  }
}

export function convertOperation(
  op: Operation,
  interfaceName: string,
  global: boolean,
  config: ObjectMap
): string {
  const name = rustify(op.name);
  const indexConstant = constantCase(`${interfaceName}_${name}`);

  const comment = convertDescription(op.description);

  const inputFields = op.parameters
    .map((p) => {
      return `
  #[serde(rename = "${p.name}")]
  pub(crate) ${rustify(p.name)}: ${convertType(p.type, config, true, "'a")},
  `;
    })
    .join("\n");

  return `
${trimLines([comment])}
pub(crate) fn ${name}(
  inputs: ${name}::Inputs<'_>,
) -> wasmrs_guest::Mono<${name}::Outputs, PayloadError> {
  let op_id_bytes = ${indexConstant}_INDEX_BYTES.as_slice();
  let payload = match wasmrs_guest::serialize(&inputs) {
      Ok(bytes) => Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()),
      Err(e) => return Mono::new_error(PayloadError::application_error(e.to_string())),
  };
  let fut = Host::default().request_response(payload).map(|result| {
      result
          .map(|payload| Ok(deserialize::<${name}::Outputs>(&payload.data.unwrap())?))?
  });
  Mono::from_future(fut)
}

pub(crate) mod ${name} {
  use super::*;

  #[derive(serde::Serialize)]
  pub struct Inputs<'a> {
    ${inputFields}
  }

  pub(crate) type Outputs = ${convertType(op.type, config)};
}
`;
}
