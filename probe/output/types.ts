// deno-lint-ignore-file

import { Opaque } from "./safe-ffi.ts";

// ./llvm-c/Types.h:28:13
export const LLVMBool: Opaque<"i32", "LLVMBool"> = "i32" as never;

// ./llvm-c/Types.h:48:40
export const LLVMMemoryBufferRef: Opaque<"pointer", "LLVMMemoryBufferRef"> = "pointer" as never;

// ./llvm-c/Types.h:53:35
export const LLVMContextRef: Opaque<"pointer", "LLVMContextRef"> = "pointer" as never;

// ./llvm-c/Types.h:61:34
export const LLVMModuleRef: Opaque<"pointer", "LLVMModuleRef"> = "pointer" as never;

// ./llvm-c/Types.h:68:32
export const LLVMTypeRef: Opaque<"pointer", "LLVMTypeRef"> = "pointer" as never;

// ./llvm-c/Types.h:75:33
export const LLVMValueRef: Opaque<"pointer", "LLVMValueRef"> = "pointer" as never;

// ./llvm-c/Types.h:82:38
export const LLVMBasicBlockRef: Opaque<"pointer", "LLVMBasicBlockRef"> = "pointer" as never;

// ./llvm-c/Types.h:89:36
export const LLVMMetadataRef: Opaque<"pointer", "LLVMMetadataRef"> = "pointer" as never;

// ./llvm-c/Types.h:96:39
export const LLVMNamedMDNodeRef: Opaque<"pointer", "LLVMNamedMDNodeRef"> = "pointer" as never;

// ./llvm-c/Types.h:103:45
export const LLVMValueMetadataEntry: Opaque<"pointer", "LLVMValueMetadataEntry"> = "pointer" as never;

// ./llvm-c/Types.h:110:35
export const LLVMBuilderRef: Opaque<"pointer", "LLVMBuilderRef"> = "pointer" as never;

// ./llvm-c/Types.h:117:37
export const LLVMDIBuilderRef: Opaque<"pointer", "LLVMDIBuilderRef"> = "pointer" as never;

// ./llvm-c/Types.h:124:42
export const LLVMModuleProviderRef: Opaque<"pointer", "LLVMModuleProviderRef"> = "pointer" as never;

// ./llvm-c/Types.h:127:39
export const LLVMPassManagerRef: Opaque<"pointer", "LLVMPassManagerRef"> = "pointer" as never;

// ./llvm-c/Types.h:130:40
export const LLVMPassRegistryRef: Opaque<"pointer", "LLVMPassRegistryRef"> = "pointer" as never;

// ./llvm-c/Types.h:136:31
export const LLVMUseRef: Opaque<"pointer", "LLVMUseRef"> = "pointer" as never;

// ./llvm-c/Types.h:143:40
export const LLVMAttributeRef: Opaque<"pointer", "LLVMAttributeRef"> = "pointer" as never;

// ./llvm-c/Types.h:148:42
export const LLVMDiagnosticInfoRef: Opaque<"pointer", "LLVMDiagnosticInfoRef"> = "pointer" as never;

// ./llvm-c/Types.h:153:28
export const LLVMComdatRef: Opaque<"pointer", "LLVMComdatRef"> = "pointer" as never;

// ./llvm-c/Types.h:158:42
export const LLVMModuleFlagEntry: Opaque<"pointer", "LLVMModuleFlagEntry"> = "pointer" as never;

// ./llvm-c/Types.h:163:44
export const LLVMJITEventListenerRef: Opaque<"pointer", "LLVMJITEventListenerRef"> = "pointer" as never;

// ./llvm-c/Types.h:168:34
export const LLVMBinaryRef: Opaque<"pointer", "LLVMBinaryRef"> = "pointer" as never;

// ./llvm-c/Analysis.h:38:3
export const LLVMVerifierFailureAction: Opaque<"i32", "LLVMVerifierFailureAction"> = "i32" as never;

// ./llvm-c/Target.h:37:38
export const LLVMTargetDataRef: Opaque<"pointer", "LLVMTargetDataRef"> = "pointer" as never;

