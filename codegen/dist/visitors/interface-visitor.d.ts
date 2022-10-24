import { Context, Interface, ObjectMap, Operation } from "@apexlang/core/model";
import { SourceGenerator } from "./base.js";
export declare class InterfaceVisitor extends SourceGenerator<Interface> {
    constructor(context: Context);
    buffer(): string;
    visitOperation(context: Context): void;
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
export declare function convertOperation(op: Operation, global: boolean, config: ObjectMap): string;
//# sourceMappingURL=interface-visitor.d.ts.map