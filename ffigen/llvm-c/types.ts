// deno-lint-ignore-file

import { Opaque } from "./safe-ffi.ts";

// ./llvm-c/Types.h:28:13
export type LLVMBool = Opaque<number, "LLVMBool">;
export const LLVMBool_: Opaque<"i32", "LLVMBool"> = "i32" as never;

// ./llvm-c/Types.h:48:40
export type LLVMMemoryBufferRef = Opaque<bigint, "LLVMMemoryBufferRef">;
export const LLVMMemoryBufferRef_: Opaque<"pointer", "LLVMMemoryBufferRef"> = "pointer" as never;

// ./llvm-c/Types.h:53:35
export type LLVMContextRef = Opaque<bigint, "LLVMContextRef">;
export const LLVMContextRef_: Opaque<"pointer", "LLVMContextRef"> = "pointer" as never;

// ./llvm-c/Types.h:61:34
export type LLVMModuleRef = Opaque<bigint, "LLVMModuleRef">;
export const LLVMModuleRef_: Opaque<"pointer", "LLVMModuleRef"> = "pointer" as never;

// ./llvm-c/Types.h:68:32
export type LLVMTypeRef = Opaque<bigint, "LLVMTypeRef">;
export const LLVMTypeRef_: Opaque<"pointer", "LLVMTypeRef"> = "pointer" as never;

// ./llvm-c/Types.h:75:33
export type LLVMValueRef = Opaque<bigint, "LLVMValueRef">;
export const LLVMValueRef_: Opaque<"pointer", "LLVMValueRef"> = "pointer" as never;

// ./llvm-c/Types.h:82:38
export type LLVMBasicBlockRef = Opaque<bigint, "LLVMBasicBlockRef">;
export const LLVMBasicBlockRef_: Opaque<"pointer", "LLVMBasicBlockRef"> = "pointer" as never;

// ./llvm-c/Types.h:89:36
export type LLVMMetadataRef = Opaque<bigint, "LLVMMetadataRef">;
export const LLVMMetadataRef_: Opaque<"pointer", "LLVMMetadataRef"> = "pointer" as never;

// ./llvm-c/Types.h:96:39
export type LLVMNamedMDNodeRef = Opaque<bigint, "LLVMNamedMDNodeRef">;
export const LLVMNamedMDNodeRef_: Opaque<"pointer", "LLVMNamedMDNodeRef"> = "pointer" as never;

// ./llvm-c/Types.h:103:45
export type LLVMValueMetadataEntry = Opaque<bigint, "LLVMValueMetadataEntry">;
export const LLVMValueMetadataEntry_: Opaque<"pointer", "LLVMValueMetadataEntry"> = "pointer" as never;

// ./llvm-c/Types.h:110:35
export type LLVMBuilderRef = Opaque<bigint, "LLVMBuilderRef">;
export const LLVMBuilderRef_: Opaque<"pointer", "LLVMBuilderRef"> = "pointer" as never;

// ./llvm-c/Types.h:117:37
export type LLVMDIBuilderRef = Opaque<bigint, "LLVMDIBuilderRef">;
export const LLVMDIBuilderRef_: Opaque<"pointer", "LLVMDIBuilderRef"> = "pointer" as never;

// ./llvm-c/Types.h:124:42
export type LLVMModuleProviderRef = Opaque<bigint, "LLVMModuleProviderRef">;
export const LLVMModuleProviderRef_: Opaque<"pointer", "LLVMModuleProviderRef"> = "pointer" as never;

// ./llvm-c/Types.h:127:39
export type LLVMPassManagerRef = Opaque<bigint, "LLVMPassManagerRef">;
export const LLVMPassManagerRef_: Opaque<"pointer", "LLVMPassManagerRef"> = "pointer" as never;

// ./llvm-c/Types.h:130:40
export type LLVMPassRegistryRef = Opaque<bigint, "LLVMPassRegistryRef">;
export const LLVMPassRegistryRef_: Opaque<"pointer", "LLVMPassRegistryRef"> = "pointer" as never;

// ./llvm-c/Types.h:136:31
export type LLVMUseRef = Opaque<bigint, "LLVMUseRef">;
export const LLVMUseRef_: Opaque<"pointer", "LLVMUseRef"> = "pointer" as never;

// ./llvm-c/Types.h:143:40
export type LLVMAttributeRef = Opaque<bigint, "LLVMAttributeRef">;
export const LLVMAttributeRef_: Opaque<"pointer", "LLVMAttributeRef"> = "pointer" as never;

// ./llvm-c/Types.h:148:42
export type LLVMDiagnosticInfoRef = Opaque<bigint, "LLVMDiagnosticInfoRef">;
export const LLVMDiagnosticInfoRef_: Opaque<"pointer", "LLVMDiagnosticInfoRef"> = "pointer" as never;

// ./llvm-c/Types.h:153:28
export type LLVMComdatRef = Opaque<bigint, "LLVMComdatRef">;
export const LLVMComdatRef_: Opaque<"pointer", "LLVMComdatRef"> = "pointer" as never;