// ./llvm-c/Target.h:38:50
export const LLVMTargetLibraryInfoRef: Opaque<"pointer", "LLVMTargetLibraryInfoRef"> = "pointer" as never;

// ./llvm-c/TargetMachine.h:34:41
export const LLVMTargetMachineRef: Opaque<"pointer", "LLVMTargetMachineRef"> = "pointer" as never;

// ./llvm-c/TargetMachine.h:35:28
export const LLVMTargetRef: Opaque<"pointer", "LLVMTargetRef"> = "pointer" as never;

// ./llvm-c/TargetMachine.h:42:3
export const LLVMCodeGenOptLevel: Opaque<"i32", "LLVMCodeGenOptLevel"> = "i32" as never;

// ./llvm-c/TargetMachine.h:52:3
export const LLVMRelocMode: Opaque<"i32", "LLVMRelocMode"> = "i32" as never;

// ./llvm-c/TargetMachine.h:62:3
export const LLVMCodeModel: Opaque<"i32", "LLVMCodeModel"> = "i32" as never;

// ./llvm-c/TargetMachine.h:67:3
export const LLVMCodeGenFileType: Opaque<"i32", "LLVMCodeGenFileType"> = "i32" as never;

// ./llvm-c/ExecutionEngine.h:39:40
export const LLVMGenericValueRef: Opaque<"pointer", "LLVMGenericValueRef"> = "pointer" as never;

// ./llvm-c/ExecutionEngine.h:40:43
export const LLVMExecutionEngineRef: Opaque<"pointer", "LLVMExecutionEngineRef"> = "pointer" as never;

// ./llvm-c/ExecutionEngine.h:41:46
export const LLVMMCJITMemoryManagerRef: Opaque<"pointer", "LLVMMCJITMemoryManagerRef"> = "pointer" as never;

// ./llvm-c/ExecutionEngine.h:159:20
export const LLVMMemoryManagerAllocateCodeSectionCallback: Opaque<"function", "LLVMMemoryManagerAllocateCodeSectionCallback"> = "function" as never;

// ./llvm-c/ExecutionEngine.h:162:20
export const LLVMMemoryManagerAllocateDataSectionCallback: Opaque<"function", "LLVMMemoryManagerAllocateDataSectionCallback"> = "function" as never;

// ./llvm-c/ExecutionEngine.h:165:20
export const LLVMMemoryManagerFinalizeMemoryCallback: Opaque<"function", "LLVMMemoryManagerFinalizeMemoryCallback"> = "function" as never;

// ./llvm-c/ExecutionEngine.h:167:16
export const LLVMMemoryManagerDestroyCallback: Opaque<"function", "LLVMMemoryManagerDestroyCallback"> = "function" as never;

// ./llvm-c/DisassemblerTypes.h:29:15
export const LLVMDisasmContextRef: Opaque<"pointer", "LLVMDisasmContextRef"> = "pointer" as never;

// ./llvm-c/DisassemblerTypes.h:48:15
export const LLVMOpInfoCallback: Opaque<"function", "LLVMOpInfoCallback"> = "function" as never;

// ./llvm-c/DisassemblerTypes.h:118:23
export const LLVMSymbolLookupCallback: Opaque<"function", "LLVMSymbolLookupCallback"> = "function" as never;

// ./llvm-c/DebugInfo.h:73:3
export const LLVMDIFlags: Opaque<"i32", "LLVMDIFlags"> = "i32" as never;

// ./llvm-c/DebugInfo.h:123:3
export const LLVMDWARFSourceLanguage: Opaque<"i32", "LLVMDWARFSourceLanguage"> = "i32" as never;

// ./llvm-c/DebugInfo.h:132:3
export const LLVMDWARFEmissionKind: Opaque<"i32", "LLVMDWARFEmissionKind"> = "i32" as never;

// ./llvm-c/DebugInfo.h:174:18
export const LLVMMetadataKind: Opaque<"u32", "LLVMMetadataKind"> = "u32" as never;

