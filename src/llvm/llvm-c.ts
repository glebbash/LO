import { ExternalFunctionConfig, wrap } from "./ffi-lib.ts";

const StringPtrType = "pointer";
const LLVMValueArrayType = "pointer";
const LLVMBlockArrayType = "pointer";
const LLVMTypeArrayType = "pointer";
const StringType = "pointer";
const BoolType = "i8";
const FunctionType = "pointer";

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

function loadLibLLVMInternal(libFile = "/usr/lib/llvm-14/lib/libLLVM.so") {
  // deno-lint-ignore ban-types
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
              buildBool(isVarArg),
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
    structTypeInContext: fn({
      name: "LLVMStructTypeInContext",
      type: [LLVMType.TYPE, [
        LLVMContext.TYPE,
        LLVMTypeArrayType,
        "i32",
        BoolType,
      ]],
      wrap: (call) =>
        (ctx: LLVMContext, elementTypes: LLVMType[], packed = true) =>
          new LLVMType(
            call(
              ctx.value,
              buildPointerArray(elementTypes),
              elementTypes.length,
              buildBool(packed),
            ),
          ),
    }),
    typeOf: fn({
      name: "LLVMTypeOf",
      type: [LLVMType.TYPE, [LLVMValue.TYPE]],
      wrap: (call) => (value: LLVMValue) => new LLVMType(call(value.value)),
    }),
    printTypeToString: fn({
      name: "LLVMPrintTypeToString",
      type: [StringType, [LLVMType.TYPE]],
      wrap: (call) => (value: LLVMType) => new LLVMMessage(call(value.value)),
    }),

    structCreateNamed: fn({
      name: "LLVMStructCreateNamed",
      type: [LLVMType.TYPE, [
        LLVMContext.TYPE,
        StringType,
      ]],
      wrap: (call) =>
        (ctx: LLVMContext, name: string) =>
          new LLVMType(call(ctx.value, buildStringPtr(name))),
    }),
    structSetBody: fn({
      name: "LLVMStructSetBody",
      type: ["void", [
        LLVMType.TYPE,
        LLVMTypeArrayType,
        "i32",
        BoolType,
      ]],
      wrap: (call) =>
        (structType: LLVMType, elementTypes: LLVMType[], packed = true): void =>
          call(
            structType.value,
            buildPointerArray(elementTypes),
            elementTypes.length,
            buildBool(packed),
          ),
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
          new LLVMValue(call(type.value, value, buildBool(signExtend))),
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
    buildBitCast: fn({
      name: "LLVMBuildBitCast",
      type: [LLVMValue.TYPE, [
        LLVMIRBuilder.TYPE,
        LLVMValue.TYPE,
        LLVMType.TYPE,
        StringType,
      ]],
      wrap: (call) =>
        (
          builder: LLVMIRBuilder,
          value: LLVMValue,
          newType: LLVMType,
          name = "",
        ) =>
          new LLVMValue(
            call(
              builder.value,
              value.value,
              newType.value,
              buildStringPtr(name),
            ),
          ),
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
      wrap: (call) =>
        (fn: LLVMValue) => ({ ok: !unBuildBool(call(fn.value, 2)) }),
    }),
    verifyModule: fn({
      name: "LLVMVerifyModule",
      type: [BoolType, [LLVMModule.TYPE, "i32", StringPtrType]],
      wrap: (call) =>
        (module: LLVMModule) => {
          const messageRef = new BigUint64Array(1);

          const err = unBuildBool(call(module.value, 2, messageRef));
          const message = new Deno.UnsafePointerView(messageRef[0])
            .getCString();

          return { ok: !err, message };
        },
    }),
    linkInMCJIT: fn({
      name: "LLVMLinkInMCJIT",
      type: ["void", []],
      wrap: (call) => () => call(),
    }),
    initializeX86Target: fn({
      name: "LLVMInitializeX86Target",
      type: [BoolType, []],
      wrap: (call) => () => call(),
    }),
    initializeX86AsmPrinter: fn({
      name: "LLVMInitializeX86AsmPrinter",
      type: [BoolType, []],
      wrap: (call) => () => call(),
    }),
    initializeX86AsmParser: fn({
      name: "LLVMInitializeX86AsmParser",
      type: [BoolType, []],
      wrap: (call) => () => call(),
    }),
    initializeX86TargetMC: fn({
      name: "LLVMInitializeX86TargetMC",
      type: [BoolType, []],
      wrap: (call) => () => call(),
    }),
    createJITCompilerForModule: fn({
      name: "LLVMCreateJITCompilerForModule",
      type: [BoolType, [
        LLVMExecutionEngine.TYPE,
        LLVMModule.TYPE,
        "i32",
        StringPtrType,
      ]],
      wrap: (call) =>
        (module: LLVMModule) => {
          const enginePtr = new BigUint64Array(1);
          const messageRef = new BigUint64Array(1);

          const err = unBuildBool(call(enginePtr, module.value, 2, messageRef));
          const message = new Deno.UnsafePointerView(messageRef[0])
            .getCString();

          if (err) {
            return { ok: false, message } as const;
          } else {
            return {
              ok: true,
              engine: new LLVMExecutionEngine(enginePtr[0]),
            } as const;
          }
        },
    }),
    getFunctionAddress: fn({
      name: "LLVMGetFunctionAddress",
      type: [FunctionType, [LLVMExecutionEngine.TYPE, StringPtrType]],
      wrap: (call) =>
        <Fn extends Deno.ForeignFunction>(
          engine: LLVMExecutionEngine,
          fnName: string,
          fnType: Fn,
        ) => new Deno.UnsafeFnPointer(call(engine.value, fnName), fnType),
    }),
    dumpModule: fn({
      name: "LLVMDumpModule",
      type: ["void", []],
      wrap: (call) => (module: LLVMModule) => call(module),
    }),
    removeModule: fn({
      name: "LLVMRemoveModule",
      type: [BoolType, []],
      wrap: (call) =>
        (engine: LLVMExecutionEngine, module: LLVMModule) => {
          const messageRef = new BigUint64Array(1);

          const ok = unBuildBool(call(engine, module, module, messageRef));
          const message = new Deno.UnsafePointerView(messageRef[0])
            .getCString();

          return { ok, message };
        },
    }),
    disposeExecutionEngine: fn({
      name: "LLVMDisposeExecutionEngine",
      type: ["void", [LLVMExecutionEngine.TYPE]],
      wrap: (call) => (engine: LLVMExecutionEngine) => call(engine),
    }),

    printModuleToString: fn({
      name: "LLVMPrintModuleToString",
      type: ["pointer", [LLVMModule.TYPE]],
      wrap: (call) =>
        (module: LLVMModule) => new LLVMMessage(call(module.value)),
    }),
    disposeMessage: fn({
      name: "LLVMDisposeMessage",
      type: ["void", ["pointer"]],
      wrap: (call) => (message: LLVMMessage): void => call(message.value),
    }),
  };

  return wrap(lib, libFile);
}

class TypedPointer {
  static TYPE = "pointer" as const;

  constructor(public readonly value: bigint) {}
}

export class LLVMContext extends TypedPointer {
  private __name = this;
}

export class LLVMIRBuilder extends TypedPointer {
  private __name = this;
}

export class LLVMModule extends TypedPointer {
  private __name = this;
}

export class LLVMValue extends TypedPointer {
  private __name = this;

  isNull() {
    return this.value === 0n;
  }
}

export class LLVMType extends TypedPointer {
  private __name = this;
}

export class LLVMBasicBlock extends TypedPointer {
  private __name = this;
}

export class LLVMMessage extends TypedPointer {
  private __name = this;

  stringValue() {
    return new Deno.UnsafePointerView(this.value).getCString();
  }
}

export class LLVMExecutionEngine extends TypedPointer {
  private __name = this;
}

function buildPointerArray(pointers: TypedPointer[]) {
  return Deno.UnsafePointer.of(
    BigInt64Array.from(pointers.map((p) => p.value)),
  );
}

function buildStringPtr(str: string): unknown {
  return Uint8Array.from(toCharCodes(str + "\0"));
}

function toCharCodes(str: string): number[] {
  return [...str].map((_, i) => str.charCodeAt(i));
}

function buildBool(bool: boolean): number {
  return bool ? 1 : 0;
}

function unBuildBool(num: number): boolean {
  return num !== 0;
}