// ./llvm-c/Types.h:158:42
export type LLVMModuleFlagEntry = Opaque<bigint, "LLVMModuleFlagEntry">;
export const LLVMModuleFlagEntry_: Opaque<"pointer", "LLVMModuleFlagEntry"> = "pointer" as never;

// ./llvm-c/Types.h:163:44
export type LLVMJITEventListenerRef = Opaque<bigint, "LLVMJITEventListenerRef">;
export const LLVMJITEventListenerRef_: Opaque<"pointer", "LLVMJITEventListenerRef"> = "pointer" as never;

// ./llvm-c/Types.h:168:34
export type LLVMBinaryRef = Opaque<bigint, "LLVMBinaryRef">;
export const LLVMBinaryRef_: Opaque<"pointer", "LLVMBinaryRef"> = "pointer" as never;

// ./llvm-c/Analysis.h:38:3
export type LLVMVerifierFailureAction = Opaque<number, "LLVMVerifierFailureAction">;
export const LLVMVerifierFailureAction_: Opaque<"i32", "LLVMVerifierFailureAction"> = "i32" as never;

// ./llvm-c/Target.h:37:38
export type LLVMTargetDataRef = Opaque<bigint, "LLVMTargetDataRef">;
export const LLVMTargetDataRef_: Opaque<"pointer", "LLVMTargetDataRef"> = "pointer" as never;

// ./llvm-c/Target.h:38:50
export type LLVMTargetLibraryInfoRef = Opaque<bigint, "LLVMTargetLibraryInfoRef">;
export const LLVMTargetLibraryInfoRef_: Opaque<"pointer", "LLVMTargetLibraryInfoRef"> = "pointer" as never;

// ./llvm-c/TargetMachine.h:34:41
export type LLVMTargetMachineRef = Opaque<bigint, "LLVMTargetMachineRef">;
export const LLVMTargetMachineRef_: Opaque<"pointer", "LLVMTargetMachineRef"> = "pointer" as never;

// ./llvm-c/TargetMachine.h:35:28
export type LLVMTargetRef = Opaque<bigint, "LLVMTargetRef">;
export const LLVMTargetRef_: Opaque<"pointer", "LLVMTargetRef"> = "pointer" as never;

// ./llvm-c/TargetMachine.h:42:3
export type LLVMCodeGenOptLevel = Opaque<number, "LLVMCodeGenOptLevel">;
export const LLVMCodeGenOptLevel_: Opaque<"i32", "LLVMCodeGenOptLevel"> = "i32" as never;

// ./llvm-c/TargetMachine.h:52:3
export type LLVMRelocMode = Opaque<number, "LLVMRelocMode">;
export const LLVMRelocMode_: Opaque<"i32", "LLVMRelocMode"> = "i32" as never;

// ./llvm-c/TargetMachine.h:62:3
export type LLVMCodeModel = Opaque<number, "LLVMCodeModel">;
export const LLVMCodeModel_: Opaque<"i32", "LLVMCodeModel"> = "i32" as never;

// ./llvm-c/TargetMachine.h:67:3
export type LLVMCodeGenFileType = Opaque<number, "LLVMCodeGenFileType">;
export const LLVMCodeGenFileType_: Opaque<"i32", "LLVMCodeGenFileType"> = "i32" as never;

// ./llvm-c/ExecutionEngine.h:39:40
export type LLVMGenericValueRef = Opaque<bigint, "LLVMGenericValueRef">;
export const LLVMGenericValueRef_: Opaque<"pointer", "LLVMGenericValueRef"> = "pointer" as never;

// ./llvm-c/ExecutionEngine.h:40:43
export type LLVMExecutionEngineRef = Opaque<bigint, "LLVMExecutionEngineRef">;
export const LLVMExecutionEngineRef_: Opaque<"pointer", "LLVMExecutionEngineRef"> = "pointer" as never;

// ./llvm-c/ExecutionEngine.h:41:46
export type LLVMMCJITMemoryManagerRef = Opaque<bigint, "LLVMMCJITMemoryManagerRef">;
export const LLVMMCJITMemoryManagerRef_: Opaque<"pointer", "LLVMMCJITMemoryManagerRef"> = "pointer" as never;

// ./llvm-c/ExecutionEngine.h:159:20
export type LLVMMemoryManagerAllocateCodeSectionCallback = Opaque<bigint, "LLVMMemoryManagerAllocateCodeSectionCallback">;
export const LLVMMemoryManagerAllocateCodeSectionCallback_: Opaque<"function", "LLVMMemoryManagerAllocateCodeSectionCallback"> = "function" as never;

// ./llvm-c/ExecutionEngine.h:162:20
export type LLVMMemoryManagerAllocateDataSectionCallback = Opaque<bigint, "LLVMMemoryManagerAllocateDataSectionCallback">;
export const LLVMMemoryManagerAllocateDataSectionCallback_: Opaque<"function", "LLVMMemoryManagerAllocateDataSectionCallback"> = "function" as never;

