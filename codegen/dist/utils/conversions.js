import { utils } from "@apexlang/codegen/rust";
import { convertType } from "./types";
/**
 * Convert a description to the appropriate format for the destination.
 *
 * @param description - A string description.
 * @returns A string suitable for the destination format or an empty string.
 */
export function convertDescription(description) {
    return utils.rustDoc(description);
}
/**
 * Generate new source for a Parameter
 *
 * @param param - A Parameter node to convert
 * @param config - The context's configuration.
 * @returns The new generated output for the Parameter
 *
 */
export function convertParameter(param, config) {
    // The name of the Parameter
    const name = param.name;
    // The type of the Parameter, converted via `convertType()`
    const type = convertType(param.type, config);
    // Combine the above to create and return new output here.
    return ``;
}
//# sourceMappingURL=conversions.js.map