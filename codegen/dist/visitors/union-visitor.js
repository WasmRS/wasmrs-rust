import { convertDescription } from "../utils/conversions.js";
import { convertType } from "../utils/types.js";
import { SourceGenerator } from "./base.js";
/**
 * Apex type definitions come from syntax like this:
 *
 * ```apexlang
 * union Animal = Dog | Cat
 * ```
 *
 * View a sample model here:
 * https://apexlang.github.io/ast-viewer/#dW5pb24gQW5pbWFsID0gRG9nIHwgQ2F0
 */
export class UnionVisitor extends SourceGenerator {
    constructor(context) {
        super(context.union, context);
    }
    buffer() {
        // Iterate over each type in the Union and convert it
        // with `convertType()`
        const types = this.node.types.map((t) => {
            return convertType(t, this.config);
        });
        // A comment generated from the description.
        const comment = convertDescription(this.node.description);
        // The name of the Union as defined in the Apex schema.
        const name = this.node.name;
        // Combine the above to create and return new output.
        return ``;
    }
}
//# sourceMappingURL=union-visitor.js.map