// ./llvm-c/ExecutionEngine.h:165:20
export type LLVMMemoryManagerFinalizeMemoryCallback = Opaque<bigint, "LLVMMemoryManagerFinalizeMemoryCallback">;
export const LLVMMemoryManagerFinalizeMemoryCallback_: Opaque<"function", "LLVMMemoryManagerFinalizeMemoryCallback"> = "function" as never;

// ./llvm-c/ExecutionEngine.h:167:16
export type LLVMMemoryManagerDestroyCallback = Opaque<bigint, "LLVMMemoryManagerDestroyCallback">;
export const LLVMMemoryManagerDestroyCallback_: Opaque<"function", "LLVMMemoryManagerDestroyCallback"> = "function" as never;

// ./llvm-c/DisassemblerTypes.h:29:15
export type LLVMDisasmContextRef = Opaque<bigint, "LLVMDisasmContextRef">;
export const LLVMDisasmContextRef_: Opaque<"pointer", "LLVMDisasmContextRef"> = "pointer" as never;

// ./llvm-c/DisassemblerTypes.h:48:15
export type LLVMOpInfoCallback = Opaque<bigint, "LLVMOpInfoCallback">;
export const LLVMOpInfoCallback_: Opaque<"function", "LLVMOpInfoCallback"> = "function" as never;

// ./llvm-c/DisassemblerTypes.h:118:23
export type LLVMSymbolLookupCallback = Opaque<bigint, "LLVMSymbolLookupCallback">;
export const LLVMSymbolLookupCallback_: Opaque<"function", "LLVMSymbolLookupCallback"> = "function" as never;

// ./llvm-c/DebugInfo.h:73:3
export type LLVMDIFlags = Opaque<number, "LLVMDIFlags">;
export const LLVMDIFlags_: Opaque<"i32", "LLVMDIFlags"> = "i32" as never;

// ./llvm-c/DebugInfo.h:123:3
export type LLVMDWARFSourceLanguage = Opaque<number, "LLVMDWARFSourceLanguage">;
export const LLVMDWARFSourceLanguage_: Opaque<"i32", "LLVMDWARFSourceLanguage"> = "i32" as never;

// ./llvm-c/DebugInfo.h:132:3
export type LLVMDWARFEmissionKind = Opaque<number, "LLVMDWARFEmissionKind">;
export const LLVMDWARFEmissionKind_: Opaque<"i32", "LLVMDWARFEmissionKind"> = "i32" as never;

// ./llvm-c/DebugInfo.h:174:18
export type LLVMMetadataKind = Opaque<number, "LLVMMetadataKind">;
export const LLVMMetadataKind_: Opaque<"u32", "LLVMMetadataKind"> = "u32" as never;

// ./llvm-c/DebugInfo.h:179:18
export type LLVMDWARFTypeEncoding = Opaque<number, "LLVMDWARFTypeEncoding">;
export const LLVMDWARFTypeEncoding_: Opaque<"u32", "LLVMDWARFTypeEncoding"> = "u32" as never;

// ./llvm-c/DebugInfo.h:192:3
export type LLVMDWARFMacinfoRecordType = Opaque<number, "LLVMDWARFMacinfoRecordType">;
export const LLVMDWARFMacinfoRecordType_: Opaque<"i32", "LLVMDWARFMacinfoRecordType"> = "i32" as never;

// ./llvm-c/Error.h:33:33
export type LLVMErrorRef = Opaque<bigint, "LLVMErrorRef">;
export const LLVMErrorRef_: Opaque<"pointer", "LLVMErrorRef"> = "pointer" as never;

// ./llvm-c/Error.h:38:21
export type LLVMErrorTypeId = Opaque<bigint, "LLVMErrorTypeId">;
export const LLVMErrorTypeId_: Opaque<"pointer", "LLVMErrorTypeId"> = "pointer" as never;

// ./llvm-c/Orc.h:46:18
export type LLVMOrcJITTargetAddress = Opaque<bigint, "LLVMOrcJITTargetAddress">;
export const LLVMOrcJITTargetAddress_: Opaque<"u64", "LLVMOrcJITTargetAddress"> = "u64" as never;

// ./llvm-c/Orc.h:51:18
export type LLVMOrcExecutorAddress = Opaque<bigint, "LLVMOrcExecutorAddress">;
export const LLVMOrcExecutorAddress_: Opaque<"u64", "LLVMOrcExecutorAddress"> = "u64" as never;

// ./llvm-c/Orc.h:61:3
export type LLVMJITSymbolGenericFlags = Opaque<number, "LLVMJITSymbolGenericFlags">;
export const LLVMJITSymbolGenericFlags_: Opaque<"i32", "LLVMJITSymbolGenericFlags"> = "i32" as never;

