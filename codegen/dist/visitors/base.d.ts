import { Alias, BaseVisitor, Context, Enum, Type, Union, Interface, ObjectMap } from "@apexlang/core/model";
export declare type VisitorTypes = Alias | Type | Union | Enum | Interface;
/**
 * A utility class to isolate a buffer and provide
 * easy access to the root node and configuration.
 *
 *
 * @param node - The root node to start from.
 * @param context - The visitor context to work in.
 */
export declare class SourceGenerator<T extends VisitorTypes> extends BaseVisitor {
    node: T;
    context: Context;
    config: ObjectMap;
    /**
     * Creates a new visitor with an isolated Writer and
     * a reference to the root node and context configuration.
     *
     * @param node - The root node to start from.
     * @param context - The visitor context to work in.
     */
    constructor(node: T, context: Context);
    /**
     * Get the buffer's contents.
     *
     * @returns The buffer's contents.
     */
    buffer(): string;
}
//# sourceMappingURL=base.d.ts.map