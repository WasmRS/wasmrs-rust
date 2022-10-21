import { Context, Union } from "@apexlang/core/model";
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
export declare class UnionVisitor extends SourceGenerator<Union> {
    constructor(context: Context);
    buffer(): string;
}
//# sourceMappingURL=union-visitor.d.ts.map