// ./llvm-c/Orc.h:66:17
export type LLVMJITSymbolTargetFlags = Opaque<number, "LLVMJITSymbolTargetFlags">;
export const LLVMJITSymbolTargetFlags_: Opaque<"u8", "LLVMJITSymbolTargetFlags"> = "u8" as never;

// ./llvm-c/Orc.h:74:3
export type LLVMJITSymbolFlags = Opaque<bigint, "LLVMJITSymbolFlags">;
export const LLVMJITSymbolFlags_: Opaque<"pointer", "LLVMJITSymbolFlags"> = "pointer" as never;

// ./llvm-c/Orc.h:82:3
export type LLVMJITEvaluatedSymbol = Opaque<bigint, "LLVMJITEvaluatedSymbol">;
export const LLVMJITEvaluatedSymbol_: Opaque<"pointer", "LLVMJITEvaluatedSymbol"> = "pointer" as never;

// ./llvm-c/Orc.h:87:47
export type LLVMOrcExecutionSessionRef = Opaque<bigint, "LLVMOrcExecutionSessionRef">;
export const LLVMOrcExecutionSessionRef_: Opaque<"pointer", "LLVMOrcExecutionSessionRef"> = "pointer" as never;

// ./llvm-c/Orc.h:92:16
export type LLVMOrcErrorReporterFunction = Opaque<bigint, "LLVMOrcErrorReporterFunction">;
export const LLVMOrcErrorReporterFunction_: Opaque<"function", "LLVMOrcErrorReporterFunction"> = "function" as never;

// ./llvm-c/Orc.h:97:47
export type LLVMOrcSymbolStringPoolRef = Opaque<bigint, "LLVMOrcSymbolStringPoolRef">;
export const LLVMOrcSymbolStringPoolRef_: Opaque<"pointer", "LLVMOrcSymbolStringPoolRef"> = "pointer" as never;

// ./llvm-c/Orc.h:103:6
export type LLVMOrcSymbolStringPoolEntryRef = Opaque<bigint, "LLVMOrcSymbolStringPoolEntryRef">;
export const LLVMOrcSymbolStringPoolEntryRef_: Opaque<"pointer", "LLVMOrcSymbolStringPoolEntryRef"> = "pointer" as never;

// ./llvm-c/Orc.h:111:3
export type LLVMOrcCSymbolFlagsMapPair = Opaque<bigint, "LLVMOrcCSymbolFlagsMapPair">;
export const LLVMOrcCSymbolFlagsMapPair_: Opaque<"pointer", "LLVMOrcCSymbolFlagsMapPair"> = "pointer" as never;

// ./llvm-c/Orc.h:117:37
export type LLVMOrcCSymbolFlagsMapPairs = Opaque<bigint, "LLVMOrcCSymbolFlagsMapPairs">;
export const LLVMOrcCSymbolFlagsMapPairs_: Opaque<"pointer", "LLVMOrcCSymbolFlagsMapPairs"> = "pointer" as never;

// ./llvm-c/Orc.h:125:3
export type LLVMJITCSymbolMapPair = Opaque<bigint, "LLVMJITCSymbolMapPair">;
export const LLVMJITCSymbolMapPair_: Opaque<"pointer", "LLVMJITCSymbolMapPair"> = "pointer" as never;

// ./llvm-c/Orc.h:131:32
export type LLVMOrcCSymbolMapPairs = Opaque<bigint, "LLVMOrcCSymbolMapPairs">;
export const LLVMOrcCSymbolMapPairs_: Opaque<"pointer", "LLVMOrcCSymbolMapPairs"> = "pointer" as never;

// ./llvm-c/Orc.h:139:3
export type LLVMOrcCSymbolAliasMapEntry = Opaque<bigint, "LLVMOrcCSymbolAliasMapEntry">;
export const LLVMOrcCSymbolAliasMapEntry_: Opaque<"pointer", "LLVMOrcCSymbolAliasMapEntry"> = "pointer" as never;

// ./llvm-c/Orc.h:147:3
export type LLVMOrcCSymbolAliasMapPair = Opaque<bigint, "LLVMOrcCSymbolAliasMapPair">;
export const LLVMOrcCSymbolAliasMapPair_: Opaque<"pointer", "LLVMOrcCSymbolAliasMapPair"> = "pointer" as never;

// ./llvm-c/Orc.h:153:37
export type LLVMOrcCSymbolAliasMapPairs = Opaque<bigint, "LLVMOrcCSymbolAliasMapPairs">;
export const LLVMOrcCSymbolAliasMapPairs_: Opaque<"pointer", "LLVMOrcCSymbolAliasMapPairs"> = "pointer" as never;

// ./llvm-c/Orc.h:158:39
export type LLVMOrcJITDylibRef = Opaque<bigint, "LLVMOrcJITDylibRef">;
export const LLVMOrcJITDylibRef_: Opaque<"pointer", "LLVMOrcJITDylibRef"> = "pointer" as never;

