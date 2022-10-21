import { Context, Interface } from "@apexlang/core/model";
import { SourceGenerator } from "./base.js";
/**
 * Apex interfaces come from syntax like this:
 *
 * ```apexlang
 * interface RetailStore {
 *   order(item:u32): u32
 * }
 * ```
 *
 * View a sample model here:
 * https://apexlang.github.io/ast-viewer/#aW50ZXJmYWNlIFJldGFpbFN0b3JlIHsKICBvcmRlcihpdGVtOnUzMik6IHUzMgp9Cgo=
 */
export declare class InterfaceVisitor extends SourceGenerator<Interface> {
    constructor(context: Context);
    buffer(): string;
    visitOperation(context: Context): void;
}
//# sourceMappingURL=interface-visitor.d.ts.map