// deno-lint-ignore-file

import * as types from "./types.ts";

// ./llvm-c/Analysis.h:44:10
export const LLVMVerifyModule = {
  parameters: [types.LLVMModuleRef, types.LLVMVerifierFailureAction, "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/Analysis.h:49:10
export const LLVMVerifyFunction = {
  parameters: [types.LLVMValueRef, types.LLVMVerifierFailureAction],
  result: types.LLVMBool
} as const;

// ./llvm-c/Analysis.h:53:6
export const LLVMViewFunctionCFG = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Analysis.h:54:6
export const LLVMViewFunctionCFGOnly = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:83:1>
export const LLVMInitializeWebAssemblyTargetInfo = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:85:1>
export const LLVMInitializeX86TargetInfo = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:119:1>
export const LLVMInitializeWebAssemblyTarget = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:121:1>
export const LLVMInitializeX86Target = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:155:1>
export const LLVMInitializeWebAssemblyTargetMC = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:157:1>
export const LLVMInitializeX86TargetMC = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:42:1 <Spelling=<scratch space>:2:1>
export const LLVMInitializeWebAssemblyAsmPrinter = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:43:1 <Spelling=<scratch space>:4:1>
export const LLVMInitializeX86AsmPrinter = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:41:1 <Spelling=<scratch space>:36:1>
export const LLVMInitializeWebAssemblyAsmParser = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:42:1 <Spelling=<scratch space>:38:1>
export const LLVMInitializeX86AsmParser = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:41:1 <Spelling=<scratch space>:68:1>
export const LLVMInitializeWebAssemblyDisassembler = {
  parameters: [],
  result: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:42:1 <Spelling=<scratch space>:70:1>
export const LLVMInitializeX86Disassembler = {
  parameters: [],
  result: "void"
} as const;

// ./llvm-c/Target.h:186:19
export const LLVMGetModuleDataLayout = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMTargetDataRef
} as const;

// ./llvm-c/Target.h:193:6
export const LLVMSetModuleDataLayout = {
  parameters: [types.LLVMModuleRef, types.LLVMTargetDataRef],
  result: "void"
} as const;

// ./llvm-c/Target.h:197:19
export const LLVMCreateTargetData = {
  parameters: ["pointer"],
  result: types.LLVMTargetDataRef
} as const;

// ./llvm-c/Target.h:201:6
export const LLVMDisposeTargetData = {
  parameters: [types.LLVMTargetDataRef],
  result: "void"
} as const;

// ./llvm-c/Target.h:206:6
export const LLVMAddTargetLibraryInfo = {
  parameters: [types.LLVMTargetLibraryInfoRef, types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Target.h:212:7
export const LLVMCopyStringRepOfTargetData = {
  parameters: [types.LLVMTargetDataRef],
  result: "pointer"
} as const;

// ./llvm-c/Target.h:217:23
export const LLVMByteOrder = {
  parameters: [types.LLVMTargetDataRef],
  result: "i32"
} as const;

// ./llvm-c/Target.h:221:10
export const LLVMPointerSize = {
  parameters: [types.LLVMTargetDataRef],
  result: "u32"
} as const;

// ./llvm-c/Target.h:226:10
export const LLVMPointerSizeForAS = {
  parameters: [types.LLVMTargetDataRef, "u32"],
  result: "u32"
} as const;

// ./llvm-c/Target.h:230:13
export const LLVMIntPtrType = {
  parameters: [types.LLVMTargetDataRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Target.h:235:13
export const LLVMIntPtrTypeForAS = {
  parameters: [types.LLVMTargetDataRef, "u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Target.h:239:13
export const LLVMIntPtrTypeInContext = {
  parameters: [types.LLVMContextRef, types.LLVMTargetDataRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Target.h:244:13
export const LLVMIntPtrTypeForASInContext = {
  parameters: [types.LLVMContextRef, types.LLVMTargetDataRef, "u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Target.h:249:20
export const LLVMSizeOfTypeInBits = {
  parameters: [types.LLVMTargetDataRef, types.LLVMTypeRef],
  result: "u64"
} as const;

// ./llvm-c/Target.h:253:20
export const LLVMStoreSizeOfType = {
  parameters: [types.LLVMTargetDataRef, types.LLVMTypeRef],
  result: "u64"
} as const;

// ./llvm-c/Target.h:257:20
export const LLVMABISizeOfType = {
  parameters: [types.LLVMTargetDataRef, types.LLVMTypeRef],
  result: "u64"
} as const;

// ./llvm-c/Target.h:261:10
export const LLVMABIAlignmentOfType = {
  parameters: [types.LLVMTargetDataRef, types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Target.h:265:10
export const LLVMCallFrameAlignmentOfType = {
  parameters: [types.LLVMTargetDataRef, types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Target.h:269:10
export const LLVMPreferredAlignmentOfType = {
  parameters: [types.LLVMTargetDataRef, types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Target.h:273:10
export const LLVMPreferredAlignmentOfGlobal = {
  parameters: [types.LLVMTargetDataRef, types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Target.h:278:10
export const LLVMElementAtOffset = {
  parameters: [types.LLVMTargetDataRef, types.LLVMTypeRef, "u64"],
  result: "u32"
} as const;

// ./llvm-c/Target.h:283:20
export const LLVMOffsetOfElement = {
  parameters: [types.LLVMTargetDataRef, types.LLVMTypeRef, "u32"],
  result: "u64"
} as const;

// ./llvm-c/TargetMachine.h:70:15
export const LLVMGetFirstTarget = {
  parameters: [],
  result: types.LLVMTargetRef
} as const;

// ./llvm-c/TargetMachine.h:72:15
export const LLVMGetNextTarget = {
  parameters: [types.LLVMTargetRef],
  result: types.LLVMTargetRef
} as const;

// ./llvm-c/TargetMachine.h:77:15
export const LLVMGetTargetFromName = {
  parameters: ["pointer"],
  result: types.LLVMTargetRef
} as const;

// ./llvm-c/TargetMachine.h:82:10
export const LLVMGetTargetFromTriple = {
  parameters: ["pointer", "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/TargetMachine.h:86:13
export const LLVMGetTargetName = {
  parameters: [types.LLVMTargetRef],
  result: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:89:13
export const LLVMGetTargetDescription = {
  parameters: [types.LLVMTargetRef],
  result: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:92:10
export const LLVMTargetHasJIT = {
  parameters: [types.LLVMTargetRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/TargetMachine.h:95:10
export const LLVMTargetHasTargetMachine = {
  parameters: [types.LLVMTargetRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/TargetMachine.h:98:10
export const LLVMTargetHasAsmBackend = {
  parameters: [types.LLVMTargetRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/TargetMachine.h:102:22
export const LLVMCreateTargetMachine = {
  parameters: [types.LLVMTargetRef, "pointer", "pointer", "pointer", types.LLVMCodeGenOptLevel, types.LLVMRelocMode, types.LLVMCodeModel],
  result: types.LLVMTargetMachineRef
} as const;

// ./llvm-c/TargetMachine.h:108:6
export const LLVMDisposeTargetMachine = {
  parameters: [types.LLVMTargetMachineRef],
  result: "void"
} as const;

// ./llvm-c/TargetMachine.h:111:15
export const LLVMGetTargetMachineTarget = {
  parameters: [types.LLVMTargetMachineRef],
  result: types.LLVMTargetRef
} as const;

// ./llvm-c/TargetMachine.h:116:7
export const LLVMGetTargetMachineTriple = {
  parameters: [types.LLVMTargetMachineRef],
  result: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:121:7
export const LLVMGetTargetMachineCPU = {
  parameters: [types.LLVMTargetMachineRef],
  result: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:126:7
export const LLVMGetTargetMachineFeatureString = {
  parameters: [types.LLVMTargetMachineRef],
  result: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:129:19
export const LLVMCreateTargetDataLayout = {
  parameters: [types.LLVMTargetMachineRef],
  result: types.LLVMTargetDataRef
} as const;

// ./llvm-c/TargetMachine.h:132:6
export const LLVMSetTargetMachineAsmVerbosity = {
  parameters: [types.LLVMTargetMachineRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/TargetMachine.h:138:10
export const LLVMTargetMachineEmitToFile = {
  parameters: [types.LLVMTargetMachineRef, types.LLVMModuleRef, "pointer", types.LLVMCodeGenFileType, "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/TargetMachine.h:142:10
export const LLVMTargetMachineEmitToMemoryBuffer = {
  parameters: [types.LLVMTargetMachineRef, types.LLVMModuleRef, types.LLVMCodeGenFileType, "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/TargetMachine.h:148:7
export const LLVMGetDefaultTargetTriple = {
  parameters: [],
  result: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:152:7
export const LLVMNormalizeTargetTriple = {
  parameters: ["pointer"],
  result: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:156:7
export const LLVMGetHostCPUName = {
  parameters: [],
  result: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:160:7
export const LLVMGetHostCPUFeatures = {
  parameters: [],
  result: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:163:6
export const LLVMAddAnalysisPasses = {
  parameters: [types.LLVMTargetMachineRef, types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:36:6
export const LLVMLinkInMCJIT = {
  parameters: [],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:37:6
export const LLVMLinkInInterpreter = {
  parameters: [],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:53:21
export const LLVMCreateGenericValueOfInt = {
  parameters: [types.LLVMTypeRef, "u64", types.LLVMBool],
  result: types.LLVMGenericValueRef
} as const;

// ./llvm-c/ExecutionEngine.h:57:21
export const LLVMCreateGenericValueOfPointer = {
  parameters: ["pointer"],
  result: types.LLVMGenericValueRef
} as const;

// ./llvm-c/ExecutionEngine.h:59:21
export const LLVMCreateGenericValueOfFloat = {
  parameters: [types.LLVMTypeRef, "i64"],
  result: types.LLVMGenericValueRef
} as const;

// ./llvm-c/ExecutionEngine.h:61:10
export const LLVMGenericValueIntWidth = {
  parameters: [types.LLVMGenericValueRef],
  result: "u32"
} as const;

// ./llvm-c/ExecutionEngine.h:63:20
export const LLVMGenericValueToInt = {
  parameters: [types.LLVMGenericValueRef, types.LLVMBool],
  result: "u64"
} as const;

// ./llvm-c/ExecutionEngine.h:66:7
export const LLVMGenericValueToPointer = {
  parameters: [types.LLVMGenericValueRef],
  result: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:68:8
export const LLVMGenericValueToFloat = {
  parameters: [types.LLVMTypeRef, types.LLVMGenericValueRef],
  result: "i64"
} as const;

// ./llvm-c/ExecutionEngine.h:70:6
export const LLVMDisposeGenericValue = {
  parameters: [types.LLVMGenericValueRef],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:74:10
export const LLVMCreateExecutionEngineForModule = {
  parameters: ["pointer", types.LLVMModuleRef, "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/ExecutionEngine.h:78:10
export const LLVMCreateInterpreterForModule = {
  parameters: ["pointer", types.LLVMModuleRef, "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/ExecutionEngine.h:82:10
export const LLVMCreateJITCompilerForModule = {
  parameters: ["pointer", types.LLVMModuleRef, "u32", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/ExecutionEngine.h:87:6
export const LLVMInitializeMCJITCompilerOptions = {
  parameters: ["pointer", "u64"],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:107:10
export const LLVMCreateMCJITCompilerForModule = {
  parameters: ["pointer", types.LLVMModuleRef, "pointer", "u64", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/ExecutionEngine.h:112:6
export const LLVMDisposeExecutionEngine = {
  parameters: [types.LLVMExecutionEngineRef],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:114:6
export const LLVMRunStaticConstructors = {
  parameters: [types.LLVMExecutionEngineRef],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:116:6
export const LLVMRunStaticDestructors = {
  parameters: [types.LLVMExecutionEngineRef],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:118:5
export const LLVMRunFunctionAsMain = {
  parameters: [types.LLVMExecutionEngineRef, types.LLVMValueRef, "u32", "pointer", "pointer"],
  result: "i32"
} as const;

// ./llvm-c/ExecutionEngine.h:122:21
export const LLVMRunFunction = {
  parameters: [types.LLVMExecutionEngineRef, types.LLVMValueRef, "u32", "pointer"],
  result: types.LLVMGenericValueRef
} as const;

// ./llvm-c/ExecutionEngine.h:126:6
export const LLVMFreeMachineCodeForFunction = {
  parameters: [types.LLVMExecutionEngineRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:128:6
export const LLVMAddModule = {
  parameters: [types.LLVMExecutionEngineRef, types.LLVMModuleRef],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:130:10
export const LLVMRemoveModule = {
  parameters: [types.LLVMExecutionEngineRef, types.LLVMModuleRef, "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/ExecutionEngine.h:133:10
export const LLVMFindFunction = {
  parameters: [types.LLVMExecutionEngineRef, "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/ExecutionEngine.h:136:7
export const LLVMRecompileAndRelinkFunction = {
  parameters: [types.LLVMExecutionEngineRef, types.LLVMValueRef],
  result: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:139:19
export const LLVMGetExecutionEngineTargetData = {
  parameters: [types.LLVMExecutionEngineRef],
  result: types.LLVMTargetDataRef
} as const;

// ./llvm-c/ExecutionEngine.h:141:1
export const LLVMGetExecutionEngineTargetMachine = {
  parameters: [types.LLVMExecutionEngineRef],
  result: types.LLVMTargetMachineRef
} as const;

// ./llvm-c/ExecutionEngine.h:143:6
export const LLVMAddGlobalMapping = {
  parameters: [types.LLVMExecutionEngineRef, types.LLVMValueRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:146:7
export const LLVMGetPointerToGlobal = {
  parameters: [types.LLVMExecutionEngineRef, types.LLVMValueRef],
  result: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:148:10
export const LLVMGetGlobalValueAddress = {
  parameters: [types.LLVMExecutionEngineRef, "pointer"],
  result: "u64"
} as const;

// ./llvm-c/ExecutionEngine.h:150:10
export const LLVMGetFunctionAddress = {
  parameters: [types.LLVMExecutionEngineRef, "pointer"],
  result: "u64"
} as const;

// ./llvm-c/ExecutionEngine.h:154:10
export const LLVMExecutionEngineGetErrMsg = {
  parameters: [types.LLVMExecutionEngineRef, "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/ExecutionEngine.h:180:27
export const LLVMCreateSimpleMCJITMemoryManager = {
  parameters: ["pointer", types.LLVMMemoryManagerAllocateCodeSectionCallback, types.LLVMMemoryManagerAllocateDataSectionCallback, types.LLVMMemoryManagerFinalizeMemoryCallback, types.LLVMMemoryManagerDestroyCallback],
  result: types.LLVMMCJITMemoryManagerRef
} as const;

// ./llvm-c/ExecutionEngine.h:187:6
export const LLVMDisposeMCJITMemoryManager = {
  parameters: [types.LLVMMCJITMemoryManagerRef],
  result: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:191:25
export const LLVMCreateGDBRegistrationListener = {
  parameters: [],
  result: types.LLVMJITEventListenerRef
} as const;

// ./llvm-c/ExecutionEngine.h:192:25
export const LLVMCreateIntelJITEventListener = {
  parameters: [],
  result: types.LLVMJITEventListenerRef
} as const;

// ./llvm-c/ExecutionEngine.h:193:25
export const LLVMCreateOProfileJITEventListener = {
  parameters: [],
  result: types.LLVMJITEventListenerRef
} as const;

// ./llvm-c/ExecutionEngine.h:194:25
export const LLVMCreatePerfJITEventListener = {
  parameters: [],
  result: types.LLVMJITEventListenerRef
} as const;

// ./llvm-c/Disassembler.h:38:22
export const LLVMCreateDisasm = {
  parameters: ["pointer", "pointer", "i32", types.LLVMOpInfoCallback, types.LLVMSymbolLookupCallback],
  result: types.LLVMDisasmContextRef
} as const;

// ./llvm-c/Disassembler.h:50:22
export const LLVMCreateDisasmCPU = {
  parameters: ["pointer", "pointer", "pointer", "i32", types.LLVMOpInfoCallback, types.LLVMSymbolLookupCallback],
  result: types.LLVMDisasmContextRef
} as const;

// ./llvm-c/Disassembler.h:63:1
export const LLVMCreateDisasmCPUFeatures = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "i32", types.LLVMOpInfoCallback, types.LLVMSymbolLookupCallback],
  result: types.LLVMDisasmContextRef
} as const;

// ./llvm-c/Disassembler.h:72:5
export const LLVMSetDisasmOptions = {
  parameters: [types.LLVMDisasmContextRef, "u64"],
  result: "i32"
} as const;

// ./llvm-c/Disassembler.h:88:6
export const LLVMDisasmDispose = {
  parameters: [types.LLVMDisasmContextRef],
  result: "void"
} as const;

// ./llvm-c/Disassembler.h:100:8
export const LLVMDisasmInstruction = {
  parameters: [types.LLVMDisasmContextRef, "pointer", "u64", "u64", "pointer", "u64"],
  result: "u64"
} as const;

// ./llvm-c/DebugInfo.h:197:10
export const LLVMDebugMetadataVersion = {
  parameters: [],
  result: "u32"
} as const;

// ./llvm-c/DebugInfo.h:202:10
export const LLVMGetModuleDebugMetadataVersion = {
  parameters: [types.LLVMModuleRef],
  result: "u32"
} as const;

// ./llvm-c/DebugInfo.h:210:10
export const LLVMStripModuleDebugInfo = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/DebugInfo.h:216:18
export const LLVMCreateDIBuilderDisallowUnresolved = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMDIBuilderRef
} as const;

// ./llvm-c/DebugInfo.h:223:18
export const LLVMCreateDIBuilder = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMDIBuilderRef
} as const;

// ./llvm-c/DebugInfo.h:229:6
export const LLVMDisposeDIBuilder = {
  parameters: [types.LLVMDIBuilderRef],
  result: "void"
} as const;

// ./llvm-c/DebugInfo.h:234:6
export const LLVMDIBuilderFinalize = {
  parameters: [types.LLVMDIBuilderRef],
  result: "void"
} as const;

// ./llvm-c/DebugInfo.h:240:6
export const LLVMDIBuilderFinalizeSubprogram = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef],
  result: "void"
} as const;

// ./llvm-c/DebugInfo.h:275:17
export const LLVMDIBuilderCreateCompileUnit = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMDWARFSourceLanguage, types.LLVMMetadataRef, "pointer", "u64", types.LLVMBool, "pointer", "u64", "u32", "pointer", "u64", types.LLVMDWARFEmissionKind, "u32", types.LLVMBool, types.LLVMBool, "pointer", "u64", "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:293:1
export const LLVMDIBuilderCreateFile = {
  parameters: [types.LLVMDIBuilderRef, "pointer", "u64", "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:312:1
export const LLVMDIBuilderCreateModule = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", "pointer", "u64", "pointer", "u64", "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:328:1
export const LLVMDIBuilderCreateNameSpace = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMBool],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:351:17
export const LLVMDIBuilderCreateFunction = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", "pointer", "u64", types.LLVMMetadataRef, "u32", types.LLVMMetadataRef, types.LLVMBool, types.LLVMBool, "u32", types.LLVMDIFlags, types.LLVMBool],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:366:17
export const LLVMDIBuilderCreateLexicalBlock = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, types.LLVMMetadataRef, "u32", "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:378:1
export const LLVMDIBuilderCreateLexicalBlockFile = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, types.LLVMMetadataRef, "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:392:1
export const LLVMDIBuilderCreateImportedModuleFromNamespace = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMMetadataRef, "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:409:17
export const LLVMDIBuilderCreateImportedModuleFromAlias = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMMetadataRef, "u32", "pointer", "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:424:17
export const LLVMDIBuilderCreateImportedModuleFromModule = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMMetadataRef, "u32", "pointer", "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:444:17
export const LLVMDIBuilderCreateImportedDeclaration = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMMetadataRef, "u32", "pointer", "u64", "pointer", "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:460:1
export const LLVMDIBuilderCreateDebugLocation = {
  parameters: [types.LLVMContextRef, "u32", "u32", types.LLVMMetadataRef, types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:470:10
export const LLVMDILocationGetLine = {
  parameters: [types.LLVMMetadataRef],
  result: "u32"
} as const;

// ./llvm-c/DebugInfo.h:478:10
export const LLVMDILocationGetColumn = {
  parameters: [types.LLVMMetadataRef],
  result: "u32"
} as const;

// ./llvm-c/DebugInfo.h:486:17
export const LLVMDILocationGetScope = {
  parameters: [types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:494:17
export const LLVMDILocationGetInlinedAt = {
  parameters: [types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:502:17
export const LLVMDIScopeGetFile = {
  parameters: [types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:511:13
export const LLVMDIFileGetDirectory = {
  parameters: [types.LLVMMetadataRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:520:13
export const LLVMDIFileGetFilename = {
  parameters: [types.LLVMMetadataRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:529:13
export const LLVMDIFileGetSource = {
  parameters: [types.LLVMMetadataRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:537:17
export const LLVMDIBuilderGetOrCreateTypeArray = {
  parameters: [types.LLVMDIBuilderRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:552:1
export const LLVMDIBuilderCreateSubroutineType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u32", types.LLVMDIFlags],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:569:17
export const LLVMDIBuilderCreateMacro = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "u32", types.LLVMDWARFMacinfoRecordType, "pointer", "u64", "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:586:1
export const LLVMDIBuilderCreateTempMacroFile = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "u32", types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:598:17
export const LLVMDIBuilderCreateEnumerator = {
  parameters: [types.LLVMDIBuilderRef, "pointer", "u64", "i64", types.LLVMBool],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:617:17
export const LLVMDIBuilderCreateEnumerationType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMMetadataRef, "u32", "u64", "u32", "pointer", "u32", types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:640:17
export const LLVMDIBuilderCreateUnionType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMMetadataRef, "u32", "u64", "u32", types.LLVMDIFlags, "pointer", "u32", "u32", "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:658:1
export const LLVMDIBuilderCreateArrayType = {
  parameters: [types.LLVMDIBuilderRef, "u64", "u32", types.LLVMMetadataRef, "pointer", "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:673:1
export const LLVMDIBuilderCreateVectorType = {
  parameters: [types.LLVMDIBuilderRef, "u64", "u32", types.LLVMMetadataRef, "pointer", "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:685:1
export const LLVMDIBuilderCreateUnspecifiedType = {
  parameters: [types.LLVMDIBuilderRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:699:1
export const LLVMDIBuilderCreateBasicType = {
  parameters: [types.LLVMDIBuilderRef, "pointer", "u64", "u64", types.LLVMDWARFTypeEncoding, types.LLVMDIFlags],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:714:17
export const LLVMDIBuilderCreatePointerType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "u64", "u32", "u32", "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:737:17
export const LLVMDIBuilderCreateStructType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMMetadataRef, "u32", "u64", "u32", types.LLVMDIFlags, types.LLVMMetadataRef, "pointer", "u32", "u32", types.LLVMMetadataRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:759:17
export const LLVMDIBuilderCreateMemberType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMMetadataRef, "u32", "u64", "u32", "u64", types.LLVMDIFlags, types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:780:1
export const LLVMDIBuilderCreateStaticMemberType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMMetadataRef, "u32", types.LLVMMetadataRef, types.LLVMDIFlags, types.LLVMValueRef, "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:796:1
export const LLVMDIBuilderCreateMemberPointerType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, types.LLVMMetadataRef, "u64", "u32", types.LLVMDIFlags],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:817:1
export const LLVMDIBuilderCreateObjCIVar = {
  parameters: [types.LLVMDIBuilderRef, "pointer", "u64", types.LLVMMetadataRef, "u32", "u64", "u32", "u64", types.LLVMDIFlags, types.LLVMMetadataRef, types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:839:1
export const LLVMDIBuilderCreateObjCProperty = {
  parameters: [types.LLVMDIBuilderRef, "pointer", "u64", types.LLVMMetadataRef, "u32", "pointer", "u64", "pointer", "u64", "u32", types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:853:1
export const LLVMDIBuilderCreateObjectPointerType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:865:1
export const LLVMDIBuilderCreateQualifiedType = {
  parameters: [types.LLVMDIBuilderRef, "u32", types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:876:1
export const LLVMDIBuilderCreateReferenceType = {
  parameters: [types.LLVMDIBuilderRef, "u32", types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:884:1
export const LLVMDIBuilderCreateNullPtrType = {
  parameters: [types.LLVMDIBuilderRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:896:1
export const LLVMDIBuilderCreateTypedef = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMMetadataRef, "u32", types.LLVMMetadataRef, "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:912:1
export const LLVMDIBuilderCreateInheritance = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, types.LLVMMetadataRef, "u64", "u32", types.LLVMDIFlags],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:933:17
export const LLVMDIBuilderCreateForwardDecl = {
  parameters: [types.LLVMDIBuilderRef, "u32", "pointer", "u64", types.LLVMMetadataRef, types.LLVMMetadataRef, "u32", "u32", "u64", "u32", "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:957:1
export const LLVMDIBuilderCreateReplaceableCompositeType = {
  parameters: [types.LLVMDIBuilderRef, "u32", "pointer", "u64", types.LLVMMetadataRef, types.LLVMMetadataRef, "u32", "u32", "u64", "u32", types.LLVMDIFlags, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:979:1
export const LLVMDIBuilderCreateBitFieldMemberType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMMetadataRef, "u32", "u64", "u64", "u64", types.LLVMDIFlags, types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1010:17
export const LLVMDIBuilderCreateClassType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMMetadataRef, "u32", "u64", "u32", "u64", types.LLVMDIFlags, types.LLVMMetadataRef, "pointer", "u32", types.LLVMMetadataRef, types.LLVMMetadataRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1025:1
export const LLVMDIBuilderCreateArtificialType = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1035:13
export const LLVMDITypeGetName = {
  parameters: [types.LLVMMetadataRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1043:10
export const LLVMDITypeGetSizeInBits = {
  parameters: [types.LLVMMetadataRef],
  result: "u64"
} as const;

// ./llvm-c/DebugInfo.h:1051:10
export const LLVMDITypeGetOffsetInBits = {
  parameters: [types.LLVMMetadataRef],
  result: "u64"
} as const;

// ./llvm-c/DebugInfo.h:1059:10
export const LLVMDITypeGetAlignInBits = {
  parameters: [types.LLVMMetadataRef],
  result: "u32"
} as const;

// ./llvm-c/DebugInfo.h:1067:10
export const LLVMDITypeGetLine = {
  parameters: [types.LLVMMetadataRef],
  result: "u32"
} as const;

// ./llvm-c/DebugInfo.h:1075:13
export const LLVMDITypeGetFlags = {
  parameters: [types.LLVMMetadataRef],
  result: types.LLVMDIFlags
} as const;

// ./llvm-c/DebugInfo.h:1083:17
export const LLVMDIBuilderGetOrCreateSubrange = {
  parameters: [types.LLVMDIBuilderRef, "i64", "i64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1093:17
export const LLVMDIBuilderGetOrCreateArray = {
  parameters: [types.LLVMDIBuilderRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1104:17
export const LLVMDIBuilderCreateExpression = {
  parameters: [types.LLVMDIBuilderRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1114:1
export const LLVMDIBuilderCreateConstantValueExpression = {
  parameters: [types.LLVMDIBuilderRef, "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1136:17
export const LLVMDIBuilderCreateGlobalVariableExpression = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", "pointer", "u64", types.LLVMMetadataRef, "u32", types.LLVMMetadataRef, types.LLVMBool, types.LLVMMetadataRef, types.LLVMMetadataRef, "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1148:17
export const LLVMDIGlobalVariableExpressionGetVariable = {
  parameters: [types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1156:17
export const LLVMDIGlobalVariableExpressionGetExpression = {
  parameters: [types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1165:17
export const LLVMDIVariableGetFile = {
  parameters: [types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1173:17
export const LLVMDIVariableGetScope = {
  parameters: [types.LLVMMetadataRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1181:10
export const LLVMDIVariableGetLine = {
  parameters: [types.LLVMMetadataRef],
  result: "u32"
} as const;

// ./llvm-c/DebugInfo.h:1191:17
export const LLVMTemporaryMDNode = {
  parameters: [types.LLVMContextRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1201:6
export const LLVMDisposeTemporaryMDNode = {
  parameters: [types.LLVMMetadataRef],
  result: "void"
} as const;

// ./llvm-c/DebugInfo.h:1208:6
export const LLVMMetadataReplaceAllUsesWith = {
  parameters: [types.LLVMMetadataRef, types.LLVMMetadataRef],
  result: "void"
} as const;

// ./llvm-c/DebugInfo.h:1228:17
export const LLVMDIBuilderCreateTempGlobalVariableFwdDecl = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", "pointer", "u64", types.LLVMMetadataRef, "u32", types.LLVMMetadataRef, types.LLVMBool, types.LLVMMetadataRef, "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1243:14
export const LLVMDIBuilderInsertDeclareBefore = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMValueRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/DebugInfo.h:1258:14
export const LLVMDIBuilderInsertDeclareAtEnd = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMValueRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/DebugInfo.h:1271:14
export const LLVMDIBuilderInsertDbgValueBefore = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMValueRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/DebugInfo.h:1289:14
export const LLVMDIBuilderInsertDbgValueAtEnd = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMValueRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMMetadataRef, types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/DebugInfo.h:1309:17
export const LLVMDIBuilderCreateAutoVariable = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", types.LLVMMetadataRef, "u32", types.LLVMMetadataRef, types.LLVMBool, types.LLVMDIFlags, "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1327:17
export const LLVMDIBuilderCreateParameterVariable = {
  parameters: [types.LLVMDIBuilderRef, types.LLVMMetadataRef, "pointer", "u64", "u32", types.LLVMMetadataRef, "u32", types.LLVMMetadataRef, types.LLVMBool, types.LLVMDIFlags],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1337:17
export const LLVMGetSubprogram = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1344:6
export const LLVMSetSubprogram = {
  parameters: [types.LLVMValueRef, types.LLVMMetadataRef],
  result: "void"
} as const;

// ./llvm-c/DebugInfo.h:1352:10
export const LLVMDISubprogramGetLine = {
  parameters: [types.LLVMMetadataRef],
  result: "u32"
} as const;

// ./llvm-c/DebugInfo.h:1359:17
export const LLVMInstructionGetDebugLoc = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/DebugInfo.h:1368:6
export const LLVMInstructionSetDebugLoc = {
  parameters: [types.LLVMValueRef, types.LLVMMetadataRef],
  result: "void"
} as const;

// ./llvm-c/DebugInfo.h:1375:18
export const LLVMGetMetadataKind = {
  parameters: [types.LLVMMetadataRef],
  result: types.LLVMMetadataKind
} as const;

// ./llvm-c/Error.h:44:17
export const LLVMGetErrorTypeId = {
  parameters: [types.LLVMErrorRef],
  result: types.LLVMErrorTypeId
} as const;

// ./llvm-c/Error.h:52:6
export const LLVMConsumeError = {
  parameters: [types.LLVMErrorRef],
  result: "void"
} as const;

// ./llvm-c/Error.h:60:7
export const LLVMGetErrorMessage = {
  parameters: [types.LLVMErrorRef],
  result: "pointer"
} as const;

// ./llvm-c/Error.h:65:6
export const LLVMDisposeErrorMessage = {
  parameters: ["pointer"],
  result: "void"
} as const;

// ./llvm-c/Error.h:70:17
export const LLVMGetStringErrorTypeId = {
  parameters: [],
  result: types.LLVMErrorTypeId
} as const;

// ./llvm-c/Error.h:75:14
export const LLVMCreateStringError = {
  parameters: ["pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:457:6
export const LLVMOrcExecutionSessionSetErrorReporter = {
  parameters: [types.LLVMOrcExecutionSessionRef, types.LLVMOrcErrorReporterFunction, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Orc.h:468:1
export const LLVMOrcExecutionSessionGetSymbolStringPool = {
  parameters: [types.LLVMOrcExecutionSessionRef],
  result: types.LLVMOrcSymbolStringPoolRef
} as const;

// ./llvm-c/Orc.h:480:6
export const LLVMOrcSymbolStringPoolClearDeadEntries = {
  parameters: [types.LLVMOrcSymbolStringPoolRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:495:1
export const LLVMOrcExecutionSessionIntern = {
  parameters: [types.LLVMOrcExecutionSessionRef, "pointer"],
  result: types.LLVMOrcSymbolStringPoolEntryRef
} as const;

// ./llvm-c/Orc.h:500:6
export const LLVMOrcRetainSymbolStringPoolEntry = {
  parameters: [types.LLVMOrcSymbolStringPoolEntryRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:505:6
export const LLVMOrcReleaseSymbolStringPoolEntry = {
  parameters: [types.LLVMOrcSymbolStringPoolEntryRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:507:13
export const LLVMOrcSymbolStringPoolEntryStr = {
  parameters: [types.LLVMOrcSymbolStringPoolEntryRef],
  result: "pointer"
} as const;

// ./llvm-c/Orc.h:512:6
export const LLVMOrcReleaseResourceTracker = {
  parameters: [types.LLVMOrcResourceTrackerRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:518:6
export const LLVMOrcResourceTrackerTransferTo = {
  parameters: [types.LLVMOrcResourceTrackerRef, types.LLVMOrcResourceTrackerRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:525:14
export const LLVMOrcResourceTrackerRemove = {
  parameters: [types.LLVMOrcResourceTrackerRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:532:6
export const LLVMOrcDisposeDefinitionGenerator = {
  parameters: [types.LLVMOrcDefinitionGeneratorRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:537:6
export const LLVMOrcDisposeMaterializationUnit = {
  parameters: [types.LLVMOrcMaterializationUnitRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:572:31
export const LLVMOrcCreateCustomMaterializationUnit = {
  parameters: ["pointer", "pointer", types.LLVMOrcCSymbolFlagsMapPairs, "u64", types.LLVMOrcSymbolStringPoolEntryRef, types.LLVMOrcMaterializationUnitMaterializeFunction, types.LLVMOrcMaterializationUnitDiscardFunction, types.LLVMOrcMaterializationUnitDestroyFunction],
  result: types.LLVMOrcMaterializationUnitRef
} as const;

// ./llvm-c/Orc.h:601:1
export const LLVMOrcAbsoluteSymbols = {
  parameters: [types.LLVMOrcCSymbolMapPairs, "u64"],
  result: types.LLVMOrcMaterializationUnitRef
} as const;

// ./llvm-c/Orc.h:624:31
export const LLVMOrcLazyReexports = {
  parameters: [types.LLVMOrcLazyCallThroughManagerRef, types.LLVMOrcIndirectStubsManagerRef, types.LLVMOrcJITDylibRef, types.LLVMOrcCSymbolAliasMapPairs, "u64"],
  result: types.LLVMOrcMaterializationUnitRef
} as const;

// ./llvm-c/Orc.h:639:6
export const LLVMOrcDisposeMaterializationResponsibility = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:645:20
export const LLVMOrcMaterializationResponsibilityGetTargetDylib = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef],
  result: types.LLVMOrcJITDylibRef
} as const;

// ./llvm-c/Orc.h:652:1
export const LLVMOrcMaterializationResponsibilityGetExecutionSession = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef],
  result: types.LLVMOrcExecutionSessionRef
} as const;

// ./llvm-c/Orc.h:665:29
export const LLVMOrcMaterializationResponsibilityGetSymbols = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef, "pointer"],
  result: types.LLVMOrcCSymbolFlagsMapPairs
} as const;

// ./llvm-c/Orc.h:673:6
export const LLVMOrcDisposeCSymbolFlagsMap = {
  parameters: [types.LLVMOrcCSymbolFlagsMapPairs],
  result: "void"
} as const;

// ./llvm-c/Orc.h:684:1
export const LLVMOrcMaterializationResponsibilityGetInitializerSymbol = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef],
  result: types.LLVMOrcSymbolStringPoolEntryRef
} as const;

// ./llvm-c/Orc.h:694:1
export const LLVMOrcMaterializationResponsibilityGetRequestedSymbols = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Orc.h:702:6
export const LLVMOrcDisposeSymbols = {
  parameters: ["pointer"],
  result: "void"
} as const;

// ./llvm-c/Orc.h:720:14
export const LLVMOrcMaterializationResponsibilityNotifyResolved = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef, types.LLVMOrcCSymbolMapPairs, "u64"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:737:14
export const LLVMOrcMaterializationResponsibilityNotifyEmitted = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:753:14
export const LLVMOrcMaterializationResponsibilityDefineMaterializing = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef, types.LLVMOrcCSymbolFlagsMapPairs, "u64"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:764:6
export const LLVMOrcMaterializationResponsibilityFailMaterialization = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:774:14
export const LLVMOrcMaterializationResponsibilityReplace = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef, types.LLVMOrcMaterializationUnitRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:786:14
export const LLVMOrcMaterializationResponsibilityDelegate = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef, "pointer", "u64", "pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:809:6
export const LLVMOrcMaterializationResponsibilityAddDependencies = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef, types.LLVMOrcSymbolStringPoolEntryRef, types.LLVMOrcCDependenceMapPairs, "u64"],
  result: "void"
} as const;

// ./llvm-c/Orc.h:819:6
export const LLVMOrcMaterializationResponsibilityAddDependenciesForAll = {
  parameters: [types.LLVMOrcMaterializationResponsibilityRef, types.LLVMOrcCDependenceMapPairs, "u64"],
  result: "void"
} as const;

// ./llvm-c/Orc.h:833:1
export const LLVMOrcExecutionSessionCreateBareJITDylib = {
  parameters: [types.LLVMOrcExecutionSessionRef, "pointer"],
  result: types.LLVMOrcJITDylibRef
} as const;

// ./llvm-c/Orc.h:849:1
export const LLVMOrcExecutionSessionCreateJITDylib = {
  parameters: [types.LLVMOrcExecutionSessionRef, "pointer", "pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:858:1
export const LLVMOrcExecutionSessionGetJITDylibByName = {
  parameters: [types.LLVMOrcExecutionSessionRef, "pointer"],
  result: types.LLVMOrcJITDylibRef
} as const;

// ./llvm-c/Orc.h:867:1
export const LLVMOrcJITDylibCreateResourceTracker = {
  parameters: [types.LLVMOrcJITDylibRef],
  result: types.LLVMOrcResourceTrackerRef
} as const;

// ./llvm-c/Orc.h:875:1
export const LLVMOrcJITDylibGetDefaultResourceTracker = {
  parameters: [types.LLVMOrcJITDylibRef],
  result: types.LLVMOrcResourceTrackerRef
} as const;

// ./llvm-c/Orc.h:884:14
export const LLVMOrcJITDylibDefine = {
  parameters: [types.LLVMOrcJITDylibRef, types.LLVMOrcMaterializationUnitRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:891:14
export const LLVMOrcJITDylibClear = {
  parameters: [types.LLVMOrcJITDylibRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:899:6
export const LLVMOrcJITDylibAddGenerator = {
  parameters: [types.LLVMOrcJITDylibRef, types.LLVMOrcDefinitionGeneratorRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:905:31
export const LLVMOrcCreateCustomCAPIDefinitionGenerator = {
  parameters: [types.LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction, "pointer"],
  result: types.LLVMOrcDefinitionGeneratorRef
} as const;

// ./llvm-c/Orc.h:926:14
export const LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess = {
  parameters: ["pointer", "u8", types.LLVMOrcSymbolPredicate, "pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:951:14
export const LLVMOrcCreateDynamicLibrarySearchGeneratorForPath = {
  parameters: ["pointer", "pointer", "u8", types.LLVMOrcSymbolPredicate, "pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:969:14
export const LLVMOrcCreateStaticLibrarySearchGeneratorForPath = {
  parameters: ["pointer", types.LLVMOrcObjectLayerRef, "pointer", "pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:981:29
export const LLVMOrcCreateNewThreadSafeContext = {
  parameters: [],
  result: types.LLVMOrcThreadSafeContextRef
} as const;

// ./llvm-c/Orc.h:987:1
export const LLVMOrcThreadSafeContextGetContext = {
  parameters: [types.LLVMOrcThreadSafeContextRef],
  result: types.LLVMContextRef
} as const;

// ./llvm-c/Orc.h:992:6
export const LLVMOrcDisposeThreadSafeContext = {
  parameters: [types.LLVMOrcThreadSafeContextRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1005:1
export const LLVMOrcCreateNewThreadSafeModule = {
  parameters: [types.LLVMModuleRef, types.LLVMOrcThreadSafeContextRef],
  result: types.LLVMOrcThreadSafeModuleRef
} as const;

// ./llvm-c/Orc.h:1013:6
export const LLVMOrcDisposeThreadSafeModule = {
  parameters: [types.LLVMOrcThreadSafeModuleRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1019:1
export const LLVMOrcThreadSafeModuleWithModuleDo = {
  parameters: [types.LLVMOrcThreadSafeModuleRef, types.LLVMOrcGenericIRModuleOperationFunction, "pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:1031:14
export const LLVMOrcJITTargetMachineBuilderDetectHost = {
  parameters: ["pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:1044:1
export const LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine = {
  parameters: [types.LLVMTargetMachineRef],
  result: types.LLVMOrcJITTargetMachineBuilderRef
} as const;

// ./llvm-c/Orc.h:1049:6
export const LLVMOrcDisposeJITTargetMachineBuilder = {
  parameters: [types.LLVMOrcJITTargetMachineBuilderRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1058:7
export const LLVMOrcJITTargetMachineBuilderGetTargetTriple = {
  parameters: [types.LLVMOrcJITTargetMachineBuilderRef],
  result: "pointer"
} as const;

// ./llvm-c/Orc.h:1065:6
export const LLVMOrcJITTargetMachineBuilderSetTargetTriple = {
  parameters: [types.LLVMOrcJITTargetMachineBuilderRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1079:14
export const LLVMOrcObjectLayerAddObjectFile = {
  parameters: [types.LLVMOrcObjectLayerRef, types.LLVMOrcJITDylibRef, types.LLVMMemoryBufferRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:1105:6
export const LLVMOrcObjectLayerEmit = {
  parameters: [types.LLVMOrcObjectLayerRef, types.LLVMOrcMaterializationResponsibilityRef, types.LLVMMemoryBufferRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1112:6
export const LLVMOrcDisposeObjectLayer = {
  parameters: [types.LLVMOrcObjectLayerRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1114:6
export const LLVMOrcIRTransformLayerEmit = {
  parameters: [types.LLVMOrcIRTransformLayerRef, types.LLVMOrcMaterializationResponsibilityRef, types.LLVMOrcThreadSafeModuleRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1122:6
export const LLVMOrcIRTransformLayerSetTransform = {
  parameters: [types.LLVMOrcIRTransformLayerRef, types.LLVMOrcIRTransformLayerTransformFunction, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1129:6
export const LLVMOrcObjectTransformLayerSetTransform = {
  parameters: [types.LLVMOrcObjectTransformLayerRef, types.LLVMOrcObjectTransformLayerTransformFunction, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1140:1
export const LLVMOrcCreateLocalIndirectStubsManager = {
  parameters: ["pointer"],
  result: types.LLVMOrcIndirectStubsManagerRef
} as const;

// ./llvm-c/Orc.h:1145:6
export const LLVMOrcDisposeIndirectStubsManager = {
  parameters: [types.LLVMOrcIndirectStubsManagerRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1147:14
export const LLVMOrcCreateLocalLazyCallThroughManager = {
  parameters: ["pointer", types.LLVMOrcExecutionSessionRef, types.LLVMOrcJITTargetAddress, "pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Orc.h:1155:6
export const LLVMOrcDisposeLazyCallThroughManager = {
  parameters: [types.LLVMOrcLazyCallThroughManagerRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1172:23
export const LLVMOrcCreateDumpObjects = {
  parameters: ["pointer", "pointer"],
  result: types.LLVMOrcDumpObjectsRef
} as const;

// ./llvm-c/Orc.h:1178:6
export const LLVMOrcDisposeDumpObjects = {
  parameters: [types.LLVMOrcDumpObjectsRef],
  result: "void"
} as const;

// ./llvm-c/Orc.h:1183:14
export const LLVMOrcDumpObjects_CallOperator = {
  parameters: [types.LLVMOrcDumpObjectsRef, "pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/OrcEE.h:47:1
export const LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager = {
  parameters: [types.LLVMOrcExecutionSessionRef],
  result: types.LLVMOrcObjectLayerRef
} as const;

// ./llvm-c/OrcEE.h:56:6
export const LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener = {
  parameters: [types.LLVMOrcObjectLayerRef, types.LLVMJITEventListenerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:32:27
export const LLVMPassManagerBuilderCreate = {
  parameters: [],
  result: types.LLVMPassManagerBuilderRef
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:33:6
export const LLVMPassManagerBuilderDispose = {
  parameters: [types.LLVMPassManagerBuilderRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:37:1
export const LLVMPassManagerBuilderSetOptLevel = {
  parameters: [types.LLVMPassManagerBuilderRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:42:1
export const LLVMPassManagerBuilderSetSizeLevel = {
  parameters: [types.LLVMPassManagerBuilderRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:47:1
export const LLVMPassManagerBuilderSetDisableUnitAtATime = {
  parameters: [types.LLVMPassManagerBuilderRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:52:1
export const LLVMPassManagerBuilderSetDisableUnrollLoops = {
  parameters: [types.LLVMPassManagerBuilderRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:57:1
export const LLVMPassManagerBuilderSetDisableSimplifyLibCalls = {
  parameters: [types.LLVMPassManagerBuilderRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:62:1
export const LLVMPassManagerBuilderUseInlinerWithThreshold = {
  parameters: [types.LLVMPassManagerBuilderRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:67:1
export const LLVMPassManagerBuilderPopulateFunctionPassManager = {
  parameters: [types.LLVMPassManagerBuilderRef, types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:72:1
export const LLVMPassManagerBuilderPopulateModulePassManager = {
  parameters: [types.LLVMPassManagerBuilderRef, types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Utils.h:35:6
export const LLVMAddLowerSwitchPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Utils.h:38:6
export const LLVMAddPromoteMemoryToRegisterPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Utils.h:41:6
export const LLVMAddAddDiscriminatorsPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:31:6
export const LLVMAddConstantMergePass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:34:6
export const LLVMAddMergeFunctionsPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:37:6
export const LLVMAddCalledValuePropagationPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:40:6
export const LLVMAddDeadArgEliminationPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:43:6
export const LLVMAddFunctionAttrsPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:46:6
export const LLVMAddFunctionInliningPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:49:6
export const LLVMAddAlwaysInlinerPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:52:6
export const LLVMAddGlobalDCEPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:55:6
export const LLVMAddGlobalOptimizerPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:58:6
export const LLVMAddPruneEHPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:61:6
export const LLVMAddIPSCCPPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:64:6
export const LLVMAddInternalizePass = {
  parameters: [types.LLVMPassManagerRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:76:6
export const LLVMAddInternalizePassWithMustPreservePredicate = {
  parameters: [types.LLVMPassManagerRef, "pointer", "function"],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:82:6
export const LLVMAddStripDeadPrototypesPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:85:6
export const LLVMAddStripSymbolsPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:49:14
export const LLVMRunPasses = {
  parameters: [types.LLVMModuleRef, "pointer", types.LLVMTargetMachineRef, types.LLVMPassBuilderOptionsRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/Transforms/PassBuilder.h:60:27
export const LLVMCreatePassBuilderOptions = {
  parameters: [],
  result: types.LLVMPassBuilderOptionsRef
} as const;

// ./llvm-c/Transforms/PassBuilder.h:66:6
export const LLVMPassBuilderOptionsSetVerifyEach = {
  parameters: [types.LLVMPassBuilderOptionsRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:72:6
export const LLVMPassBuilderOptionsSetDebugLogging = {
  parameters: [types.LLVMPassBuilderOptionsRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:75:6
export const LLVMPassBuilderOptionsSetLoopInterleaving = {
  parameters: [types.LLVMPassBuilderOptionsRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:78:6
export const LLVMPassBuilderOptionsSetLoopVectorization = {
  parameters: [types.LLVMPassBuilderOptionsRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:81:6
export const LLVMPassBuilderOptionsSetSLPVectorization = {
  parameters: [types.LLVMPassBuilderOptionsRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:84:6
export const LLVMPassBuilderOptionsSetLoopUnrolling = {
  parameters: [types.LLVMPassBuilderOptionsRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:87:6
export const LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll = {
  parameters: [types.LLVMPassBuilderOptionsRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:90:6
export const LLVMPassBuilderOptionsSetLicmMssaOptCap = {
  parameters: [types.LLVMPassBuilderOptionsRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:93:6
export const LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap = {
  parameters: [types.LLVMPassBuilderOptionsRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:96:6
export const LLVMPassBuilderOptionsSetCallGraphProfile = {
  parameters: [types.LLVMPassBuilderOptionsRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:99:6
export const LLVMPassBuilderOptionsSetMergeFunctions = {
  parameters: [types.LLVMPassBuilderOptionsRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:105:6
export const LLVMDisposePassBuilderOptions = {
  parameters: [types.LLVMPassBuilderOptionsRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/AggressiveInstCombine.h:31:6
export const LLVMAddAggressiveInstCombinerPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:35:6
export const LLVMAddAggressiveDCEPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:38:6
export const LLVMAddDCEPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:41:6
export const LLVMAddBitTrackingDCEPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:44:6
export const LLVMAddAlignmentFromAssumptionsPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:47:6
export const LLVMAddCFGSimplificationPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:50:6
export const LLVMAddDeadStoreEliminationPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:53:6
export const LLVMAddScalarizerPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:56:6
export const LLVMAddMergedLoadStoreMotionPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:59:6
export const LLVMAddGVNPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:62:6
export const LLVMAddNewGVNPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:65:6
export const LLVMAddIndVarSimplifyPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:68:6
export const LLVMAddInstructionCombiningPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:71:6
export const LLVMAddInstructionSimplifyPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:74:6
export const LLVMAddJumpThreadingPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:77:6
export const LLVMAddLICMPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:80:6
export const LLVMAddLoopDeletionPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:83:6
export const LLVMAddLoopIdiomPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:86:6
export const LLVMAddLoopRotatePass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:89:6
export const LLVMAddLoopRerollPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:92:6
export const LLVMAddLoopUnrollPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:95:6
export const LLVMAddLoopUnrollAndJamPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:98:6
export const LLVMAddLowerAtomicPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:101:6
export const LLVMAddMemCpyOptPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:104:6
export const LLVMAddPartiallyInlineLibCallsPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:107:6
export const LLVMAddReassociatePass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:110:6
export const LLVMAddSCCPPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:113:6
export const LLVMAddScalarReplAggregatesPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:116:6
export const LLVMAddScalarReplAggregatesPassSSA = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:119:6
export const LLVMAddScalarReplAggregatesPassWithThreshold = {
  parameters: [types.LLVMPassManagerRef, "i32"],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:123:6
export const LLVMAddSimplifyLibCallsPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:126:6
export const LLVMAddTailCallEliminationPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:129:6
export const LLVMAddDemoteMemoryToRegisterPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:132:6
export const LLVMAddVerifierPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:135:6
export const LLVMAddCorrelatedValuePropagationPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:138:6
export const LLVMAddEarlyCSEPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:141:6
export const LLVMAddEarlyCSEMemSSAPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:144:6
export const LLVMAddLowerExpectIntrinsicPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:147:6
export const LLVMAddLowerConstantIntrinsicsPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:150:6
export const LLVMAddTypeBasedAliasAnalysisPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:153:6
export const LLVMAddScopedNoAliasAAPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:156:6
export const LLVMAddBasicAliasAnalysisPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:159:6
export const LLVMAddUnifyFunctionExitNodesPass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Vectorize.h:36:6
export const LLVMAddLoopVectorizePass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Transforms/Vectorize.h:39:6
export const LLVMAddSLPVectorizePass = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Linker.h:41:10
export const LLVMLinkModules2 = {
  parameters: [types.LLVMModuleRef, types.LLVMModuleRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Remarks.h:64:20
export const LLVMRemarkStringGetData = {
  parameters: [types.LLVMRemarkStringRef],
  result: "pointer"
} as const;

// ./llvm-c/Remarks.h:71:17
export const LLVMRemarkStringGetLen = {
  parameters: [types.LLVMRemarkStringRef],
  result: "u32"
} as const;

// ./llvm-c/Remarks.h:86:1
export const LLVMRemarkDebugLocGetSourceFilePath = {
  parameters: [types.LLVMRemarkDebugLocRef],
  result: types.LLVMRemarkStringRef
} as const;

// ./llvm-c/Remarks.h:93:17
export const LLVMRemarkDebugLocGetSourceLine = {
  parameters: [types.LLVMRemarkDebugLocRef],
  result: "u32"
} as const;

// ./llvm-c/Remarks.h:100:17
export const LLVMRemarkDebugLocGetSourceColumn = {
  parameters: [types.LLVMRemarkDebugLocRef],
  result: "u32"
} as const;

// ./llvm-c/Remarks.h:117:28
export const LLVMRemarkArgGetKey = {
  parameters: [types.LLVMRemarkArgRef],
  result: types.LLVMRemarkStringRef
} as const;

// ./llvm-c/Remarks.h:124:28
export const LLVMRemarkArgGetValue = {
  parameters: [types.LLVMRemarkArgRef],
  result: types.LLVMRemarkStringRef
} as const;

// ./llvm-c/Remarks.h:133:30
export const LLVMRemarkArgGetDebugLoc = {
  parameters: [types.LLVMRemarkArgRef],
  result: types.LLVMRemarkDebugLocRef
} as const;

// ./llvm-c/Remarks.h:147:13
export const LLVMRemarkEntryDispose = {
  parameters: [types.LLVMRemarkEntryRef],
  result: "void"
} as const;

// ./llvm-c/Remarks.h:155:28
export const LLVMRemarkEntryGetType = {
  parameters: [types.LLVMRemarkEntryRef],
  result: "i32"
} as const;

// ./llvm-c/Remarks.h:163:1
export const LLVMRemarkEntryGetPassName = {
  parameters: [types.LLVMRemarkEntryRef],
  result: types.LLVMRemarkStringRef
} as const;

// ./llvm-c/Remarks.h:171:1
export const LLVMRemarkEntryGetRemarkName = {
  parameters: [types.LLVMRemarkEntryRef],
  result: types.LLVMRemarkStringRef
} as const;

// ./llvm-c/Remarks.h:179:1
export const LLVMRemarkEntryGetFunctionName = {
  parameters: [types.LLVMRemarkEntryRef],
  result: types.LLVMRemarkStringRef
} as const;

// ./llvm-c/Remarks.h:189:1
export const LLVMRemarkEntryGetDebugLoc = {
  parameters: [types.LLVMRemarkEntryRef],
  result: types.LLVMRemarkDebugLocRef
} as const;

// ./llvm-c/Remarks.h:198:17
export const LLVMRemarkEntryGetHotness = {
  parameters: [types.LLVMRemarkEntryRef],
  result: "u64"
} as const;

// ./llvm-c/Remarks.h:205:17
export const LLVMRemarkEntryGetNumArgs = {
  parameters: [types.LLVMRemarkEntryRef],
  result: "u32"
} as const;

// ./llvm-c/Remarks.h:216:25
export const LLVMRemarkEntryGetFirstArg = {
  parameters: [types.LLVMRemarkEntryRef],
  result: types.LLVMRemarkArgRef
} as const;

// ./llvm-c/Remarks.h:227:25
export const LLVMRemarkEntryGetNextArg = {
  parameters: [types.LLVMRemarkArgRef, types.LLVMRemarkEntryRef],
  result: types.LLVMRemarkArgRef
} as const;

// ./llvm-c/Remarks.h:243:28
export const LLVMRemarkParserCreateYAML = {
  parameters: ["pointer", "u64"],
  result: types.LLVMRemarkParserRef
} as const;

// ./llvm-c/Remarks.h:257:28
export const LLVMRemarkParserCreateBitstream = {
  parameters: ["pointer", "u64"],
  result: types.LLVMRemarkParserRef
} as const;

// ./llvm-c/Remarks.h:302:27
export const LLVMRemarkParserGetNext = {
  parameters: [types.LLVMRemarkParserRef],
  result: types.LLVMRemarkEntryRef
} as const;

// ./llvm-c/Remarks.h:309:17
export const LLVMRemarkParserHasError = {
  parameters: [types.LLVMRemarkParserRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Remarks.h:322:20
export const LLVMRemarkParserGetErrorMessage = {
  parameters: [types.LLVMRemarkParserRef],
  result: "pointer"
} as const;

// ./llvm-c/Remarks.h:329:13
export const LLVMRemarkParserDispose = {
  parameters: [types.LLVMRemarkParserRef],
  result: "void"
} as const;

// ./llvm-c/ErrorHandling.h:36:6
export const LLVMInstallFatalErrorHandler = {
  parameters: [types.LLVMFatalErrorHandler],
  result: "void"
} as const;

// ./llvm-c/ErrorHandling.h:42:6
export const LLVMResetFatalErrorHandler = {
  parameters: [],
  result: "void"
} as const;

// ./llvm-c/ErrorHandling.h:49:6
export const LLVMEnablePrettyStackTrace = {
  parameters: [],
  result: "void"
} as const;

// ./llvm-c/Core.h:475:6
export const LLVMInitializeCore = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:480:6
export const LLVMShutdown = {
  parameters: [],
  result: "void"
} as const;

// ./llvm-c/Core.h:484:7
export const LLVMCreateMessage = {
  parameters: ["pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:485:6
export const LLVMDisposeMessage = {
  parameters: ["pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:508:16
export const LLVMContextCreate = {
  parameters: [],
  result: types.LLVMContextRef
} as const;

// ./llvm-c/Core.h:513:16
export const LLVMGetGlobalContext = {
  parameters: [],
  result: types.LLVMContextRef
} as const;

// ./llvm-c/Core.h:518:6
export const LLVMContextSetDiagnosticHandler = {
  parameters: [types.LLVMContextRef, types.LLVMDiagnosticHandler, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:525:23
export const LLVMContextGetDiagnosticHandler = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMDiagnosticHandler
} as const;

// ./llvm-c/Core.h:530:7
export const LLVMContextGetDiagnosticContext = {
  parameters: [types.LLVMContextRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:537:6
export const LLVMContextSetYieldCallback = {
  parameters: [types.LLVMContextRef, types.LLVMYieldCallback, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:545:10
export const LLVMContextShouldDiscardValueNames = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:555:6
export const LLVMContextSetDiscardValueNames = {
  parameters: [types.LLVMContextRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:562:6
export const LLVMContextSetOpaquePointers = {
  parameters: [types.LLVMContextRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:570:6
export const LLVMContextDispose = {
  parameters: [types.LLVMContextRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:578:7
export const LLVMGetDiagInfoDescription = {
  parameters: [types.LLVMDiagnosticInfoRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:585:24
export const LLVMGetDiagInfoSeverity = {
  parameters: [types.LLVMDiagnosticInfoRef],
  result: types.LLVMDiagnosticSeverity
} as const;

// ./llvm-c/Core.h:587:10
export const LLVMGetMDKindIDInContext = {
  parameters: [types.LLVMContextRef, "pointer", "u32"],
  result: "u32"
} as const;

// ./llvm-c/Core.h:589:10
export const LLVMGetMDKindID = {
  parameters: ["pointer", "u32"],
  result: "u32"
} as const;

// ./llvm-c/Core.h:602:10
export const LLVMGetEnumAttributeKindForName = {
  parameters: ["pointer", "u64"],
  result: "u32"
} as const;

// ./llvm-c/Core.h:603:10
export const LLVMGetLastEnumAttributeKind = {
  parameters: [],
  result: "u32"
} as const;

// ./llvm-c/Core.h:608:18
export const LLVMCreateEnumAttribute = {
  parameters: [types.LLVMContextRef, "u32", "u64"],
  result: types.LLVMAttributeRef
} as const;

// ./llvm-c/Core.h:615:10
export const LLVMGetEnumAttributeKind = {
  parameters: [types.LLVMAttributeRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:620:10
export const LLVMGetEnumAttributeValue = {
  parameters: [types.LLVMAttributeRef],
  result: "u64"
} as const;

// ./llvm-c/Core.h:625:18
export const LLVMCreateTypeAttribute = {
  parameters: [types.LLVMContextRef, "u32", types.LLVMTypeRef],
  result: types.LLVMAttributeRef
} as const;

// ./llvm-c/Core.h:631:13
export const LLVMGetTypeAttributeValue = {
  parameters: [types.LLVMAttributeRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:636:18
export const LLVMCreateStringAttribute = {
  parameters: [types.LLVMContextRef, "pointer", "u32", "pointer", "u32"],
  result: types.LLVMAttributeRef
} as const;

// ./llvm-c/Core.h:643:13
export const LLVMGetStringAttributeKind = {
  parameters: [types.LLVMAttributeRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:648:13
export const LLVMGetStringAttributeValue = {
  parameters: [types.LLVMAttributeRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:653:10
export const LLVMIsEnumAttribute = {
  parameters: [types.LLVMAttributeRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:654:10
export const LLVMIsStringAttribute = {
  parameters: [types.LLVMAttributeRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:655:10
export const LLVMIsTypeAttribute = {
  parameters: [types.LLVMAttributeRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:660:13
export const LLVMGetTypeByName2 = {
  parameters: [types.LLVMContextRef, "pointer"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:685:15
export const LLVMModuleCreateWithName = {
  parameters: ["pointer"],
  result: types.LLVMModuleRef
} as const;

// ./llvm-c/Core.h:693:15
export const LLVMModuleCreateWithNameInContext = {
  parameters: ["pointer", types.LLVMContextRef],
  result: types.LLVMModuleRef
} as const;

// ./llvm-c/Core.h:698:15
export const LLVMCloneModule = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMModuleRef
} as const;

// ./llvm-c/Core.h:706:6
export const LLVMDisposeModule = {
  parameters: [types.LLVMModuleRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:716:13
export const LLVMGetModuleIdentifier = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:726:6
export const LLVMSetModuleIdentifier = {
  parameters: [types.LLVMModuleRef, "pointer", "u64"],
  result: "void"
} as const;

// ./llvm-c/Core.h:736:13
export const LLVMGetSourceFileName = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:747:6
export const LLVMSetSourceFileName = {
  parameters: [types.LLVMModuleRef, "pointer", "u64"],
  result: "void"
} as const;

// ./llvm-c/Core.h:758:13
export const LLVMGetDataLayoutStr = {
  parameters: [types.LLVMModuleRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:759:13
export const LLVMGetDataLayout = {
  parameters: [types.LLVMModuleRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:766:6
export const LLVMSetDataLayout = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:773:13
export const LLVMGetTarget = {
  parameters: [types.LLVMModuleRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:780:6
export const LLVMSetTarget = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:789:22
export const LLVMCopyModuleFlagsMetadata = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:794:6
export const LLVMDisposeModuleFlagsMetadata = {
  parameters: ["pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:802:1
export const LLVMModuleFlagEntriesGetFlagBehavior = {
  parameters: ["pointer", "u32"],
  result: types.LLVMModuleFlagBehavior
} as const;

// ./llvm-c/Core.h:810:13
export const LLVMModuleFlagEntriesGetKey = {
  parameters: ["pointer", "u32", "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:818:17
export const LLVMModuleFlagEntriesGetMetadata = {
  parameters: ["pointer", "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/Core.h:827:17
export const LLVMGetModuleFlag = {
  parameters: [types.LLVMModuleRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/Core.h:836:6
export const LLVMAddModuleFlag = {
  parameters: [types.LLVMModuleRef, types.LLVMModuleFlagBehavior, "pointer", "u64", types.LLVMMetadataRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:845:6
export const LLVMDumpModule = {
  parameters: [types.LLVMModuleRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:853:10
export const LLVMPrintModuleToFile = {
  parameters: [types.LLVMModuleRef, "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:862:7
export const LLVMPrintModuleToString = {
  parameters: [types.LLVMModuleRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:869:13
export const LLVMGetModuleInlineAsm = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:876:6
export const LLVMSetModuleInlineAsm2 = {
  parameters: [types.LLVMModuleRef, "pointer", "u64"],
  result: "void"
} as const;

// ./llvm-c/Core.h:883:6
export const LLVMAppendModuleInlineAsm = {
  parameters: [types.LLVMModuleRef, "pointer", "u64"],
  result: "void"
} as const;

// ./llvm-c/Core.h:890:14
export const LLVMGetInlineAsm = {
  parameters: [types.LLVMTypeRef, "pointer", "u64", "pointer", "u64", types.LLVMBool, types.LLVMBool, types.LLVMInlineAsmDialect, types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:901:16
export const LLVMGetModuleContext = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMContextRef
} as const;

// ./llvm-c/Core.h:904:13
export const LLVMGetTypeByName = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:911:20
export const LLVMGetFirstNamedMetadata = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMNamedMDNodeRef
} as const;

// ./llvm-c/Core.h:918:20
export const LLVMGetLastNamedMetadata = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMNamedMDNodeRef
} as const;

// ./llvm-c/Core.h:926:20
export const LLVMGetNextNamedMetadata = {
  parameters: [types.LLVMNamedMDNodeRef],
  result: types.LLVMNamedMDNodeRef
} as const;

// ./llvm-c/Core.h:934:20
export const LLVMGetPreviousNamedMetadata = {
  parameters: [types.LLVMNamedMDNodeRef],
  result: types.LLVMNamedMDNodeRef
} as const;

// ./llvm-c/Core.h:942:20
export const LLVMGetNamedMetadata = {
  parameters: [types.LLVMModuleRef, "pointer", "u64"],
  result: types.LLVMNamedMDNodeRef
} as const;

// ./llvm-c/Core.h:951:20
export const LLVMGetOrInsertNamedMetadata = {
  parameters: [types.LLVMModuleRef, "pointer", "u64"],
  result: types.LLVMNamedMDNodeRef
} as const;

// ./llvm-c/Core.h:960:13
export const LLVMGetNamedMetadataName = {
  parameters: [types.LLVMNamedMDNodeRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:968:10
export const LLVMGetNamedMetadataNumOperands = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: "u32"
} as const;

// ./llvm-c/Core.h:981:6
export const LLVMGetNamedMetadataOperands = {
  parameters: [types.LLVMModuleRef, "pointer", "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:990:6
export const LLVMAddNamedMetadataOperand = {
  parameters: [types.LLVMModuleRef, "pointer", types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:1001:13
export const LLVMGetDebugLocDirectory = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:1011:13
export const LLVMGetDebugLocFilename = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:1021:10
export const LLVMGetDebugLocLine = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:1029:10
export const LLVMGetDebugLocColumn = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:1036:14
export const LLVMAddFunction = {
  parameters: [types.LLVMModuleRef, "pointer", types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1046:14
export const LLVMGetNamedFunction = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1053:14
export const LLVMGetFirstFunction = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1060:14
export const LLVMGetLastFunction = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1068:14
export const LLVMGetNextFunction = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1076:14
export const LLVMGetPreviousFunction = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1079:6
export const LLVMSetModuleInlineAsm = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:1119:14
export const LLVMGetTypeKind = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMTypeKind
} as const;

// ./llvm-c/Core.h:1128:10
export const LLVMTypeIsSized = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1135:16
export const LLVMGetTypeContext = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMContextRef
} as const;

// ./llvm-c/Core.h:1142:6
export const LLVMDumpType = {
  parameters: [types.LLVMTypeRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:1150:7
export const LLVMPrintTypeToString = {
  parameters: [types.LLVMTypeRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:1163:13
export const LLVMInt1TypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1164:13
export const LLVMInt8TypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1165:13
export const LLVMInt16TypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1166:13
export const LLVMInt32TypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1167:13
export const LLVMInt64TypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1168:13
export const LLVMInt128TypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1169:13
export const LLVMIntTypeInContext = {
  parameters: [types.LLVMContextRef, "u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1175:13
export const LLVMInt1Type = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1176:13
export const LLVMInt8Type = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1177:13
export const LLVMInt16Type = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1178:13
export const LLVMInt32Type = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1179:13
export const LLVMInt64Type = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1180:13
export const LLVMInt128Type = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1181:13
export const LLVMIntType = {
  parameters: ["u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1182:10
export const LLVMGetIntTypeWidth = {
  parameters: [types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:1197:13
export const LLVMHalfTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1202:13
export const LLVMBFloatTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1207:13
export const LLVMFloatTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1212:13
export const LLVMDoubleTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1217:13
export const LLVMX86FP80TypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1223:13
export const LLVMFP128TypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1228:13
export const LLVMPPCFP128TypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1235:13
export const LLVMHalfType = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1236:13
export const LLVMBFloatType = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1237:13
export const LLVMFloatType = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1238:13
export const LLVMDoubleType = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1239:13
export const LLVMX86FP80Type = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1240:13
export const LLVMFP128Type = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1241:13
export const LLVMPPCFP128Type = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1259:13
export const LLVMFunctionType = {
  parameters: [types.LLVMTypeRef, "pointer", "u32", types.LLVMBool],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1266:10
export const LLVMIsFunctionVarArg = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1271:13
export const LLVMGetReturnType = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1276:10
export const LLVMCountParamTypes = {
  parameters: [types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:1289:6
export const LLVMGetParamTypes = {
  parameters: [types.LLVMTypeRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:1313:13
export const LLVMStructTypeInContext = {
  parameters: [types.LLVMContextRef, "pointer", "u32", types.LLVMBool],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1321:13
export const LLVMStructType = {
  parameters: ["pointer", "u32", types.LLVMBool],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1329:13
export const LLVMStructCreateNamed = {
  parameters: [types.LLVMContextRef, "pointer"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1336:13
export const LLVMGetStructName = {
  parameters: [types.LLVMTypeRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:1343:6
export const LLVMStructSetBody = {
  parameters: [types.LLVMTypeRef, "pointer", "u32", types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:1351:10
export const LLVMCountStructElementTypes = {
  parameters: [types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:1363:6
export const LLVMGetStructElementTypes = {
  parameters: [types.LLVMTypeRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:1370:13
export const LLVMStructGetTypeAtIndex = {
  parameters: [types.LLVMTypeRef, "u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1377:10
export const LLVMIsPackedStruct = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1384:10
export const LLVMIsOpaqueStruct = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1391:10
export const LLVMIsLiteralStruct = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1413:13
export const LLVMGetElementType = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1420:6
export const LLVMGetSubtypes = {
  parameters: [types.LLVMTypeRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:1427:10
export const LLVMGetNumContainedTypes = {
  parameters: [types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:1437:13
export const LLVMArrayType = {
  parameters: [types.LLVMTypeRef, "u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1446:10
export const LLVMGetArrayLength = {
  parameters: [types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:1456:13
export const LLVMPointerType = {
  parameters: [types.LLVMTypeRef, "u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1465:10
export const LLVMPointerTypeIsOpaque = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1472:13
export const LLVMPointerTypeInContext = {
  parameters: [types.LLVMContextRef, "u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1481:10
export const LLVMGetPointerAddressSpace = {
  parameters: [types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:1492:13
export const LLVMVectorType = {
  parameters: [types.LLVMTypeRef, "u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1503:13
export const LLVMScalableVectorType = {
  parameters: [types.LLVMTypeRef, "u32"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1513:10
export const LLVMGetVectorSize = {
  parameters: [types.LLVMTypeRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:1528:13
export const LLVMVoidTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1533:13
export const LLVMLabelTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1538:13
export const LLVMX86MMXTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1543:13
export const LLVMX86AMXTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1548:13
export const LLVMTokenTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1553:13
export const LLVMMetadataTypeInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1559:13
export const LLVMVoidType = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1560:13
export const LLVMLabelType = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1561:13
export const LLVMX86MMXType = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1562:13
export const LLVMX86AMXType = {
  parameters: [],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1698:13
export const LLVMTypeOf = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:1705:15
export const LLVMGetValueKind = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueKind
} as const;

// ./llvm-c/Core.h:1712:13
export const LLVMGetValueName2 = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:1719:6
export const LLVMSetValueName2 = {
  parameters: [types.LLVMValueRef, "pointer", "u64"],
  result: "void"
} as const;

// ./llvm-c/Core.h:1726:6
export const LLVMDumpValue = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:1734:7
export const LLVMPrintValueToString = {
  parameters: [types.LLVMValueRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:1741:6
export const LLVMReplaceAllUsesWith = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:1746:10
export const LLVMIsConstant = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1751:10
export const LLVMIsUndef = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1756:10
export const LLVMIsPoison = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:67:1>
export const LLVMIsAArgument = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:68:1>
export const LLVMIsABasicBlock = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:69:1>
export const LLVMIsAInlineAsm = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:70:1>
export const LLVMIsAUser = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:71:1>
export const LLVMIsAConstant = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:72:1>
export const LLVMIsABlockAddress = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:73:1>
export const LLVMIsAConstantAggregateZero = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:74:1>
export const LLVMIsAConstantArray = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:75:1>
export const LLVMIsAConstantDataSequential = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:76:1>
export const LLVMIsAConstantDataArray = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:77:1>
export const LLVMIsAConstantDataVector = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:78:1>
export const LLVMIsAConstantExpr = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:79:1>
export const LLVMIsAConstantFP = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:80:1>
export const LLVMIsAConstantInt = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:81:1>
export const LLVMIsAConstantPointerNull = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:82:1>
export const LLVMIsAConstantStruct = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:83:1>
export const LLVMIsAConstantTokenNone = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:84:1>
export const LLVMIsAConstantVector = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:85:1>
export const LLVMIsAGlobalValue = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:86:1>
export const LLVMIsAGlobalAlias = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:87:1>
export const LLVMIsAGlobalObject = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:88:1>
export const LLVMIsAFunction = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:89:1>
export const LLVMIsAGlobalVariable = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:90:1>
export const LLVMIsAGlobalIFunc = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:91:1>
export const LLVMIsAUndefValue = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:92:1>
export const LLVMIsAPoisonValue = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:93:1>
export const LLVMIsAInstruction = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:94:1>
export const LLVMIsAUnaryOperator = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:95:1>
export const LLVMIsABinaryOperator = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:96:1>
export const LLVMIsACallInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:97:1>
export const LLVMIsAIntrinsicInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:98:1>
export const LLVMIsADbgInfoIntrinsic = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:99:1>
export const LLVMIsADbgVariableIntrinsic = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:100:1>
export const LLVMIsADbgDeclareInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:101:1>
export const LLVMIsADbgLabelInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:102:1>
export const LLVMIsAMemIntrinsic = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:103:1>
export const LLVMIsAMemCpyInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:104:1>
export const LLVMIsAMemMoveInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:105:1>
export const LLVMIsAMemSetInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:106:1>
export const LLVMIsACmpInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:107:1>
export const LLVMIsAFCmpInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:108:1>
export const LLVMIsAICmpInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:109:1>
export const LLVMIsAExtractElementInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:110:1>
export const LLVMIsAGetElementPtrInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:111:1>
export const LLVMIsAInsertElementInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:112:1>
export const LLVMIsAInsertValueInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:113:1>
export const LLVMIsALandingPadInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:114:1>
export const LLVMIsAPHINode = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:115:1>
export const LLVMIsASelectInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:116:1>
export const LLVMIsAShuffleVectorInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:117:1>
export const LLVMIsAStoreInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:118:1>
export const LLVMIsABranchInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:119:1>
export const LLVMIsAIndirectBrInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:120:1>
export const LLVMIsAInvokeInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:121:1>
export const LLVMIsAReturnInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:122:1>
export const LLVMIsASwitchInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:123:1>
export const LLVMIsAUnreachableInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:124:1>
export const LLVMIsAResumeInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:125:1>
export const LLVMIsACleanupReturnInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:126:1>
export const LLVMIsACatchReturnInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:127:1>
export const LLVMIsACatchSwitchInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:128:1>
export const LLVMIsACallBrInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:129:1>
export const LLVMIsAFuncletPadInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:130:1>
export const LLVMIsACatchPadInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:131:1>
export const LLVMIsACleanupPadInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:132:1>
export const LLVMIsAUnaryInstruction = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:133:1>
export const LLVMIsAAllocaInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:134:1>
export const LLVMIsACastInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:135:1>
export const LLVMIsAAddrSpaceCastInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:136:1>
export const LLVMIsABitCastInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:137:1>
export const LLVMIsAFPExtInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:138:1>
export const LLVMIsAFPToSIInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:139:1>
export const LLVMIsAFPToUIInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:140:1>
export const LLVMIsAFPTruncInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:141:1>
export const LLVMIsAIntToPtrInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:142:1>
export const LLVMIsAPtrToIntInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:143:1>
export const LLVMIsASExtInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:144:1>
export const LLVMIsASIToFPInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:145:1>
export const LLVMIsATruncInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:146:1>
export const LLVMIsAUIToFPInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:147:1>
export const LLVMIsAZExtInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:148:1>
export const LLVMIsAExtractValueInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:149:1>
export const LLVMIsALoadInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:150:1>
export const LLVMIsAVAArgInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:151:1>
export const LLVMIsAFreezeInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:152:1>
export const LLVMIsAAtomicCmpXchgInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:153:1>
export const LLVMIsAAtomicRMWInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:154:1>
export const LLVMIsAFenceInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1773:14
export const LLVMIsAMDNode = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1774:14
export const LLVMIsAMDString = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1777:13
export const LLVMGetValueName = {
  parameters: [types.LLVMValueRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:1779:6
export const LLVMSetValueName = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:1808:12
export const LLVMGetFirstUse = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMUseRef
} as const;

// ./llvm-c/Core.h:1816:12
export const LLVMGetNextUse = {
  parameters: [types.LLVMUseRef],
  result: types.LLVMUseRef
} as const;

// ./llvm-c/Core.h:1825:14
export const LLVMGetUser = {
  parameters: [types.LLVMUseRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1832:14
export const LLVMGetUsedValue = {
  parameters: [types.LLVMUseRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1853:14
export const LLVMGetOperand = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1860:12
export const LLVMGetOperandUse = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMUseRef
} as const;

// ./llvm-c/Core.h:1867:6
export const LLVMSetOperand = {
  parameters: [types.LLVMValueRef, "u32", types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:1874:5
export const LLVMGetNumOperands = {
  parameters: [types.LLVMValueRef],
  result: "i32"
} as const;

// ./llvm-c/Core.h:1897:14
export const LLVMConstNull = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1907:14
export const LLVMConstAllOnes = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1914:14
export const LLVMGetUndef = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1921:14
export const LLVMGetPoison = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1928:10
export const LLVMIsNull = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:1934:14
export const LLVMConstPointerNull = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1963:14
export const LLVMConstInt = {
  parameters: [types.LLVMTypeRef, "u64", types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1971:14
export const LLVMConstIntOfArbitraryPrecision = {
  parameters: [types.LLVMTypeRef, "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1984:14
export const LLVMConstIntOfString = {
  parameters: [types.LLVMTypeRef, "pointer", "u8"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1993:14
export const LLVMConstIntOfStringAndSize = {
  parameters: [types.LLVMTypeRef, "pointer", "u32", "u8"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:1999:14
export const LLVMConstReal = {
  parameters: [types.LLVMTypeRef, "i64"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2007:14
export const LLVMConstRealOfString = {
  parameters: [types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2012:14
export const LLVMConstRealOfStringAndSize = {
  parameters: [types.LLVMTypeRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2020:20
export const LLVMConstIntGetZExtValue = {
  parameters: [types.LLVMValueRef],
  result: "u64"
} as const;

// ./llvm-c/Core.h:2027:11
export const LLVMConstIntGetSExtValue = {
  parameters: [types.LLVMValueRef],
  result: "i64"
} as const;

// ./llvm-c/Core.h:2035:8
export const LLVMConstRealGetDouble = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "i64"
} as const;

// ./llvm-c/Core.h:2054:14
export const LLVMConstStringInContext = {
  parameters: [types.LLVMContextRef, "pointer", "u32", types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2066:14
export const LLVMConstString = {
  parameters: ["pointer", "u32", types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2074:10
export const LLVMIsConstantString = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:2081:13
export const LLVMGetAsString = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:2088:14
export const LLVMConstStructInContext = {
  parameters: [types.LLVMContextRef, "pointer", "u32", types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2100:14
export const LLVMConstStruct = {
  parameters: ["pointer", "u32", types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2108:14
export const LLVMConstArray = {
  parameters: [types.LLVMTypeRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2116:14
export const LLVMConstNamedStruct = {
  parameters: [types.LLVMTypeRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2128:14
export const LLVMGetAggregateElement = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2135:1 <Spelling=/data/./llvm-c/Core.h:2136:18>
export const LLVMGetElementAsConstant = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2144:14
export const LLVMConstVector = {
  parameters: ["pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2159:12
export const LLVMGetConstOpcode = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMOpcode
} as const;

// ./llvm-c/Core.h:2160:14
export const LLVMAlignOf = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2161:14
export const LLVMSizeOf = {
  parameters: [types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2162:14
export const LLVMConstNeg = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2163:14
export const LLVMConstNSWNeg = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2164:14
export const LLVMConstNUWNeg = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2165:14
export const LLVMConstFNeg = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2166:14
export const LLVMConstNot = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2167:14
export const LLVMConstAdd = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2168:14
export const LLVMConstNSWAdd = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2169:14
export const LLVMConstNUWAdd = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2170:14
export const LLVMConstSub = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2171:14
export const LLVMConstNSWSub = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2172:14
export const LLVMConstNUWSub = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2173:14
export const LLVMConstMul = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2174:14
export const LLVMConstNSWMul = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2175:14
export const LLVMConstNUWMul = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2176:14
export const LLVMConstAnd = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2177:14
export const LLVMConstOr = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2178:14
export const LLVMConstXor = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2179:14
export const LLVMConstICmp = {
  parameters: [types.LLVMIntPredicate, types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2181:14
export const LLVMConstFCmp = {
  parameters: [types.LLVMRealPredicate, types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2183:14
export const LLVMConstShl = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2184:14
export const LLVMConstLShr = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2185:14
export const LLVMConstAShr = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2186:1 <Spelling=/data/./llvm-c/Core.h:2187:18>
export const LLVMConstGEP = {
  parameters: [types.LLVMValueRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2191:14
export const LLVMConstGEP2 = {
  parameters: [types.LLVMTypeRef, types.LLVMValueRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2193:1 <Spelling=/data/./llvm-c/Core.h:2194:18>
export const LLVMConstInBoundsGEP = {
  parameters: [types.LLVMValueRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2198:14
export const LLVMConstInBoundsGEP2 = {
  parameters: [types.LLVMTypeRef, types.LLVMValueRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2201:14
export const LLVMConstTrunc = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2202:14
export const LLVMConstSExt = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2203:14
export const LLVMConstZExt = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2204:14
export const LLVMConstFPTrunc = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2205:14
export const LLVMConstFPExt = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2206:14
export const LLVMConstUIToFP = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2207:14
export const LLVMConstSIToFP = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2208:14
export const LLVMConstFPToUI = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2209:14
export const LLVMConstFPToSI = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2210:14
export const LLVMConstPtrToInt = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2211:14
export const LLVMConstIntToPtr = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2212:14
export const LLVMConstBitCast = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2213:14
export const LLVMConstAddrSpaceCast = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2214:14
export const LLVMConstZExtOrBitCast = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2216:14
export const LLVMConstSExtOrBitCast = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2218:14
export const LLVMConstTruncOrBitCast = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2220:14
export const LLVMConstPointerCast = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2222:14
export const LLVMConstIntCast = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef, types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2224:14
export const LLVMConstFPCast = {
  parameters: [types.LLVMValueRef, types.LLVMTypeRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2225:14
export const LLVMConstSelect = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2228:14
export const LLVMConstExtractElement = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2230:14
export const LLVMConstInsertElement = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2233:14
export const LLVMConstShuffleVector = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2236:14
export const LLVMBlockAddress = {
  parameters: [types.LLVMValueRef, types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2239:14
export const LLVMConstInlineAsm = {
  parameters: [types.LLVMTypeRef, "pointer", "pointer", types.LLVMBool, types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2258:15
export const LLVMGetGlobalParent = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMModuleRef
} as const;

// ./llvm-c/Core.h:2259:10
export const LLVMIsDeclaration = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:2260:13
export const LLVMGetLinkage = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMLinkage
} as const;

// ./llvm-c/Core.h:2261:6
export const LLVMSetLinkage = {
  parameters: [types.LLVMValueRef, types.LLVMLinkage],
  result: "void"
} as const;

// ./llvm-c/Core.h:2262:13
export const LLVMGetSection = {
  parameters: [types.LLVMValueRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:2263:6
export const LLVMSetSection = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2264:16
export const LLVMGetVisibility = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMVisibility
} as const;

// ./llvm-c/Core.h:2265:6
export const LLVMSetVisibility = {
  parameters: [types.LLVMValueRef, types.LLVMVisibility],
  result: "void"
} as const;

// ./llvm-c/Core.h:2266:21
export const LLVMGetDLLStorageClass = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMDLLStorageClass
} as const;

// ./llvm-c/Core.h:2267:6
export const LLVMSetDLLStorageClass = {
  parameters: [types.LLVMValueRef, types.LLVMDLLStorageClass],
  result: "void"
} as const;

// ./llvm-c/Core.h:2268:17
export const LLVMGetUnnamedAddress = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMUnnamedAddr
} as const;

// ./llvm-c/Core.h:2269:6
export const LLVMSetUnnamedAddress = {
  parameters: [types.LLVMValueRef, types.LLVMUnnamedAddr],
  result: "void"
} as const;

// ./llvm-c/Core.h:2277:13
export const LLVMGlobalGetValueType = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:2280:10
export const LLVMHasUnnamedAddr = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:2282:6
export const LLVMSetUnnamedAddr = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:2300:10
export const LLVMGetAlignment = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:2311:6
export const LLVMSetAlignment = {
  parameters: [types.LLVMValueRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2319:6
export const LLVMGlobalSetMetadata = {
  parameters: [types.LLVMValueRef, "u32", types.LLVMMetadataRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2327:6
export const LLVMGlobalEraseMetadata = {
  parameters: [types.LLVMValueRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2334:6
export const LLVMGlobalClearMetadata = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2343:25
export const LLVMGlobalCopyAllMetadata = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:2349:6
export const LLVMDisposeValueMetadataEntries = {
  parameters: ["pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2354:10
export const LLVMValueMetadataEntriesGetKind = {
  parameters: ["pointer", "u32"],
  result: "u32"
} as const;

// ./llvm-c/Core.h:2362:1
export const LLVMValueMetadataEntriesGetMetadata = {
  parameters: ["pointer", "u32"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/Core.h:2378:14
export const LLVMAddGlobal = {
  parameters: [types.LLVMModuleRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2379:14
export const LLVMAddGlobalInAddressSpace = {
  parameters: [types.LLVMModuleRef, types.LLVMTypeRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2382:14
export const LLVMGetNamedGlobal = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2383:14
export const LLVMGetFirstGlobal = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2384:14
export const LLVMGetLastGlobal = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2385:14
export const LLVMGetNextGlobal = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2386:14
export const LLVMGetPreviousGlobal = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2387:6
export const LLVMDeleteGlobal = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2388:14
export const LLVMGetInitializer = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2389:6
export const LLVMSetInitializer = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2390:10
export const LLVMIsThreadLocal = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:2391:6
export const LLVMSetThreadLocal = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:2392:10
export const LLVMIsGlobalConstant = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:2393:6
export const LLVMSetGlobalConstant = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:2394:21
export const LLVMGetThreadLocalMode = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMThreadLocalMode
} as const;

// ./llvm-c/Core.h:2395:6
export const LLVMSetThreadLocalMode = {
  parameters: [types.LLVMValueRef, types.LLVMThreadLocalMode],
  result: "void"
} as const;

// ./llvm-c/Core.h:2396:10
export const LLVMIsExternallyInitialized = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:2397:6
export const LLVMSetExternallyInitialized = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:2413:1 <Spelling=/data/./llvm-c/Core.h:2414:18>
export const LLVMAddAlias = {
  parameters: [types.LLVMModuleRef, types.LLVMTypeRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2423:14
export const LLVMAddAlias2 = {
  parameters: [types.LLVMModuleRef, types.LLVMTypeRef, "u32", types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2434:14
export const LLVMGetNamedGlobalAlias = {
  parameters: [types.LLVMModuleRef, "pointer", "u64"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2442:14
export const LLVMGetFirstGlobalAlias = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2449:14
export const LLVMGetLastGlobalAlias = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2457:14
export const LLVMGetNextGlobalAlias = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2465:14
export const LLVMGetPreviousGlobalAlias = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2470:14
export const LLVMAliasGetAliasee = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2475:6
export const LLVMAliasSetAliasee = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2497:6
export const LLVMDeleteFunction = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2504:10
export const LLVMHasPersonalityFn = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:2511:14
export const LLVMGetPersonalityFn = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2518:6
export const LLVMSetPersonalityFn = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2525:10
export const LLVMLookupIntrinsicID = {
  parameters: ["pointer", "u64"],
  result: "u32"
} as const;

// ./llvm-c/Core.h:2532:10
export const LLVMGetIntrinsicID = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:2540:14
export const LLVMGetIntrinsicDeclaration = {
  parameters: [types.LLVMModuleRef, "u32", "pointer", "u64"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2551:13
export const LLVMIntrinsicGetType = {
  parameters: [types.LLVMContextRef, "u32", "pointer", "u64"],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:2559:13
export const LLVMIntrinsicGetName = {
  parameters: ["u32", "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:2562:13
export const LLVMIntrinsicCopyOverloadedName = {
  parameters: ["u32", "pointer", "u64", "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:2578:13
export const LLVMIntrinsicCopyOverloadedName2 = {
  parameters: [types.LLVMModuleRef, "u32", "pointer", "u64", "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:2588:10
export const LLVMIntrinsicIsOverloaded = {
  parameters: ["u32"],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:2597:10
export const LLVMGetFunctionCallConv = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:2607:6
export const LLVMSetFunctionCallConv = {
  parameters: [types.LLVMValueRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2615:13
export const LLVMGetGC = {
  parameters: [types.LLVMValueRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:2622:6
export const LLVMSetGC = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2629:6
export const LLVMAddAttributeAtIndex = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, types.LLVMAttributeRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2631:10
export const LLVMGetAttributeCountAtIndex = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex],
  result: "u32"
} as const;

// ./llvm-c/Core.h:2632:6
export const LLVMGetAttributesAtIndex = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2634:18
export const LLVMGetEnumAttributeAtIndex = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "u32"],
  result: types.LLVMAttributeRef
} as const;

// ./llvm-c/Core.h:2637:18
export const LLVMGetStringAttributeAtIndex = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "pointer", "u32"],
  result: types.LLVMAttributeRef
} as const;

// ./llvm-c/Core.h:2640:6
export const LLVMRemoveEnumAttributeAtIndex = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2642:6
export const LLVMRemoveStringAttributeAtIndex = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "pointer", "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2649:6
export const LLVMAddTargetDependentFunctionAttr = {
  parameters: [types.LLVMValueRef, "pointer", "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2668:10
export const LLVMCountParams = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:2681:6
export const LLVMGetParams = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2690:14
export const LLVMGetParam = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2701:14
export const LLVMGetParamParent = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2708:14
export const LLVMGetFirstParam = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2715:14
export const LLVMGetLastParam = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2724:14
export const LLVMGetNextParam = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2731:14
export const LLVMGetPreviousParam = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2739:6
export const LLVMSetParamAlignment = {
  parameters: [types.LLVMValueRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2761:14
export const LLVMAddGlobalIFunc = {
  parameters: [types.LLVMModuleRef, "pointer", "u64", types.LLVMTypeRef, "u32", types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2773:14
export const LLVMGetNamedGlobalIFunc = {
  parameters: [types.LLVMModuleRef, "pointer", "u64"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2781:14
export const LLVMGetFirstGlobalIFunc = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2788:14
export const LLVMGetLastGlobalIFunc = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2796:14
export const LLVMGetNextGlobalIFunc = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2804:14
export const LLVMGetPreviousGlobalIFunc = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2812:14
export const LLVMGetGlobalIFuncResolver = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2819:6
export const LLVMSetGlobalIFuncResolver = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2826:6
export const LLVMEraseGlobalIFunc = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2836:6
export const LLVMRemoveGlobalIFunc = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:2868:17
export const LLVMMDStringInContext2 = {
  parameters: [types.LLVMContextRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/Core.h:2876:17
export const LLVMMDNodeInContext2 = {
  parameters: [types.LLVMContextRef, "pointer", "u64"],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/Core.h:2882:14
export const LLVMMetadataAsValue = {
  parameters: [types.LLVMContextRef, types.LLVMMetadataRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2887:17
export const LLVMValueAsMetadata = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/Core.h:2896:13
export const LLVMGetMDString = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:2904:10
export const LLVMGetMDNodeNumOperands = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:2917:6
export const LLVMGetMDNodeOperands = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:2920:14
export const LLVMMDStringInContext = {
  parameters: [types.LLVMContextRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2923:14
export const LLVMMDString = {
  parameters: ["pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2925:14
export const LLVMMDNodeInContext = {
  parameters: [types.LLVMContextRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2928:14
export const LLVMMDNode = {
  parameters: ["pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2954:14
export const LLVMBasicBlockAsValue = {
  parameters: [types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2959:10
export const LLVMValueIsBasicBlock = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:2964:19
export const LLVMValueAsBasicBlock = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:2969:13
export const LLVMGetBasicBlockName = {
  parameters: [types.LLVMBasicBlockRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:2976:14
export const LLVMGetBasicBlockParent = {
  parameters: [types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2988:14
export const LLVMGetBasicBlockTerminator = {
  parameters: [types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:2995:10
export const LLVMCountBasicBlocks = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:3005:6
export const LLVMGetBasicBlocks = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:3015:19
export const LLVMGetFirstBasicBlock = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3022:19
export const LLVMGetLastBasicBlock = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3027:19
export const LLVMGetNextBasicBlock = {
  parameters: [types.LLVMBasicBlockRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3032:19
export const LLVMGetPreviousBasicBlock = {
  parameters: [types.LLVMBasicBlockRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3040:19
export const LLVMGetEntryBasicBlock = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3049:6
export const LLVMInsertExistingBasicBlockAfterInsertBlock = {
  parameters: [types.LLVMBuilderRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3057:6
export const LLVMAppendExistingBasicBlock = {
  parameters: [types.LLVMValueRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3065:19
export const LLVMCreateBasicBlockInContext = {
  parameters: [types.LLVMContextRef, "pointer"],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3073:19
export const LLVMAppendBasicBlockInContext = {
  parameters: [types.LLVMContextRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3083:19
export const LLVMAppendBasicBlock = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3093:19
export const LLVMInsertBasicBlockInContext = {
  parameters: [types.LLVMContextRef, types.LLVMBasicBlockRef, "pointer"],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3102:19
export const LLVMInsertBasicBlock = {
  parameters: [types.LLVMBasicBlockRef, "pointer"],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3113:6
export const LLVMDeleteBasicBlock = {
  parameters: [types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3123:6
export const LLVMRemoveBasicBlockFromParent = {
  parameters: [types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3130:6
export const LLVMMoveBasicBlockBefore = {
  parameters: [types.LLVMBasicBlockRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3137:6
export const LLVMMoveBasicBlockAfter = {
  parameters: [types.LLVMBasicBlockRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3145:14
export const LLVMGetFirstInstruction = {
  parameters: [types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3152:14
export const LLVMGetLastInstruction = {
  parameters: [types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3178:5
export const LLVMHasMetadata = {
  parameters: [types.LLVMValueRef],
  result: "i32"
} as const;

// ./llvm-c/Core.h:3183:14
export const LLVMGetMetadata = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3188:6
export const LLVMSetMetadata = {
  parameters: [types.LLVMValueRef, "u32", types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3197:1
export const LLVMInstructionGetAllMetadataOtherThanDebugLoc = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:3205:19
export const LLVMGetInstructionParent = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3215:14
export const LLVMGetNextInstruction = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3223:14
export const LLVMGetPreviousInstruction = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3233:6
export const LLVMInstructionRemoveFromParent = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3243:6
export const LLVMInstructionEraseFromParent = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3253:6
export const LLVMDeleteInstruction = {
  parameters: [types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3260:12
export const LLVMGetInstructionOpcode = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMOpcode
} as const;

// ./llvm-c/Core.h:3270:18
export const LLVMGetICmpPredicate = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMIntPredicate
} as const;

// ./llvm-c/Core.h:3280:19
export const LLVMGetFCmpPredicate = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMRealPredicate
} as const;

// ./llvm-c/Core.h:3290:14
export const LLVMInstructionClone = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3299:14
export const LLVMIsATerminatorInst = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3321:10
export const LLVMGetNumArgOperands = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:3332:6
export const LLVMSetInstructionCallConv = {
  parameters: [types.LLVMValueRef, "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:3342:10
export const LLVMGetInstructionCallConv = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:3344:6
export const LLVMSetInstrParamAlignment = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:3347:6
export const LLVMAddCallSiteAttribute = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, types.LLVMAttributeRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3349:10
export const LLVMGetCallSiteAttributeCount = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex],
  result: "u32"
} as const;

// ./llvm-c/Core.h:3350:6
export const LLVMGetCallSiteAttributes = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:3352:18
export const LLVMGetCallSiteEnumAttribute = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "u32"],
  result: types.LLVMAttributeRef
} as const;

// ./llvm-c/Core.h:3355:18
export const LLVMGetCallSiteStringAttribute = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "pointer", "u32"],
  result: types.LLVMAttributeRef
} as const;

// ./llvm-c/Core.h:3358:6
export const LLVMRemoveCallSiteEnumAttribute = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:3360:6
export const LLVMRemoveCallSiteStringAttribute = {
  parameters: [types.LLVMValueRef, types.LLVMAttributeIndex, "pointer", "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:3368:13
export const LLVMGetCalledFunctionType = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:3379:14
export const LLVMGetCalledValue = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3388:10
export const LLVMIsTailCall = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:3397:6
export const LLVMSetTailCall = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:3406:19
export const LLVMGetNormalDest = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3418:19
export const LLVMGetUnwindDest = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3427:6
export const LLVMSetNormalDest = {
  parameters: [types.LLVMValueRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3439:6
export const LLVMSetUnwindDest = {
  parameters: [types.LLVMValueRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3459:10
export const LLVMGetNumSuccessors = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:3466:19
export const LLVMGetSuccessor = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3473:6
export const LLVMSetSuccessor = {
  parameters: [types.LLVMValueRef, "u32", types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3482:10
export const LLVMIsConditional = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:3491:14
export const LLVMGetCondition = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3500:6
export const LLVMSetCondition = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3509:19
export const LLVMGetSwitchDefaultDest = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3527:13
export const LLVMGetAllocatedType = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:3545:10
export const LLVMIsInBounds = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:3550:6
export const LLVMSetIsInBounds = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:3555:13
export const LLVMGetGEPSourceElementType = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMTypeRef
} as const;

// ./llvm-c/Core.h:3573:6
export const LLVMAddIncoming = {
  parameters: [types.LLVMValueRef, "pointer", "pointer", "u32"],
  result: "void"
} as const;

// ./llvm-c/Core.h:3579:10
export const LLVMCountIncoming = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:3584:14
export const LLVMGetIncomingValue = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3589:19
export const LLVMGetIncomingBlock = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3609:10
export const LLVMGetNumIndices = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:3614:17
export const LLVMGetIndices = {
  parameters: [types.LLVMValueRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:3637:16
export const LLVMCreateBuilderInContext = {
  parameters: [types.LLVMContextRef],
  result: types.LLVMBuilderRef
} as const;

// ./llvm-c/Core.h:3638:16
export const LLVMCreateBuilder = {
  parameters: [],
  result: types.LLVMBuilderRef
} as const;

// ./llvm-c/Core.h:3639:6
export const LLVMPositionBuilder = {
  parameters: [types.LLVMBuilderRef, types.LLVMBasicBlockRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3641:6
export const LLVMPositionBuilderBefore = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3642:6
export const LLVMPositionBuilderAtEnd = {
  parameters: [types.LLVMBuilderRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3643:19
export const LLVMGetInsertBlock = {
  parameters: [types.LLVMBuilderRef],
  result: types.LLVMBasicBlockRef
} as const;

// ./llvm-c/Core.h:3644:6
export const LLVMClearInsertionPosition = {
  parameters: [types.LLVMBuilderRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3645:6
export const LLVMInsertIntoBuilder = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3646:6
export const LLVMInsertIntoBuilderWithName = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:3648:6
export const LLVMDisposeBuilder = {
  parameters: [types.LLVMBuilderRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3657:17
export const LLVMGetCurrentDebugLocation2 = {
  parameters: [types.LLVMBuilderRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/Core.h:3666:6
export const LLVMSetCurrentDebugLocation2 = {
  parameters: [types.LLVMBuilderRef, types.LLVMMetadataRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3678:6
export const LLVMSetInstDebugLocation = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3685:6
export const LLVMAddMetadataToInst = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3692:17
export const LLVMBuilderGetDefaultFPMathTag = {
  parameters: [types.LLVMBuilderRef],
  result: types.LLVMMetadataRef
} as const;

// ./llvm-c/Core.h:3701:6
export const LLVMBuilderSetDefaultFPMathTag = {
  parameters: [types.LLVMBuilderRef, types.LLVMMetadataRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3708:6
export const LLVMSetCurrentDebugLocation = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3713:14
export const LLVMGetCurrentDebugLocation = {
  parameters: [types.LLVMBuilderRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3716:14
export const LLVMBuildRetVoid = {
  parameters: [types.LLVMBuilderRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3717:14
export const LLVMBuildRet = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3718:14
export const LLVMBuildAggregateRet = {
  parameters: [types.LLVMBuilderRef, "pointer", "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3720:14
export const LLVMBuildBr = {
  parameters: [types.LLVMBuilderRef, types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3721:14
export const LLVMBuildCondBr = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMBasicBlockRef, types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3723:14
export const LLVMBuildSwitch = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMBasicBlockRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3725:14
export const LLVMBuildIndirectBr = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3727:1 <Spelling=/data/./llvm-c/Core.h:3728:18>
export const LLVMBuildInvoke = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer", "u32", types.LLVMBasicBlockRef, types.LLVMBasicBlockRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3733:14
export const LLVMBuildInvoke2 = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, "pointer", "u32", types.LLVMBasicBlockRef, types.LLVMBasicBlockRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3737:14
export const LLVMBuildUnreachable = {
  parameters: [types.LLVMBuilderRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3740:14
export const LLVMBuildResume = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3741:14
export const LLVMBuildLandingPad = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3744:14
export const LLVMBuildCleanupRet = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3746:14
export const LLVMBuildCatchRet = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMBasicBlockRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3748:14
export const LLVMBuildCatchPad = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer", "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3751:14
export const LLVMBuildCleanupPad = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer", "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3754:14
export const LLVMBuildCatchSwitch = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMBasicBlockRef, "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3759:6
export const LLVMAddCase = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3763:6
export const LLVMAddDestination = {
  parameters: [types.LLVMValueRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3766:10
export const LLVMGetNumClauses = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:3769:14
export const LLVMGetClause = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3772:6
export const LLVMAddClause = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3775:10
export const LLVMIsCleanup = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:3778:6
export const LLVMSetCleanup = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:3781:6
export const LLVMAddHandler = {
  parameters: [types.LLVMValueRef, types.LLVMBasicBlockRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3784:10
export const LLVMGetNumHandlers = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:3797:6
export const LLVMGetHandlers = {
  parameters: [types.LLVMValueRef, "pointer"],
  result: "void"
} as const;

// ./llvm-c/Core.h:3802:14
export const LLVMGetArgOperand = {
  parameters: [types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3805:6
export const LLVMSetArgOperand = {
  parameters: [types.LLVMValueRef, "u32", types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3814:14
export const LLVMGetParentCatchSwitch = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3823:6
export const LLVMSetParentCatchSwitch = {
  parameters: [types.LLVMValueRef, types.LLVMValueRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:3826:14
export const LLVMBuildAdd = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3828:14
export const LLVMBuildNSWAdd = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3830:14
export const LLVMBuildNUWAdd = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3832:14
export const LLVMBuildFAdd = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3834:14
export const LLVMBuildSub = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3836:14
export const LLVMBuildNSWSub = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3838:14
export const LLVMBuildNUWSub = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3840:14
export const LLVMBuildFSub = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3842:14
export const LLVMBuildMul = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3844:14
export const LLVMBuildNSWMul = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3846:14
export const LLVMBuildNUWMul = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3848:14
export const LLVMBuildFMul = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3850:14
export const LLVMBuildUDiv = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3852:14
export const LLVMBuildExactUDiv = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3854:14
export const LLVMBuildSDiv = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3856:14
export const LLVMBuildExactSDiv = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3858:14
export const LLVMBuildFDiv = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3860:14
export const LLVMBuildURem = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3862:14
export const LLVMBuildSRem = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3864:14
export const LLVMBuildFRem = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3866:14
export const LLVMBuildShl = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3868:14
export const LLVMBuildLShr = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3870:14
export const LLVMBuildAShr = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3872:14
export const LLVMBuildAnd = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3874:14
export const LLVMBuildOr = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3876:14
export const LLVMBuildXor = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3878:14
export const LLVMBuildBinOp = {
  parameters: [types.LLVMBuilderRef, types.LLVMOpcode, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3881:14
export const LLVMBuildNeg = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3882:14
export const LLVMBuildNSWNeg = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3884:14
export const LLVMBuildNUWNeg = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3886:14
export const LLVMBuildFNeg = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3887:14
export const LLVMBuildNot = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3890:14
export const LLVMBuildMalloc = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3891:14
export const LLVMBuildArrayMalloc = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3900:14
export const LLVMBuildMemSet = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, types.LLVMValueRef, "u32"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3908:14
export const LLVMBuildMemCpy = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "u32", types.LLVMValueRef, "u32", types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3917:14
export const LLVMBuildMemMove = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "u32", types.LLVMValueRef, "u32", types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3922:14
export const LLVMBuildAlloca = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3923:14
export const LLVMBuildArrayAlloca = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3925:14
export const LLVMBuildFree = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3926:1 <Spelling=/data/./llvm-c/Core.h:3927:18>
export const LLVMBuildLoad = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3930:14
export const LLVMBuildLoad2 = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3932:14
export const LLVMBuildStore = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3933:1 <Spelling=/data/./llvm-c/Core.h:3934:18>
export const LLVMBuildGEP = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer", "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3938:1 <Spelling=/data/./llvm-c/Core.h:3939:18>
export const LLVMBuildInBoundsGEP = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer", "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3943:1 <Spelling=/data/./llvm-c/Core.h:3944:18>
export const LLVMBuildStructGEP = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3947:14
export const LLVMBuildGEP2 = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, "pointer", "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3950:14
export const LLVMBuildInBoundsGEP2 = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, "pointer", "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3953:14
export const LLVMBuildStructGEP2 = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3956:14
export const LLVMBuildGlobalString = {
  parameters: [types.LLVMBuilderRef, "pointer", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3958:14
export const LLVMBuildGlobalStringPtr = {
  parameters: [types.LLVMBuilderRef, "pointer", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3960:10
export const LLVMGetVolatile = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:3961:6
export const LLVMSetVolatile = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:3962:10
export const LLVMGetWeak = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:3963:6
export const LLVMSetWeak = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:3964:20
export const LLVMGetOrdering = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMAtomicOrdering
} as const;

// ./llvm-c/Core.h:3965:6
export const LLVMSetOrdering = {
  parameters: [types.LLVMValueRef, types.LLVMAtomicOrdering],
  result: "void"
} as const;

// ./llvm-c/Core.h:3966:20
export const LLVMGetAtomicRMWBinOp = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMAtomicRMWBinOp
} as const;

// ./llvm-c/Core.h:3967:6
export const LLVMSetAtomicRMWBinOp = {
  parameters: [types.LLVMValueRef, types.LLVMAtomicRMWBinOp],
  result: "void"
} as const;

// ./llvm-c/Core.h:3970:14
export const LLVMBuildTrunc = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3972:14
export const LLVMBuildZExt = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3974:14
export const LLVMBuildSExt = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3976:14
export const LLVMBuildFPToUI = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3978:14
export const LLVMBuildFPToSI = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3980:14
export const LLVMBuildUIToFP = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3982:14
export const LLVMBuildSIToFP = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3984:14
export const LLVMBuildFPTrunc = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3986:14
export const LLVMBuildFPExt = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3988:14
export const LLVMBuildPtrToInt = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3990:14
export const LLVMBuildIntToPtr = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3992:14
export const LLVMBuildBitCast = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3994:14
export const LLVMBuildAddrSpaceCast = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3996:14
export const LLVMBuildZExtOrBitCast = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:3998:14
export const LLVMBuildSExtOrBitCast = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4000:14
export const LLVMBuildTruncOrBitCast = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4002:14
export const LLVMBuildCast = {
  parameters: [types.LLVMBuilderRef, types.LLVMOpcode, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4004:14
export const LLVMBuildPointerCast = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4006:14
export const LLVMBuildIntCast2 = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, types.LLVMBool, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4009:14
export const LLVMBuildFPCast = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4013:14
export const LLVMBuildIntCast = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4016:12
export const LLVMGetCastOpcode = {
  parameters: [types.LLVMValueRef, types.LLVMBool, types.LLVMTypeRef, types.LLVMBool],
  result: types.LLVMOpcode
} as const;

// ./llvm-c/Core.h:4020:14
export const LLVMBuildICmp = {
  parameters: [types.LLVMBuilderRef, types.LLVMIntPredicate, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4023:14
export const LLVMBuildFCmp = {
  parameters: [types.LLVMBuilderRef, types.LLVMRealPredicate, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4028:14
export const LLVMBuildPhi = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4029:1 <Spelling=/data/./llvm-c/Core.h:4030:18>
export const LLVMBuildCall = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer", "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4034:14
export const LLVMBuildCall2 = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, "pointer", "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4037:14
export const LLVMBuildSelect = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4040:14
export const LLVMBuildVAArg = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMTypeRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4042:14
export const LLVMBuildExtractElement = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4044:14
export const LLVMBuildInsertElement = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4047:14
export const LLVMBuildShuffleVector = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4050:14
export const LLVMBuildExtractValue = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4052:14
export const LLVMBuildInsertValue = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "u32", "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4055:14
export const LLVMBuildFreeze = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4058:14
export const LLVMBuildIsNull = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4060:14
export const LLVMBuildIsNotNull = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4062:1 <Spelling=/data/./llvm-c/Core.h:4063:18>
export const LLVMBuildPtrDiff = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4066:14
export const LLVMBuildPtrDiff2 = {
  parameters: [types.LLVMBuilderRef, types.LLVMTypeRef, types.LLVMValueRef, types.LLVMValueRef, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4069:14
export const LLVMBuildFence = {
  parameters: [types.LLVMBuilderRef, types.LLVMAtomicOrdering, types.LLVMBool, "pointer"],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4071:14
export const LLVMBuildAtomicRMW = {
  parameters: [types.LLVMBuilderRef, types.LLVMAtomicRMWBinOp, types.LLVMValueRef, types.LLVMValueRef, types.LLVMAtomicOrdering, types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4075:14
export const LLVMBuildAtomicCmpXchg = {
  parameters: [types.LLVMBuilderRef, types.LLVMValueRef, types.LLVMValueRef, types.LLVMValueRef, types.LLVMAtomicOrdering, types.LLVMAtomicOrdering, types.LLVMBool],
  result: types.LLVMValueRef
} as const;

// ./llvm-c/Core.h:4084:10
export const LLVMGetNumMaskElements = {
  parameters: [types.LLVMValueRef],
  result: "u32"
} as const;

// ./llvm-c/Core.h:4090:5
export const LLVMGetUndefMaskElem = {
  parameters: [],
  result: "i32"
} as const;

// ./llvm-c/Core.h:4099:5
export const LLVMGetMaskValue = {
  parameters: [types.LLVMValueRef, "u32"],
  result: "i32"
} as const;

// ./llvm-c/Core.h:4101:10
export const LLVMIsAtomicSingleThread = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:4102:6
export const LLVMSetAtomicSingleThread = {
  parameters: [types.LLVMValueRef, types.LLVMBool],
  result: "void"
} as const;

// ./llvm-c/Core.h:4104:20
export const LLVMGetCmpXchgSuccessOrdering = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMAtomicOrdering
} as const;

// ./llvm-c/Core.h:4105:6
export const LLVMSetCmpXchgSuccessOrdering = {
  parameters: [types.LLVMValueRef, types.LLVMAtomicOrdering],
  result: "void"
} as const;

// ./llvm-c/Core.h:4107:20
export const LLVMGetCmpXchgFailureOrdering = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMAtomicOrdering
} as const;

// ./llvm-c/Core.h:4108:6
export const LLVMSetCmpXchgFailureOrdering = {
  parameters: [types.LLVMValueRef, types.LLVMAtomicOrdering],
  result: "void"
} as const;

// ./llvm-c/Core.h:4126:1
export const LLVMCreateModuleProviderForExistingModule = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMModuleProviderRef
} as const;

// ./llvm-c/Core.h:4131:6
export const LLVMDisposeModuleProvider = {
  parameters: [types.LLVMModuleProviderRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:4143:10
export const LLVMCreateMemoryBufferWithContentsOfFile = {
  parameters: ["pointer", "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:4146:10
export const LLVMCreateMemoryBufferWithSTDIN = {
  parameters: ["pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:4148:21
export const LLVMCreateMemoryBufferWithMemoryRange = {
  parameters: ["pointer", "u64", "pointer", types.LLVMBool],
  result: types.LLVMMemoryBufferRef
} as const;

// ./llvm-c/Core.h:4152:21
export const LLVMCreateMemoryBufferWithMemoryRangeCopy = {
  parameters: ["pointer", "u64", "pointer"],
  result: types.LLVMMemoryBufferRef
} as const;

// ./llvm-c/Core.h:4155:13
export const LLVMGetBufferStart = {
  parameters: [types.LLVMMemoryBufferRef],
  result: "pointer"
} as const;

// ./llvm-c/Core.h:4156:8
export const LLVMGetBufferSize = {
  parameters: [types.LLVMMemoryBufferRef],
  result: "u64"
} as const;

// ./llvm-c/Core.h:4157:6
export const LLVMDisposeMemoryBuffer = {
  parameters: [types.LLVMMemoryBufferRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:4172:21
export const LLVMGetGlobalPassRegistry = {
  parameters: [],
  result: types.LLVMPassRegistryRef
} as const;

// ./llvm-c/Core.h:4188:20
export const LLVMCreatePassManager = {
  parameters: [],
  result: types.LLVMPassManagerRef
} as const;

// ./llvm-c/Core.h:4194:20
export const LLVMCreateFunctionPassManagerForModule = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMPassManagerRef
} as const;

// ./llvm-c/Core.h:4197:20
export const LLVMCreateFunctionPassManager = {
  parameters: [types.LLVMModuleProviderRef],
  result: types.LLVMPassManagerRef
} as const;

// ./llvm-c/Core.h:4203:10
export const LLVMRunPassManager = {
  parameters: [types.LLVMPassManagerRef, types.LLVMModuleRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:4208:10
export const LLVMInitializeFunctionPassManager = {
  parameters: [types.LLVMPassManagerRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:4214:10
export const LLVMRunFunctionPassManager = {
  parameters: [types.LLVMPassManagerRef, types.LLVMValueRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:4219:10
export const LLVMFinalizeFunctionPassManager = {
  parameters: [types.LLVMPassManagerRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:4224:6
export const LLVMDisposePassManager = {
  parameters: [types.LLVMPassManagerRef],
  result: "void"
} as const;

// ./llvm-c/Core.h:4241:10
export const LLVMStartMultithreaded = {
  parameters: [],
  result: types.LLVMBool
} as const;

// ./llvm-c/Core.h:4245:6
export const LLVMStopMultithreaded = {
  parameters: [],
  result: "void"
} as const;

// ./llvm-c/Core.h:4249:10
export const LLVMIsMultithreaded = {
  parameters: [],
  result: types.LLVMBool
} as const;

// ./llvm-c/Object.h:76:15
export const LLVMCreateBinary = {
  parameters: [types.LLVMMemoryBufferRef, types.LLVMContextRef, "pointer"],
  result: types.LLVMBinaryRef
} as const;

// ./llvm-c/Object.h:86:6
export const LLVMDisposeBinary = {
  parameters: [types.LLVMBinaryRef],
  result: "void"
} as const;

// ./llvm-c/Object.h:97:21
export const LLVMBinaryCopyMemoryBuffer = {
  parameters: [types.LLVMBinaryRef],
  result: types.LLVMMemoryBufferRef
} as const;

// ./llvm-c/Object.h:104:16
export const LLVMBinaryGetType = {
  parameters: [types.LLVMBinaryRef],
  result: types.LLVMBinaryType
} as const;

// ./llvm-c/Object.h:117:15
export const LLVMMachOUniversalBinaryCopyObjectForArch = {
  parameters: [types.LLVMBinaryRef, "pointer", "u64", "pointer"],
  result: types.LLVMBinaryRef
} as const;

// ./llvm-c/Object.h:133:24
export const LLVMObjectFileCopySectionIterator = {
  parameters: [types.LLVMBinaryRef],
  result: types.LLVMSectionIteratorRef
} as const;

// ./llvm-c/Object.h:140:10
export const LLVMObjectFileIsSectionIteratorAtEnd = {
  parameters: [types.LLVMBinaryRef, types.LLVMSectionIteratorRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Object.h:154:23
export const LLVMObjectFileCopySymbolIterator = {
  parameters: [types.LLVMBinaryRef],
  result: types.LLVMSymbolIteratorRef
} as const;

// ./llvm-c/Object.h:161:10
export const LLVMObjectFileIsSymbolIteratorAtEnd = {
  parameters: [types.LLVMBinaryRef, types.LLVMSymbolIteratorRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Object.h:164:6
export const LLVMDisposeSectionIterator = {
  parameters: [types.LLVMSectionIteratorRef],
  result: "void"
} as const;

// ./llvm-c/Object.h:166:6
export const LLVMMoveToNextSection = {
  parameters: [types.LLVMSectionIteratorRef],
  result: "void"
} as const;

// ./llvm-c/Object.h:167:6
export const LLVMMoveToContainingSection = {
  parameters: [types.LLVMSectionIteratorRef, types.LLVMSymbolIteratorRef],
  result: "void"
} as const;

// ./llvm-c/Object.h:171:6
export const LLVMDisposeSymbolIterator = {
  parameters: [types.LLVMSymbolIteratorRef],
  result: "void"
} as const;

// ./llvm-c/Object.h:172:6
export const LLVMMoveToNextSymbol = {
  parameters: [types.LLVMSymbolIteratorRef],
  result: "void"
} as const;

// ./llvm-c/Object.h:175:13
export const LLVMGetSectionName = {
  parameters: [types.LLVMSectionIteratorRef],
  result: "pointer"
} as const;

// ./llvm-c/Object.h:176:10
export const LLVMGetSectionSize = {
  parameters: [types.LLVMSectionIteratorRef],
  result: "u64"
} as const;

// ./llvm-c/Object.h:177:13
export const LLVMGetSectionContents = {
  parameters: [types.LLVMSectionIteratorRef],
  result: "pointer"
} as const;

// ./llvm-c/Object.h:178:10
export const LLVMGetSectionAddress = {
  parameters: [types.LLVMSectionIteratorRef],
  result: "u64"
} as const;

// ./llvm-c/Object.h:179:10
export const LLVMGetSectionContainsSymbol = {
  parameters: [types.LLVMSectionIteratorRef, types.LLVMSymbolIteratorRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Object.h:183:27
export const LLVMGetRelocations = {
  parameters: [types.LLVMSectionIteratorRef],
  result: types.LLVMRelocationIteratorRef
} as const;

// ./llvm-c/Object.h:184:6
export const LLVMDisposeRelocationIterator = {
  parameters: [types.LLVMRelocationIteratorRef],
  result: "void"
} as const;

// ./llvm-c/Object.h:185:10
export const LLVMIsRelocationIteratorAtEnd = {
  parameters: [types.LLVMSectionIteratorRef, types.LLVMRelocationIteratorRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Object.h:187:6
export const LLVMMoveToNextRelocation = {
  parameters: [types.LLVMRelocationIteratorRef],
  result: "void"
} as const;

// ./llvm-c/Object.h:191:13
export const LLVMGetSymbolName = {
  parameters: [types.LLVMSymbolIteratorRef],
  result: "pointer"
} as const;

// ./llvm-c/Object.h:192:10
export const LLVMGetSymbolAddress = {
  parameters: [types.LLVMSymbolIteratorRef],
  result: "u64"
} as const;

// ./llvm-c/Object.h:193:10
export const LLVMGetSymbolSize = {
  parameters: [types.LLVMSymbolIteratorRef],
  result: "u64"
} as const;

// ./llvm-c/Object.h:196:10
export const LLVMGetRelocationOffset = {
  parameters: [types.LLVMRelocationIteratorRef],
  result: "u64"
} as const;

// ./llvm-c/Object.h:197:23
export const LLVMGetRelocationSymbol = {
  parameters: [types.LLVMRelocationIteratorRef],
  result: types.LLVMSymbolIteratorRef
} as const;

// ./llvm-c/Object.h:198:10
export const LLVMGetRelocationType = {
  parameters: [types.LLVMRelocationIteratorRef],
  result: "u64"
} as const;

// ./llvm-c/Object.h:201:13
export const LLVMGetRelocationTypeName = {
  parameters: [types.LLVMRelocationIteratorRef],
  result: "pointer"
} as const;

// ./llvm-c/Object.h:202:13
export const LLVMGetRelocationValueString = {
  parameters: [types.LLVMRelocationIteratorRef],
  result: "pointer"
} as const;

// ./llvm-c/Object.h:208:19
export const LLVMCreateObjectFile = {
  parameters: [types.LLVMMemoryBufferRef],
  result: types.LLVMObjectFileRef
} as const;

// ./llvm-c/Object.h:211:6
export const LLVMDisposeObjectFile = {
  parameters: [types.LLVMObjectFileRef],
  result: "void"
} as const;

// ./llvm-c/Object.h:214:24
export const LLVMGetSections = {
  parameters: [types.LLVMObjectFileRef],
  result: types.LLVMSectionIteratorRef
} as const;

// ./llvm-c/Object.h:217:10
export const LLVMIsSectionIteratorAtEnd = {
  parameters: [types.LLVMObjectFileRef, types.LLVMSectionIteratorRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/Object.h:221:23
export const LLVMGetSymbols = {
  parameters: [types.LLVMObjectFileRef],
  result: types.LLVMSymbolIteratorRef
} as const;

// ./llvm-c/Object.h:224:10
export const LLVMIsSymbolIteratorAtEnd = {
  parameters: [types.LLVMObjectFileRef, types.LLVMSymbolIteratorRef],
  result: types.LLVMBool
} as const;

// ./llvm-c/BitWriter.h:37:5
export const LLVMWriteBitcodeToFile = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: "i32"
} as const;

// ./llvm-c/BitWriter.h:40:5
export const LLVMWriteBitcodeToFD = {
  parameters: [types.LLVMModuleRef, "i32", "i32", "i32"],
  result: "i32"
} as const;

// ./llvm-c/BitWriter.h:45:5
export const LLVMWriteBitcodeToFileHandle = {
  parameters: [types.LLVMModuleRef, "i32"],
  result: "i32"
} as const;

// ./llvm-c/BitWriter.h:48:21
export const LLVMWriteBitcodeToMemoryBuffer = {
  parameters: [types.LLVMModuleRef],
  result: types.LLVMMemoryBufferRef
} as const;

// ./llvm-c/LLJIT.h:74:24
export const LLVMOrcCreateLLJITBuilder = {
  parameters: [],
  result: types.LLVMOrcLLJITBuilderRef
} as const;

// ./llvm-c/LLJIT.h:81:6
export const LLVMOrcDisposeLLJITBuilder = {
  parameters: [types.LLVMOrcLLJITBuilderRef],
  result: "void"
} as const;

// ./llvm-c/LLJIT.h:92:6
export const LLVMOrcLLJITBuilderSetJITTargetMachineBuilder = {
  parameters: [types.LLVMOrcLLJITBuilderRef, types.LLVMOrcJITTargetMachineBuilderRef],
  result: "void"
} as const;

// ./llvm-c/LLJIT.h:98:6
export const LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator = {
  parameters: [types.LLVMOrcLLJITBuilderRef, types.LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction, "pointer"],
  result: "void"
} as const;

// ./llvm-c/LLJIT.h:116:14
export const LLVMOrcCreateLLJIT = {
  parameters: ["pointer", types.LLVMOrcLLJITBuilderRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/LLJIT.h:122:14
export const LLVMOrcDisposeLLJIT = {
  parameters: [types.LLVMOrcLLJITRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/LLJIT.h:130:28
export const LLVMOrcLLJITGetExecutionSession = {
  parameters: [types.LLVMOrcLLJITRef],
  result: types.LLVMOrcExecutionSessionRef
} as const;

// ./llvm-c/LLJIT.h:138:20
export const LLVMOrcLLJITGetMainJITDylib = {
  parameters: [types.LLVMOrcLLJITRef],
  result: types.LLVMOrcJITDylibRef
} as const;

// ./llvm-c/LLJIT.h:144:13
export const LLVMOrcLLJITGetTripleString = {
  parameters: [types.LLVMOrcLLJITRef],
  result: "pointer"
} as const;

// ./llvm-c/LLJIT.h:149:6
export const LLVMOrcLLJITGetGlobalPrefix = {
  parameters: [types.LLVMOrcLLJITRef],
  result: "u8"
} as const;

// ./llvm-c/LLJIT.h:159:1
export const LLVMOrcLLJITMangleAndIntern = {
  parameters: [types.LLVMOrcLLJITRef, "pointer"],
  result: types.LLVMOrcSymbolStringPoolEntryRef
} as const;

// ./llvm-c/LLJIT.h:170:14
export const LLVMOrcLLJITAddObjectFile = {
  parameters: [types.LLVMOrcLLJITRef, types.LLVMOrcJITDylibRef, types.LLVMMemoryBufferRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/LLJIT.h:182:14
export const LLVMOrcLLJITAddObjectFileWithRT = {
  parameters: [types.LLVMOrcLLJITRef, types.LLVMOrcResourceTrackerRef, types.LLVMMemoryBufferRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/LLJIT.h:195:14
export const LLVMOrcLLJITAddLLVMIRModule = {
  parameters: [types.LLVMOrcLLJITRef, types.LLVMOrcJITDylibRef, types.LLVMOrcThreadSafeModuleRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/LLJIT.h:208:14
export const LLVMOrcLLJITAddLLVMIRModuleWithRT = {
  parameters: [types.LLVMOrcLLJITRef, types.LLVMOrcResourceTrackerRef, types.LLVMOrcThreadSafeModuleRef],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/LLJIT.h:217:14
export const LLVMOrcLLJITLookup = {
  parameters: [types.LLVMOrcLLJITRef, "pointer", "pointer"],
  result: types.LLVMErrorRef
} as const;

// ./llvm-c/LLJIT.h:224:23
export const LLVMOrcLLJITGetObjLinkingLayer = {
  parameters: [types.LLVMOrcLLJITRef],
  result: types.LLVMOrcObjectLayerRef
} as const;

// ./llvm-c/LLJIT.h:230:1
export const LLVMOrcLLJITGetObjTransformLayer = {
  parameters: [types.LLVMOrcLLJITRef],
  result: types.LLVMOrcObjectTransformLayerRef
} as const;

// ./llvm-c/LLJIT.h:235:28
export const LLVMOrcLLJITGetIRTransformLayer = {
  parameters: [types.LLVMOrcLLJITRef],
  result: types.LLVMOrcIRTransformLayerRef
} as const;

// ./llvm-c/LLJIT.h:243:13
export const LLVMOrcLLJITGetDataLayoutStr = {
  parameters: [types.LLVMOrcLLJITRef],
  result: "pointer"
} as const;

// ./llvm-c/Support.h:35:10
export const LLVMLoadLibraryPermanently = {
  parameters: ["pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/Support.h:45:6
export const LLVMParseCommandLineOptions = {
  parameters: ["i32", "pointer", "pointer"],
  result: "void"
} as const;

// ./llvm-c/Support.h:55:7
export const LLVMSearchForAddressOfSymbol = {
  parameters: ["pointer"],
  result: "pointer"
} as const;

// ./llvm-c/Support.h:64:6
export const LLVMAddSymbol = {
  parameters: ["pointer", "pointer"],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:34:6
export const LLVMInitializeTransformUtils = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:35:6
export const LLVMInitializeScalarOpts = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:36:6
export const LLVMInitializeObjCARCOpts = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:37:6
export const LLVMInitializeVectorization = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:38:6
export const LLVMInitializeInstCombine = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:39:6
export const LLVMInitializeAggressiveInstCombiner = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:40:6
export const LLVMInitializeIPO = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:41:6
export const LLVMInitializeInstrumentation = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:42:6
export const LLVMInitializeAnalysis = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:43:6
export const LLVMInitializeIPA = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:44:6
export const LLVMInitializeCodeGen = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/Initialization.h:45:6
export const LLVMInitializeTarget = {
  parameters: [types.LLVMPassRegistryRef],
  result: "void"
} as const;

// ./llvm-c/BitReader.h:39:10
export const LLVMParseBitcode = {
  parameters: [types.LLVMMemoryBufferRef, "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/BitReader.h:44:10
export const LLVMParseBitcode2 = {
  parameters: [types.LLVMMemoryBufferRef, "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/BitReader.h:48:10
export const LLVMParseBitcodeInContext = {
  parameters: [types.LLVMContextRef, types.LLVMMemoryBufferRef, "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/BitReader.h:52:10
export const LLVMParseBitcodeInContext2 = {
  parameters: [types.LLVMContextRef, types.LLVMMemoryBufferRef, "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/BitReader.h:60:10
export const LLVMGetBitcodeModuleInContext = {
  parameters: [types.LLVMContextRef, types.LLVMMemoryBufferRef, "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/BitReader.h:71:10
export const LLVMGetBitcodeModuleInContext2 = {
  parameters: [types.LLVMContextRef, types.LLVMMemoryBufferRef, "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/BitReader.h:76:10
export const LLVMGetBitcodeModule = {
  parameters: [types.LLVMMemoryBufferRef, "pointer", "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/BitReader.h:79:10
export const LLVMGetBitcodeModule2 = {
  parameters: [types.LLVMMemoryBufferRef, "pointer"],
  result: types.LLVMBool
} as const;

// ./llvm-c/Comdat.h:46:15
export const LLVMGetOrInsertComdat = {
  parameters: [types.LLVMModuleRef, "pointer"],
  result: types.LLVMComdatRef
} as const;

// ./llvm-c/Comdat.h:53:15
export const LLVMGetComdat = {
  parameters: [types.LLVMValueRef],
  result: types.LLVMComdatRef
} as const;

// ./llvm-c/Comdat.h:60:6
export const LLVMSetComdat = {
  parameters: [types.LLVMValueRef, types.LLVMComdatRef],
  result: "void"
} as const;

// ./llvm-c/Comdat.h:67:25
export const LLVMGetComdatSelectionKind = {
  parameters: [types.LLVMComdatRef],
  result: types.LLVMComdatSelectionKind
} as const;

// ./llvm-c/Comdat.h:74:6
export const LLVMSetComdatSelectionKind = {
  parameters: [types.LLVMComdatRef, types.LLVMComdatSelectionKind],
  result: "void"
} as const;

// ./llvm-c/IRReader.h:38:10
export const LLVMParseIRInContext = {
  parameters: [types.LLVMContextRef, types.LLVMMemoryBufferRef, "pointer", "pointer"],
  result: types.LLVMBool
} as const;