// ./llvm-c/Orc.h:167:3
export type LLVMOrcCSymbolsList = Opaque<bigint, "LLVMOrcCSymbolsList">;
export const LLVMOrcCSymbolsList_: Opaque<"pointer", "LLVMOrcCSymbolsList"> = "pointer" as never;

// ./llvm-c/Orc.h:175:3
export type LLVMOrcCDependenceMapPair = Opaque<bigint, "LLVMOrcCDependenceMapPair">;
export const LLVMOrcCDependenceMapPair_: Opaque<"pointer", "LLVMOrcCDependenceMapPair"> = "pointer" as never;

// ./llvm-c/Orc.h:181:36
export type LLVMOrcCDependenceMapPairs = Opaque<bigint, "LLVMOrcCDependenceMapPairs">;
export const LLVMOrcCDependenceMapPairs_: Opaque<"pointer", "LLVMOrcCDependenceMapPairs"> = "pointer" as never;

// ./llvm-c/Orc.h:192:3
export type LLVMOrcLookupKind = Opaque<number, "LLVMOrcLookupKind">;
export const LLVMOrcLookupKind_: Opaque<"i32", "LLVMOrcLookupKind"> = "i32" as never;

// ./llvm-c/Orc.h:203:3
export type LLVMOrcJITDylibLookupFlags = Opaque<number, "LLVMOrcJITDylibLookupFlags">;
export const LLVMOrcJITDylibLookupFlags_: Opaque<"i32", "LLVMOrcJITDylibLookupFlags"> = "i32" as never;

// ./llvm-c/Orc.h:212:3
export type LLVMOrcSymbolLookupFlags = Opaque<number, "LLVMOrcSymbolLookupFlags">;
export const LLVMOrcSymbolLookupFlags_: Opaque<"i32", "LLVMOrcSymbolLookupFlags"> = "i32" as never;

// ./llvm-c/Orc.h:220:3
export type LLVMOrcCLookupSetElement = Opaque<bigint, "LLVMOrcCLookupSetElement">;
export const LLVMOrcCLookupSetElement_: Opaque<"pointer", "LLVMOrcCLookupSetElement"> = "pointer" as never;

// ./llvm-c/Orc.h:233:35
export type LLVMOrcCLookupSet = Opaque<bigint, "LLVMOrcCLookupSet">;
export const LLVMOrcCLookupSet_: Opaque<"pointer", "LLVMOrcCLookupSet"> = "pointer" as never;

// ./llvm-c/Orc.h:238:50
export type LLVMOrcMaterializationUnitRef = Opaque<bigint, "LLVMOrcMaterializationUnitRef">;
export const LLVMOrcMaterializationUnitRef_: Opaque<"pointer", "LLVMOrcMaterializationUnitRef"> = "pointer" as never;

// ./llvm-c/Orc.h:246:6
export type LLVMOrcMaterializationResponsibilityRef = Opaque<bigint, "LLVMOrcMaterializationResponsibilityRef">;
export const LLVMOrcMaterializationResponsibilityRef_: Opaque<"pointer", "LLVMOrcMaterializationResponsibilityRef"> = "pointer" as never;

// ./llvm-c/Orc.h:258:16
export type LLVMOrcMaterializationUnitMaterializeFunction = Opaque<bigint, "LLVMOrcMaterializationUnitMaterializeFunction">;
export const LLVMOrcMaterializationUnitMaterializeFunction_: Opaque<"function", "LLVMOrcMaterializationUnitMaterializeFunction"> = "function" as never;

// ./llvm-c/Orc.h:267:16
export type LLVMOrcMaterializationUnitDiscardFunction = Opaque<bigint, "LLVMOrcMaterializationUnitDiscardFunction">;
export const LLVMOrcMaterializationUnitDiscardFunction_: Opaque<"function", "LLVMOrcMaterializationUnitDiscardFunction"> = "function" as never;

// ./llvm-c/Orc.h:277:16
export type LLVMOrcMaterializationUnitDestroyFunction = Opaque<bigint, "LLVMOrcMaterializationUnitDestroyFunction">;
export const LLVMOrcMaterializationUnitDestroyFunction_: Opaque<"function", "LLVMOrcMaterializationUnitDestroyFunction"> = "function" as never;

// ./llvm-c/Orc.h:282:46
export type LLVMOrcResourceTrackerRef = Opaque<bigint, "LLVMOrcResourceTrackerRef">;
export const LLVMOrcResourceTrackerRef_: Opaque<"pointer", "LLVMOrcResourceTrackerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:288:6
export type LLVMOrcDefinitionGeneratorRef = Opaque<bigint, "LLVMOrcDefinitionGeneratorRef">;
export const LLVMOrcDefinitionGeneratorRef_: Opaque<"pointer", "LLVMOrcDefinitionGeneratorRef"> = "pointer" as never;

// ./llvm-c/Orc.h:302:42
export type LLVMOrcLookupStateRef = Opaque<bigint, "LLVMOrcLookupStateRef">;
export const LLVMOrcLookupStateRef_: Opaque<"pointer", "LLVMOrcLookupStateRef"> = "pointer" as never;

