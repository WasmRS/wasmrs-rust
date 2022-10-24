import { Context } from "@apexlang/core/model";
import { RustBasic } from "@apexlang/codegen/rust";
export declare class DefaultVisitor extends RustBasic {
    namespace: string;
    exports: [string, string][];
    imports: [string, string][];
    visitNamespace(context: Context): void;
    visitContextBefore(context: Context): void;
    visitContextAfter(context: Context): void;
    visitInterface(context: Context): void;
}
//# sourceMappingURL=default-visitor.d.ts.map