// ./llvm-c/DebugInfo.h:179:18
export const LLVMDWARFTypeEncoding: Opaque<"u32", "LLVMDWARFTypeEncoding"> = "u32" as never;

// ./llvm-c/DebugInfo.h:192:3
export const LLVMDWARFMacinfoRecordType: Opaque<"i32", "LLVMDWARFMacinfoRecordType"> = "i32" as never;

// ./llvm-c/Error.h:33:33
export const LLVMErrorRef: Opaque<"pointer", "LLVMErrorRef"> = "pointer" as never;

// ./llvm-c/Error.h:38:21
export const LLVMErrorTypeId: Opaque<"pointer", "LLVMErrorTypeId"> = "pointer" as never;

// ./llvm-c/Orc.h:46:18
export const LLVMOrcJITTargetAddress: Opaque<"u64", "LLVMOrcJITTargetAddress"> = "u64" as never;

// ./llvm-c/Orc.h:51:18
export const LLVMOrcExecutorAddress: Opaque<"u64", "LLVMOrcExecutorAddress"> = "u64" as never;

// ./llvm-c/Orc.h:61:3
export const LLVMJITSymbolGenericFlags: Opaque<"i32", "LLVMJITSymbolGenericFlags"> = "i32" as never;

// ./llvm-c/Orc.h:66:17
export const LLVMJITSymbolTargetFlags: Opaque<"u8", "LLVMJITSymbolTargetFlags"> = "u8" as never;

// ./llvm-c/Orc.h:74:3
export const LLVMJITSymbolFlags: Opaque<"pointer", "LLVMJITSymbolFlags"> = "pointer" as never;

// ./llvm-c/Orc.h:82:3
export const LLVMJITEvaluatedSymbol: Opaque<"pointer", "LLVMJITEvaluatedSymbol"> = "pointer" as never;

// ./llvm-c/Orc.h:87:47
export const LLVMOrcExecutionSessionRef: Opaque<"pointer", "LLVMOrcExecutionSessionRef"> = "pointer" as never;

// ./llvm-c/Orc.h:92:16
export const LLVMOrcErrorReporterFunction: Opaque<"function", "LLVMOrcErrorReporterFunction"> = "function" as never;

// ./llvm-c/Orc.h:97:47
export const LLVMOrcSymbolStringPoolRef: Opaque<"pointer", "LLVMOrcSymbolStringPoolRef"> = "pointer" as never;

// ./llvm-c/Orc.h:103:6
export const LLVMOrcSymbolStringPoolEntryRef: Opaque<"pointer", "LLVMOrcSymbolStringPoolEntryRef"> = "pointer" as never;

// ./llvm-c/Orc.h:111:3
export const LLVMOrcCSymbolFlagsMapPair: Opaque<"pointer", "LLVMOrcCSymbolFlagsMapPair"> = "pointer" as never;

// ./llvm-c/Orc.h:117:37
export const LLVMOrcCSymbolFlagsMapPairs: Opaque<"pointer", "LLVMOrcCSymbolFlagsMapPairs"> = "pointer" as never;

// ./llvm-c/Orc.h:125:3
export const LLVMJITCSymbolMapPair: Opaque<"pointer", "LLVMJITCSymbolMapPair"> = "pointer" as never;

// ./llvm-c/Orc.h:131:32
export const LLVMOrcCSymbolMapPairs: Opaque<"pointer", "LLVMOrcCSymbolMapPairs"> = "pointer" as never;

// ./llvm-c/Orc.h:139:3
export const LLVMOrcCSymbolAliasMapEntry: Opaque<"pointer", "LLVMOrcCSymbolAliasMapEntry"> = "pointer" as never;

// ./llvm-c/Orc.h:147:3
export const LLVMOrcCSymbolAliasMapPair: Opaque<"pointer", "LLVMOrcCSymbolAliasMapPair"> = "pointer" as never;

// ./llvm-c/Orc.h:153:37
export const LLVMOrcCSymbolAliasMapPairs: Opaque<"pointer", "LLVMOrcCSymbolAliasMapPairs"> = "pointer" as never;