// ./llvm-c/Orc.h:337:24
export type LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction = Opaque<bigint, "LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction">;
export const LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction_: Opaque<"function", "LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction"> = "function" as never;

// ./llvm-c/Orc.h:346:15
export type LLVMOrcSymbolPredicate = Opaque<bigint, "LLVMOrcSymbolPredicate">;
export const LLVMOrcSymbolPredicate_: Opaque<"function", "LLVMOrcSymbolPredicate"> = "function" as never;

// ./llvm-c/Orc.h:352:48
export type LLVMOrcThreadSafeContextRef = Opaque<bigint, "LLVMOrcThreadSafeContextRef">;
export const LLVMOrcThreadSafeContextRef_: Opaque<"pointer", "LLVMOrcThreadSafeContextRef"> = "pointer" as never;

// ./llvm-c/Orc.h:357:47
export type LLVMOrcThreadSafeModuleRef = Opaque<bigint, "LLVMOrcThreadSafeModuleRef">;
export const LLVMOrcThreadSafeModuleRef_: Opaque<"pointer", "LLVMOrcThreadSafeModuleRef"> = "pointer" as never;

// ./llvm-c/Orc.h:363:24
export type LLVMOrcGenericIRModuleOperationFunction = Opaque<bigint, "LLVMOrcGenericIRModuleOperationFunction">;
export const LLVMOrcGenericIRModuleOperationFunction_: Opaque<"function", "LLVMOrcGenericIRModuleOperationFunction"> = "function" as never;

// ./llvm-c/Orc.h:370:6
export type LLVMOrcJITTargetMachineBuilderRef = Opaque<bigint, "LLVMOrcJITTargetMachineBuilderRef">;
export const LLVMOrcJITTargetMachineBuilderRef_: Opaque<"pointer", "LLVMOrcJITTargetMachineBuilderRef"> = "pointer" as never;

// ./llvm-c/Orc.h:375:42
export type LLVMOrcObjectLayerRef = Opaque<bigint, "LLVMOrcObjectLayerRef">;
export const LLVMOrcObjectLayerRef_: Opaque<"pointer", "LLVMOrcObjectLayerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:380:49
export type LLVMOrcObjectLinkingLayerRef = Opaque<bigint, "LLVMOrcObjectLinkingLayerRef">;
export const LLVMOrcObjectLinkingLayerRef_: Opaque<"pointer", "LLVMOrcObjectLinkingLayerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:385:47
export type LLVMOrcIRTransformLayerRef = Opaque<bigint, "LLVMOrcIRTransformLayerRef">;
export const LLVMOrcIRTransformLayerRef_: Opaque<"pointer", "LLVMOrcIRTransformLayerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:402:24
export type LLVMOrcIRTransformLayerTransformFunction = Opaque<bigint, "LLVMOrcIRTransformLayerTransformFunction">;
export const LLVMOrcIRTransformLayerTransformFunction_: Opaque<"function", "LLVMOrcIRTransformLayerTransformFunction"> = "function" as never;

// ./llvm-c/Orc.h:410:6
export type LLVMOrcObjectTransformLayerRef = Opaque<bigint, "LLVMOrcObjectTransformLayerRef">;
export const LLVMOrcObjectTransformLayerRef_: Opaque<"pointer", "LLVMOrcObjectTransformLayerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:425:24
export type LLVMOrcObjectTransformLayerTransformFunction = Opaque<bigint, "LLVMOrcObjectTransformLayerTransformFunction">;
export const LLVMOrcObjectTransformLayerTransformFunction_: Opaque<"function", "LLVMOrcObjectTransformLayerTransformFunction"> = "function" as never;

// ./llvm-c/Orc.h:432:6
export type LLVMOrcIndirectStubsManagerRef = Opaque<bigint, "LLVMOrcIndirectStubsManagerRef">;
export const LLVMOrcIndirectStubsManagerRef_: Opaque<"pointer", "LLVMOrcIndirectStubsManagerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:438:6
export type LLVMOrcLazyCallThroughManagerRef = Opaque<bigint, "LLVMOrcLazyCallThroughManagerRef">;
export const LLVMOrcLazyCallThroughManagerRef_: Opaque<"pointer", "LLVMOrcLazyCallThroughManagerRef"> = "pointer" as never;

// ./llvm-c/Orc.h:446:42
export type LLVMOrcDumpObjectsRef = Opaque<bigint, "LLVMOrcDumpObjectsRef">;
export const LLVMOrcDumpObjectsRef_: Opaque<"pointer", "LLVMOrcDumpObjectsRef"> = "pointer" as never;

// ./llvm-c/Transforms/PassManagerBuilder.h:20:46
export type LLVMPassManagerBuilderRef = Opaque<bigint, "LLVMPassManagerBuilderRef">;
export const LLVMPassManagerBuilderRef_: Opaque<"pointer", "LLVMPassManagerBuilderRef"> = "pointer" as never;

