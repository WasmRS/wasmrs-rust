export * as types from "./types.js";
export * as convert from "./conversions.js";
import { noCase } from "@apexlang/codegen/utils";

/**
 * A utility function to checks if a name is a reserved word.
 *
 * @param name - The name to check.
 * @returns true or false depending on if the name is found in the reservedWords list.
 */
export function isReservedWord(name: string): boolean {
  return reservedWords.includes(name);
}

/**
 * A list of reserved words that should not be used as identifier names
 *
 * @remarks
 * Modify this list with reserved words for your destination format, or empty
 * it for looser formats.
 */
const reservedWords = [
  "new", // examples
  "function",
  "class",
];

export function constantCase(str: string): string {
  return noCase(str, {
    delimiter: "_",
    transform: (str) => str.toUpperCase(),
  });
}
