import { utils } from "@apexlang/codegen/rust";
import { Context, Interface, ObjectMap, Operation } from "@apexlang/core/model";
import { convertDescription } from "../utils/conversions.js";
import { convertType } from "../utils/types.js";

import { SourceGenerator } from "./base.js";

const { rustify, rustifyCaps, trimLines } = utils;

export class ServiceVisitor extends SourceGenerator<Interface> {
  exports: [string, string][] = [];
  wrappers: string[] = [];
  types: string[] = [];

  constructor(context: Context) {
    super(context.interface, context);
    this.walk();
  }

  buffer(): string {
    const rootName = rustifyCaps(this.node.name);
    const componentName = `${rootName}Component`;
    const serviceName = `${rootName}Service`;
    const service_module = `${rustify(this.node.name)}_service`;

    const innerSource = this.writer.string();

    const comment = convertDescription(this.node.description);

    return `
pub(crate) struct ${componentName}();

impl ${componentName} {
  ${this.wrappers.join("\n")}
}

#[async_trait::async_trait(?Send)]
${trimLines([comment])}
pub(crate) trait ${serviceName} {
  ${innerSource}
}

pub mod ${service_module} {
  use super::*;
  ${this.types.join("\n")}
}
`;
  }

  visitOperation(context: Context): void {
    const { operation } = context;

    const [traitFn, wrapper, types] = convertOperation(
      operation,
      false,
      this.config
    );
    this.wrappers.push(wrapper);
    this.types.push(types);
    this.exports.push([this.node.name, operation.name]);

    this.write(traitFn);
  }
}

export function convertOperation(
  op: Operation,
  global: boolean,
  config: ObjectMap
): [string, string, string] {
  const name = rustify(op.name);
  const service_module = `${op.name}_service`;
  const component_name = `${rustifyCaps(op.name)}Component`;

  const comment = convertDescription(op.description);

  const traitFn = `
${trimLines([comment])}
async fn ${name}(
  inputs: Mono<${service_module}::${name}::Inputs, PayloadError>,
) -> Result<${service_module}::${name}::Outputs, GenericError>;
`;

  const wrapper = `
fn ${name}_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
  let (tx, rx) = runtime::oneshot();

  let input = Mono::from_future(input.map(|r| r.map(|v| Ok(deserialize(&v.data)?))?));
  let task = ${component_name}::
      ${name}(input)
      .map(|result| {
          let output = result?;
          Ok(serialize(&output).map(|bytes| Payload::new_optional(None, Some(bytes.into())))?)
      })
      .map(|output| tx.send(output).unwrap());

  spawn(task);

  Ok(Mono::from_future(async move { rx.await? }))
}`;

  const inputFields = op.parameters
    .map((p) => {
      return `
  #[serde(rename = "${p.name}")]
  pub(crate) ${rustify(p.name)}: ${convertType(p.type, config)},
  `;
    })
    .join("\n");

  const types = `
pub mod ${name} {
  use super::*;
  #[derive(serde::Deserialize, Debug)]
  pub(crate) struct Inputs {
    ${inputFields}
  }

  pub(crate) type Outputs = ${convertType(op.type, config)};
}  `;
  return [traitFn, wrapper, types];
}