// ./llvm-c/Orc.h:158:39
export const LLVMOrcJITDylibRef: Opaque<"pointer", "LLVMOrcJITDylibRef"> = "pointer" as never;

// ./llvm-c/Orc.h:167:3
export const LLVMOrcCSymbolsList: Opaque<"pointer", "LLVMOrcCSymbolsList"> = "pointer" as never;

// ./llvm-c/Orc.h:175:3
export const LLVMOrcCDependenceMapPair: Opaque<"pointer", "LLVMOrcCDependenceMapPair"> = "pointer" as never;

// ./llvm-c/Orc.h:181:36
export const LLVMOrcCDependenceMapPairs: Opaque<"pointer", "LLVMOrcCDependenceMapPairs"> = "pointer" as never;

// ./llvm-c/Orc.h:192:3
export const LLVMOrcLookupKind: Opaque<"i32", "LLVMOrcLookupKind"> = "i32" as never;

// ./llvm-c/Orc.h:203:3
export const LLVMOrcJITDylibLookupFlags: Opaque<"i32", "LLVMOrcJITDylibLookupFlags"> = "i32" as never;

// ./llvm-c/Orc.h:212:3
export const LLVMOrcSymbolLookupFlags: Opaque<"i32", "LLVMOrcSymbolLookupFlags"> = "i32" as never;

// ./llvm-c/Orc.h:220:3
export const LLVMOrcCLookupSetElement: Opaque<"pointer", "LLVMOrcCLookupSetElement"> = "pointer" as never;

// ./llvm-c/Orc.h:233:35
export const LLVMOrcCLookupSet: Opaque<"pointer", "LLVMOrcCLookupSet"> = "pointer" as never;

// ./llvm-c/Orc.h:238:50
export const LLVMOrcMaterializationUnitRef: Opaque<"pointer", "LLVMOrcMaterializationUnitRef"> = "pointer" as never;

// ./llvm-c/Orc.h:246:6
export const LLVMOrcMaterializationResponsibilityRef: Opaque<"pointer", "LLVMOrcMaterializationResponsibilityRef"> = "pointer" as never;

// ./llvm-c/Orc.h:258:16
export const LLVMOrcMaterializationUnitMaterializeFunction: Opaque<"function", "LLVMOrcMaterializationUnitMaterializeFunction"> = "function" as never;

// ./llvm-c/Orc.h:267:16
export const LLVMOrcMaterializationUnitDiscardFunction: Opaque<"function", "LLVMOrcMaterializationUnitDiscardFunction"> = "function" as never;

// ./llvm-c/Orc.h:277:16
export const LLVMOrcMaterializationUnitDestroyFunction: Opaque<"function", "LLVMOrcMaterializationUnitDestroyFunction"> = "function" as never;

// ./llvm-c/Orc.h:282:46
export const LLVMOrcResourceTrackerRef: Opaque<"pointer", "LLVMOrcResourceTrackerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:288:6
export const LLVMOrcDefinitionGeneratorRef: Opaque<"pointer", "LLVMOrcDefinitionGeneratorRef"> = "pointer" as never;

// ./llvm-c/Orc.h:302:42
export const LLVMOrcLookupStateRef: Opaque<"pointer", "LLVMOrcLookupStateRef"> = "pointer" as never;

// ./llvm-c/Orc.h:337:24
export const LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction: Opaque<"function", "LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction"> = "function" as never;

// ./llvm-c/Orc.h:346:15
export const LLVMOrcSymbolPredicate: Opaque<"function", "LLVMOrcSymbolPredicate"> = "function" as never;

// ./llvm-c/Orc.h:352:48
export const LLVMOrcThreadSafeContextRef: Opaque<"pointer", "LLVMOrcThreadSafeContextRef"> = "pointer" as never;

// ./llvm-c/Orc.h:357:47
export const LLVMOrcThreadSafeModuleRef: Opaque<"pointer", "LLVMOrcThreadSafeModuleRef"> = "pointer" as never;

// ./llvm-c/Orc.h:363:24
export const LLVMOrcGenericIRModuleOperationFunction: Opaque<"function", "LLVMOrcGenericIRModuleOperationFunction"> = "function" as never;

