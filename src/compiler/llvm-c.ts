const StringPtrType = "pointer";
const NullPtrType = "pointer";
const LLVMValueArrayType = "pointer";
const LLVMBlockArrayType = "pointer";
const LLVMTypeArrayType = "pointer";
const StringType = "pointer";
const BoolType = "i8";
// TODO: test this
const NULL = 0;

export type LibLLVM = ReturnType<typeof loadLibLLVMInternal>;
export const loadLibLLVM: (libFile?: string) => LibLLVM = loadLibLLVMInternal;

export enum LLVMIntPredicate {
  LLVMIntEQ = 32, /**< equal */
  LLVMIntNE, /**< not equal */
  LLVMIntUGT, /**< unsigned greater than */
  LLVMIntUGE, /**< unsigned greater or equal */
  LLVMIntULT, /**< unsigned less than */
  LLVMIntULE, /**< unsigned less or equal */
  LLVMIntSGT, /**< signed greater than */
  LLVMIntSGE, /**< signed greater or equal */
  LLVMIntSLT, /**< signed less than */
  LLVMIntSLE, /**< signed less or equal */
}

function loadLibLLVMInternal(libFile = "/usr/lib/llvm-13/lib/libLLVM.so") {
  const fn = <T extends Function>(options: ExternalFunctionConfig<T>) =>
    options;

  const lib = {
    contextCreate: fn({
      name: "LLVMContextCreate",
      type: [LLVMContext.TYPE, []],
      wrap: (call) => () => new LLVMContext(call()),
    }),
    contextDispose: fn({
      name: "LLVMContextDispose",
      type: ["void", [LLVMContext.TYPE]],
      wrap: (call) => (ctx: LLVMContext) => call(ctx.value),
    }),

    moduleCreateWithNameInContext: fn({
      name: "LLVMModuleCreateWithNameInContext",
      type: [LLVMModule.TYPE, [StringType, LLVMContext.TYPE]],
      wrap: (call) =>
        (moduleName: string, ctx: LLVMContext) =>
          new LLVMModule(call(buildStringPtr(moduleName), ctx.value)),
    }),
    disposeModule: fn({
      name: "LLVMDisposeModule",
      type: ["void", [LLVMModule.TYPE]],
      wrap: (call) => (module: LLVMModule) => call(module.value),
    }),

    createBuilderInContext: fn({
      name: "LLVMCreateBuilderInContext",
      type: [LLVMIRBuilder.TYPE, [LLVMContext.TYPE]],

      wrap: (call) => (ctx: LLVMContext) => new LLVMIRBuilder(call(ctx.value)),
    }),
    disposeBuilder: fn({
      name: "LLVMDisposeBuilder",
      type: ["void", [LLVMIRBuilder.TYPE]],
      wrap: (call) => (builder: LLVMIRBuilder) => call(builder.value),
    }),

    setTarget: fn({
      name: "LLVMSetTarget",
      type: ["void", [LLVMModule.TYPE, StringType]],
      wrap: (call) =>
        (module: LLVMModule, targetTriple: string) =>
          call(module.value, buildStringPtr(targetTriple)),
    }),

    pointerType: fn({
      name: "LLVMPointerType",
      type: [LLVMType.TYPE, [LLVMType.TYPE, "i32"]],
      wrap: (call) => (type: LLVMType) => new LLVMType(call(type.value, 0)),
    }),
    voidTypeInContext: fn({
      name: "LLVMVoidTypeInContext",
      type: [LLVMType.TYPE, [LLVMContext.TYPE]],
      wrap: (call) => (ctx: LLVMContext) => new LLVMType(call(ctx.value)),
    }),
    i1TypeInContext: fn({
      name: "LLVMInt1TypeInContext",
      type: [LLVMType.TYPE, [LLVMContext.TYPE]],
      wrap: (call) => (ctx: LLVMContext) => new LLVMType(call(ctx.value)),
    }),
    i8TypeInContext: fn({
      name: "LLVMInt8TypeInContext",
      type: [LLVMType.TYPE, [LLVMContext.TYPE]],
      wrap: (call) => (ctx: LLVMContext) => new LLVMType(call(ctx.value)),
    }),
    i32TypeInContext: fn({
      name: "LLVMInt32TypeInContext",
      type: [LLVMType.TYPE, [LLVMContext.TYPE]],
      wrap: (call) => (ctx: LLVMContext) => new LLVMType(call(ctx.value)),
    }),
    i64TypeInContext: fn({
      name: "LLVMInt64TypeInContext",
      type: [LLVMType.TYPE, [LLVMContext.TYPE]],
      wrap: (call) => (ctx: LLVMContext) => new LLVMType(call(ctx.value)),
    }),
    functionType: fn({
      name: "LLVMFunctionType",
      type: [LLVMType.TYPE, [
        LLVMType.TYPE,
        LLVMTypeArrayType,
        "i32",
        BoolType,
      ]],
      wrap: (call) =>
        (returnType: LLVMType, argTypes: LLVMType[], isVarArg = false) =>
          new LLVMType(
            call(
              returnType.value,
              buildPointerArray(argTypes),
              argTypes.length,
              isVarArg,
            ),
          ),
    }),
    arrayType: fn({
      name: "LLVMArrayType",
      type: [LLVMType.TYPE, [LLVMType.TYPE, "i32"]],
      wrap: (call) =>
        (type: LLVMType, length: number) =>
          new LLVMType(call(type.value, length)),
    }),
    typeOf: fn({
      name: "LLVMTypeOf",
      type: [LLVMType.TYPE, [LLVMValue.TYPE]],
      wrap: (call) => (value: LLVMValue) => new LLVMType(call(value.value)),
    }),

    getUndef: fn({
      name: "LLVMGetUndef",
      type: [LLVMValue.TYPE, [LLVMType.TYPE]],
      wrap: (call) => (type: LLVMType) => new LLVMValue(call(type.value)),
    }),
    getParam: fn({
      name: "LLVMGetParam",
      type: [LLVMValue.TYPE, [LLVMValue.TYPE, "i32"]],
      wrap: (call) =>
        (fn: LLVMValue, index: number) => new LLVMValue(call(fn.value, index)),
    }),

    constInt: fn({
      name: "LLVMConstInt",
      type: [LLVMValue.TYPE, [LLVMType.TYPE, "i32", BoolType]],
      wrap: (call) =>
        (type: LLVMType, value: number, signExtend = false) =>
          new LLVMValue(call(type.value, value, signExtend)),
    }),
    constPointerNull: fn({
      name: "LLVMConstPointerNull",
      type: [LLVMValue.TYPE, [LLVMType.TYPE]],
      wrap: (call) => (type: LLVMType) => new LLVMValue(call(type.value)),
    }),

    addFunction: fn({
      name: "LLVMAddFunction",
      type: [LLVMValue.TYPE, [LLVMModule.TYPE, StringType, LLVMType.TYPE]],
      wrap: (call) =>
        (module: LLVMModule, fnName: string, type: LLVMType) =>
          new LLVMValue(call(module.value, buildStringPtr(fnName), type.value)),
    }),
    getNamedFunction: fn({
      name: "LLVMGetNamedFunction",
      type: [LLVMValue.TYPE, [LLVMModule.TYPE, StringType]],
      wrap: (call) =>
        (module: LLVMModule, fnName: string) =>
          new LLVMValue(call(module.value, buildStringPtr(fnName))),
    }),

    createBasicBlockInContext: fn({
      name: "LLVMCreateBasicBlockInContext",
      type: [LLVMBasicBlock.TYPE, [LLVMContext.TYPE, StringType]],
      wrap: (call) =>
        (ctx: LLVMContext, name = "") =>
          new LLVMBasicBlock(call(ctx.value, buildStringPtr(name))),
    }),
    appendBasicBlockInContext: fn({
      name: "LLVMAppendBasicBlockInContext",
      type: [LLVMBasicBlock.TYPE, [
        LLVMContext.TYPE,
        LLVMValue.TYPE,
        StringType,
      ]],
      wrap: (call) =>
        (ctx: LLVMContext, fn: LLVMValue, name = "") =>
          new LLVMBasicBlock(call(ctx.value, fn.value, buildStringPtr(name))),
    }),
    insertExistingBasicBlockAfterInsertBlock: fn({
      name: "LLVMInsertExistingBasicBlockAfterInsertBlock",
      type: [LLVMValue.TYPE, [LLVMIRBuilder.TYPE, LLVMBasicBlock.TYPE]],
      wrap: (call) =>
        (builder: LLVMIRBuilder, block: LLVMBasicBlock) =>
          new LLVMValue(call(builder.value, block.value)),
    }),
    getInsertBlock: fn({
      name: "LLVMGetInsertBlock",
      type: [LLVMBasicBlock.TYPE, [LLVMIRBuilder.TYPE]],
      wrap: (call) =>
        (builder: LLVMIRBuilder) => new LLVMBasicBlock(call(builder.value)),
    }),
    getBasicBlockParent: fn({
      name: "LLVMGetBasicBlockParent",
      type: [LLVMValue.TYPE, [LLVMBasicBlock.TYPE]],
      wrap: (call) =>
        (block: LLVMBasicBlock) => new LLVMValue(call(block.value)),
    }),

    positionBuilderAtEnd: fn({
      name: "LLVMPositionBuilderAtEnd",
      type: ["void", [LLVMIRBuilder.TYPE, LLVMBasicBlock.TYPE]],
      wrap: (call) =>
        (builder: LLVMIRBuilder, block: LLVMBasicBlock) =>
          call(builder.value, block.value),
    }),

    buildGlobalStringPtr: fn({
      name: "LLVMBuildGlobalStringPtr",
      type: [LLVMValue.TYPE, [LLVMIRBuilder.TYPE, StringType, StringType]],
      wrap: (call) =>
        (builder: LLVMIRBuilder, content: string, name = "str") =>
          new LLVMValue(
            call(builder.value, buildStringPtr(content), buildStringPtr(name)),
          ),
    }),
    buildRet: fn({
      name: "LLVMBuildRet",
      type: [LLVMValue.TYPE, [LLVMIRBuilder.TYPE, LLVMValue.TYPE]],
      wrap: (call) =>
        (builder: LLVMIRBuilder, value: LLVMValue) =>
          new LLVMValue(call(builder.value, value.value)),
    }),
    buildCall: fn({
      name: "LLVMBuildCall",
      type: [
        LLVMValue.TYPE,
        [
          LLVMIRBuilder.TYPE,
          LLVMValue.TYPE,
          LLVMValueArrayType,
          "i32",
          StringType,
        ],
      ],
      wrap: (call) =>
        (builder: LLVMIRBuilder, fn: LLVMValue, args: LLVMValue[], name = "") =>
          new LLVMValue(
            call(
              builder.value,
              fn.value,
              buildPointerArray(args),
              args.length,
              buildStringPtr(name),
            ),
          ),
    }),
    buildAlloca: fn({
      name: "LLVMBuildAlloca",
      type: [LLVMValue.TYPE, [LLVMIRBuilder.TYPE, LLVMType.TYPE, StringType]],
      wrap: (call) =>
        (builder: LLVMIRBuilder, type: LLVMType, name = "") =>
          new LLVMValue(call(builder.value, type.value, buildStringPtr(name))),
    }),
    buildLoad: fn({
      name: "LLVMBuildLoad",
      type: [LLVMValue.TYPE, [LLVMIRBuilder.TYPE, LLVMValue.TYPE, StringType]],
      wrap: (call) =>
        (builder: LLVMIRBuilder, pointer: LLVMValue, name = "") =>
          new LLVMValue(
            call(builder.value, pointer.value, buildStringPtr(name)),
          ),
    }),
    buildStore: fn({
      name: "LLVMBuildStore",
      type: [
        LLVMValue.TYPE,
        [LLVMIRBuilder.TYPE, LLVMValue.TYPE, LLVMValue.TYPE],
      ],
      wrap: (call) =>
        (builder: LLVMIRBuilder, value: LLVMValue, pointer: LLVMValue) =>
          new LLVMValue(call(builder.value, value.value, pointer.value)),
    }),
    buildGEP: fn({
      name: "LLVMBuildGEP",
      type: [
        LLVMValue.TYPE,
        [
          LLVMIRBuilder.TYPE,
          LLVMValue.TYPE,
          LLVMValueArrayType,
          "i32",
          StringType,
        ],
      ],
      wrap: (call) =>
        (
          builder: LLVMIRBuilder,
          pointer: LLVMValue,
          indices: LLVMValue[],
          name = "",
        ) => {
          return new LLVMValue(
            call(
              builder.value,
              pointer.value,
              buildPointerArray(indices),
              indices.length,
              buildStringPtr(name),
            ),
          );
        },
    }),
    buildAdd: fn({
      name: "LLVMBuildAdd",
      type: [
        LLVMValue.TYPE,
        [LLVMIRBuilder.TYPE, LLVMValue.TYPE, LLVMValue.TYPE, StringType],
      ],
      wrap: (call) =>
        (builder: LLVMIRBuilder, lhs: LLVMValue, rhs: LLVMValue, name = "") =>
          new LLVMValue(
            call(builder.value, lhs.value, rhs.value, buildStringPtr(name)),
          ),
    }),
    buildICmp: fn({
      name: "LLVMBuildICmp",
      type: [
        LLVMValue.TYPE,
        [LLVMIRBuilder.TYPE, "i32", LLVMValue.TYPE, LLVMValue.TYPE, StringType],
      ],
      wrap: (call) =>
        (
          builder: LLVMIRBuilder,
          predicate: LLVMIntPredicate,
          lhs: LLVMValue,
          rhs: LLVMValue,
          name = "",
        ) =>
          new LLVMValue(
            call(
              builder.value,
              predicate,
              lhs.value,
              rhs.value,
              buildStringPtr(name),
            ),
          ),
    }),
    buildCondBr: fn({
      name: "LLVMBuildCondBr",
      type: [
        LLVMValue.TYPE,
        [
          LLVMIRBuilder.TYPE,
          LLVMValue.TYPE,
          LLVMBasicBlock.TYPE,
          LLVMBasicBlock.TYPE,
        ],
      ],
      wrap: (call) =>
        (
          builder: LLVMIRBuilder,
          cond: LLVMValue,
          ifTrue: LLVMBasicBlock,
          ifFalse: LLVMBasicBlock,
        ) =>
          new LLVMValue(
            call(builder.value, cond.value, ifTrue.value, ifFalse.value),
          ),
    }),
    buildBr: fn({
      name: "LLVMBuildBr",
      type: [LLVMValue.TYPE, [LLVMIRBuilder.TYPE, LLVMBasicBlock.TYPE]],
      wrap: (call) =>
        (builder: LLVMIRBuilder, dest: LLVMBasicBlock) =>
          new LLVMValue(call(builder.value, dest.value)),
    }),
    buildPhi: fn({
      name: "LLVMBuildPhi",
      type: [LLVMValue.TYPE, [LLVMIRBuilder.TYPE, LLVMType.TYPE, StringType]],
      wrap: (call) =>
        (builder: LLVMIRBuilder, type: LLVMType, name = "") =>
          new LLVMValue(call(builder.value, type.value, buildStringPtr(name))),
    }),

    addIncoming: fn({
      name: "LLVMAddIncoming",
      type: ["void", [
        LLVMValue.TYPE,
        LLVMValueArrayType,
        LLVMBlockArrayType,
        "i32",
      ]],
      wrap: (call) =>
        (phi: LLVMValue, values: LLVMValue[], blocks: LLVMBasicBlock[]) =>
          call(
            phi.value,
            buildPointerArray(values),
            buildPointerArray(blocks),
            values.length,
          ),
    }),

    verifyFunction: fn({
      name: "LLVMVerifyFunction",
      type: [BoolType, [LLVMValue.TYPE, "i32"]],
      wrap: (call) => (fn: LLVMValue) => ({ ok: !call(fn.value, 2) }),
    }),
    // TODO: test this
    verifyModule: fn({
      name: "LLVMVerifyModule",
      type: [BoolType, [LLVMModule.TYPE, "i32", StringPtrType]],
      wrap: (call) =>
        (module: LLVMModule) => {
          const messageRef = new Int8Array(2048);
          const err = call(module.value, 2, messageRef);
          const message = messageRef.toString();

          return { ok: !err, message };
        },
    }),

    printModuleToFile: fn({
      name: "LLVMPrintModuleToFile",
      type: ["void", [LLVMModule.TYPE, StringType, NullPtrType]],
      wrap: (call) =>
        (module: LLVMModule, fileName: string) =>
          call(module.value, buildStringPtr(fileName), NULL),
    }),
  };

  return wrap(lib, libFile);
}

