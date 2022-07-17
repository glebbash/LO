import {
  CEnum,
  CFunction,
  CReturnType,
  CStruct,
  CSymbol,
  CType,
  CTypeDef,
} from "./types.ts";

export async function generateBindings(
  symbolsFile: string,
  exposedFunctions: string[],
  outputFolder: string,
  libName: string,
) {
  const allSymbols: CSymbol[] = JSON.parse(
    Deno.readTextFileSync(symbolsFile),
  );

  const llvmSymbols = allSymbols.filter((s: { name: string }) =>
    s.name.startsWith("LLVM")
  );

  const { typeDefs, typeDefsSource } = buildTypes(llvmSymbols);
  const enumsGen = buildEnums(llvmSymbols);
  const structsGen = buildStructs(llvmSymbols);
  const functionsGen = buildFunctions(llvmSymbols, exposedFunctions, typeDefs);
  const modGen = buildMod(libName);

  await Deno.mkdir(outputFolder, { recursive: true }).catch();
  await Deno.copyFile(`ffigen/safe-ffi.ts`, `${outputFolder}/safe-ffi.ts`);
  await Deno.writeTextFile(`${outputFolder}/types.ts`, typeDefsSource);
  await Deno.writeTextFile(`${outputFolder}/enums.ts`, enumsGen);
  await Deno.writeTextFile(`${outputFolder}/structs.ts`, structsGen);
  await Deno.writeTextFile(`${outputFolder}/functions.ts`, functionsGen);
  await Deno.writeTextFile(`${outputFolder}/mod.ts`, modGen);
}

function buildMod(libName: string): string {
  return `import { SafeDynamicLibrary } from "./safe-ffi.ts";
import * as functions from "./functions.ts";

export type ${libName} = ReturnType<typeof load${libName}>;

export function load${libName}(path: string) {
  const lib = Deno.dlopen(path, functions) as SafeDynamicLibrary<
    typeof functions
  >;

  return { ...lib.symbols, close: () => lib.close() };
}`;
}

function buildTypes(
  llvmSymbols: CSymbol[],
): { typeDefs: Set<string>; typeDefsSource: string } {
  const typeDefs = llvmSymbols.filter((s): s is CTypeDef =>
    s.tag === "typedef"
  );
  console.log("Total types:", typeDefs.length);

  const typeDefGen = typeDefs.map((t) => {
    const typeName = getTypeNameWithoutAliases(t.type);
    return `// ${cleanupLocation(t.location)}\n` +
      `export type ${t.name} = Opaque<${getJsType(typeName)}, "${t.name}">;\n` +
      `export const ${t.name}_: Opaque<${typeName}, "${t.name}"> = ${typeName} as never;`;
  }).join("\n\n");

  const imports = `// deno-lint-ignore-file\n\n` +
    `import { Opaque } from "./safe-ffi.ts";\n\n`;

  return {
    typeDefs: new Set(typeDefs.map((t) => t.name)),
    typeDefsSource: imports + typeDefGen + "\n",
  };
}

function buildEnums(llvmSymbols: CSymbol[]): string {
  const enums = llvmSymbols.filter((s): s is CEnum => s.tag === "enum");
  console.log("Total enums:", enums.length);

  const enumsGen = enums.map((e) => {
    const fieldsGen = e.fields.map((f) => `  ${f.name} = ${f.value}`)
      .join(",\n");

    return `// ${cleanupLocation(e.location)}\n` +
      `export enum ${e.name} {\n${fieldsGen}\n}`;
  }).join("\n\n") + "\n";

  return enumsGen;
}

function buildStructs(llvmSymbols: CSymbol[]) {
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

  return structsGen;
}

function buildFunctions(
  llvmSymbols: CSymbol[],
  exposedFunctions: string[],
  typeDefs: Set<string>,
): string {
  const allFunctions = llvmSymbols.filter((s): s is CFunction =>
    s.tag === "function"
  );
  const uniqueFunctions = uniqueByKey(allFunctions, "name");
  const nonInlinedFunctions = uniqueFunctions.filter((f) => !f.inline);
  const functions = nonInlinedFunctions.filter((f) =>
    exposedFunctions.includes(f.name as never)
  );
  console.log("Total functions:", functions.length);

  const generateFunctionDef = (f: CFunction) => {
    const parameterTypes = f.parameters.map((p) =>
      getTypeName(p.type, typeDefs)
    );
    const returnType = getReturnTypeName(f["return-type"], typeDefs);

    return "{\n  parameters: [" + parameterTypes.join(", ") + "],\n" +
      "  result: " + returnType + "\n} as const;";
  };

  const functionsGen = functions.map((f) => {
    const functionDef = generateFunctionDef(f);

    return "// " + cleanupLocation(f.location) + "\n" +
      "export const " + f.name + " = " + functionDef +
      "\n";
  }).join("\n");

  const imports = `// deno-lint-ignore-file\n\n` +
    `import * as types from "./types.ts";\n\n`;

  return imports + functionsGen + "\n";
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

function getTypeNameWithoutAliases(type: CType): string {
  const typeName = getBasicTypeName(type);

  if (typeName) {
    return wrapTypeName(typeName);
  }

  throw new Error("Unknown type: " + JSON.stringify(type));
}

function getTypeName(
  type: CType,
  typeDefs: Set<string>,
): string {
  const typeName = getBasicTypeName(type);

  if (typeName) {
    return wrapTypeName(typeName);
  }

  if (typeDefs.has(type.tag)) {
    return `types.${type.tag}_`;
  }

  throw new Error("Unknown type: " + JSON.stringify(type));
}

function getReturnTypeName(
  type: CReturnType,
  typeDefs: Set<string>,
): string {
  const typeName = getBasicTypeName(type as never);

  if (typeName) {
    return wrapTypeName(typeName);
  }

  if (type.tag === ":void") {
    return `"void"`;
  }

  if (typeDefs.has(type.tag)) {
    return `types.${type.tag}_`;
  }

  throw new Error("Unknown type: " + JSON.stringify(type));
}

function getBasicTypeName(type: CType): string | null {
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

  return null;
}

function wrapTypeName(typeName: string) {
  if (typeName.startsWith("types.")) {
    return typeName;
  }

  return `"${typeName}"`;
}

const NUMBER_TYPES = [
  `"u8"`,
  `"i8"`,
  `"u16"`,
  `"i16"`,
  `"u32"`,
  `"i32"`,
  `"f32"`,
  `"f64"`,
];

function getJsType(nativeType: string) {
  if (NUMBER_TYPES.includes(nativeType)) {
    return "number";
  }

  return "bigint";
}