// ./llvm-c/Orc.h:370:6
export const LLVMOrcJITTargetMachineBuilderRef: Opaque<"pointer", "LLVMOrcJITTargetMachineBuilderRef"> = "pointer" as never;

// ./llvm-c/Orc.h:375:42
export const LLVMOrcObjectLayerRef: Opaque<"pointer", "LLVMOrcObjectLayerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:380:49
export const LLVMOrcObjectLinkingLayerRef: Opaque<"pointer", "LLVMOrcObjectLinkingLayerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:385:47
export const LLVMOrcIRTransformLayerRef: Opaque<"pointer", "LLVMOrcIRTransformLayerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:402:24
export const LLVMOrcIRTransformLayerTransformFunction: Opaque<"function", "LLVMOrcIRTransformLayerTransformFunction"> = "function" as never;

// ./llvm-c/Orc.h:410:6
export const LLVMOrcObjectTransformLayerRef: Opaque<"pointer", "LLVMOrcObjectTransformLayerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:425:24
export const LLVMOrcObjectTransformLayerTransformFunction: Opaque<"function", "LLVMOrcObjectTransformLayerTransformFunction"> = "function" as never;

// ./llvm-c/Orc.h:432:6
export const LLVMOrcIndirectStubsManagerRef: Opaque<"pointer", "LLVMOrcIndirectStubsManagerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:438:6
export const LLVMOrcLazyCallThroughManagerRef: Opaque<"pointer", "LLVMOrcLazyCallThroughManagerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:446:42
export const LLVMOrcDumpObjectsRef: Opaque<"pointer", "LLVMOrcDumpObjectsRef"> = "pointer" as never;

// ./llvm-c/Transforms/PassManagerBuilder.h:20:46
export const LLVMPassManagerBuilderRef: Opaque<"pointer", "LLVMPassManagerBuilderRef"> = "pointer" as never;

// ./llvm-c/Transforms/PassBuilder.h:38:46
export const LLVMPassBuilderOptionsRef: Opaque<"pointer", "LLVMPassBuilderOptionsRef"> = "pointer" as never;

// ./llvm-c/Linker.h:34:3
export const LLVMLinkerMode: Opaque<"i32", "LLVMLinkerMode"> = "i32" as never;

// ./llvm-c/Remarks.h:57:40
export const LLVMRemarkStringRef: Opaque<"pointer", "LLVMRemarkStringRef"> = "pointer" as never;

// ./llvm-c/Remarks.h:78:42
export const LLVMRemarkDebugLocRef: Opaque<"pointer", "LLVMRemarkDebugLocRef"> = "pointer" as never;

// ./llvm-c/Remarks.h:109:37
export const LLVMRemarkArgRef: Opaque<"pointer", "LLVMRemarkArgRef"> = "pointer" as never;

// ./llvm-c/Remarks.h:140:39
export const LLVMRemarkEntryRef: Opaque<"pointer", "LLVMRemarkEntryRef"> = "pointer" as never;

// ./llvm-c/Remarks.h:230:40
export const LLVMRemarkParserRef: Opaque<"pointer", "LLVMRemarkParserRef"> = "pointer" as never;

// ./llvm-c/ErrorHandling.h:27:16
export const LLVMFatalErrorHandler: Opaque<"function", "LLVMFatalErrorHandler"> = "function" as never;

// ./llvm-c/Core.h:146:3
export const LLVMOpcode: Opaque<"i32", "LLVMOpcode"> = "i32" as never;

// ./llvm-c/Core.h:169:3
export const LLVMTypeKind: Opaque<"i32", "LLVMTypeKind"> = "i32" as never;

// ./llvm-c/Core.h:192:3
export const LLVMLinkage: Opaque<"i32", "LLVMLinkage"> = "i32" as never;

// ./llvm-c/Core.h:198:3
export const LLVMVisibility: Opaque<"i32", "LLVMVisibility"> = "i32" as never;

// ./llvm-c/Core.h:204:3
export const LLVMUnnamedAddr: Opaque<"i32", "LLVMUnnamedAddr"> = "i32" as never;

