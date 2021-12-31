import {
  DynamicLibrary,
  ForeignFunction,
  LibraryObject,
  LibraryObjectDefinitionBase,
  LibraryObjectDefinitionInferenceMarker,
  LibraryObjectDefinitionToLibraryDefinition,
} from 'ffi-napi';
import { allocCString, NULL, Pointer, ref, refType } from 'ref-napi';

const stringRef = refType('string');
const nullRef = refType('void');

export type LibLLVM = ReturnType<typeof loadLibLLVMInternal>;
export const loadLibLLVM: (libFile?: string) => LibLLVM = loadLibLLVMInternal;

function loadLibLLVMInternal(libFile = '/usr/lib/llvm-13/lib/libLLVM.so') {
  const fn = wrapLib(DynamicLibrary(libFile));

  return {
    contextCreate: fn({
      name: 'LLVMContextCreate',
      type: [LLVMContext.TYPE, []],
      wrap: (call) => () => new LLVMContext(call()),
    }),
    contextDispose: fn({
      name: 'LLVMContextDispose',
      type: ['void', [LLVMContext.TYPE]],
      wrap: (call) => (ctx: LLVMContext) => call(ctx.value),
    }),

    moduleCreateWithNameInContext: fn({
      name: 'LLVMModuleCreateWithNameInContext',
      type: [LLVMModule.TYPE, ['string', LLVMContext.TYPE]],
      wrap: (call) => (moduleName: string, ctx: LLVMContext) =>
        new LLVMModule(call(moduleName, ctx.value)),
    }),
    disposeModule: fn({
      name: 'LLVMDisposeModule',
      type: ['void', [LLVMModule.TYPE]],
      wrap: (call) => (module: LLVMModule) => call(module.value),
    }),

    createBuilderInContext: fn({
      name: 'LLVMCreateBuilderInContext',
      type: [LLVMIRBuilder.TYPE, [LLVMContext.TYPE]],

      wrap: (call) => (ctx: LLVMContext) => new LLVMIRBuilder(call(ctx.value)),
    }),
    disposeBuilder: fn({
      name: 'LLVMDisposeBuilder',
      type: ['void', [LLVMIRBuilder.TYPE]],
      wrap: (call) => (builder: LLVMIRBuilder) => call(builder.value),
    }),

    setTarget: fn({
      name: 'LLVMSetTarget',
      type: ['void', [LLVMModule.TYPE, 'string']],
      wrap: (call) => (module: LLVMModule, targetTriple: string) =>
        call(module.value, targetTriple),
    }),

    voidTypeInContext: fn({
      name: 'LLVMVoidTypeInContext',
      type: [LLVMType.TYPE, [LLVMContext.TYPE]],
      wrap: (call) => (ctx: LLVMContext) => new LLVMType(call(ctx.value)),
    }),

    getUndef: fn({
      name: 'LLVMGetUndef',
      type: [LLVMValue.TYPE, [LLVMType.TYPE]],
      wrap: (call) => (type: LLVMType) => new LLVMValue(call(type.value)),
    }),

    addFunction: fn({
      name: 'LLVMAddFunction',
      type: [LLVMValue.TYPE, [LLVMModule.TYPE, 'string', LLVMType.TYPE]],
      wrap: (call) => (module: LLVMModule, fnName: string, type: LLVMType) =>
        call(module.value, fnName, type.value),
    }),

    // functionType: fn({
    //   name: 'LLVMAddFunction',
    //   type: [LLVMType.TYPE, [LLVMType.TYPE, LLVMType.TYPE, 'int', 'bool']],
    //   wrap: (call) => (module: LLVMModule, fnName: string, type: LLVMType) =>
    //     call(module.value, fnName, type.value),
    // }),

    verifyModule: fn({
      name: 'LLVMVerifyModule',
      type: ['bool', [LLVMModule.TYPE, 'int', stringRef]],
      wrap: (call) => (module: LLVMModule) => {
        const messageRef = ref(allocCString(' '.repeat(2048)));
        const err = call(module.value, 2, messageRef);
        const message = messageRef.deref();

        return { ok: !err, message };
      },
    }),

    printModuleToFile: fn({
      name: 'LLVMPrintModuleToFile',
      type: ['void', [LLVMModule.TYPE, 'string', nullRef]],
      wrap: (call) => (module: LLVMModule, fileName: string) =>
        call(module.value, fileName, NULL as never),
    }),
  };
}

type ValueOf<R> = R extends Record<string, infer V> ? V : never;

type FunctionType<
  T extends
    | ValueOf<LibraryObjectDefinitionBase>
    | ValueOf<LibraryObjectDefinitionInferenceMarker>,
> = (
  value: T,
) => LibraryObject<LibraryObjectDefinitionToLibraryDefinition<{ k: T }>>['k'];

type WrapFunctionParams<
  W,
  T extends
    | ValueOf<LibraryObjectDefinitionBase>
    | ValueOf<LibraryObjectDefinitionInferenceMarker>,
> = {
  name: string;
  type: T;
  wrap: (fn: ReturnType<FunctionType<T>>) => W;
};

function wrapLib(lib: DynamicLibrary) {
  return <
    W,
    T extends
      | ValueOf<LibraryObjectDefinitionBase>
      | ValueOf<LibraryObjectDefinitionInferenceMarker>,
  >({
    name,
    type: [returnType, argTypes],
    wrap,
  }: WrapFunctionParams<W, T>) => {
    // TODO: try to not use `as never`
    return wrap(ForeignFunction(lib.get(name), returnType, argTypes) as never);
  };
}

class UniqueType<T> {
  constructor(public readonly value: T) {}
}

export class LLVMContext extends UniqueType<Pointer<void>> {
  static TYPE = refType('void');

  private __name = this;
}

export class LLVMIRBuilder extends UniqueType<Pointer<void>> {
  static TYPE = refType('void');

  private __name = this;
}

export class LLVMModule extends UniqueType<Pointer<void>> {
  static TYPE = refType('void');

  private __name = this;
}

export class LLVMValue extends UniqueType<Pointer<void>> {
  static TYPE = refType('void');

  private __name = this;
}

export class LLVMType extends UniqueType<Pointer<void>> {
  static TYPE = refType('void');

  private __name = this;
}

export class LLVMFunction extends UniqueType<Pointer<void>> {
  static TYPE = refType('void');

  private __name = this;
}