// ./llvm-c/Transforms/PassBuilder.h:38:46
export type LLVMPassBuilderOptionsRef = Opaque<bigint, "LLVMPassBuilderOptionsRef">;
export const LLVMPassBuilderOptionsRef_: Opaque<"pointer", "LLVMPassBuilderOptionsRef"> = "pointer" as never;

// ./llvm-c/Linker.h:34:3
export type LLVMLinkerMode = Opaque<number, "LLVMLinkerMode">;
export const LLVMLinkerMode_: Opaque<"i32", "LLVMLinkerMode"> = "i32" as never;

// ./llvm-c/Remarks.h:57:40
export type LLVMRemarkStringRef = Opaque<bigint, "LLVMRemarkStringRef">;
export const LLVMRemarkStringRef_: Opaque<"pointer", "LLVMRemarkStringRef"> = "pointer" as never;

// ./llvm-c/Remarks.h:78:42
export type LLVMRemarkDebugLocRef = Opaque<bigint, "LLVMRemarkDebugLocRef">;
export const LLVMRemarkDebugLocRef_: Opaque<"pointer", "LLVMRemarkDebugLocRef"> = "pointer" as never;

// ./llvm-c/Remarks.h:109:37
export type LLVMRemarkArgRef = Opaque<bigint, "LLVMRemarkArgRef">;
export const LLVMRemarkArgRef_: Opaque<"pointer", "LLVMRemarkArgRef"> = "pointer" as never;

// ./llvm-c/Remarks.h:140:39
export type LLVMRemarkEntryRef = Opaque<bigint, "LLVMRemarkEntryRef">;
export const LLVMRemarkEntryRef_: Opaque<"pointer", "LLVMRemarkEntryRef"> = "pointer" as never;

// ./llvm-c/Remarks.h:230:40
export type LLVMRemarkParserRef = Opaque<bigint, "LLVMRemarkParserRef">;
export const LLVMRemarkParserRef_: Opaque<"pointer", "LLVMRemarkParserRef"> = "pointer" as never;

// ./llvm-c/ErrorHandling.h:27:16
export type LLVMFatalErrorHandler = Opaque<bigint, "LLVMFatalErrorHandler">;
export const LLVMFatalErrorHandler_: Opaque<"function", "LLVMFatalErrorHandler"> = "function" as never;

// ./llvm-c/Core.h:146:3
export type LLVMOpcode = Opaque<number, "LLVMOpcode">;
export const LLVMOpcode_: Opaque<"i32", "LLVMOpcode"> = "i32" as never;

// ./llvm-c/Core.h:169:3
export type LLVMTypeKind = Opaque<number, "LLVMTypeKind">;
export const LLVMTypeKind_: Opaque<"i32", "LLVMTypeKind"> = "i32" as never;

// ./llvm-c/Core.h:192:3
export type LLVMLinkage = Opaque<number, "LLVMLinkage">;
export const LLVMLinkage_: Opaque<"i32", "LLVMLinkage"> = "i32" as never;

// ./llvm-c/Core.h:198:3
export type LLVMVisibility = Opaque<number, "LLVMVisibility">;
export const LLVMVisibility_: Opaque<"i32", "LLVMVisibility"> = "i32" as never;

// ./llvm-c/Core.h:204:3
export type LLVMUnnamedAddr = Opaque<number, "LLVMUnnamedAddr">;
export const LLVMUnnamedAddr_: Opaque<"i32", "LLVMUnnamedAddr"> = "i32" as never;

// ./llvm-c/Core.h:210:3
export type LLVMDLLStorageClass = Opaque<number, "LLVMDLLStorageClass">;
export const LLVMDLLStorageClass_: Opaque<"i32", "LLVMDLLStorageClass"> = "i32" as never;

// ./llvm-c/Core.h:255:3
export type LLVMCallConv = Opaque<number, "LLVMCallConv">;
export const LLVMCallConv_: Opaque<"i32", "LLVMCallConv"> = "i32" as never;

// ./llvm-c/Core.h:288:3
export type LLVMValueKind = Opaque<number, "LLVMValueKind">;
export const LLVMValueKind_: Opaque<"i32", "LLVMValueKind"> = "i32" as never;

// ./llvm-c/Core.h:301:3
export type LLVMIntPredicate = Opaque<number, "LLVMIntPredicate">;
export const LLVMIntPredicate_: Opaque<"i32", "LLVMIntPredicate"> = "i32" as never;

// ./llvm-c/Core.h:320:3
export type LLVMRealPredicate = Opaque<number, "LLVMRealPredicate">;
export const LLVMRealPredicate_: Opaque<"i32", "LLVMRealPredicate"> = "i32" as never;

// ./llvm-c/Core.h:325:3
export type LLVMLandingPadClauseTy = Opaque<number, "LLVMLandingPadClauseTy">;
export const LLVMLandingPadClauseTy_: Opaque<"i32", "LLVMLandingPadClauseTy"> = "i32" as never;

