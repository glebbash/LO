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
  inline: boolean;
  parameters: { name: string; type: CTypeDefType }[];
  "return-type": CReturnType;
};

type CReturnType = CTypeDefType | { tag: ":void" };

type CTypeDefType =
  | { tag: ":int"; "bit-size": number; "bit-alignment": number }
  | { tag: ":unsigned-int"; "bit-size": number; "bit-alignment": number }
  | { tag: ":unsigned-long-long"; "bit-size": number; "bit-alignment": number }
  | { tag: ":long-long"; "bit-size": number; "bit-alignment": number }
  | { tag: ":double"; "bit-size": number; "bit-alignment": number }
  | { tag: ":char"; "bit-size": number; "bit-alignment": number }
  | { tag: ":function-pointer" }
  | { tag: ":pointer" }
  | { tag: "struct" }
  | { tag: ":enum" }
  | { tag: "size_t" }
  | { tag: "int64_t" }
  | { tag: "uint64_t" }
  | { tag: "uint32_t" }
  | { tag: "uint8_t" };

const llvmC: CSymbol[] = JSON.parse(Deno.readTextFileSync("./llvm-c.json"));

const llvmSymbols = llvmC.filter((s: { name: string }) =>
  s.name.startsWith("LLVM")
);
console.log("Total symbols:", llvmSymbols.length);

const typeDefMap = new Map(
  llvmSymbols.filter((t): t is CTypeDef => t.tag === "typedef")
    .map((t) => [t.name, getTypeDefTypeName(t.type)]),
);

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

  // TODO: figure out better struct representation
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
  const allFunctions = llvmSymbols.filter((s): s is CFunction =>
    s.tag === "function"
  );
  const functions = uniqueByKey(allFunctions, "name");
  console.log("Total functions:", functions.length);

  const generateFunctionDef = (f: CFunction) => {
    if (f.inline) {
      return `{\n  type: "inline"\n};`;
    }

    const parameterTypes = f.parameters.map((p) => {
      return getTypeDefTypeName(p.type);
    });
    const returnType = getReturnTypeName(f["return-type"]);

    // return JSON.stringify({ parameters: parameterTypes, returnType }, null, 2);
    return "{\n  parameters: [" +
      parameterTypes.map((p) => `"${p}"`).join(", ") + "],\n" +
      '  returnType: "' + returnType + '"\n} as const;';
  };

  // TODO: implement functions
  const functionsGen = functions.map((f) => {
    const functionDef = generateFunctionDef(f);

    return "// " + cleanupLocation(f.location) + "\n" +
      "export const " + f.name + " = " + functionDef +
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

function getReturnTypeName(type: CReturnType): string {
  const typeName = getBasicTypeName(type as never);

  if (typeName) {
    return typeName;
  }

  if (type.tag === ":void") {
    return "void";
  }

  throw new Error("Unknown type: " + JSON.stringify(type));
}

function getTypeDefTypeName(type: CTypeDefType): string {
  const typeName = getBasicTypeName(type);

  if (!typeName) {
    throw new Error("Unknown type: " + JSON.stringify(type));
  }

  return typeName;
}

function getBasicTypeName(type: CTypeDefType): string | null {
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

  if (type.tag === ":unsigned-long-long" && type["bit-size"] === 64) {
    return "u64";
  }

  if (type.tag === ":long-long" && type["bit-size"] === 64) {
    return "i64";
  }

  if (type.tag === ":double" && type["bit-size"] === 64) {
    return "i64";
  }

  if (type.tag === ":char" && type["bit-size"] === 8) {
    return "u8";
  }

  // TODO: unhardcode this
  if (type.tag === "size_t") {
    return "u64";
  }

  if (type.tag === "struct") {
    return "pointer";
  }

  // TODO: link typedefs and enums somehow
  if (type.tag === ":enum") {
    return "i32";
  }

  if (type.tag === "int64_t") {
    return "i64";
  }

  if (type.tag === "uint64_t") {
    return "u64";
  }

  if (type.tag === "uint32_t") {
    return "u32";
  }

  if (type.tag === "uint8_t") {
    return "u8";
  }

  if (typeDefMap.has(type.tag)) {
    return typeDefMap.get(type.tag)!;
  }

  return null;
}
