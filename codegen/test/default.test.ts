import { parse } from "./parse.js";
import { DefaultVisitor } from "../src/default-visitor.js";
import { Writer } from "@apexlang/core/model";

describe("default visitor", () => {
  test("should generate apex from apex", () => {
    // Define test apex we can parse.
    const apex = `
    namespace "test"

    interface TestInterface {
      op1(arg1: string, arg2: bool): CustomType
    }

    type CustomType {
      date: datetime
    }

    type OtherType {
      message: string
    }
    `;

    // Parse apex above and generate a Context.
    const context = parse(apex);

    // Instantiate DefaultVisitor with a new Writer (string buffer).
    const visitor = new DefaultVisitor(new Writer());

    // Pass our visitor to the context.
    context.accept(context, visitor);

    // Retrieve our generated source.
    const generated = visitor.writer.string();

    // Assert that our generated output equals what we expect.
    expect(generated).toEqual(`something!`);
  });
});