// ./llvm-c/Core.h:210:3
export const LLVMDLLStorageClass: Opaque<"i32", "LLVMDLLStorageClass"> = "i32" as never;

// ./llvm-c/Core.h:255:3
export const LLVMCallConv: Opaque<"i32", "LLVMCallConv"> = "i32" as never;

// ./llvm-c/Core.h:288:3
export const LLVMValueKind: Opaque<"i32", "LLVMValueKind"> = "i32" as never;

// ./llvm-c/Core.h:301:3
export const LLVMIntPredicate: Opaque<"i32", "LLVMIntPredicate"> = "i32" as never;

// ./llvm-c/Core.h:320:3
export const LLVMRealPredicate: Opaque<"i32", "LLVMRealPredicate"> = "i32" as never;

// ./llvm-c/Core.h:325:3
export const LLVMLandingPadClauseTy: Opaque<"i32", "LLVMLandingPadClauseTy"> = "i32" as never;

// ./llvm-c/Core.h:333:3
export const LLVMThreadLocalMode: Opaque<"i32", "LLVMThreadLocalMode"> = "i32" as never;

// ./llvm-c/Core.h:360:3
export const LLVMAtomicOrdering: Opaque<"i32", "LLVMAtomicOrdering"> = "i32" as never;

// ./llvm-c/Core.h:392:3
export const LLVMAtomicRMWBinOp: Opaque<"i32", "LLVMAtomicRMWBinOp"> = "i32" as never;

// ./llvm-c/Core.h:399:3
export const LLVMDiagnosticSeverity: Opaque<"i32", "LLVMDiagnosticSeverity"> = "i32" as never;

// ./llvm-c/Core.h:404:3
export const LLVMInlineAsmDialect: Opaque<"i32", "LLVMInlineAsmDialect"> = "i32" as never;

// ./llvm-c/Core.h:455:3
export const LLVMModuleFlagBehavior: Opaque<"i32", "LLVMModuleFlagBehavior"> = "i32" as never;

// ./llvm-c/Core.h:469:18
export const LLVMAttributeIndex: Opaque<"u32", "LLVMAttributeIndex"> = "u32" as never;

// ./llvm-c/Core.h:499:16
export const LLVMDiagnosticHandler: Opaque<"function", "LLVMDiagnosticHandler"> = "function" as never;

// ./llvm-c/Core.h:500:16
export const LLVMYieldCallback: Opaque<"function", "LLVMYieldCallback"> = "function" as never;

// ./llvm-c/Object.h:36:43
export const LLVMSectionIteratorRef: Opaque<"pointer", "LLVMSectionIteratorRef"> = "pointer" as never;

// ./llvm-c/Object.h:37:42
export const LLVMSymbolIteratorRef: Opaque<"pointer", "LLVMSymbolIteratorRef"> = "pointer" as never;

// ./llvm-c/Object.h:38:46
export const LLVMRelocationIteratorRef: Opaque<"pointer", "LLVMRelocationIteratorRef"> = "pointer" as never;

// ./llvm-c/Object.h:58:3
export const LLVMBinaryType: Opaque<"i32", "LLVMBinaryType"> = "i32" as never;

// ./llvm-c/Object.h:205:38
export const LLVMObjectFileRef: Opaque<"pointer", "LLVMObjectFileRef"> = "pointer" as never;

// ./llvm-c/LLJIT.h:55:6
export const LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction: Opaque<"function", "LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction"> = "function" as never;

// ./llvm-c/LLJIT.h:61:43
export const LLVMOrcLLJITBuilderRef: Opaque<"pointer", "LLVMOrcLLJITBuilderRef"> = "pointer" as never;

// ./llvm-c/LLJIT.h:66:36
export const LLVMOrcLLJITRef: Opaque<"pointer", "LLVMOrcLLJITRef"> = "pointer" as never;

// ./llvm-c/Comdat.h:38:3
export const LLVMComdatSelectionKind: Opaque<"i32", "LLVMComdatSelectionKind"> = "i32" as never;

