import {
  Alias,
  BaseVisitor,
  Context,
  Enum,
  Type,
  Union,
  Writer,
  Interface,
  ObjectMap,
} from "@apexlang/core/model";

export type VisitorTypes = Alias | Type | Union | Enum | Interface;

/**
 * A utility class to isolate a buffer and provide
 * easy access to the root node and configuration.
 *
 *
 * @param node - The root node to start from.
 * @param context - The visitor context to work in.
 */
export class SourceGenerator<T extends VisitorTypes> extends BaseVisitor {
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
  constructor(node: T, context: Context) {
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
  walk<U extends SourceGenerator<T>>(this: U): U {
    this.node.accept(this.context, this);
    return this;
  }

  /**
   * Get the buffer's contents.
   *
   * @returns The buffer's contents.
   */
  buffer(): string {
    return this.writer.string();
  }
}
