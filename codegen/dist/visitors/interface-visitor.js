import { convertDescription, convertParameter } from "../utils/conversions.js";
import { convertType } from "../utils/types.js";
import { SourceGenerator } from "./base.js";
export class InterfaceVisitor extends SourceGenerator {
    constructor(context) {
        super(context.interface, context);
        this.walk();
    }
    buffer() {
        // The name of the Interface from the Apex schema.
        const name = this.node.name;
        // Get the buffered output. Your visitor operations write
        // to this buffer when they call `.write()`.
        const innerSource = this.writer.string();
        // A comment generated from the description.
        const comment = convertDescription(this.node.description);
        // Combine the above to create and return new output here.
        return ``;
    }
    visitOperation(context) {
        const { operation } = context;
        // Generate new output from `convertOperation()` below.
        const converted = convertOperation(operation, false, this.config);
        // Append to the buffer in `this.writer`. Get the buffer's
        // state by calling `this.writer.string()`.
        this.write(``);
    }
}
/**
 * Generate new source for an Operation
 *
 * @param op - An Operation node to convert
 * @param global - Whether this is a global operation (`func`) or a method in an interface.
 * @param config - The context's configuration.
 * @returns The new generated output for the Operation
 *
 */
export function convertOperation(op, global, config) {
    // The name of the Operation.
    const name = op.name;
    // A comment generated from the description.
    const comment = convertDescription(op.description);
    // The return type of the operation, converted via `convertType()`
    const type = convertType(op.type, config);
    // Iterate over the Operation's Parameters and generate new output.
    const params = op.parameters.map((arg) => convertParameter(arg, config));
    if (global) {
        // Generate output for global functions here.
    }
    else {
        // Generate method output here.
    }
    // Combine the above to create and return new output.
    return ``;
}
//# sourceMappingURL=interface-visitor.js.map