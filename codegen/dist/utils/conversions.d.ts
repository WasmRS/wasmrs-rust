import { ObjectMap, Parameter } from "@apexlang/core/model";
/**
 * Convert a description to the appropriate format for the destination.
 *
 * @param description - A string description.
 * @returns A string suitable for the destination format or an empty string.
 */
export declare function convertDescription(description?: string): string;
/**
 * Generate new source for a Parameter
 *
 * @param param - A Parameter node to convert
 * @param config - The context's configuration.
 * @returns The new generated output for the Parameter
 *
 */
export declare function convertParameter(param: Parameter, config: ObjectMap): string;
//# sourceMappingURL=conversions.d.ts.map