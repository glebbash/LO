type CSymbol = CTypeDef | CEnum | CStruct | CFunction;

type CTypeDef = {
  tag: "typedef";
  name: string;
  location: string;
  type: CTypeDefType;
};

type CEnum = {
  tag: "enum";
  name: string;
  location: string;
  id: number;
  fields: { tag: "field"; name: string; value: number }[];
};

type CStruct = {
  tag: "struct";
  name: string;
  location: string;
  id: number;
  "bit-size": number;
  "bit-alignment": number;
  fields: {
    tag: "field";
    name: string;
    "bit-size": number;
    "bit-alignment": number;
    type:
      | { tag: string }
      | { tag: string; "bit-size": number; "bit-alignment": number };
  }[];
};

type CFunction = {
  tag: "function";
  name: string;
  location: string;
};

type CTypeDefType =
  | { tag: ":int"; "bit-size": number; "bit-alignment": number }
  | { tag: ":unsigned-int"; "bit-size": number; "bit-alignment": number }
  | { tag: ":function-pointer" }
  | { tag: ":pointer" }
  | { tag: "struct" }
  | { tag: ":enum" }
  | { tag: "uint64_t" }
  | { tag: "uint8_t" };

const llvmC: CSymbol[] = JSON.parse(Deno.readTextFileSync("./llvm-c.json"));

const llvmSymbols = llvmC.filter((s: { name: string }) =>
  s.name.startsWith("LLVM")
);
console.log("Total symbols:", llvmSymbols.length);

{
  const typeDefs = llvmSymbols.filter((s): s is CTypeDef =>
    s.tag === "typedef"
  );
  console.log("Total type definitions:", typeDefs.length);

  const typeDefGen = typeDefs.map((t) => {
    return "// " + cleanupLocation(t.location) + "\n" +
      "export const " + t.name + " = '" + getTypeDefTypeName(t.type) + "';" +
      "\n";
  }).join("\n");

  Deno.writeTextFileSync("ffigen-output/types.ts", typeDefGen);
}

{
  const enums = llvmSymbols.filter((s): s is CEnum => s.tag === "enum");
  console.log("Total enums:", enums.length);

  const enumsGen = enums.map((e) => {
    const fieldsGen = e.fields.map((f) => {
      return "  " + f.name + " = " + f.value;
    }).join(",\n");

    return "// " + cleanupLocation(e.location) + "\n" +
      "export enum " + e.name + " {\n" + fieldsGen + "\n}" +
      "\n";
  }).join("\n");

  Deno.writeTextFileSync("ffigen-output/enums.ts", enumsGen);
}

{
  const structs = llvmSymbols.filter((s): s is CStruct => s.tag === "struct");
  console.log("Total structs:", structs.length);

  // TODO:
  const generateStructDef = (
    struct: CStruct,
  ) => {
    if (struct.fields.length === 0) {
      return '{ type: "opaque" }';
    }

    const structDef = {
      "bit-size": struct["bit-size"],
      "bit-alignment": struct["bit-alignment"],
      fields: Object.fromEntries(
        struct.fields.map(({ tag: _, name, ...data }) => [name, data]),
      ),
    };

    return JSON.stringify(structDef, null, 2) + " as const";
  };

  const structsGen = structs.map((s) => {
    const fieldsGen = generateStructDef(s);

    return "// " + cleanupLocation(s.location) + "\n" +
      "export const " + s.name + " = " + fieldsGen + ";" +
      "\n";
  }).join("\n");

  Deno.writeTextFileSync("ffigen-output/structs.ts", structsGen);
}

{
  const functions = llvmSymbols.filter((s): s is CFunction =>
    s.tag === "function"
  );
  const uniqueFunctions = uniqueByKey(functions, "name");
  console.log("Total functions:", uniqueFunctions.length);

  // TODO: implement functions
  const functionsGen = uniqueFunctions.map((f) => {
    return "// " + cleanupLocation(f.location) + "\n" +
      "export const " + f.name + " = " + JSON.stringify(f, null, 2) + ";" +
      "\n";
  }).join("\n");

  Deno.writeTextFileSync("ffigen-output/functions.ts", functionsGen);
}

function uniqueByKey<T>(values: T[], key: keyof T): T[] {
  const seen = new Set<T[keyof T]>();
  const result: T[] = [];

  for (const value of values) {
    const keyValue = value[key];
    if (!seen.has(keyValue)) {
      seen.add(keyValue);
      result.push(value);
    }
  }

  return result;
}

function cleanupLocation(location: string): string {
  if (location.startsWith("/usr/include/llvm-c")) {
    return "./" + location.substring("/usr/include/".length);
  }

  if (location.startsWith("/data/./llvm-c")) {
    return location.substring("/data/".length);
  }

  return location;
}

function getTypeDefTypeName(type: CTypeDefType): string {
  if (type.tag === ":pointer") {
    return "pointer";
  }

  if (type.tag === ":function-pointer") {
    return "function";
  }

  if (type.tag === ":int" && type["bit-size"] === 32) {
    return "i32";
  }

  if (type.tag === ":unsigned-int" && type["bit-size"] === 32) {
    return "u32";
  }

  if (type.tag === "struct") {
    return "pointer";
  }

  // TODO: link typedefs and enums somehow
  if (type.tag === ":enum") {
    return "i32";
  }

  if (type.tag === "uint64_t") {
    // TODO: this is not the best idea
    return "pointer";
  }

  if (type.tag === "uint8_t") {
    return "u8";
  }

  throw new Error("Unknown type: " + JSON.stringify(type));
}
