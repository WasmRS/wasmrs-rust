import { convertType } from "./types";
/**
 * Convert a description to the appropriate format for the destination.
 *
 * @param description - A string description.
 * @returns A string suitable for the destination format or an empty string.
 */
export function convertDescription(description) {
    if (description) {
        // Return what descriptions should look like in your destination format
        // Oftentimes descriptions map to comments in code, e.g.
        //
        // return `// ${description}`;
        return "";
    }
    else {
        // Else return nothing
        return "";
    }
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
export function convertOperation(op, global, config) {
    // The name of the Operation.
    const name = op.name;
    // A comment generated from the description.
    const comment = convertDescription(op.description);
    // The return type of the operation, converted via `convertType()`
    const type = convertType(op.type, config);
    // Iterate over the Operation's Parameters and generate new output.
    const params = op.parameters.map((arg) => convertParameter(arg, config));
    if (global) {
        // Generate output for global functions here.
    }
    else {
        // Generate method output here.
    }
    // Combine the above to create and return new output.
    return ``;
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