import { Context, Type } from "@apexlang/core/model";
import { SourceGenerator } from "./base.js";
/**
 * Apex type definitions come from syntax like this:
 *
 * ```apexlang
 * type Person {
 *   name: string
 *   age: u8
 * }
 * ```
 *
 * View a sample model here:
 * https://apexlang.github.io/ast-viewer/#dHlwZSBQZXJzb24gewogIG5hbWU6IHN0cmluZwogIGFnZTogdTgKfQ==
 */
export declare class TypeVisitor extends SourceGenerator<Type> {
    constructor(context: Context);
    buffer(): string;
    visitTypeField(context: Context): void;
}
//# sourceMappingURL=type-visitor.d.ts.map