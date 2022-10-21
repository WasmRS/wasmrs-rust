import { Kind, PrimitiveName, } from "@apexlang/core/model";
/**
 * Convert an Apex type to a type suitable for the destination format.
 *
 * @param typ - The Type node to convert.
 * @param config - The context's configuration.
 * @returns A string suitable for the destination format.
 *
 * @throws Throws if there is a type unaccounted for.
 */
export function convertType(typ, config) {
    switch (typ.kind) {
        case Kind.List: {
            /**
             * Generate the destination List/Array types here.
             * `itemType` is your element type
             *
             * For example:
             * `[string]` in Apex could be `string[]` in TypeScript.
             * where :
             *
             * @example
             * return `${itemType}[]`
             * */
            const t = typ;
            // The list element's type
            const itemType = convertType(t.type, config);
            return ``;
        }
        case Kind.Map: {
            /**
             * Generate destination Map/Object types here.
             *
             * For example:
             * `{string: u32}` in Apex could be `Record<string, number>` in TypeScript:
             *
             * @example
             * return `Record<${keyType}, ${valueType}>`;
             * */
            const t = typ;
            // The type of the keys in the Map.
            const keyType = convertType(t.keyType, config);
            // The type of the values in the Map.
            const valueType = convertType(t.valueType, config);
            return ``;
        }
        case Kind.Optional: {
            /**
             * Generate the output for Optional types here.
             *
             * For example:
             * `string?` in Apex could be `string | undefined` in TypeScript:
             *
             * @example
             * return `${innerType} | undefined`;
             * */
            const t = typ;
            // The inner type of the Optional node.
            const innerType = convertType(t.type, config);
            return ``;
        }
        case Kind.Union:
        case Kind.Enum:
        case Kind.Alias:
        case Kind.Type: {
            /**
             * Generate the output for named types.
             * Usually this is just the name itself, but you
             * may need to account for package or namespace paths.
             */
            const t = typ;
            // The name of the Type, Union, Alias, or Enum.
            const name = t.name;
            return name;
        }
        case Kind.Void: {
            /**
             * Generate output for `void`, non-existent, or undefined types.
             *
             * For example:
             * TypeScript may return `undefined` here, Rust would return `()`.
             */
            return "";
        }
        case Kind.Primitive: {
            /**
             * Primitive kinds are types inherent to Apex, i.e. string, u32, bool.
             *
             * They typically map to a primitive in the destination language or
             * are printed as-is for documentation.
             */
            const t = typ;
            return convertPrimitive(t, config);
        }
        default: {
            // Throw an error if we've come across a type not listed here.
            throw new Error(`Unhandled type conversion for type: ${typ.kind}`);
        }
    }
}
/**
 * Convert an Apex primitive type (i.e. `string`, `u32`) to a type suitable
 * for the destination.
 *
 * @param typ - The `Primitive` node to convert.
 * @param config - The context's configuration.
 * @returns A string suitable for the destination format.
 *
 * @throws Throws if there is a type unaccounted for.
 */
function convertPrimitive(typ, config) {
    switch (typ.name) {
        case PrimitiveName.Bool:
            return "bool";
        case PrimitiveName.Bytes:
            return "bytes";
        case PrimitiveName.DateTime:
            return "datetime";
        case PrimitiveName.F32:
            return "f32";
        case PrimitiveName.F64:
            return "f64";
        case PrimitiveName.U64:
            return "u64";
        case PrimitiveName.U32:
            return "u32";
        case PrimitiveName.U16:
            return "u16";
        case PrimitiveName.U8:
            return "u8";
        case PrimitiveName.I64:
            return "i64";
        case PrimitiveName.I32:
            return "i32";
        case PrimitiveName.I16:
            return "i16";
        case PrimitiveName.I8:
            return "i8";
        case PrimitiveName.String:
            return "string";
        case PrimitiveName.Any:
            // Any is a special type and could be considered `any` if the destination
            // language/format supports it or a JSON-like object value if not.
            return "any";
        default:
            throw new Error(`Unhandled primitive type conversion for type: ${typ.name}`);
    }
}
//# sourceMappingURL=types.js.map