// ./llvm-c/Core.h:333:3
export type LLVMThreadLocalMode = Opaque<number, "LLVMThreadLocalMode">;
export const LLVMThreadLocalMode_: Opaque<"i32", "LLVMThreadLocalMode"> = "i32" as never;

// ./llvm-c/Core.h:360:3
export type LLVMAtomicOrdering = Opaque<number, "LLVMAtomicOrdering">;
export const LLVMAtomicOrdering_: Opaque<"i32", "LLVMAtomicOrdering"> = "i32" as never;

// ./llvm-c/Core.h:392:3
export type LLVMAtomicRMWBinOp = Opaque<number, "LLVMAtomicRMWBinOp">;
export const LLVMAtomicRMWBinOp_: Opaque<"i32", "LLVMAtomicRMWBinOp"> = "i32" as never;

// ./llvm-c/Core.h:399:3
export type LLVMDiagnosticSeverity = Opaque<number, "LLVMDiagnosticSeverity">;
export const LLVMDiagnosticSeverity_: Opaque<"i32", "LLVMDiagnosticSeverity"> = "i32" as never;

// ./llvm-c/Core.h:404:3
export type LLVMInlineAsmDialect = Opaque<number, "LLVMInlineAsmDialect">;
export const LLVMInlineAsmDialect_: Opaque<"i32", "LLVMInlineAsmDialect"> = "i32" as never;

// ./llvm-c/Core.h:455:3
export type LLVMModuleFlagBehavior = Opaque<number, "LLVMModuleFlagBehavior">;
export const LLVMModuleFlagBehavior_: Opaque<"i32", "LLVMModuleFlagBehavior"> = "i32" as never;

// ./llvm-c/Core.h:469:18
export type LLVMAttributeIndex = Opaque<number, "LLVMAttributeIndex">;
export const LLVMAttributeIndex_: Opaque<"u32", "LLVMAttributeIndex"> = "u32" as never;

// ./llvm-c/Core.h:499:16
export type LLVMDiagnosticHandler = Opaque<bigint, "LLVMDiagnosticHandler">;
export const LLVMDiagnosticHandler_: Opaque<"function", "LLVMDiagnosticHandler"> = "function" as never;

// ./llvm-c/Core.h:500:16
export type LLVMYieldCallback = Opaque<bigint, "LLVMYieldCallback">;
export const LLVMYieldCallback_: Opaque<"function", "LLVMYieldCallback"> = "function" as never;

// ./llvm-c/Object.h:36:43
export type LLVMSectionIteratorRef = Opaque<bigint, "LLVMSectionIteratorRef">;
export const LLVMSectionIteratorRef_: Opaque<"pointer", "LLVMSectionIteratorRef"> = "pointer" as never;

// ./llvm-c/Object.h:37:42
export type LLVMSymbolIteratorRef = Opaque<bigint, "LLVMSymbolIteratorRef">;
export const LLVMSymbolIteratorRef_: Opaque<"pointer", "LLVMSymbolIteratorRef"> = "pointer" as never;

// ./llvm-c/Object.h:38:46
export type LLVMRelocationIteratorRef = Opaque<bigint, "LLVMRelocationIteratorRef">;
export const LLVMRelocationIteratorRef_: Opaque<"pointer", "LLVMRelocationIteratorRef"> = "pointer" as never;

// ./llvm-c/Object.h:58:3
export type LLVMBinaryType = Opaque<number, "LLVMBinaryType">;
export const LLVMBinaryType_: Opaque<"i32", "LLVMBinaryType"> = "i32" as never;

// ./llvm-c/Object.h:205:38
export type LLVMObjectFileRef = Opaque<bigint, "LLVMObjectFileRef">;
export const LLVMObjectFileRef_: Opaque<"pointer", "LLVMObjectFileRef"> = "pointer" as never;

// ./llvm-c/LLJIT.h:55:6
export type LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction = Opaque<bigint, "LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction">;
export const LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction_: Opaque<"function", "LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction"> = "function" as never;

// ./llvm-c/LLJIT.h:61:43
export type LLVMOrcLLJITBuilderRef = Opaque<bigint, "LLVMOrcLLJITBuilderRef">;
export const LLVMOrcLLJITBuilderRef_: Opaque<"pointer", "LLVMOrcLLJITBuilderRef"> = "pointer" as never;

// ./llvm-c/LLJIT.h:66:36
export type LLVMOrcLLJITRef = Opaque<bigint, "LLVMOrcLLJITRef">;
export const LLVMOrcLLJITRef_: Opaque<"pointer", "LLVMOrcLLJITRef"> = "pointer" as never;

// ./llvm-c/Comdat.h:38:3
export type LLVMComdatSelectionKind = Opaque<number, "LLVMComdatSelectionKind">;
export const LLVMComdatSelectionKind_: Opaque<"i32", "LLVMComdatSelectionKind"> = "i32" as never;