function wrap<T extends Record<string, ExternalFunctionConfig<Function>>>(
  lib: T,
  libPath: string,
): WrappedLib<T> {
  const foreignFunctions = Object.fromEntries(
    Object.values(lib).map((
      { name, type: [result, parameters] },
    ): [string, Deno.ForeignFunction] => [name, {
      parameters,
      result,
    }]),
  );

  const dynLib = Deno.dlopen(libPath, foreignFunctions);

  const wrappedFunctions = Object.fromEntries(
    Object.entries(lib).map((
      [fnName, { name }],
    ) => [fnName, dynLib.symbols[name]]),
  );

  return {
    ...wrappedFunctions,
    close: () => dynLib.close(),
  } as unknown as WrappedLib<T>;
}

type WrappedLib<T extends Record<string, ExternalFunctionConfig<Function>>> =
  & {
    [P in keyof T]: GetWrappedFunction<T[P]>;
  }
  & { close: () => void };

type GetWrappedFunction<T> = T extends ExternalFunctionConfig<infer T> ? T
  : never;

type ExternalFunctionConfig<T extends Function> = {
  name: string;
  type: [Deno.NativeType, Deno.NativeType[]];
  wrap: (call: (...args: unknown[]) => any) => T;
};

class Pointer {
  constructor(public readonly value: Deno.UnsafePointer) {}
}

export class LLVMContext extends Pointer {
  static TYPE = "pointer" as const;

  private __name = this;
}

export class LLVMIRBuilder extends Pointer {
  static TYPE = "pointer" as const;

  private __name = this;
}

export class LLVMModule extends Pointer {
  static TYPE = "pointer" as const;

  private __name = this;
}

export class LLVMValue extends Pointer {
  static TYPE = "pointer" as const;

  private __name = this;

  // TODO: test this
  isNull() {
    return this.value.value === 0n;
  }
}

export class LLVMType extends Pointer {
  static TYPE = "pointer" as const;

  private __name = this;
}

export class LLVMBasicBlock extends Pointer {
  static TYPE = "pointer" as const;

  private __name = this;
}

// TODO: check if this works
function buildPointerArray(pointers: Pointer[]): Int32Array {
  return Int32Array.from(pointers.map((p) => Number(p.value.value)));
}

function buildStringPtr(str: string): unknown {
  return str;
}
