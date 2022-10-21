import { Context, Enum } from "@apexlang/core/model";
import { SourceGenerator } from "./base.js";
/**
 * Apex enums come from syntax like this:
 *
 * ```apexlang
 * enum TrafficLight {
 *  red = 0 as "Red"
 *  yellow = 2 as "Yellow"
 *  green = 3 as "Green"
 * }
 * ```
 *
 * View a sample model here:
 * https://apexlang.github.io/ast-viewer/#CmVudW0gVHJhZmZpY0xpZ2h0IHsKCXJlZCA9IDAgYXMgIlJlZCIKICAgIHllbGxvdyA9IDIgYXMgIlllbGxvdyIKICAgIGdyZWVuID0gMyBhcyAiR3JlZW4iCn0K
 */
export declare class EnumVisitor extends SourceGenerator<Enum> {
    constructor(context: Context);
    buffer(): string;
    visitEnumValue(context: Context): void;
}
//# sourceMappingURL=enum-visitor.d.ts.map