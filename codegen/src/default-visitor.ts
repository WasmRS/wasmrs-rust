// import { AnyType, BaseVisitor, Context, Writer } from "@apexlang/core/model";
// import { TypeVisitor } from "./visitors/type-visitor.js";
// import { InterfaceVisitor } from "./visitors/interface-visitor.js";
// import { EnumVisitor } from "./visitors/enum-visitor.js";
// import { UnionVisitor } from "./visitors/union-visitor.js";
// import { AliasVisitor } from "./visitors/alias-visitor.js";
// // import { convertOperation } from "./utils/conversions.js";
// // import * as utils from "@apexlang/codegen/utils";
// import { utils } from "@apexlang/codegen/rust";
// import { RustBasic } from "@apexlang/codegen/rust";
// import { DefaultVisitor } from "./default-visitor copy.js";

// enum RequestType {
//   RequestResponse,
//   RequestChannel,
//   RequestStream,
//   FireAndForget,
// }

// interface Operation {
//   type: RequestType;
//   namespace: string;
//   name: string;
//   inputs: Record<string, AnyType>;
//   outputs: Record<string, AnyType>;
// }

export { RustBasic as DefaultVisitor } from "@apexlang/codegen/rust";
