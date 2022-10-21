import { Context, Enum } from "@apexlang/core/model";
import { convertDescription } from "../utils/conversions.js";

import { SourceGenerator } from "./base.js";

/**
 * Apex enums come from syntax like this:
 *
 * ```apexlang
 * enum TrafficLight {
 *  red = 0 as "Red"
 *  yellow = 2 as "Yellow"
 *  green = 3 as "Green"
 * }
 * ```
 *
 * View a sample model here:
 * https://apexlang.github.io/ast-viewer/#CmVudW0gVHJhZmZpY0xpZ2h0IHsKCXJlZCA9IDAgYXMgIlJlZCIKICAgIHllbGxvdyA9IDIgYXMgIlllbGxvdyIKICAgIGdyZWVuID0gMyBhcyAiR3JlZW4iCn0K
 */
export class EnumVisitor extends SourceGenerator<Enum> {
  constructor(context: Context) {
    super(context.enum, context);
  }

  buffer(): string {
    // The name of the Enum from the Apex schema.
    const name = this.node.name;

    // Get the buffered output. Your visitor operations write
    // to this buffer when they call `.write()`.
    const innerSource = this.writer.string();

    // A comment generated from the description.
    const comment = convertDescription(this.node.description);

    // Combine the above to create and return new output here.
    return ``;
  }

  visitEnumValue(context: Context): void {
    const { enumValue } = context;

    // The name of the EnumValue variant.
    const name = enumValue.name;

    // The display value for the Enum (if defined).
    const display = enumValue.display;

    // The index of the Enum (if defined).
    const index = enumValue.index;

    // A comment generated from the description.
    const comment = convertDescription(this.node.description);

    // Append to the buffer in `this.writer`. Get the buffer's
    // state by calling `this.writer.string()`.
    this.write(``);
  }
}
