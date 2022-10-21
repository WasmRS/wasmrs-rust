import { convertDescription } from "../utils/conversions.js";
import { convertType } from "../utils/types.js";
import { SourceGenerator } from "./base.js";
/**
 * Apex aliases come from syntax like this:
 *
 * ```apexlang
 *
 * alias MyType = string
 *
 * ```
 *
 * View the model here: https://apexlang.github.io/ast-viewer/#CmFsaWFzIE15VHlwZSA9IHN0cmluZwo=
 *
 */
export class AliasVisitor extends SourceGenerator {
    constructor(context) {
        super(context.alias, context);
    }
    buffer() {
        // The name of the Alias from the Apex schema.
        const name = this.node.name;
        // A comment generated from the description.
        const comment = convertDescription(this.node.description);
        const type = convertType(this.node.type, this.context.config);
        // Combine the above to create and return new output here.
        return ``;
    }
}
//# sourceMappingURL=alias-visitor.js.map