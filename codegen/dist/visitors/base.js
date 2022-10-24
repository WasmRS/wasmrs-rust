import { BaseVisitor, Writer, } from "@apexlang/core/model";
/**
 * A utility class to isolate a buffer and provide
 * easy access to the root node and configuration.
 *
 *
 * @param node - The root node to start from.
 * @param context - The visitor context to work in.
 */
export class SourceGenerator extends BaseVisitor {
    /**
     * Creates a new visitor with an isolated Writer and
     * a reference to the root node and context configuration.
     *
     * @param node - The root node to start from.
     * @param context - The visitor context to work in.
     */
    constructor(node, context) {
        super(new Writer());
        this.node = node;
        this.context = context;
        this.config = context.config;
    }
    /**
     * Walk the node, calling visitor functions as it traverses the tree.
     *
     * @returns Itself.
     */
    walk() {
        this.node.accept(this.context, this);
        return this;
    }
    /**
     * Get the buffer's contents.
     *
     * @returns The buffer's contents.
     */
    buffer() {
        return this.writer.string();
    }
}
//# sourceMappingURL=base.js.map