import { Alias, Context } from "@apexlang/core/model";
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
export declare class AliasVisitor extends SourceGenerator<Alias> {
    constructor(context: Context);
    buffer(): string;
}
//# sourceMappingURL=alias-visitor.d.ts.map