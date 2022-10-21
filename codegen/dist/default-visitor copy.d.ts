import { Context } from "@apexlang/core/model";
import { RustBasic } from "@apexlang/codegen/rust";
export declare class DefaultVisitor extends RustBasic {
    namespace: string;
    visitContextBefore(context: Context): void;
    visitContextAfter(context: Context): void;
    visitNamespace(context: Context): void;
    visitFunction(context: Context): void;
}
//# sourceMappingURL=default-visitor%20copy.d.ts.map