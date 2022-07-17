// deno-lint-ignore-file
import { Opaque, Pointer, FnPointer, StructPointer } from "./safe-ffi.ts";

export namespace LLVM {
  /** ./llvm-c/Types.h:28:13 */
  export type Bool = Opaque<number, "Bool">;

  /** ./llvm-c/Types.h:48:40 */
  export type MemoryBufferRef = Pointer<"MemoryBufferRef">;

  /** ./llvm-c/Types.h:53:35 */
  export type ContextRef = Pointer<"ContextRef">;

  /** ./llvm-c/Types.h:61:34 */
  export type ModuleRef = Pointer<"ModuleRef">;

  /** ./llvm-c/Types.h:68:32 */
  export type TypeRef = Pointer<"TypeRef">;

  /** ./llvm-c/Types.h:75:33 */
  export type ValueRef = Pointer<"ValueRef">;

  /** ./llvm-c/Types.h:82:38 */
  export type BasicBlockRef = Pointer<"BasicBlockRef">;

  /** ./llvm-c/Types.h:89:36 */
  export type MetadataRef = Pointer<"MetadataRef">;

  /** ./llvm-c/Types.h:96:39 */
  export type NamedMDNodeRef = Pointer<"NamedMDNodeRef">;

  /** ./llvm-c/Types.h:103:45 */
  export type ValueMetadataEntry = StructPointer<"ValueMetadataEntry">;

  /** ./llvm-c/Types.h:110:35 */
  export type BuilderRef = Pointer<"BuilderRef">;

  /** ./llvm-c/Types.h:117:37 */
  export type DIBuilderRef = Pointer<"DIBuilderRef">;

  /** ./llvm-c/Types.h:124:42 */
  export type ModuleProviderRef = Pointer<"ModuleProviderRef">;

  /** ./llvm-c/Types.h:127:39 */
  export type PassManagerRef = Pointer<"PassManagerRef">;

  /** ./llvm-c/Types.h:130:40 */
  export type PassRegistryRef = Pointer<"PassRegistryRef">;

  /** ./llvm-c/Types.h:136:31 */
  export type UseRef = Pointer<"UseRef">;

  /** ./llvm-c/Types.h:143:40 */
  export type AttributeRef = Pointer<"AttributeRef">;

  /** ./llvm-c/Types.h:148:42 */
  export type DiagnosticInfoRef = Pointer<"DiagnosticInfoRef">;

  /** ./llvm-c/Types.h:153:28 */
  export type ComdatRef = Pointer<"ComdatRef">;

  /** ./llvm-c/Types.h:158:42 */
  export type ModuleFlagEntry = StructPointer<"ModuleFlagEntry">;

  /** ./llvm-c/Types.h:163:44 */
  export type JITEventListenerRef = Pointer<"JITEventListenerRef">;

  /** ./llvm-c/Types.h:168:34 */
  export type BinaryRef = Pointer<"BinaryRef">;

  /** ./llvm-c/Target.h:37:38 */
  export type TargetDataRef = Pointer<"TargetDataRef">;

  /** ./llvm-c/Target.h:38:50 */
  export type TargetLibraryInfoRef = Pointer<"TargetLibraryInfoRef">;

  /** ./llvm-c/TargetMachine.h:34:41 */
  export type TargetMachineRef = Pointer<"TargetMachineRef">;

  /** ./llvm-c/TargetMachine.h:35:28 */
  export type TargetRef = Pointer<"TargetRef">;

  /** ./llvm-c/ExecutionEngine.h:39:40 */
  export type GenericValueRef = Pointer<"GenericValueRef">;

  /** ./llvm-c/ExecutionEngine.h:40:43 */
  export type ExecutionEngineRef = Pointer<"ExecutionEngineRef">;

  /** ./llvm-c/ExecutionEngine.h:41:46 */
  export type MCJITMemoryManagerRef = Pointer<"MCJITMemoryManagerRef">;

  /** ./llvm-c/ExecutionEngine.h:159:20 */
  export type MemoryManagerAllocateCodeSectionCallback = FnPointer<"MemoryManagerAllocateCodeSectionCallback">;

  /** ./llvm-c/ExecutionEngine.h:162:20 */
  export type MemoryManagerAllocateDataSectionCallback = FnPointer<"MemoryManagerAllocateDataSectionCallback">;

  /** ./llvm-c/ExecutionEngine.h:165:20 */
  export type MemoryManagerFinalizeMemoryCallback = FnPointer<"MemoryManagerFinalizeMemoryCallback">;

  /** ./llvm-c/ExecutionEngine.h:167:16 */
  export type MemoryManagerDestroyCallback = FnPointer<"MemoryManagerDestroyCallback">;

  /** ./llvm-c/DisassemblerTypes.h:29:15 */
  export type DisasmContextRef = Pointer<"DisasmContextRef">;

  /** ./llvm-c/DisassemblerTypes.h:48:15 */
  export type OpInfoCallback = FnPointer<"OpInfoCallback">;

  /** ./llvm-c/DisassemblerTypes.h:118:23 */
  export type SymbolLookupCallback = FnPointer<"SymbolLookupCallback">;

  /** ./llvm-c/DebugInfo.h:174:18 */
  export type MetadataKind = Opaque<number, "MetadataKind">;

  /** ./llvm-c/DebugInfo.h:179:18 */
  export type DWARFTypeEncoding = Opaque<number, "DWARFTypeEncoding">;

  /** ./llvm-c/Error.h:33:33 */
  export type ErrorRef = Pointer<"ErrorRef">;

  /** ./llvm-c/Error.h:38:21 */
  export type ErrorTypeId = Pointer<"ErrorTypeId">;

  /** ./llvm-c/Orc.h:46:18 */
  export type OrcJITTargetAddress = Opaque<bigint, "OrcJITTargetAddress">;

  /** ./llvm-c/Orc.h:51:18 */
  export type OrcExecutorAddress = Opaque<bigint, "OrcExecutorAddress">;

  /** ./llvm-c/Orc.h:66:17 */
  export type JITSymbolTargetFlags = Opaque<number, "JITSymbolTargetFlags">;

  /** ./llvm-c/Orc.h:74:3 */
  export type JITSymbolFlags = StructPointer<"JITSymbolFlags">;

  /** ./llvm-c/Orc.h:82:3 */
  export type JITEvaluatedSymbol = StructPointer<"JITEvaluatedSymbol">;

  /** ./llvm-c/Orc.h:87:47 */
  export type OrcExecutionSessionRef = Pointer<"OrcExecutionSessionRef">;

  /** ./llvm-c/Orc.h:92:16 */
  export type OrcErrorReporterFunction = FnPointer<"OrcErrorReporterFunction">;

  /** ./llvm-c/Orc.h:97:47 */
  export type OrcSymbolStringPoolRef = Pointer<"OrcSymbolStringPoolRef">;

  /** ./llvm-c/Orc.h:103:6 */
  export type OrcSymbolStringPoolEntryRef = Pointer<"OrcSymbolStringPoolEntryRef">;

  /** ./llvm-c/Orc.h:111:3 */
  export type OrcCSymbolFlagsMapPair = StructPointer<"OrcCSymbolFlagsMapPair">;

  /** ./llvm-c/Orc.h:117:37 */
  export type OrcCSymbolFlagsMapPairs = Pointer<"OrcCSymbolFlagsMapPairs">;

  /** ./llvm-c/Orc.h:125:3 */
  export type JITCSymbolMapPair = StructPointer<"JITCSymbolMapPair">;

  /** ./llvm-c/Orc.h:131:32 */
  export type OrcCSymbolMapPairs = Pointer<"OrcCSymbolMapPairs">;

  /** ./llvm-c/Orc.h:139:3 */
  export type OrcCSymbolAliasMapEntry = StructPointer<"OrcCSymbolAliasMapEntry">;

  /** ./llvm-c/Orc.h:147:3 */
  export type OrcCSymbolAliasMapPair = StructPointer<"OrcCSymbolAliasMapPair">;

  /** ./llvm-c/Orc.h:153:37 */
  export type OrcCSymbolAliasMapPairs = Pointer<"OrcCSymbolAliasMapPairs">;

  /** ./llvm-c/Orc.h:158:39 */
  export type OrcJITDylibRef = Pointer<"OrcJITDylibRef">;

  /** ./llvm-c/Orc.h:167:3 */
  export type OrcCSymbolsList = StructPointer<"OrcCSymbolsList">;

  /** ./llvm-c/Orc.h:175:3 */
  export type OrcCDependenceMapPair = StructPointer<"OrcCDependenceMapPair">;

  /** ./llvm-c/Orc.h:181:36 */
  export type OrcCDependenceMapPairs = Pointer<"OrcCDependenceMapPairs">;

  /** ./llvm-c/Orc.h:220:3 */
  export type OrcCLookupSetElement = StructPointer<"OrcCLookupSetElement">;

  /** ./llvm-c/Orc.h:233:35 */
  export type OrcCLookupSet = Pointer<"OrcCLookupSet">;

  /** ./llvm-c/Orc.h:238:50 */
  export type OrcMaterializationUnitRef = Pointer<"OrcMaterializationUnitRef">;

  /** ./llvm-c/Orc.h:246:6 */
  export type OrcMaterializationResponsibilityRef = Pointer<"OrcMaterializationResponsibilityRef">;

  /** ./llvm-c/Orc.h:258:16 */
  export type OrcMaterializationUnitMaterializeFunction = FnPointer<"OrcMaterializationUnitMaterializeFunction">;

  /** ./llvm-c/Orc.h:267:16 */
  export type OrcMaterializationUnitDiscardFunction = FnPointer<"OrcMaterializationUnitDiscardFunction">;

  /** ./llvm-c/Orc.h:277:16 */
  export type OrcMaterializationUnitDestroyFunction = FnPointer<"OrcMaterializationUnitDestroyFunction">;

  /** ./llvm-c/Orc.h:282:46 */
  export type OrcResourceTrackerRef = Pointer<"OrcResourceTrackerRef">;

  /** ./llvm-c/Orc.h:288:6 */
  export type OrcDefinitionGeneratorRef = Pointer<"OrcDefinitionGeneratorRef">;

  /** ./llvm-c/Orc.h:302:42 */
  export type OrcLookupStateRef = Pointer<"OrcLookupStateRef">;

  /** ./llvm-c/Orc.h:337:24 */
  export type OrcCAPIDefinitionGeneratorTryToGenerateFunction = FnPointer<"OrcCAPIDefinitionGeneratorTryToGenerateFunction">;

  /** ./llvm-c/Orc.h:346:15 */
  export type OrcSymbolPredicate = FnPointer<"OrcSymbolPredicate">;

  /** ./llvm-c/Orc.h:352:48 */
  export type OrcThreadSafeContextRef = Pointer<"OrcThreadSafeContextRef">;

  /** ./llvm-c/Orc.h:357:47 */
  export type OrcThreadSafeModuleRef = Pointer<"OrcThreadSafeModuleRef">;

  /** ./llvm-c/Orc.h:363:24 */
  export type OrcGenericIRModuleOperationFunction = FnPointer<"OrcGenericIRModuleOperationFunction">;

  /** ./llvm-c/Orc.h:370:6 */
  export type OrcJITTargetMachineBuilderRef = Pointer<"OrcJITTargetMachineBuilderRef">;

  /** ./llvm-c/Orc.h:375:42 */
  export type OrcObjectLayerRef = Pointer<"OrcObjectLayerRef">;

  /** ./llvm-c/Orc.h:380:49 */
  export type OrcObjectLinkingLayerRef = Pointer<"OrcObjectLinkingLayerRef">;

  /** ./llvm-c/Orc.h:385:47 */
  export type OrcIRTransformLayerRef = Pointer<"OrcIRTransformLayerRef">;

  /** ./llvm-c/Orc.h:402:24 */
  export type OrcIRTransformLayerTransformFunction = FnPointer<"OrcIRTransformLayerTransformFunction">;

  /** ./llvm-c/Orc.h:410:6 */
  export type OrcObjectTransformLayerRef = Pointer<"OrcObjectTransformLayerRef">;

  /** ./llvm-c/Orc.h:425:24 */
  export type OrcObjectTransformLayerTransformFunction = FnPointer<"OrcObjectTransformLayerTransformFunction">;

  /** ./llvm-c/Orc.h:432:6 */
  export type OrcIndirectStubsManagerRef = Pointer<"OrcIndirectStubsManagerRef">;

  /** ./llvm-c/Orc.h:438:6 */
  export type OrcLazyCallThroughManagerRef = Pointer<"OrcLazyCallThroughManagerRef">;

  /** ./llvm-c/Orc.h:446:42 */
  export type OrcDumpObjectsRef = Pointer<"OrcDumpObjectsRef">;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:20:46 */
  export type PassManagerBuilderRef = Pointer<"PassManagerBuilderRef">;

  /** ./llvm-c/Transforms/PassBuilder.h:38:46 */
  export type PassBuilderOptionsRef = Pointer<"PassBuilderOptionsRef">;

  /** ./llvm-c/Remarks.h:57:40 */
  export type RemarkStringRef = Pointer<"RemarkStringRef">;

  /** ./llvm-c/Remarks.h:78:42 */
  export type RemarkDebugLocRef = Pointer<"RemarkDebugLocRef">;

  /** ./llvm-c/Remarks.h:109:37 */
  export type RemarkArgRef = Pointer<"RemarkArgRef">;

  /** ./llvm-c/Remarks.h:140:39 */
  export type RemarkEntryRef = Pointer<"RemarkEntryRef">;

  /** ./llvm-c/Remarks.h:230:40 */
  export type RemarkParserRef = Pointer<"RemarkParserRef">;

  /** ./llvm-c/ErrorHandling.h:27:16 */
  export type FatalErrorHandler = FnPointer<"FatalErrorHandler">;

  /** ./llvm-c/Core.h:469:18 */
  export type AttributeIndex = Opaque<number, "AttributeIndex">;

  /** ./llvm-c/Core.h:499:16 */
  export type DiagnosticHandler = FnPointer<"DiagnosticHandler">;

  /** ./llvm-c/Core.h:500:16 */
  export type YieldCallback = FnPointer<"YieldCallback">;

  /** ./llvm-c/Object.h:36:43 */
  export type SectionIteratorRef = Pointer<"SectionIteratorRef">;

  /** ./llvm-c/Object.h:37:42 */
  export type SymbolIteratorRef = Pointer<"SymbolIteratorRef">;

  /** ./llvm-c/Object.h:38:46 */
  export type RelocationIteratorRef = Pointer<"RelocationIteratorRef">;

  /** ./llvm-c/Object.h:205:38 */
  export type ObjectFileRef = Pointer<"ObjectFileRef">;

  /** ./llvm-c/LLJIT.h:55:6 */
  export type OrcLLJITBuilderObjectLinkingLayerCreatorFunction = FnPointer<"OrcLLJITBuilderObjectLinkingLayerCreatorFunction">;

  /** ./llvm-c/LLJIT.h:61:43 */
  export type OrcLLJITBuilderRef = Pointer<"OrcLLJITBuilderRef">;

  /** ./llvm-c/LLJIT.h:66:36 */
  export type OrcLLJITRef = Pointer<"OrcLLJITRef">;

  /** ./llvm-c/Analysis.h:34:9 */
  export enum VerifierFailureAction {
    LLVMAbortProcessAction = 0,
    LLVMPrintMessageAction = 1,
    LLVMReturnStatusAction = 2,
  }

  /** ./llvm-c/Target.h:35:6 */
  export enum ByteOrdering {
    LLVMBigEndian = 0,
    LLVMLittleEndian = 1,
  }

  /** ./llvm-c/TargetMachine.h:37:9 */
  export enum CodeGenOptLevel {
    LLVMCodeGenLevelNone = 0,
    LLVMCodeGenLevelLess = 1,
    LLVMCodeGenLevelDefault = 2,
    LLVMCodeGenLevelAggressive = 3,
  }

  /** ./llvm-c/TargetMachine.h:44:9 */
  export enum RelocMode {
    LLVMRelocDefault = 0,
    LLVMRelocStatic = 1,
    LLVMRelocPIC = 2,
    LLVMRelocDynamicNoPic = 3,
    LLVMRelocROPI = 4,
    LLVMRelocRWPI = 5,
    LLVMRelocROPI_RWPI = 6,
  }

  /** ./llvm-c/TargetMachine.h:54:9 */
  export enum CodeModel {
    LLVMCodeModelDefault = 0,
    LLVMCodeModelJITDefault = 1,
    LLVMCodeModelTiny = 2,
    LLVMCodeModelSmall = 3,
    LLVMCodeModelKernel = 4,
    LLVMCodeModelMedium = 5,
    LLVMCodeModelLarge = 6,
  }

  /** ./llvm-c/TargetMachine.h:64:9 */
  export enum CodeGenFileType {
    LLVMAssemblyFile = 0,
    LLVMObjectFile = 1,
  }

  /** ./llvm-c/DebugInfo.h:34:9 */
  export enum DIFlags {
    LLVMDIFlagZero = 0,
    LLVMDIFlagPrivate = 1,
    LLVMDIFlagProtected = 2,
    LLVMDIFlagPublic = 3,
    LLVMDIFlagFwdDecl = 4,
    LLVMDIFlagAppleBlock = 8,
    LLVMDIFlagReservedBit4 = 16,
    LLVMDIFlagVirtual = 32,
    LLVMDIFlagArtificial = 64,
    LLVMDIFlagExplicit = 128,
    LLVMDIFlagPrototyped = 256,
    LLVMDIFlagObjcClassComplete = 512,
    LLVMDIFlagObjectPointer = 1024,
    LLVMDIFlagVector = 2048,
    LLVMDIFlagStaticMember = 4096,
    LLVMDIFlagLValueReference = 8192,
    LLVMDIFlagRValueReference = 16384,
    LLVMDIFlagReserved = 32768,
    LLVMDIFlagSingleInheritance = 65536,
    LLVMDIFlagMultipleInheritance = 131072,
    LLVMDIFlagVirtualInheritance = 196608,
    LLVMDIFlagIntroducedVirtual = 262144,
    LLVMDIFlagBitField = 524288,
    LLVMDIFlagNoReturn = 1048576,
    LLVMDIFlagTypePassByValue = 4194304,
    LLVMDIFlagTypePassByReference = 8388608,
    LLVMDIFlagEnumClass = 16777216,
    LLVMDIFlagFixedEnum = 16777216,
    LLVMDIFlagThunk = 33554432,
    LLVMDIFlagNonTrivial = 67108864,
    LLVMDIFlagBigEndian = 134217728,
    LLVMDIFlagLittleEndian = 268435456,
    LLVMDIFlagIndirectVirtualBase = 36,
    LLVMDIFlagAccessibility = 3,
    LLVMDIFlagPtrToMemberRep = 196608,
  }

  /** ./llvm-c/DebugInfo.h:78:9 */
  export enum DWARFSourceLanguage {
    LLVMDWARFSourceLanguageC89 = 0,
    LLVMDWARFSourceLanguageC = 1,
    LLVMDWARFSourceLanguageAda83 = 2,
    LLVMDWARFSourceLanguageC_plus_plus = 3,
    LLVMDWARFSourceLanguageCobol74 = 4,
    LLVMDWARFSourceLanguageCobol85 = 5,
    LLVMDWARFSourceLanguageFortran77 = 6,
    LLVMDWARFSourceLanguageFortran90 = 7,
    LLVMDWARFSourceLanguagePascal83 = 8,
    LLVMDWARFSourceLanguageModula2 = 9,
    LLVMDWARFSourceLanguageJava = 10,
    LLVMDWARFSourceLanguageC99 = 11,
    LLVMDWARFSourceLanguageAda95 = 12,
    LLVMDWARFSourceLanguageFortran95 = 13,
    LLVMDWARFSourceLanguagePLI = 14,
    LLVMDWARFSourceLanguageObjC = 15,
    LLVMDWARFSourceLanguageObjC_plus_plus = 16,
    LLVMDWARFSourceLanguageUPC = 17,
    LLVMDWARFSourceLanguageD = 18,
    LLVMDWARFSourceLanguagePython = 19,
    LLVMDWARFSourceLanguageOpenCL = 20,
    LLVMDWARFSourceLanguageGo = 21,
    LLVMDWARFSourceLanguageModula3 = 22,
    LLVMDWARFSourceLanguageHaskell = 23,
    LLVMDWARFSourceLanguageC_plus_plus_03 = 24,
    LLVMDWARFSourceLanguageC_plus_plus_11 = 25,
    LLVMDWARFSourceLanguageOCaml = 26,
    LLVMDWARFSourceLanguageRust = 27,
    LLVMDWARFSourceLanguageC11 = 28,
    LLVMDWARFSourceLanguageSwift = 29,
    LLVMDWARFSourceLanguageJulia = 30,
    LLVMDWARFSourceLanguageDylan = 31,
    LLVMDWARFSourceLanguageC_plus_plus_14 = 32,
    LLVMDWARFSourceLanguageFortran03 = 33,
    LLVMDWARFSourceLanguageFortran08 = 34,
    LLVMDWARFSourceLanguageRenderScript = 35,
    LLVMDWARFSourceLanguageBLISS = 36,
    LLVMDWARFSourceLanguageMips_Assembler = 37,
    LLVMDWARFSourceLanguageGOOGLE_RenderScript = 38,
    LLVMDWARFSourceLanguageBORLAND_Delphi = 39,
  }

  /** ./llvm-c/DebugInfo.h:128:9 */
  export enum DWARFEmissionKind {
    LLVMDWARFEmissionNone = 0,
    LLVMDWARFEmissionFull = 1,
    LLVMDWARFEmissionLineTablesOnly = 2,
  }

  /** ./llvm-c/DebugInfo.h:186:9 */
  export enum DWARFMacinfoRecordType {
    LLVMDWARFMacinfoRecordTypeDefine = 1,
    LLVMDWARFMacinfoRecordTypeMacro = 2,
    LLVMDWARFMacinfoRecordTypeStartFile = 3,
    LLVMDWARFMacinfoRecordTypeEndFile = 4,
    LLVMDWARFMacinfoRecordTypeVendorExt = 255,
  }

  /** ./llvm-c/Orc.h:56:9 */
  export enum JITSymbolGenericFlags {
    LLVMJITSymbolGenericFlagsExported = 1,
    LLVMJITSymbolGenericFlagsWeak = 2,
    LLVMJITSymbolGenericFlagsCallable = 4,
    LLVMJITSymbolGenericFlagsMaterializationSideEffectsOnly = 8,
  }

  /** ./llvm-c/Orc.h:189:9 */
  export enum OrcLookupKind {
    LLVMOrcLookupKindStatic = 0,
    LLVMOrcLookupKindDLSym = 1,
  }

  /** ./llvm-c/Orc.h:200:9 */
  export enum OrcJITDylibLookupFlags {
    LLVMOrcJITDylibLookupFlagsMatchExportedSymbolsOnly = 0,
    LLVMOrcJITDylibLookupFlagsMatchAllSymbols = 1,
  }

  /** ./llvm-c/Orc.h:209:9 */
  export enum OrcSymbolLookupFlags {
    LLVMOrcSymbolLookupFlagsRequiredSymbol = 0,
    LLVMOrcSymbolLookupFlagsWeaklyReferencedSymbol = 1,
  }

  /** ./llvm-c/Linker.h:30:9 */
  export enum LinkerMode {
    LLVMLinkerDestroySource = 0,
    LLVMLinkerPreserveSource_Removed = 1,
  }

  /** ./llvm-c/Remarks.h:41:6 */
  export enum RemarkType {
    LLVMRemarkTypeUnknown = 0,
    LLVMRemarkTypePassed = 1,
    LLVMRemarkTypeMissed = 2,
    LLVMRemarkTypeAnalysis = 3,
    LLVMRemarkTypeAnalysisFPCommute = 4,
    LLVMRemarkTypeAnalysisAliasing = 5,
    LLVMRemarkTypeFailure = 6,
  }

  /** ./llvm-c/Core.h:60:9 */
  export enum Opcode {
    LLVMRet = 1,
    LLVMBr = 2,
    LLVMSwitch = 3,
    LLVMIndirectBr = 4,
    LLVMInvoke = 5,
    LLVMUnreachable = 7,
    LLVMCallBr = 67,
    LLVMFNeg = 66,
    LLVMAdd = 8,
    LLVMFAdd = 9,
    LLVMSub = 10,
    LLVMFSub = 11,
    LLVMMul = 12,
    LLVMFMul = 13,
    LLVMUDiv = 14,
    LLVMSDiv = 15,
    LLVMFDiv = 16,
    LLVMURem = 17,
    LLVMSRem = 18,
    LLVMFRem = 19,
    LLVMShl = 20,
    LLVMLShr = 21,
    LLVMAShr = 22,
    LLVMAnd = 23,
    LLVMOr = 24,
    LLVMXor = 25,
    LLVMAlloca = 26,
    LLVMLoad = 27,
    LLVMStore = 28,
    LLVMGetElementPtr = 29,
    LLVMTrunc = 30,
    LLVMZExt = 31,
    LLVMSExt = 32,
    LLVMFPToUI = 33,
    LLVMFPToSI = 34,
    LLVMUIToFP = 35,
    LLVMSIToFP = 36,
    LLVMFPTrunc = 37,
    LLVMFPExt = 38,
    LLVMPtrToInt = 39,
    LLVMIntToPtr = 40,
    LLVMBitCast = 41,
    LLVMAddrSpaceCast = 60,
    LLVMICmp = 42,
    LLVMFCmp = 43,
    LLVMPHI = 44,
    LLVMCall = 45,
    LLVMSelect = 46,
    LLVMUserOp1 = 47,
    LLVMUserOp2 = 48,
    LLVMVAArg = 49,
    LLVMExtractElement = 50,
    LLVMInsertElement = 51,
    LLVMShuffleVector = 52,
    LLVMExtractValue = 53,
    LLVMInsertValue = 54,
    LLVMFreeze = 68,
    LLVMFence = 55,
    LLVMAtomicCmpXchg = 56,
    LLVMAtomicRMW = 57,
    LLVMResume = 58,
    LLVMLandingPad = 59,
    LLVMCleanupRet = 61,
    LLVMCatchRet = 62,
    LLVMCatchPad = 63,
    LLVMCleanupPad = 64,
    LLVMCatchSwitch = 65,
  }

  /** ./llvm-c/Core.h:148:9 */
  export enum TypeKind {
    LLVMVoidTypeKind = 0,
    LLVMHalfTypeKind = 1,
    LLVMFloatTypeKind = 2,
    LLVMDoubleTypeKind = 3,
    LLVMX86_FP80TypeKind = 4,
    LLVMFP128TypeKind = 5,
    LLVMPPC_FP128TypeKind = 6,
    LLVMLabelTypeKind = 7,
    LLVMIntegerTypeKind = 8,
    LLVMFunctionTypeKind = 9,
    LLVMStructTypeKind = 10,
    LLVMArrayTypeKind = 11,
    LLVMPointerTypeKind = 12,
    LLVMVectorTypeKind = 13,
    LLVMMetadataTypeKind = 14,
    LLVMX86_MMXTypeKind = 15,
    LLVMTokenTypeKind = 16,
    LLVMScalableVectorTypeKind = 17,
    LLVMBFloatTypeKind = 18,
    LLVMX86_AMXTypeKind = 19,
  }

  /** ./llvm-c/Core.h:171:9 */
  export enum Linkage {
    LLVMExternalLinkage = 0,
    LLVMAvailableExternallyLinkage = 1,
    LLVMLinkOnceAnyLinkage = 2,
    LLVMLinkOnceODRLinkage = 3,
    LLVMLinkOnceODRAutoHideLinkage = 4,
    LLVMWeakAnyLinkage = 5,
    LLVMWeakODRLinkage = 6,
    LLVMAppendingLinkage = 7,
    LLVMInternalLinkage = 8,
    LLVMPrivateLinkage = 9,
    LLVMDLLImportLinkage = 10,
    LLVMDLLExportLinkage = 11,
    LLVMExternalWeakLinkage = 12,
    LLVMGhostLinkage = 13,
    LLVMCommonLinkage = 14,
    LLVMLinkerPrivateLinkage = 15,
    LLVMLinkerPrivateWeakLinkage = 16,
  }

  /** ./llvm-c/Core.h:194:9 */
  export enum Visibility {
    LLVMDefaultVisibility = 0,
    LLVMHiddenVisibility = 1,
    LLVMProtectedVisibility = 2,
  }

  /** ./llvm-c/Core.h:200:9 */
  export enum UnnamedAddr {
    LLVMNoUnnamedAddr = 0,
    LLVMLocalUnnamedAddr = 1,
    LLVMGlobalUnnamedAddr = 2,
  }

  /** ./llvm-c/Core.h:206:9 */
  export enum DLLStorageClass {
    LLVMDefaultStorageClass = 0,
    LLVMDLLImportStorageClass = 1,
    LLVMDLLExportStorageClass = 2,
  }

  /** ./llvm-c/Core.h:212:9 */
  export enum CallConv {
    LLVMCCallConv = 0,
    LLVMFastCallConv = 8,
    LLVMColdCallConv = 9,
    LLVMGHCCallConv = 10,
    LLVMHiPECallConv = 11,
    LLVMWebKitJSCallConv = 12,
    LLVMAnyRegCallConv = 13,
    LLVMPreserveMostCallConv = 14,
    LLVMPreserveAllCallConv = 15,
    LLVMSwiftCallConv = 16,
    LLVMCXXFASTTLSCallConv = 17,
    LLVMX86StdcallCallConv = 64,
    LLVMX86FastcallCallConv = 65,
    LLVMARMAPCSCallConv = 66,
    LLVMARMAAPCSCallConv = 67,
    LLVMARMAAPCSVFPCallConv = 68,
    LLVMMSP430INTRCallConv = 69,
    LLVMX86ThisCallCallConv = 70,
    LLVMPTXKernelCallConv = 71,
    LLVMPTXDeviceCallConv = 72,
    LLVMSPIRFUNCCallConv = 75,
    LLVMSPIRKERNELCallConv = 76,
    LLVMIntelOCLBICallConv = 77,
    LLVMX8664SysVCallConv = 78,
    LLVMWin64CallConv = 79,
    LLVMX86VectorCallCallConv = 80,
    LLVMHHVMCallConv = 81,
    LLVMHHVMCCallConv = 82,
    LLVMX86INTRCallConv = 83,
    LLVMAVRINTRCallConv = 84,
    LLVMAVRSIGNALCallConv = 85,
    LLVMAVRBUILTINCallConv = 86,
    LLVMAMDGPUVSCallConv = 87,
    LLVMAMDGPUGSCallConv = 88,
    LLVMAMDGPUPSCallConv = 89,
    LLVMAMDGPUCSCallConv = 90,
    LLVMAMDGPUKERNELCallConv = 91,
    LLVMX86RegCallCallConv = 92,
    LLVMAMDGPUHSCallConv = 93,
    LLVMMSP430BUILTINCallConv = 94,
    LLVMAMDGPULSCallConv = 95,
    LLVMAMDGPUESCallConv = 96,
  }

  /** ./llvm-c/Core.h:257:9 */
  export enum ValueKind {
    LLVMArgumentValueKind = 0,
    LLVMBasicBlockValueKind = 1,
    LLVMMemoryUseValueKind = 2,
    LLVMMemoryDefValueKind = 3,
    LLVMMemoryPhiValueKind = 4,
    LLVMFunctionValueKind = 5,
    LLVMGlobalAliasValueKind = 6,
    LLVMGlobalIFuncValueKind = 7,
    LLVMGlobalVariableValueKind = 8,
    LLVMBlockAddressValueKind = 9,
    LLVMConstantExprValueKind = 10,
    LLVMConstantArrayValueKind = 11,
    LLVMConstantStructValueKind = 12,
    LLVMConstantVectorValueKind = 13,
    LLVMUndefValueValueKind = 14,
    LLVMConstantAggregateZeroValueKind = 15,
    LLVMConstantDataArrayValueKind = 16,
    LLVMConstantDataVectorValueKind = 17,
    LLVMConstantIntValueKind = 18,
    LLVMConstantFPValueKind = 19,
    LLVMConstantPointerNullValueKind = 20,
    LLVMConstantTokenNoneValueKind = 21,
    LLVMMetadataAsValueValueKind = 22,
    LLVMInlineAsmValueKind = 23,
    LLVMInstructionValueKind = 24,
    LLVMPoisonValueValueKind = 25,
  }

  /** ./llvm-c/Core.h:290:9 */
  export enum IntPredicate {
    LLVMIntEQ = 32,
    LLVMIntNE = 33,
    LLVMIntUGT = 34,
    LLVMIntUGE = 35,
    LLVMIntULT = 36,
    LLVMIntULE = 37,
    LLVMIntSGT = 38,
    LLVMIntSGE = 39,
    LLVMIntSLT = 40,
    LLVMIntSLE = 41,
  }

  /** ./llvm-c/Core.h:303:9 */
  export enum RealPredicate {
    LLVMRealPredicateFalse = 0,
    LLVMRealOEQ = 1,
    LLVMRealOGT = 2,
    LLVMRealOGE = 3,
    LLVMRealOLT = 4,
    LLVMRealOLE = 5,
    LLVMRealONE = 6,
    LLVMRealORD = 7,
    LLVMRealUNO = 8,
    LLVMRealUEQ = 9,
    LLVMRealUGT = 10,
    LLVMRealUGE = 11,
    LLVMRealULT = 12,
    LLVMRealULE = 13,
    LLVMRealUNE = 14,
    LLVMRealPredicateTrue = 15,
  }

  /** ./llvm-c/Core.h:322:9 */
  export enum LandingPadClauseTy {
    LLVMLandingPadCatch = 0,
    LLVMLandingPadFilter = 1,
  }

  /** ./llvm-c/Core.h:327:9 */
  export enum ThreadLocalMode {
    LLVMNotThreadLocal = 0,
    LLVMGeneralDynamicTLSModel = 1,
    LLVMLocalDynamicTLSModel = 2,
    LLVMInitialExecTLSModel = 3,
    LLVMLocalExecTLSModel = 4,
  }

  /** ./llvm-c/Core.h:335:9 */
  export enum AtomicOrdering {
    LLVMAtomicOrderingNotAtomic = 0,
    LLVMAtomicOrderingUnordered = 1,
    LLVMAtomicOrderingMonotonic = 2,
    LLVMAtomicOrderingAcquire = 4,
    LLVMAtomicOrderingRelease = 5,
    LLVMAtomicOrderingAcquireRelease = 6,
    LLVMAtomicOrderingSequentiallyConsistent = 7,
  }

  /** ./llvm-c/Core.h:362:9 */
  export enum AtomicRMWBinOp {
    LLVMAtomicRMWBinOpXchg = 0,
    LLVMAtomicRMWBinOpAdd = 1,
    LLVMAtomicRMWBinOpSub = 2,
    LLVMAtomicRMWBinOpAnd = 3,
    LLVMAtomicRMWBinOpNand = 4,
    LLVMAtomicRMWBinOpOr = 5,
    LLVMAtomicRMWBinOpXor = 6,
    LLVMAtomicRMWBinOpMax = 7,
    LLVMAtomicRMWBinOpMin = 8,
    LLVMAtomicRMWBinOpUMax = 9,
    LLVMAtomicRMWBinOpUMin = 10,
    LLVMAtomicRMWBinOpFAdd = 11,
    LLVMAtomicRMWBinOpFSub = 12,
    LLVMAtomicRMWBinOpFMax = 13,
    LLVMAtomicRMWBinOpFMin = 14,
  }

  /** ./llvm-c/Core.h:394:9 */
  export enum DiagnosticSeverity {
    LLVMDSError = 0,
    LLVMDSWarning = 1,
    LLVMDSRemark = 2,
    LLVMDSNote = 3,
  }

  /** ./llvm-c/Core.h:401:9 */
  export enum InlineAsmDialect {
    LLVMInlineAsmDialectATT = 0,
    LLVMInlineAsmDialectIntel = 1,
  }

  /** ./llvm-c/Core.h:406:9 */
  export enum ModuleFlagBehavior {
    LLVMModuleFlagBehaviorError = 0,
    LLVMModuleFlagBehaviorWarning = 1,
    LLVMModuleFlagBehaviorRequire = 2,
    LLVMModuleFlagBehaviorOverride = 3,
    LLVMModuleFlagBehaviorAppend = 4,
    LLVMModuleFlagBehaviorAppendUnique = 5,
  }

  /** ./llvm-c/Object.h:40:9 */
  export enum BinaryType {
    LLVMBinaryTypeArchive = 0,
    LLVMBinaryTypeMachOUniversalBinary = 1,
    LLVMBinaryTypeCOFFImportFile = 2,
    LLVMBinaryTypeIR = 3,
    LLVMBinaryTypeWinRes = 4,
    LLVMBinaryTypeCOFF = 5,
    LLVMBinaryTypeELF32L = 6,
    LLVMBinaryTypeELF32B = 7,
    LLVMBinaryTypeELF64L = 8,
    LLVMBinaryTypeELF64B = 9,
    LLVMBinaryTypeMachO32L = 10,
    LLVMBinaryTypeMachO32B = 11,
    LLVMBinaryTypeMachO64L = 12,
    LLVMBinaryTypeMachO64B = 13,
    LLVMBinaryTypeWasm = 14,
    LLVMBinaryTypeOffload = 15,
  }

  /** ./llvm-c/Comdat.h:29:9 */
  export enum ComdatSelectionKind {
    LLVMAnyComdatSelectionKind = 0,
    LLVMExactMatchComdatSelectionKind = 1,
    LLVMLargestComdatSelectionKind = 2,
    LLVMNoDeduplicateComdatSelectionKind = 3,
    LLVMSameSizeComdatSelectionKind = 4,
  }

  /** ./llvm-c/Analysis.h:44:10 */
  export declare function VerifyModule(M: LLVM.ModuleRef, Action: LLVM.VerifierFailureAction, OutMessage: Pointer<"OutMessage">): LLVM.Bool;

  /** ./llvm-c/Analysis.h:49:10 */
  export declare function VerifyFunction(Fn: LLVM.ValueRef, Action: LLVM.VerifierFailureAction): LLVM.Bool;

  /** ./llvm-c/Analysis.h:53:6 */
  export declare function ViewFunctionCFG(Fn: LLVM.ValueRef): void;

  /** ./llvm-c/Analysis.h:54:6 */
  export declare function ViewFunctionCFGOnly(Fn: LLVM.ValueRef): void;

  /** /usr/include/llvm/Config/Targets.def:41:1 */
  export declare function InitializeWebAssemblyTargetInfo(): void;

  /** /usr/include/llvm/Config/Targets.def:42:1 */
  export declare function InitializeX86TargetInfo(): void;

  /** /usr/include/llvm/Config/Targets.def:41:1 */
  export declare function InitializeWebAssemblyTarget(): void;

  /** /usr/include/llvm/Config/Targets.def:42:1 */
  export declare function InitializeX86Target(): void;

  /** /usr/include/llvm/Config/Targets.def:41:1 */
  export declare function InitializeWebAssemblyTargetMC(): void;

  /** /usr/include/llvm/Config/Targets.def:42:1 */
  export declare function InitializeX86TargetMC(): void;

  /** /usr/include/llvm/Config/AsmPrinters.def:42:1 */
  export declare function InitializeWebAssemblyAsmPrinter(): void;

  /** /usr/include/llvm/Config/AsmPrinters.def:43:1 */
  export declare function InitializeX86AsmPrinter(): void;

  /** /usr/include/llvm/Config/AsmParsers.def:41:1 */
  export declare function InitializeWebAssemblyAsmParser(): void;

  /** /usr/include/llvm/Config/AsmParsers.def:42:1 */
  export declare function InitializeX86AsmParser(): void;

  /** /usr/include/llvm/Config/Disassemblers.def:41:1 */
  export declare function InitializeWebAssemblyDisassembler(): void;

  /** /usr/include/llvm/Config/Disassemblers.def:42:1 */
  export declare function InitializeX86Disassembler(): void;

  /** ./llvm-c/Target.h:186:19 */
  export declare function GetModuleDataLayout(M: LLVM.ModuleRef): LLVM.TargetDataRef;

  /** ./llvm-c/Target.h:193:6 */
  export declare function SetModuleDataLayout(M: LLVM.ModuleRef, DL: LLVM.TargetDataRef): void;

  /** ./llvm-c/Target.h:197:19 */
  export declare function CreateTargetData(StringRep: Pointer<"StringRep">): LLVM.TargetDataRef;

  /** ./llvm-c/Target.h:201:6 */
  export declare function DisposeTargetData(TD: LLVM.TargetDataRef): void;

  /** ./llvm-c/Target.h:206:6 */
  export declare function AddTargetLibraryInfo(TLI: LLVM.TargetLibraryInfoRef, PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Target.h:212:7 */
  export declare function CopyStringRepOfTargetData(TD: LLVM.TargetDataRef): Pointer<"LLVMCopyStringRepOfTargetData">;

  /** ./llvm-c/Target.h:217:23 */
  export declare function ByteOrder(TD: LLVM.TargetDataRef): LLVM.ByteOrdering;

  /** ./llvm-c/Target.h:221:10 */
  export declare function PointerSize(TD: LLVM.TargetDataRef): Opaque<number, "LLVMPointerSize">;

  /** ./llvm-c/Target.h:226:10 */
  export declare function PointerSizeForAS(TD: LLVM.TargetDataRef, AS: Opaque<number, "AS">): Opaque<number, "LLVMPointerSizeForAS">;

  /** ./llvm-c/Target.h:230:13 */
  export declare function IntPtrType(TD: LLVM.TargetDataRef): LLVM.TypeRef;

  /** ./llvm-c/Target.h:235:13 */
  export declare function IntPtrTypeForAS(TD: LLVM.TargetDataRef, AS: Opaque<number, "AS">): LLVM.TypeRef;

  /** ./llvm-c/Target.h:239:13 */
  export declare function IntPtrTypeInContext(C: LLVM.ContextRef, TD: LLVM.TargetDataRef): LLVM.TypeRef;

  /** ./llvm-c/Target.h:244:13 */
  export declare function IntPtrTypeForASInContext(C: LLVM.ContextRef, TD: LLVM.TargetDataRef, AS: Opaque<number, "AS">): LLVM.TypeRef;

  /** ./llvm-c/Target.h:249:20 */
  export declare function SizeOfTypeInBits(TD: LLVM.TargetDataRef, Ty: LLVM.TypeRef): Opaque<bigint, "LLVMSizeOfTypeInBits">;

  /** ./llvm-c/Target.h:253:20 */
  export declare function StoreSizeOfType(TD: LLVM.TargetDataRef, Ty: LLVM.TypeRef): Opaque<bigint, "LLVMStoreSizeOfType">;

  /** ./llvm-c/Target.h:257:20 */
  export declare function ABISizeOfType(TD: LLVM.TargetDataRef, Ty: LLVM.TypeRef): Opaque<bigint, "LLVMABISizeOfType">;

  /** ./llvm-c/Target.h:261:10 */
  export declare function ABIAlignmentOfType(TD: LLVM.TargetDataRef, Ty: LLVM.TypeRef): Opaque<number, "LLVMABIAlignmentOfType">;

  /** ./llvm-c/Target.h:265:10 */
  export declare function CallFrameAlignmentOfType(TD: LLVM.TargetDataRef, Ty: LLVM.TypeRef): Opaque<number, "LLVMCallFrameAlignmentOfType">;

  /** ./llvm-c/Target.h:269:10 */
  export declare function PreferredAlignmentOfType(TD: LLVM.TargetDataRef, Ty: LLVM.TypeRef): Opaque<number, "LLVMPreferredAlignmentOfType">;

  /** ./llvm-c/Target.h:273:10 */
  export declare function PreferredAlignmentOfGlobal(TD: LLVM.TargetDataRef, GlobalVar: LLVM.ValueRef): Opaque<number, "LLVMPreferredAlignmentOfGlobal">;

  /** ./llvm-c/Target.h:278:10 */
  export declare function ElementAtOffset(TD: LLVM.TargetDataRef, StructTy: LLVM.TypeRef, Offset: Opaque<bigint, "Offset">): Opaque<number, "LLVMElementAtOffset">;

  /** ./llvm-c/Target.h:283:20 */
  export declare function OffsetOfElement(TD: LLVM.TargetDataRef, StructTy: LLVM.TypeRef, Element: Opaque<number, "Element">): Opaque<bigint, "LLVMOffsetOfElement">;

  /** ./llvm-c/TargetMachine.h:70:15 */
  export declare function GetFirstTarget(): LLVM.TargetRef;

  /** ./llvm-c/TargetMachine.h:72:15 */
  export declare function GetNextTarget(T: LLVM.TargetRef): LLVM.TargetRef;

  /** ./llvm-c/TargetMachine.h:77:15 */
  export declare function GetTargetFromName(Name: Pointer<"Name">): LLVM.TargetRef;

  /** ./llvm-c/TargetMachine.h:82:10 */
  export declare function GetTargetFromTriple(Triple: Pointer<"Triple">, T: Pointer<"T">, ErrorMessage: Pointer<"ErrorMessage">): LLVM.Bool;

  /** ./llvm-c/TargetMachine.h:86:13 */
  export declare function GetTargetName(T: LLVM.TargetRef): Pointer<"LLVMGetTargetName">;

  /** ./llvm-c/TargetMachine.h:89:13 */
  export declare function GetTargetDescription(T: LLVM.TargetRef): Pointer<"LLVMGetTargetDescription">;

  /** ./llvm-c/TargetMachine.h:92:10 */
  export declare function TargetHasJIT(T: LLVM.TargetRef): LLVM.Bool;

  /** ./llvm-c/TargetMachine.h:95:10 */
  export declare function TargetHasTargetMachine(T: LLVM.TargetRef): LLVM.Bool;

  /** ./llvm-c/TargetMachine.h:98:10 */
  export declare function TargetHasAsmBackend(T: LLVM.TargetRef): LLVM.Bool;

  /** ./llvm-c/TargetMachine.h:102:22 */
  export declare function CreateTargetMachine(T: LLVM.TargetRef, Triple: Pointer<"Triple">, CPU: Pointer<"CPU">, Features: Pointer<"Features">, Level: LLVM.CodeGenOptLevel, Reloc: LLVM.RelocMode, CodeModel: LLVM.CodeModel): LLVM.TargetMachineRef;

  /** ./llvm-c/TargetMachine.h:108:6 */
  export declare function DisposeTargetMachine(T: LLVM.TargetMachineRef): void;

  /** ./llvm-c/TargetMachine.h:111:15 */
  export declare function GetTargetMachineTarget(T: LLVM.TargetMachineRef): LLVM.TargetRef;

  /** ./llvm-c/TargetMachine.h:116:7 */
  export declare function GetTargetMachineTriple(T: LLVM.TargetMachineRef): Pointer<"LLVMGetTargetMachineTriple">;

  /** ./llvm-c/TargetMachine.h:121:7 */
  export declare function GetTargetMachineCPU(T: LLVM.TargetMachineRef): Pointer<"LLVMGetTargetMachineCPU">;

  /** ./llvm-c/TargetMachine.h:126:7 */
  export declare function GetTargetMachineFeatureString(T: LLVM.TargetMachineRef): Pointer<"LLVMGetTargetMachineFeatureString">;

  /** ./llvm-c/TargetMachine.h:129:19 */
  export declare function CreateTargetDataLayout(T: LLVM.TargetMachineRef): LLVM.TargetDataRef;

  /** ./llvm-c/TargetMachine.h:132:6 */
  export declare function SetTargetMachineAsmVerbosity(T: LLVM.TargetMachineRef, VerboseAsm: LLVM.Bool): void;

  /** ./llvm-c/TargetMachine.h:138:10 */
  export declare function TargetMachineEmitToFile(T: LLVM.TargetMachineRef, M: LLVM.ModuleRef, Filename: Pointer<"Filename">, codegen: LLVM.CodeGenFileType, ErrorMessage: Pointer<"ErrorMessage">): LLVM.Bool;

  /** ./llvm-c/TargetMachine.h:142:10 */
  export declare function TargetMachineEmitToMemoryBuffer(T: LLVM.TargetMachineRef, M: LLVM.ModuleRef, codegen: LLVM.CodeGenFileType, ErrorMessage: Pointer<"ErrorMessage">, OutMemBuf: Pointer<"OutMemBuf">): LLVM.Bool;

  /** ./llvm-c/TargetMachine.h:148:7 */
  export declare function GetDefaultTargetTriple(): Pointer<"LLVMGetDefaultTargetTriple">;

  /** ./llvm-c/TargetMachine.h:152:7 */
  export declare function NormalizeTargetTriple(triple: Pointer<"triple">): Pointer<"LLVMNormalizeTargetTriple">;

  /** ./llvm-c/TargetMachine.h:156:7 */
  export declare function GetHostCPUName(): Pointer<"LLVMGetHostCPUName">;

  /** ./llvm-c/TargetMachine.h:160:7 */
  export declare function GetHostCPUFeatures(): Pointer<"LLVMGetHostCPUFeatures">;

  /** ./llvm-c/TargetMachine.h:163:6 */
  export declare function AddAnalysisPasses(T: LLVM.TargetMachineRef, PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/ExecutionEngine.h:36:6 */
  export declare function LinkInMCJIT(): void;

  /** ./llvm-c/ExecutionEngine.h:37:6 */
  export declare function LinkInInterpreter(): void;

  /** ./llvm-c/ExecutionEngine.h:53:21 */
  export declare function CreateGenericValueOfInt(Ty: LLVM.TypeRef, N: Opaque<bigint, "N">, IsSigned: LLVM.Bool): LLVM.GenericValueRef;

  /** ./llvm-c/ExecutionEngine.h:57:21 */
  export declare function CreateGenericValueOfPointer(P: Pointer<"P">): LLVM.GenericValueRef;

  /** ./llvm-c/ExecutionEngine.h:59:21 */
  export declare function CreateGenericValueOfFloat(Ty: LLVM.TypeRef, N: Opaque<bigint, "N">): LLVM.GenericValueRef;

  /** ./llvm-c/ExecutionEngine.h:61:10 */
  export declare function GenericValueIntWidth(GenValRef: LLVM.GenericValueRef): Opaque<number, "LLVMGenericValueIntWidth">;

  /** ./llvm-c/ExecutionEngine.h:63:20 */
  export declare function GenericValueToInt(GenVal: LLVM.GenericValueRef, IsSigned: LLVM.Bool): Opaque<bigint, "LLVMGenericValueToInt">;

  /** ./llvm-c/ExecutionEngine.h:66:7 */
  export declare function GenericValueToPointer(GenVal: LLVM.GenericValueRef): Pointer<"LLVMGenericValueToPointer">;

  /** ./llvm-c/ExecutionEngine.h:68:8 */
  export declare function GenericValueToFloat(TyRef: LLVM.TypeRef, GenVal: LLVM.GenericValueRef): Opaque<bigint, "LLVMGenericValueToFloat">;

  /** ./llvm-c/ExecutionEngine.h:70:6 */
  export declare function DisposeGenericValue(GenVal: LLVM.GenericValueRef): void;

  /** ./llvm-c/ExecutionEngine.h:74:10 */
  export declare function CreateExecutionEngineForModule(OutEE: Pointer<"OutEE">, M: LLVM.ModuleRef, OutError: Pointer<"OutError">): LLVM.Bool;

  /** ./llvm-c/ExecutionEngine.h:78:10 */
  export declare function CreateInterpreterForModule(OutInterp: Pointer<"OutInterp">, M: LLVM.ModuleRef, OutError: Pointer<"OutError">): LLVM.Bool;

  /** ./llvm-c/ExecutionEngine.h:82:10 */
  export declare function CreateJITCompilerForModule(OutJIT: Pointer<"OutJIT">, M: LLVM.ModuleRef, OptLevel: Opaque<number, "OptLevel">, OutError: Pointer<"OutError">): LLVM.Bool;

  /** ./llvm-c/ExecutionEngine.h:87:6 */
  export declare function InitializeMCJITCompilerOptions(Options: Pointer<"Options">, SizeOfOptions: Opaque<number, "SizeOfOptions">): void;

  /** ./llvm-c/ExecutionEngine.h:107:10 */
  export declare function CreateMCJITCompilerForModule(OutJIT: Pointer<"OutJIT">, M: LLVM.ModuleRef, Options: Pointer<"Options">, SizeOfOptions: Opaque<number, "SizeOfOptions">, OutError: Pointer<"OutError">): LLVM.Bool;

  /** ./llvm-c/ExecutionEngine.h:112:6 */
  export declare function DisposeExecutionEngine(EE: LLVM.ExecutionEngineRef): void;

  /** ./llvm-c/ExecutionEngine.h:114:6 */
  export declare function RunStaticConstructors(EE: LLVM.ExecutionEngineRef): void;

  /** ./llvm-c/ExecutionEngine.h:116:6 */
  export declare function RunStaticDestructors(EE: LLVM.ExecutionEngineRef): void;

  /** ./llvm-c/ExecutionEngine.h:118:5 */
  export declare function RunFunctionAsMain(EE: LLVM.ExecutionEngineRef, F: LLVM.ValueRef, ArgC: Opaque<number, "ArgC">, ArgV: Pointer<"ArgV">, EnvP: Pointer<"EnvP">): Opaque<number, "LLVMRunFunctionAsMain">;

  /** ./llvm-c/ExecutionEngine.h:122:21 */
  export declare function RunFunction(EE: LLVM.ExecutionEngineRef, F: LLVM.ValueRef, NumArgs: Opaque<number, "NumArgs">, Args: Pointer<"Args">): LLVM.GenericValueRef;

  /** ./llvm-c/ExecutionEngine.h:126:6 */
  export declare function FreeMachineCodeForFunction(EE: LLVM.ExecutionEngineRef, F: LLVM.ValueRef): void;

  /** ./llvm-c/ExecutionEngine.h:128:6 */
  export declare function AddModule(EE: LLVM.ExecutionEngineRef, M: LLVM.ModuleRef): void;

  /** ./llvm-c/ExecutionEngine.h:130:10 */
  export declare function RemoveModule(EE: LLVM.ExecutionEngineRef, M: LLVM.ModuleRef, OutMod: Pointer<"OutMod">, OutError: Pointer<"OutError">): LLVM.Bool;

  /** ./llvm-c/ExecutionEngine.h:133:10 */
  export declare function FindFunction(EE: LLVM.ExecutionEngineRef, Name: Pointer<"Name">, OutFn: Pointer<"OutFn">): LLVM.Bool;

  /** ./llvm-c/ExecutionEngine.h:136:7 */
  export declare function RecompileAndRelinkFunction(EE: LLVM.ExecutionEngineRef, Fn: LLVM.ValueRef): Pointer<"LLVMRecompileAndRelinkFunction">;

  /** ./llvm-c/ExecutionEngine.h:139:19 */
  export declare function GetExecutionEngineTargetData(EE: LLVM.ExecutionEngineRef): LLVM.TargetDataRef;

  /** ./llvm-c/ExecutionEngine.h:141:1 */
  export declare function GetExecutionEngineTargetMachine(EE: LLVM.ExecutionEngineRef): LLVM.TargetMachineRef;

  /** ./llvm-c/ExecutionEngine.h:143:6 */
  export declare function AddGlobalMapping(EE: LLVM.ExecutionEngineRef, Global: LLVM.ValueRef, Addr: Pointer<"Addr">): void;

  /** ./llvm-c/ExecutionEngine.h:146:7 */
  export declare function GetPointerToGlobal(EE: LLVM.ExecutionEngineRef, Global: LLVM.ValueRef): Pointer<"LLVMGetPointerToGlobal">;

  /** ./llvm-c/ExecutionEngine.h:148:10 */
  export declare function GetGlobalValueAddress(EE: LLVM.ExecutionEngineRef, Name: Pointer<"Name">): Opaque<bigint, "LLVMGetGlobalValueAddress">;

  /** ./llvm-c/ExecutionEngine.h:150:10 */
  export declare function GetFunctionAddress(EE: LLVM.ExecutionEngineRef, Name: Pointer<"Name">): Opaque<bigint, "LLVMGetFunctionAddress">;

  /** ./llvm-c/ExecutionEngine.h:154:10 */
  export declare function ExecutionEngineGetErrMsg(EE: LLVM.ExecutionEngineRef, OutError: Pointer<"OutError">): LLVM.Bool;

  /** ./llvm-c/ExecutionEngine.h:180:27 */
  export declare function CreateSimpleMCJITMemoryManager(Opaque: Pointer<"Opaque">, AllocateCodeSection: LLVM.MemoryManagerAllocateCodeSectionCallback, AllocateDataSection: LLVM.MemoryManagerAllocateDataSectionCallback, FinalizeMemory: LLVM.MemoryManagerFinalizeMemoryCallback, Destroy: LLVM.MemoryManagerDestroyCallback): LLVM.MCJITMemoryManagerRef;

  /** ./llvm-c/ExecutionEngine.h:187:6 */
  export declare function DisposeMCJITMemoryManager(MM: LLVM.MCJITMemoryManagerRef): void;

  /** ./llvm-c/ExecutionEngine.h:191:25 */
  export declare function CreateGDBRegistrationListener(): LLVM.JITEventListenerRef;

  /** ./llvm-c/ExecutionEngine.h:192:25 */
  export declare function CreateIntelJITEventListener(): LLVM.JITEventListenerRef;

  /** ./llvm-c/ExecutionEngine.h:193:25 */
  export declare function CreateOProfileJITEventListener(): LLVM.JITEventListenerRef;

  /** ./llvm-c/ExecutionEngine.h:194:25 */
  export declare function CreatePerfJITEventListener(): LLVM.JITEventListenerRef;

  /** ./llvm-c/Disassembler.h:38:22 */
  export declare function CreateDisasm(TripleName: Pointer<"TripleName">, DisInfo: Pointer<"DisInfo">, TagType: Opaque<number, "TagType">, GetOpInfo: LLVM.OpInfoCallback, SymbolLookUp: LLVM.SymbolLookupCallback): LLVM.DisasmContextRef;

  /** ./llvm-c/Disassembler.h:50:22 */
  export declare function CreateDisasmCPU(Triple: Pointer<"Triple">, CPU: Pointer<"CPU">, DisInfo: Pointer<"DisInfo">, TagType: Opaque<number, "TagType">, GetOpInfo: LLVM.OpInfoCallback, SymbolLookUp: LLVM.SymbolLookupCallback): LLVM.DisasmContextRef;

  /** ./llvm-c/Disassembler.h:63:1 */
  export declare function CreateDisasmCPUFeatures(Triple: Pointer<"Triple">, CPU: Pointer<"CPU">, Features: Pointer<"Features">, DisInfo: Pointer<"DisInfo">, TagType: Opaque<number, "TagType">, GetOpInfo: LLVM.OpInfoCallback, SymbolLookUp: LLVM.SymbolLookupCallback): LLVM.DisasmContextRef;

  /** ./llvm-c/Disassembler.h:72:5 */
  export declare function SetDisasmOptions(DC: LLVM.DisasmContextRef, Options: Opaque<bigint, "Options">): Opaque<number, "LLVMSetDisasmOptions">;

  /** ./llvm-c/Disassembler.h:88:6 */
  export declare function DisasmDispose(DC: LLVM.DisasmContextRef): void;

  /** ./llvm-c/Disassembler.h:100:8 */
  export declare function DisasmInstruction(DC: LLVM.DisasmContextRef, Bytes: Pointer<"Bytes">, BytesSize: Opaque<bigint, "BytesSize">, PC: Opaque<bigint, "PC">, OutString: Pointer<"OutString">, OutStringSize: Opaque<number, "OutStringSize">): Opaque<number, "LLVMDisasmInstruction">;

  /** ./llvm-c/DebugInfo.h:197:10 */
  export declare function DebugMetadataVersion(): Opaque<number, "LLVMDebugMetadataVersion">;

  /** ./llvm-c/DebugInfo.h:202:10 */
  export declare function GetModuleDebugMetadataVersion(Module: LLVM.ModuleRef): Opaque<number, "LLVMGetModuleDebugMetadataVersion">;

  /** ./llvm-c/DebugInfo.h:210:10 */
  export declare function StripModuleDebugInfo(Module: LLVM.ModuleRef): LLVM.Bool;

  /** ./llvm-c/DebugInfo.h:216:18 */
  export declare function CreateDIBuilderDisallowUnresolved(M: LLVM.ModuleRef): LLVM.DIBuilderRef;

  /** ./llvm-c/DebugInfo.h:223:18 */
  export declare function CreateDIBuilder(M: LLVM.ModuleRef): LLVM.DIBuilderRef;

  /** ./llvm-c/DebugInfo.h:229:6 */
  export declare function DisposeDIBuilder(Builder: LLVM.DIBuilderRef): void;

  /** ./llvm-c/DebugInfo.h:234:6 */
  export declare function DIBuilderFinalize(Builder: LLVM.DIBuilderRef): void;

  /** ./llvm-c/DebugInfo.h:240:6 */
  export declare function DIBuilderFinalizeSubprogram(Builder: LLVM.DIBuilderRef, Subprogram: LLVM.MetadataRef): void;

  /** ./llvm-c/DebugInfo.h:275:17 */
  export declare function DIBuilderCreateCompileUnit(Builder: LLVM.DIBuilderRef, Lang: LLVM.DWARFSourceLanguage, FileRef: LLVM.MetadataRef, Producer: Pointer<"Producer">, ProducerLen: Opaque<number, "ProducerLen">, isOptimized: LLVM.Bool, Flags: Pointer<"Flags">, FlagsLen: Opaque<number, "FlagsLen">, RuntimeVer: Opaque<number, "RuntimeVer">, SplitName: Pointer<"SplitName">, SplitNameLen: Opaque<number, "SplitNameLen">, Kind: LLVM.DWARFEmissionKind, DWOId: Opaque<number, "DWOId">, SplitDebugInlining: LLVM.Bool, DebugInfoForProfiling: LLVM.Bool, SysRoot: Pointer<"SysRoot">, SysRootLen: Opaque<number, "SysRootLen">, SDK: Pointer<"SDK">, SDKLen: Opaque<number, "SDKLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:293:1 */
  export declare function DIBuilderCreateFile(Builder: LLVM.DIBuilderRef, Filename: Pointer<"Filename">, FilenameLen: Opaque<number, "FilenameLen">, Directory: Pointer<"Directory">, DirectoryLen: Opaque<number, "DirectoryLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:312:1 */
  export declare function DIBuilderCreateModule(Builder: LLVM.DIBuilderRef, ParentScope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, ConfigMacros: Pointer<"ConfigMacros">, ConfigMacrosLen: Opaque<number, "ConfigMacrosLen">, IncludePath: Pointer<"IncludePath">, IncludePathLen: Opaque<number, "IncludePathLen">, APINotesFile: Pointer<"APINotesFile">, APINotesFileLen: Opaque<number, "APINotesFileLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:328:1 */
  export declare function DIBuilderCreateNameSpace(Builder: LLVM.DIBuilderRef, ParentScope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, ExportSymbols: LLVM.Bool): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:351:17 */
  export declare function DIBuilderCreateFunction(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, LinkageName: Pointer<"LinkageName">, LinkageNameLen: Opaque<number, "LinkageNameLen">, File: LLVM.MetadataRef, LineNo: Opaque<number, "LineNo">, Ty: LLVM.MetadataRef, IsLocalToUnit: LLVM.Bool, IsDefinition: LLVM.Bool, ScopeLine: Opaque<number, "ScopeLine">, Flags: LLVM.DIFlags, IsOptimized: LLVM.Bool): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:366:17 */
  export declare function DIBuilderCreateLexicalBlock(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, File: LLVM.MetadataRef, Line: Opaque<number, "Line">, Column: Opaque<number, "Column">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:378:1 */
  export declare function DIBuilderCreateLexicalBlockFile(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, File: LLVM.MetadataRef, Discriminator: Opaque<number, "Discriminator">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:392:1 */
  export declare function DIBuilderCreateImportedModuleFromNamespace(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, NS: LLVM.MetadataRef, File: LLVM.MetadataRef, Line: Opaque<number, "Line">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:409:17 */
  export declare function DIBuilderCreateImportedModuleFromAlias(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, ImportedEntity: LLVM.MetadataRef, File: LLVM.MetadataRef, Line: Opaque<number, "Line">, Elements: Pointer<"Elements">, NumElements: Opaque<number, "NumElements">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:424:17 */
  export declare function DIBuilderCreateImportedModuleFromModule(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, M: LLVM.MetadataRef, File: LLVM.MetadataRef, Line: Opaque<number, "Line">, Elements: Pointer<"Elements">, NumElements: Opaque<number, "NumElements">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:444:17 */
  export declare function DIBuilderCreateImportedDeclaration(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Decl: LLVM.MetadataRef, File: LLVM.MetadataRef, Line: Opaque<number, "Line">, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, Elements: Pointer<"Elements">, NumElements: Opaque<number, "NumElements">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:460:1 */
  export declare function DIBuilderCreateDebugLocation(Ctx: LLVM.ContextRef, Line: Opaque<number, "Line">, Column: Opaque<number, "Column">, Scope: LLVM.MetadataRef, InlinedAt: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:470:10 */
  export declare function DILocationGetLine(Location: LLVM.MetadataRef): Opaque<number, "LLVMDILocationGetLine">;

  /** ./llvm-c/DebugInfo.h:478:10 */
  export declare function DILocationGetColumn(Location: LLVM.MetadataRef): Opaque<number, "LLVMDILocationGetColumn">;

  /** ./llvm-c/DebugInfo.h:486:17 */
  export declare function DILocationGetScope(Location: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:494:17 */
  export declare function DILocationGetInlinedAt(Location: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:502:17 */
  export declare function DIScopeGetFile(Scope: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:511:13 */
  export declare function DIFileGetDirectory(File: LLVM.MetadataRef, Len: Pointer<"Len">): Pointer<"LLVMDIFileGetDirectory">;

  /** ./llvm-c/DebugInfo.h:520:13 */
  export declare function DIFileGetFilename(File: LLVM.MetadataRef, Len: Pointer<"Len">): Pointer<"LLVMDIFileGetFilename">;

  /** ./llvm-c/DebugInfo.h:529:13 */
  export declare function DIFileGetSource(File: LLVM.MetadataRef, Len: Pointer<"Len">): Pointer<"LLVMDIFileGetSource">;

  /** ./llvm-c/DebugInfo.h:537:17 */
  export declare function DIBuilderGetOrCreateTypeArray(Builder: LLVM.DIBuilderRef, Data: Pointer<"Data">, NumElements: Opaque<number, "NumElements">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:552:1 */
  export declare function DIBuilderCreateSubroutineType(Builder: LLVM.DIBuilderRef, File: LLVM.MetadataRef, ParameterTypes: Pointer<"ParameterTypes">, NumParameterTypes: Opaque<number, "NumParameterTypes">, Flags: LLVM.DIFlags): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:569:17 */
  export declare function DIBuilderCreateMacro(Builder: LLVM.DIBuilderRef, ParentMacroFile: LLVM.MetadataRef, Line: Opaque<number, "Line">, RecordType: LLVM.DWARFMacinfoRecordType, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, Value: Pointer<"Value">, ValueLen: Opaque<number, "ValueLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:586:1 */
  export declare function DIBuilderCreateTempMacroFile(Builder: LLVM.DIBuilderRef, ParentMacroFile: LLVM.MetadataRef, Line: Opaque<number, "Line">, File: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:598:17 */
  export declare function DIBuilderCreateEnumerator(Builder: LLVM.DIBuilderRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, Value: Opaque<bigint, "Value">, IsUnsigned: LLVM.Bool): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:617:17 */
  export declare function DIBuilderCreateEnumerationType(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNumber: Opaque<number, "LineNumber">, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, Elements: Pointer<"Elements">, NumElements: Opaque<number, "NumElements">, ClassTy: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:640:17 */
  export declare function DIBuilderCreateUnionType(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNumber: Opaque<number, "LineNumber">, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, Flags: LLVM.DIFlags, Elements: Pointer<"Elements">, NumElements: Opaque<number, "NumElements">, RunTimeLang: Opaque<number, "RunTimeLang">, UniqueId: Pointer<"UniqueId">, UniqueIdLen: Opaque<number, "UniqueIdLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:658:1 */
  export declare function DIBuilderCreateArrayType(Builder: LLVM.DIBuilderRef, Size: Opaque<bigint, "Size">, AlignInBits: Opaque<number, "AlignInBits">, Ty: LLVM.MetadataRef, Subscripts: Pointer<"Subscripts">, NumSubscripts: Opaque<number, "NumSubscripts">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:673:1 */
  export declare function DIBuilderCreateVectorType(Builder: LLVM.DIBuilderRef, Size: Opaque<bigint, "Size">, AlignInBits: Opaque<number, "AlignInBits">, Ty: LLVM.MetadataRef, Subscripts: Pointer<"Subscripts">, NumSubscripts: Opaque<number, "NumSubscripts">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:685:1 */
  export declare function DIBuilderCreateUnspecifiedType(Builder: LLVM.DIBuilderRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:699:1 */
  export declare function DIBuilderCreateBasicType(Builder: LLVM.DIBuilderRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, SizeInBits: Opaque<bigint, "SizeInBits">, Encoding: LLVM.DWARFTypeEncoding, Flags: LLVM.DIFlags): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:714:17 */
  export declare function DIBuilderCreatePointerType(Builder: LLVM.DIBuilderRef, PointeeTy: LLVM.MetadataRef, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, AddressSpace: Opaque<number, "AddressSpace">, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:737:17 */
  export declare function DIBuilderCreateStructType(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNumber: Opaque<number, "LineNumber">, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, Flags: LLVM.DIFlags, DerivedFrom: LLVM.MetadataRef, Elements: Pointer<"Elements">, NumElements: Opaque<number, "NumElements">, RunTimeLang: Opaque<number, "RunTimeLang">, VTableHolder: LLVM.MetadataRef, UniqueId: Pointer<"UniqueId">, UniqueIdLen: Opaque<number, "UniqueIdLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:759:17 */
  export declare function DIBuilderCreateMemberType(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNo: Opaque<number, "LineNo">, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, OffsetInBits: Opaque<bigint, "OffsetInBits">, Flags: LLVM.DIFlags, Ty: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:780:1 */
  export declare function DIBuilderCreateStaticMemberType(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNumber: Opaque<number, "LineNumber">, Type: LLVM.MetadataRef, Flags: LLVM.DIFlags, ConstantVal: LLVM.ValueRef, AlignInBits: Opaque<number, "AlignInBits">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:796:1 */
  export declare function DIBuilderCreateMemberPointerType(Builder: LLVM.DIBuilderRef, PointeeType: LLVM.MetadataRef, ClassType: LLVM.MetadataRef, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, Flags: LLVM.DIFlags): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:817:1 */
  export declare function DIBuilderCreateObjCIVar(Builder: LLVM.DIBuilderRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNo: Opaque<number, "LineNo">, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, OffsetInBits: Opaque<bigint, "OffsetInBits">, Flags: LLVM.DIFlags, Ty: LLVM.MetadataRef, PropertyNode: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:839:1 */
  export declare function DIBuilderCreateObjCProperty(Builder: LLVM.DIBuilderRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNo: Opaque<number, "LineNo">, GetterName: Pointer<"GetterName">, GetterNameLen: Opaque<number, "GetterNameLen">, SetterName: Pointer<"SetterName">, SetterNameLen: Opaque<number, "SetterNameLen">, PropertyAttributes: Opaque<number, "PropertyAttributes">, Ty: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:853:1 */
  export declare function DIBuilderCreateObjectPointerType(Builder: LLVM.DIBuilderRef, Type: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:865:1 */
  export declare function DIBuilderCreateQualifiedType(Builder: LLVM.DIBuilderRef, Tag: Opaque<number, "Tag">, Type: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:876:1 */
  export declare function DIBuilderCreateReferenceType(Builder: LLVM.DIBuilderRef, Tag: Opaque<number, "Tag">, Type: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:884:1 */
  export declare function DIBuilderCreateNullPtrType(Builder: LLVM.DIBuilderRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:896:1 */
  export declare function DIBuilderCreateTypedef(Builder: LLVM.DIBuilderRef, Type: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNo: Opaque<number, "LineNo">, Scope: LLVM.MetadataRef, AlignInBits: Opaque<number, "AlignInBits">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:912:1 */
  export declare function DIBuilderCreateInheritance(Builder: LLVM.DIBuilderRef, Ty: LLVM.MetadataRef, BaseTy: LLVM.MetadataRef, BaseOffset: Opaque<bigint, "BaseOffset">, VBPtrOffset: Opaque<number, "VBPtrOffset">, Flags: LLVM.DIFlags): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:933:17 */
  export declare function DIBuilderCreateForwardDecl(Builder: LLVM.DIBuilderRef, Tag: Opaque<number, "Tag">, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, Scope: LLVM.MetadataRef, File: LLVM.MetadataRef, Line: Opaque<number, "Line">, RuntimeLang: Opaque<number, "RuntimeLang">, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, UniqueIdentifier: Pointer<"UniqueIdentifier">, UniqueIdentifierLen: Opaque<number, "UniqueIdentifierLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:957:1 */
  export declare function DIBuilderCreateReplaceableCompositeType(Builder: LLVM.DIBuilderRef, Tag: Opaque<number, "Tag">, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, Scope: LLVM.MetadataRef, File: LLVM.MetadataRef, Line: Opaque<number, "Line">, RuntimeLang: Opaque<number, "RuntimeLang">, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, Flags: LLVM.DIFlags, UniqueIdentifier: Pointer<"UniqueIdentifier">, UniqueIdentifierLen: Opaque<number, "UniqueIdentifierLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:979:1 */
  export declare function DIBuilderCreateBitFieldMemberType(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNumber: Opaque<number, "LineNumber">, SizeInBits: Opaque<bigint, "SizeInBits">, OffsetInBits: Opaque<bigint, "OffsetInBits">, StorageOffsetInBits: Opaque<bigint, "StorageOffsetInBits">, Flags: LLVM.DIFlags, Type: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1010:17 */
  export declare function DIBuilderCreateClassType(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNumber: Opaque<number, "LineNumber">, SizeInBits: Opaque<bigint, "SizeInBits">, AlignInBits: Opaque<number, "AlignInBits">, OffsetInBits: Opaque<bigint, "OffsetInBits">, Flags: LLVM.DIFlags, DerivedFrom: LLVM.MetadataRef, Elements: Pointer<"Elements">, NumElements: Opaque<number, "NumElements">, VTableHolder: LLVM.MetadataRef, TemplateParamsNode: LLVM.MetadataRef, UniqueIdentifier: Pointer<"UniqueIdentifier">, UniqueIdentifierLen: Opaque<number, "UniqueIdentifierLen">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1025:1 */
  export declare function DIBuilderCreateArtificialType(Builder: LLVM.DIBuilderRef, Type: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1035:13 */
  export declare function DITypeGetName(DType: LLVM.MetadataRef, Length: Pointer<"Length">): Pointer<"LLVMDITypeGetName">;

  /** ./llvm-c/DebugInfo.h:1043:10 */
  export declare function DITypeGetSizeInBits(DType: LLVM.MetadataRef): Opaque<bigint, "LLVMDITypeGetSizeInBits">;

  /** ./llvm-c/DebugInfo.h:1051:10 */
  export declare function DITypeGetOffsetInBits(DType: LLVM.MetadataRef): Opaque<bigint, "LLVMDITypeGetOffsetInBits">;

  /** ./llvm-c/DebugInfo.h:1059:10 */
  export declare function DITypeGetAlignInBits(DType: LLVM.MetadataRef): Opaque<number, "LLVMDITypeGetAlignInBits">;

  /** ./llvm-c/DebugInfo.h:1067:10 */
  export declare function DITypeGetLine(DType: LLVM.MetadataRef): Opaque<number, "LLVMDITypeGetLine">;

  /** ./llvm-c/DebugInfo.h:1075:13 */
  export declare function DITypeGetFlags(DType: LLVM.MetadataRef): LLVM.DIFlags;

  /** ./llvm-c/DebugInfo.h:1083:17 */
  export declare function DIBuilderGetOrCreateSubrange(Builder: LLVM.DIBuilderRef, LowerBound: Opaque<bigint, "LowerBound">, Count: Opaque<bigint, "Count">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1093:17 */
  export declare function DIBuilderGetOrCreateArray(Builder: LLVM.DIBuilderRef, Data: Pointer<"Data">, NumElements: Opaque<number, "NumElements">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1104:17 */
  export declare function DIBuilderCreateExpression(Builder: LLVM.DIBuilderRef, Addr: Pointer<"Addr">, Length: Opaque<number, "Length">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1114:1 */
  export declare function DIBuilderCreateConstantValueExpression(Builder: LLVM.DIBuilderRef, Value: Opaque<bigint, "Value">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1136:17 */
  export declare function DIBuilderCreateGlobalVariableExpression(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, Linkage: Pointer<"Linkage">, LinkLen: Opaque<number, "LinkLen">, File: LLVM.MetadataRef, LineNo: Opaque<number, "LineNo">, Ty: LLVM.MetadataRef, LocalToUnit: LLVM.Bool, Expr: LLVM.MetadataRef, Decl: LLVM.MetadataRef, AlignInBits: Opaque<number, "AlignInBits">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1148:17 */
  export declare function DIGlobalVariableExpressionGetVariable(GVE: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1156:17 */
  export declare function DIGlobalVariableExpressionGetExpression(GVE: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1165:17 */
  export declare function DIVariableGetFile(Var: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1173:17 */
  export declare function DIVariableGetScope(Var: LLVM.MetadataRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1181:10 */
  export declare function DIVariableGetLine(Var: LLVM.MetadataRef): Opaque<number, "LLVMDIVariableGetLine">;

  /** ./llvm-c/DebugInfo.h:1191:17 */
  export declare function TemporaryMDNode(Ctx: LLVM.ContextRef, Data: Pointer<"Data">, NumElements: Opaque<number, "NumElements">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1201:6 */
  export declare function DisposeTemporaryMDNode(TempNode: LLVM.MetadataRef): void;

  /** ./llvm-c/DebugInfo.h:1208:6 */
  export declare function MetadataReplaceAllUsesWith(TempTargetMetadata: LLVM.MetadataRef, Replacement: LLVM.MetadataRef): void;

  /** ./llvm-c/DebugInfo.h:1228:17 */
  export declare function DIBuilderCreateTempGlobalVariableFwdDecl(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, Linkage: Pointer<"Linkage">, LnkLen: Opaque<number, "LnkLen">, File: LLVM.MetadataRef, LineNo: Opaque<number, "LineNo">, Ty: LLVM.MetadataRef, LocalToUnit: LLVM.Bool, Decl: LLVM.MetadataRef, AlignInBits: Opaque<number, "AlignInBits">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1243:14 */
  export declare function DIBuilderInsertDeclareBefore(Builder: LLVM.DIBuilderRef, Storage: LLVM.ValueRef, VarInfo: LLVM.MetadataRef, Expr: LLVM.MetadataRef, DebugLoc: LLVM.MetadataRef, Instr: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/DebugInfo.h:1258:14 */
  export declare function DIBuilderInsertDeclareAtEnd(Builder: LLVM.DIBuilderRef, Storage: LLVM.ValueRef, VarInfo: LLVM.MetadataRef, Expr: LLVM.MetadataRef, DebugLoc: LLVM.MetadataRef, Block: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/DebugInfo.h:1271:14 */
  export declare function DIBuilderInsertDbgValueBefore(Builder: LLVM.DIBuilderRef, Val: LLVM.ValueRef, VarInfo: LLVM.MetadataRef, Expr: LLVM.MetadataRef, DebugLoc: LLVM.MetadataRef, Instr: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/DebugInfo.h:1289:14 */
  export declare function DIBuilderInsertDbgValueAtEnd(Builder: LLVM.DIBuilderRef, Val: LLVM.ValueRef, VarInfo: LLVM.MetadataRef, Expr: LLVM.MetadataRef, DebugLoc: LLVM.MetadataRef, Block: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/DebugInfo.h:1309:17 */
  export declare function DIBuilderCreateAutoVariable(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, File: LLVM.MetadataRef, LineNo: Opaque<number, "LineNo">, Ty: LLVM.MetadataRef, AlwaysPreserve: LLVM.Bool, Flags: LLVM.DIFlags, AlignInBits: Opaque<number, "AlignInBits">): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1327:17 */
  export declare function DIBuilderCreateParameterVariable(Builder: LLVM.DIBuilderRef, Scope: LLVM.MetadataRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, ArgNo: Opaque<number, "ArgNo">, File: LLVM.MetadataRef, LineNo: Opaque<number, "LineNo">, Ty: LLVM.MetadataRef, AlwaysPreserve: LLVM.Bool, Flags: LLVM.DIFlags): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1337:17 */
  export declare function GetSubprogram(Func: LLVM.ValueRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1344:6 */
  export declare function SetSubprogram(Func: LLVM.ValueRef, SP: LLVM.MetadataRef): void;

  /** ./llvm-c/DebugInfo.h:1352:10 */
  export declare function DISubprogramGetLine(Subprogram: LLVM.MetadataRef): Opaque<number, "LLVMDISubprogramGetLine">;

  /** ./llvm-c/DebugInfo.h:1359:17 */
  export declare function InstructionGetDebugLoc(Inst: LLVM.ValueRef): LLVM.MetadataRef;

  /** ./llvm-c/DebugInfo.h:1368:6 */
  export declare function InstructionSetDebugLoc(Inst: LLVM.ValueRef, Loc: LLVM.MetadataRef): void;

  /** ./llvm-c/DebugInfo.h:1375:18 */
  export declare function GetMetadataKind(Metadata: LLVM.MetadataRef): LLVM.MetadataKind;

  /** ./llvm-c/Error.h:44:17 */
  export declare function GetErrorTypeId(Err: LLVM.ErrorRef): LLVM.ErrorTypeId;

  /** ./llvm-c/Error.h:52:6 */
  export declare function ConsumeError(Err: LLVM.ErrorRef): void;

  /** ./llvm-c/Error.h:60:7 */
  export declare function GetErrorMessage(Err: LLVM.ErrorRef): Pointer<"LLVMGetErrorMessage">;

  /** ./llvm-c/Error.h:65:6 */
  export declare function DisposeErrorMessage(ErrMsg: Pointer<"ErrMsg">): void;

  /** ./llvm-c/Error.h:70:17 */
  export declare function GetStringErrorTypeId(): LLVM.ErrorTypeId;

  /** ./llvm-c/Error.h:75:14 */
  export declare function CreateStringError(ErrMsg: Pointer<"ErrMsg">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:457:6 */
  export declare function OrcExecutionSessionSetErrorReporter(ES: LLVM.OrcExecutionSessionRef, ReportError: LLVM.OrcErrorReporterFunction, Ctx: Pointer<"Ctx">): void;

  /** ./llvm-c/Orc.h:468:1 */
  export declare function OrcExecutionSessionGetSymbolStringPool(ES: LLVM.OrcExecutionSessionRef): LLVM.OrcSymbolStringPoolRef;

  /** ./llvm-c/Orc.h:480:6 */
  export declare function OrcSymbolStringPoolClearDeadEntries(SSP: LLVM.OrcSymbolStringPoolRef): void;

  /** ./llvm-c/Orc.h:495:1 */
  export declare function OrcExecutionSessionIntern(ES: LLVM.OrcExecutionSessionRef, Name: Pointer<"Name">): LLVM.OrcSymbolStringPoolEntryRef;

  /** ./llvm-c/Orc.h:500:6 */
  export declare function OrcRetainSymbolStringPoolEntry(S: LLVM.OrcSymbolStringPoolEntryRef): void;

  /** ./llvm-c/Orc.h:505:6 */
  export declare function OrcReleaseSymbolStringPoolEntry(S: LLVM.OrcSymbolStringPoolEntryRef): void;

  /** ./llvm-c/Orc.h:507:13 */
  export declare function OrcSymbolStringPoolEntryStr(S: LLVM.OrcSymbolStringPoolEntryRef): Pointer<"LLVMOrcSymbolStringPoolEntryStr">;

  /** ./llvm-c/Orc.h:512:6 */
  export declare function OrcReleaseResourceTracker(RT: LLVM.OrcResourceTrackerRef): void;

  /** ./llvm-c/Orc.h:518:6 */
  export declare function OrcResourceTrackerTransferTo(SrcRT: LLVM.OrcResourceTrackerRef, DstRT: LLVM.OrcResourceTrackerRef): void;

  /** ./llvm-c/Orc.h:525:14 */
  export declare function OrcResourceTrackerRemove(RT: LLVM.OrcResourceTrackerRef): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:532:6 */
  export declare function OrcDisposeDefinitionGenerator(DG: LLVM.OrcDefinitionGeneratorRef): void;

  /** ./llvm-c/Orc.h:537:6 */
  export declare function OrcDisposeMaterializationUnit(MU: LLVM.OrcMaterializationUnitRef): void;

  /** ./llvm-c/Orc.h:572:31 */
  export declare function OrcCreateCustomMaterializationUnit(Name: Pointer<"Name">, Ctx: Pointer<"Ctx">, Syms: LLVM.OrcCSymbolFlagsMapPairs, NumSyms: Opaque<number, "NumSyms">, InitSym: LLVM.OrcSymbolStringPoolEntryRef, Materialize: LLVM.OrcMaterializationUnitMaterializeFunction, Discard: LLVM.OrcMaterializationUnitDiscardFunction, Destroy: LLVM.OrcMaterializationUnitDestroyFunction): LLVM.OrcMaterializationUnitRef;

  /** ./llvm-c/Orc.h:601:1 */
  export declare function OrcAbsoluteSymbols(Syms: LLVM.OrcCSymbolMapPairs, NumPairs: Opaque<number, "NumPairs">): LLVM.OrcMaterializationUnitRef;

  /** ./llvm-c/Orc.h:624:31 */
  export declare function OrcLazyReexports(LCTM: LLVM.OrcLazyCallThroughManagerRef, ISM: LLVM.OrcIndirectStubsManagerRef, SourceRef: LLVM.OrcJITDylibRef, CallableAliases: LLVM.OrcCSymbolAliasMapPairs, NumPairs: Opaque<number, "NumPairs">): LLVM.OrcMaterializationUnitRef;

  /** ./llvm-c/Orc.h:639:6 */
  export declare function OrcDisposeMaterializationResponsibility(MR: LLVM.OrcMaterializationResponsibilityRef): void;

  /** ./llvm-c/Orc.h:645:20 */
  export declare function OrcMaterializationResponsibilityGetTargetDylib(MR: LLVM.OrcMaterializationResponsibilityRef): LLVM.OrcJITDylibRef;

  /** ./llvm-c/Orc.h:652:1 */
  export declare function OrcMaterializationResponsibilityGetExecutionSession(MR: LLVM.OrcMaterializationResponsibilityRef): LLVM.OrcExecutionSessionRef;

  /** ./llvm-c/Orc.h:665:29 */
  export declare function OrcMaterializationResponsibilityGetSymbols(MR: LLVM.OrcMaterializationResponsibilityRef, NumPairs: Pointer<"NumPairs">): LLVM.OrcCSymbolFlagsMapPairs;

  /** ./llvm-c/Orc.h:673:6 */
  export declare function OrcDisposeCSymbolFlagsMap(Pairs: LLVM.OrcCSymbolFlagsMapPairs): void;

  /** ./llvm-c/Orc.h:684:1 */
  export declare function OrcMaterializationResponsibilityGetInitializerSymbol(MR: LLVM.OrcMaterializationResponsibilityRef): LLVM.OrcSymbolStringPoolEntryRef;

  /** ./llvm-c/Orc.h:694:1 */
  export declare function OrcMaterializationResponsibilityGetRequestedSymbols(MR: LLVM.OrcMaterializationResponsibilityRef, NumSymbols: Pointer<"NumSymbols">): Pointer<"LLVMOrcMaterializationResponsibilityGetRequestedSymbols">;

  /** ./llvm-c/Orc.h:702:6 */
  export declare function OrcDisposeSymbols(Symbols: Pointer<"Symbols">): void;

  /** ./llvm-c/Orc.h:720:14 */
  export declare function OrcMaterializationResponsibilityNotifyResolved(MR: LLVM.OrcMaterializationResponsibilityRef, Symbols: LLVM.OrcCSymbolMapPairs, NumPairs: Opaque<number, "NumPairs">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:737:14 */
  export declare function OrcMaterializationResponsibilityNotifyEmitted(MR: LLVM.OrcMaterializationResponsibilityRef): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:753:14 */
  export declare function OrcMaterializationResponsibilityDefineMaterializing(MR: LLVM.OrcMaterializationResponsibilityRef, Pairs: LLVM.OrcCSymbolFlagsMapPairs, NumPairs: Opaque<number, "NumPairs">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:764:6 */
  export declare function OrcMaterializationResponsibilityFailMaterialization(MR: LLVM.OrcMaterializationResponsibilityRef): void;

  /** ./llvm-c/Orc.h:774:14 */
  export declare function OrcMaterializationResponsibilityReplace(MR: LLVM.OrcMaterializationResponsibilityRef, MU: LLVM.OrcMaterializationUnitRef): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:786:14 */
  export declare function OrcMaterializationResponsibilityDelegate(MR: LLVM.OrcMaterializationResponsibilityRef, Symbols: Pointer<"Symbols">, NumSymbols: Opaque<number, "NumSymbols">, Result: Pointer<"Result">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:809:6 */
  export declare function OrcMaterializationResponsibilityAddDependencies(MR: LLVM.OrcMaterializationResponsibilityRef, Name: LLVM.OrcSymbolStringPoolEntryRef, Dependencies: LLVM.OrcCDependenceMapPairs, NumPairs: Opaque<number, "NumPairs">): void;

  /** ./llvm-c/Orc.h:819:6 */
  export declare function OrcMaterializationResponsibilityAddDependenciesForAll(MR: LLVM.OrcMaterializationResponsibilityRef, Dependencies: LLVM.OrcCDependenceMapPairs, NumPairs: Opaque<number, "NumPairs">): void;

  /** ./llvm-c/Orc.h:833:1 */
  export declare function OrcExecutionSessionCreateBareJITDylib(ES: LLVM.OrcExecutionSessionRef, Name: Pointer<"Name">): LLVM.OrcJITDylibRef;

  /** ./llvm-c/Orc.h:849:1 */
  export declare function OrcExecutionSessionCreateJITDylib(ES: LLVM.OrcExecutionSessionRef, Result: Pointer<"Result">, Name: Pointer<"Name">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:858:1 */
  export declare function OrcExecutionSessionGetJITDylibByName(ES: LLVM.OrcExecutionSessionRef, Name: Pointer<"Name">): LLVM.OrcJITDylibRef;

  /** ./llvm-c/Orc.h:867:1 */
  export declare function OrcJITDylibCreateResourceTracker(JD: LLVM.OrcJITDylibRef): LLVM.OrcResourceTrackerRef;

  /** ./llvm-c/Orc.h:875:1 */
  export declare function OrcJITDylibGetDefaultResourceTracker(JD: LLVM.OrcJITDylibRef): LLVM.OrcResourceTrackerRef;

  /** ./llvm-c/Orc.h:884:14 */
  export declare function OrcJITDylibDefine(JD: LLVM.OrcJITDylibRef, MU: LLVM.OrcMaterializationUnitRef): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:891:14 */
  export declare function OrcJITDylibClear(JD: LLVM.OrcJITDylibRef): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:899:6 */
  export declare function OrcJITDylibAddGenerator(JD: LLVM.OrcJITDylibRef, DG: LLVM.OrcDefinitionGeneratorRef): void;

  /** ./llvm-c/Orc.h:905:31 */
  export declare function OrcCreateCustomCAPIDefinitionGenerator(F: LLVM.OrcCAPIDefinitionGeneratorTryToGenerateFunction, Ctx: Pointer<"Ctx">): LLVM.OrcDefinitionGeneratorRef;

  /** ./llvm-c/Orc.h:926:14 */
  export declare function OrcCreateDynamicLibrarySearchGeneratorForProcess(Result: Pointer<"Result">, GlobalPrefx: Opaque<number, "GlobalPrefx">, Filter: LLVM.OrcSymbolPredicate, FilterCtx: Pointer<"FilterCtx">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:951:14 */
  export declare function OrcCreateDynamicLibrarySearchGeneratorForPath(Result: Pointer<"Result">, FileName: Pointer<"FileName">, GlobalPrefix: Opaque<number, "GlobalPrefix">, Filter: LLVM.OrcSymbolPredicate, FilterCtx: Pointer<"FilterCtx">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:969:14 */
  export declare function OrcCreateStaticLibrarySearchGeneratorForPath(Result: Pointer<"Result">, ObjLayer: LLVM.OrcObjectLayerRef, FileName: Pointer<"FileName">, TargetTriple: Pointer<"TargetTriple">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:981:29 */
  export declare function OrcCreateNewThreadSafeContext(): LLVM.OrcThreadSafeContextRef;

  /** ./llvm-c/Orc.h:987:1 */
  export declare function OrcThreadSafeContextGetContext(TSCtx: LLVM.OrcThreadSafeContextRef): LLVM.ContextRef;

  /** ./llvm-c/Orc.h:992:6 */
  export declare function OrcDisposeThreadSafeContext(TSCtx: LLVM.OrcThreadSafeContextRef): void;

  /** ./llvm-c/Orc.h:1005:1 */
  export declare function OrcCreateNewThreadSafeModule(M: LLVM.ModuleRef, TSCtx: LLVM.OrcThreadSafeContextRef): LLVM.OrcThreadSafeModuleRef;

  /** ./llvm-c/Orc.h:1013:6 */
  export declare function OrcDisposeThreadSafeModule(TSM: LLVM.OrcThreadSafeModuleRef): void;

  /** ./llvm-c/Orc.h:1019:1 */
  export declare function OrcThreadSafeModuleWithModuleDo(TSM: LLVM.OrcThreadSafeModuleRef, F: LLVM.OrcGenericIRModuleOperationFunction, Ctx: Pointer<"Ctx">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:1031:14 */
  export declare function OrcJITTargetMachineBuilderDetectHost(Result: Pointer<"Result">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:1044:1 */
  export declare function OrcJITTargetMachineBuilderCreateFromTargetMachine(TM: LLVM.TargetMachineRef): LLVM.OrcJITTargetMachineBuilderRef;

  /** ./llvm-c/Orc.h:1049:6 */
  export declare function OrcDisposeJITTargetMachineBuilder(JTMB: LLVM.OrcJITTargetMachineBuilderRef): void;

  /** ./llvm-c/Orc.h:1058:7 */
  export declare function OrcJITTargetMachineBuilderGetTargetTriple(JTMB: LLVM.OrcJITTargetMachineBuilderRef): Pointer<"LLVMOrcJITTargetMachineBuilderGetTargetTriple">;

  /** ./llvm-c/Orc.h:1065:6 */
  export declare function OrcJITTargetMachineBuilderSetTargetTriple(JTMB: LLVM.OrcJITTargetMachineBuilderRef, TargetTriple: Pointer<"TargetTriple">): void;

  /** ./llvm-c/Orc.h:1079:14 */
  export declare function OrcObjectLayerAddObjectFile(ObjLayer: LLVM.OrcObjectLayerRef, JD: LLVM.OrcJITDylibRef, ObjBuffer: LLVM.MemoryBufferRef): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:1105:6 */
  export declare function OrcObjectLayerEmit(ObjLayer: LLVM.OrcObjectLayerRef, R: LLVM.OrcMaterializationResponsibilityRef, ObjBuffer: LLVM.MemoryBufferRef): void;

  /** ./llvm-c/Orc.h:1112:6 */
  export declare function OrcDisposeObjectLayer(ObjLayer: LLVM.OrcObjectLayerRef): void;

  /** ./llvm-c/Orc.h:1114:6 */
  export declare function OrcIRTransformLayerEmit(IRTransformLayer: LLVM.OrcIRTransformLayerRef, MR: LLVM.OrcMaterializationResponsibilityRef, TSM: LLVM.OrcThreadSafeModuleRef): void;

  /** ./llvm-c/Orc.h:1122:6 */
  export declare function OrcIRTransformLayerSetTransform(IRTransformLayer: LLVM.OrcIRTransformLayerRef, TransformFunction: LLVM.OrcIRTransformLayerTransformFunction, Ctx: Pointer<"Ctx">): void;

  /** ./llvm-c/Orc.h:1129:6 */
  export declare function OrcObjectTransformLayerSetTransform(ObjTransformLayer: LLVM.OrcObjectTransformLayerRef, TransformFunction: LLVM.OrcObjectTransformLayerTransformFunction, Ctx: Pointer<"Ctx">): void;

  /** ./llvm-c/Orc.h:1140:1 */
  export declare function OrcCreateLocalIndirectStubsManager(TargetTriple: Pointer<"TargetTriple">): LLVM.OrcIndirectStubsManagerRef;

  /** ./llvm-c/Orc.h:1145:6 */
  export declare function OrcDisposeIndirectStubsManager(ISM: LLVM.OrcIndirectStubsManagerRef): void;

  /** ./llvm-c/Orc.h:1147:14 */
  export declare function OrcCreateLocalLazyCallThroughManager(TargetTriple: Pointer<"TargetTriple">, ES: LLVM.OrcExecutionSessionRef, ErrorHandlerAddr: LLVM.OrcJITTargetAddress, LCTM: Pointer<"LCTM">): LLVM.ErrorRef;

  /** ./llvm-c/Orc.h:1155:6 */
  export declare function OrcDisposeLazyCallThroughManager(LCTM: LLVM.OrcLazyCallThroughManagerRef): void;

  /** ./llvm-c/Orc.h:1172:23 */
  export declare function OrcCreateDumpObjects(DumpDir: Pointer<"DumpDir">, IdentifierOverride: Pointer<"IdentifierOverride">): LLVM.OrcDumpObjectsRef;

  /** ./llvm-c/Orc.h:1178:6 */
  export declare function OrcDisposeDumpObjects(DumpObjects: LLVM.OrcDumpObjectsRef): void;

  /** ./llvm-c/Orc.h:1183:14 */
  export declare function OrcDumpObjects_CallOperator(DumpObjects: LLVM.OrcDumpObjectsRef, ObjBuffer: Pointer<"ObjBuffer">): LLVM.ErrorRef;

  /** ./llvm-c/OrcEE.h:47:1 */
  export declare function OrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager(ES: LLVM.OrcExecutionSessionRef): LLVM.OrcObjectLayerRef;

  /** ./llvm-c/OrcEE.h:56:6 */
  export declare function OrcRTDyldObjectLinkingLayerRegisterJITEventListener(RTDyldObjLinkingLayer: LLVM.OrcObjectLayerRef, Listener: LLVM.JITEventListenerRef): void;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:32:27 */
  export declare function PassManagerBuilderCreate(): LLVM.PassManagerBuilderRef;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:33:6 */
  export declare function PassManagerBuilderDispose(PMB: LLVM.PassManagerBuilderRef): void;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:37:1 */
  export declare function PassManagerBuilderSetOptLevel(PMB: LLVM.PassManagerBuilderRef, OptLevel: Opaque<number, "OptLevel">): void;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:42:1 */
  export declare function PassManagerBuilderSetSizeLevel(PMB: LLVM.PassManagerBuilderRef, SizeLevel: Opaque<number, "SizeLevel">): void;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:47:1 */
  export declare function PassManagerBuilderSetDisableUnitAtATime(PMB: LLVM.PassManagerBuilderRef, Value: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:52:1 */
  export declare function PassManagerBuilderSetDisableUnrollLoops(PMB: LLVM.PassManagerBuilderRef, Value: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:57:1 */
  export declare function PassManagerBuilderSetDisableSimplifyLibCalls(PMB: LLVM.PassManagerBuilderRef, Value: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:62:1 */
  export declare function PassManagerBuilderUseInlinerWithThreshold(PMB: LLVM.PassManagerBuilderRef, Threshold: Opaque<number, "Threshold">): void;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:67:1 */
  export declare function PassManagerBuilderPopulateFunctionPassManager(PMB: LLVM.PassManagerBuilderRef, PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/PassManagerBuilder.h:72:1 */
  export declare function PassManagerBuilderPopulateModulePassManager(PMB: LLVM.PassManagerBuilderRef, PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Utils.h:35:6 */
  export declare function AddLowerSwitchPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Utils.h:38:6 */
  export declare function AddPromoteMemoryToRegisterPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Utils.h:41:6 */
  export declare function AddAddDiscriminatorsPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:31:6 */
  export declare function AddConstantMergePass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:34:6 */
  export declare function AddMergeFunctionsPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:37:6 */
  export declare function AddCalledValuePropagationPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:40:6 */
  export declare function AddDeadArgEliminationPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:43:6 */
  export declare function AddFunctionAttrsPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:46:6 */
  export declare function AddFunctionInliningPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:49:6 */
  export declare function AddAlwaysInlinerPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:52:6 */
  export declare function AddGlobalDCEPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:55:6 */
  export declare function AddGlobalOptimizerPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:58:6 */
  export declare function AddPruneEHPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:61:6 */
  export declare function AddIPSCCPPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:64:6 */
  export declare function AddInternalizePass(_0: LLVM.PassManagerRef, AllButMain: Opaque<number, "AllButMain">): void;

  /** ./llvm-c/Transforms/IPO.h:76:6 */
  export declare function AddInternalizePassWithMustPreservePredicate(PM: LLVM.PassManagerRef, Context: Pointer<"Context">, MustPreserve: FnPointer<"MustPreserve">): void;

  /** ./llvm-c/Transforms/IPO.h:82:6 */
  export declare function AddStripDeadPrototypesPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/IPO.h:85:6 */
  export declare function AddStripSymbolsPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/PassBuilder.h:49:14 */
  export declare function RunPasses(M: LLVM.ModuleRef, Passes: Pointer<"Passes">, TM: LLVM.TargetMachineRef, Options: LLVM.PassBuilderOptionsRef): LLVM.ErrorRef;

  /** ./llvm-c/Transforms/PassBuilder.h:60:27 */
  export declare function CreatePassBuilderOptions(): LLVM.PassBuilderOptionsRef;

  /** ./llvm-c/Transforms/PassBuilder.h:66:6 */
  export declare function PassBuilderOptionsSetVerifyEach(Options: LLVM.PassBuilderOptionsRef, VerifyEach: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassBuilder.h:72:6 */
  export declare function PassBuilderOptionsSetDebugLogging(Options: LLVM.PassBuilderOptionsRef, DebugLogging: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassBuilder.h:75:6 */
  export declare function PassBuilderOptionsSetLoopInterleaving(Options: LLVM.PassBuilderOptionsRef, LoopInterleaving: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassBuilder.h:78:6 */
  export declare function PassBuilderOptionsSetLoopVectorization(Options: LLVM.PassBuilderOptionsRef, LoopVectorization: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassBuilder.h:81:6 */
  export declare function PassBuilderOptionsSetSLPVectorization(Options: LLVM.PassBuilderOptionsRef, SLPVectorization: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassBuilder.h:84:6 */
  export declare function PassBuilderOptionsSetLoopUnrolling(Options: LLVM.PassBuilderOptionsRef, LoopUnrolling: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassBuilder.h:87:6 */
  export declare function PassBuilderOptionsSetForgetAllSCEVInLoopUnroll(Options: LLVM.PassBuilderOptionsRef, ForgetAllSCEVInLoopUnroll: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassBuilder.h:90:6 */
  export declare function PassBuilderOptionsSetLicmMssaOptCap(Options: LLVM.PassBuilderOptionsRef, LicmMssaOptCap: Opaque<number, "LicmMssaOptCap">): void;

  /** ./llvm-c/Transforms/PassBuilder.h:93:6 */
  export declare function PassBuilderOptionsSetLicmMssaNoAccForPromotionCap(Options: LLVM.PassBuilderOptionsRef, LicmMssaNoAccForPromotionCap: Opaque<number, "LicmMssaNoAccForPromotionCap">): void;

  /** ./llvm-c/Transforms/PassBuilder.h:96:6 */
  export declare function PassBuilderOptionsSetCallGraphProfile(Options: LLVM.PassBuilderOptionsRef, CallGraphProfile: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassBuilder.h:99:6 */
  export declare function PassBuilderOptionsSetMergeFunctions(Options: LLVM.PassBuilderOptionsRef, MergeFunctions: LLVM.Bool): void;

  /** ./llvm-c/Transforms/PassBuilder.h:105:6 */
  export declare function DisposePassBuilderOptions(Options: LLVM.PassBuilderOptionsRef): void;

  /** ./llvm-c/Transforms/AggressiveInstCombine.h:31:6 */
  export declare function AddAggressiveInstCombinerPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:35:6 */
  export declare function AddAggressiveDCEPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:38:6 */
  export declare function AddDCEPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:41:6 */
  export declare function AddBitTrackingDCEPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:44:6 */
  export declare function AddAlignmentFromAssumptionsPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:47:6 */
  export declare function AddCFGSimplificationPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:50:6 */
  export declare function AddDeadStoreEliminationPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:53:6 */
  export declare function AddScalarizerPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:56:6 */
  export declare function AddMergedLoadStoreMotionPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:59:6 */
  export declare function AddGVNPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:62:6 */
  export declare function AddNewGVNPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:65:6 */
  export declare function AddIndVarSimplifyPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:68:6 */
  export declare function AddInstructionCombiningPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:71:6 */
  export declare function AddInstructionSimplifyPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:74:6 */
  export declare function AddJumpThreadingPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:77:6 */
  export declare function AddLICMPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:80:6 */
  export declare function AddLoopDeletionPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:83:6 */
  export declare function AddLoopIdiomPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:86:6 */
  export declare function AddLoopRotatePass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:89:6 */
  export declare function AddLoopRerollPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:92:6 */
  export declare function AddLoopUnrollPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:95:6 */
  export declare function AddLoopUnrollAndJamPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:98:6 */
  export declare function AddLowerAtomicPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:101:6 */
  export declare function AddMemCpyOptPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:104:6 */
  export declare function AddPartiallyInlineLibCallsPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:107:6 */
  export declare function AddReassociatePass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:110:6 */
  export declare function AddSCCPPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:113:6 */
  export declare function AddScalarReplAggregatesPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:116:6 */
  export declare function AddScalarReplAggregatesPassSSA(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:119:6 */
  export declare function AddScalarReplAggregatesPassWithThreshold(PM: LLVM.PassManagerRef, Threshold: Opaque<number, "Threshold">): void;

  /** ./llvm-c/Transforms/Scalar.h:123:6 */
  export declare function AddSimplifyLibCallsPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:126:6 */
  export declare function AddTailCallEliminationPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:129:6 */
  export declare function AddDemoteMemoryToRegisterPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:132:6 */
  export declare function AddVerifierPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:135:6 */
  export declare function AddCorrelatedValuePropagationPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:138:6 */
  export declare function AddEarlyCSEPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:141:6 */
  export declare function AddEarlyCSEMemSSAPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:144:6 */
  export declare function AddLowerExpectIntrinsicPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:147:6 */
  export declare function AddLowerConstantIntrinsicsPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:150:6 */
  export declare function AddTypeBasedAliasAnalysisPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:153:6 */
  export declare function AddScopedNoAliasAAPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:156:6 */
  export declare function AddBasicAliasAnalysisPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Scalar.h:159:6 */
  export declare function AddUnifyFunctionExitNodesPass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Vectorize.h:36:6 */
  export declare function AddLoopVectorizePass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Transforms/Vectorize.h:39:6 */
  export declare function AddSLPVectorizePass(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Linker.h:41:10 */
  export declare function LinkModules2(Dest: LLVM.ModuleRef, Src: LLVM.ModuleRef): LLVM.Bool;

  /** ./llvm-c/Remarks.h:64:20 */
  export declare function RemarkStringGetData(String: LLVM.RemarkStringRef): Pointer<"LLVMRemarkStringGetData">;

  /** ./llvm-c/Remarks.h:71:17 */
  export declare function RemarkStringGetLen(String: LLVM.RemarkStringRef): Opaque<number, "LLVMRemarkStringGetLen">;

  /** ./llvm-c/Remarks.h:86:1 */
  export declare function RemarkDebugLocGetSourceFilePath(DL: LLVM.RemarkDebugLocRef): LLVM.RemarkStringRef;

  /** ./llvm-c/Remarks.h:93:17 */
  export declare function RemarkDebugLocGetSourceLine(DL: LLVM.RemarkDebugLocRef): Opaque<number, "LLVMRemarkDebugLocGetSourceLine">;

  /** ./llvm-c/Remarks.h:100:17 */
  export declare function RemarkDebugLocGetSourceColumn(DL: LLVM.RemarkDebugLocRef): Opaque<number, "LLVMRemarkDebugLocGetSourceColumn">;

  /** ./llvm-c/Remarks.h:117:28 */
  export declare function RemarkArgGetKey(Arg: LLVM.RemarkArgRef): LLVM.RemarkStringRef;

  /** ./llvm-c/Remarks.h:124:28 */
  export declare function RemarkArgGetValue(Arg: LLVM.RemarkArgRef): LLVM.RemarkStringRef;

  /** ./llvm-c/Remarks.h:133:30 */
  export declare function RemarkArgGetDebugLoc(Arg: LLVM.RemarkArgRef): LLVM.RemarkDebugLocRef;

  /** ./llvm-c/Remarks.h:147:13 */
  export declare function RemarkEntryDispose(Remark: LLVM.RemarkEntryRef): void;

  /** ./llvm-c/Remarks.h:155:28 */
  export declare function RemarkEntryGetType(Remark: LLVM.RemarkEntryRef): LLVM.RemarkType;

  /** ./llvm-c/Remarks.h:163:1 */
  export declare function RemarkEntryGetPassName(Remark: LLVM.RemarkEntryRef): LLVM.RemarkStringRef;

  /** ./llvm-c/Remarks.h:171:1 */
  export declare function RemarkEntryGetRemarkName(Remark: LLVM.RemarkEntryRef): LLVM.RemarkStringRef;

  /** ./llvm-c/Remarks.h:179:1 */
  export declare function RemarkEntryGetFunctionName(Remark: LLVM.RemarkEntryRef): LLVM.RemarkStringRef;

  /** ./llvm-c/Remarks.h:189:1 */
  export declare function RemarkEntryGetDebugLoc(Remark: LLVM.RemarkEntryRef): LLVM.RemarkDebugLocRef;

  /** ./llvm-c/Remarks.h:198:17 */
  export declare function RemarkEntryGetHotness(Remark: LLVM.RemarkEntryRef): Opaque<bigint, "LLVMRemarkEntryGetHotness">;

  /** ./llvm-c/Remarks.h:205:17 */
  export declare function RemarkEntryGetNumArgs(Remark: LLVM.RemarkEntryRef): Opaque<number, "LLVMRemarkEntryGetNumArgs">;

  /** ./llvm-c/Remarks.h:216:25 */
  export declare function RemarkEntryGetFirstArg(Remark: LLVM.RemarkEntryRef): LLVM.RemarkArgRef;

  /** ./llvm-c/Remarks.h:227:25 */
  export declare function RemarkEntryGetNextArg(It: LLVM.RemarkArgRef, Remark: LLVM.RemarkEntryRef): LLVM.RemarkArgRef;

  /** ./llvm-c/Remarks.h:243:28 */
  export declare function RemarkParserCreateYAML(Buf: Pointer<"Buf">, Size: Opaque<bigint, "Size">): LLVM.RemarkParserRef;

  /** ./llvm-c/Remarks.h:257:28 */
  export declare function RemarkParserCreateBitstream(Buf: Pointer<"Buf">, Size: Opaque<bigint, "Size">): LLVM.RemarkParserRef;

  /** ./llvm-c/Remarks.h:302:27 */
  export declare function RemarkParserGetNext(Parser: LLVM.RemarkParserRef): LLVM.RemarkEntryRef;

  /** ./llvm-c/Remarks.h:309:17 */
  export declare function RemarkParserHasError(Parser: LLVM.RemarkParserRef): LLVM.Bool;

  /** ./llvm-c/Remarks.h:322:20 */
  export declare function RemarkParserGetErrorMessage(Parser: LLVM.RemarkParserRef): Pointer<"LLVMRemarkParserGetErrorMessage">;

  /** ./llvm-c/Remarks.h:329:13 */
  export declare function RemarkParserDispose(Parser: LLVM.RemarkParserRef): void;

  /** ./llvm-c/ErrorHandling.h:36:6 */
  export declare function InstallFatalErrorHandler(Handler: LLVM.FatalErrorHandler): void;

  /** ./llvm-c/ErrorHandling.h:42:6 */
  export declare function ResetFatalErrorHandler(): void;

  /** ./llvm-c/ErrorHandling.h:49:6 */
  export declare function EnablePrettyStackTrace(): void;

  /** ./llvm-c/Core.h:475:6 */
  export declare function InitializeCore(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Core.h:480:6 */
  export declare function Shutdown(): void;

  /** ./llvm-c/Core.h:484:7 */
  export declare function CreateMessage(Message: Pointer<"Message">): Pointer<"LLVMCreateMessage">;

  /** ./llvm-c/Core.h:485:6 */
  export declare function DisposeMessage(Message: Pointer<"Message">): void;

  /** ./llvm-c/Core.h:508:16 */
  export declare function ContextCreate(): LLVM.ContextRef;

  /** ./llvm-c/Core.h:513:16 */
  export declare function GetGlobalContext(): LLVM.ContextRef;

  /** ./llvm-c/Core.h:518:6 */
  export declare function ContextSetDiagnosticHandler(C: LLVM.ContextRef, Handler: LLVM.DiagnosticHandler, DiagnosticContext: Pointer<"DiagnosticContext">): void;

  /** ./llvm-c/Core.h:525:23 */
  export declare function ContextGetDiagnosticHandler(C: LLVM.ContextRef): LLVM.DiagnosticHandler;

  /** ./llvm-c/Core.h:530:7 */
  export declare function ContextGetDiagnosticContext(C: LLVM.ContextRef): Pointer<"LLVMContextGetDiagnosticContext">;

  /** ./llvm-c/Core.h:537:6 */
  export declare function ContextSetYieldCallback(C: LLVM.ContextRef, Callback: LLVM.YieldCallback, OpaqueHandle: Pointer<"OpaqueHandle">): void;

  /** ./llvm-c/Core.h:545:10 */
  export declare function ContextShouldDiscardValueNames(C: LLVM.ContextRef): LLVM.Bool;

  /** ./llvm-c/Core.h:555:6 */
  export declare function ContextSetDiscardValueNames(C: LLVM.ContextRef, Discard: LLVM.Bool): void;

  /** ./llvm-c/Core.h:562:6 */
  export declare function ContextSetOpaquePointers(C: LLVM.ContextRef, OpaquePointers: LLVM.Bool): void;

  /** ./llvm-c/Core.h:570:6 */
  export declare function ContextDispose(C: LLVM.ContextRef): void;

  /** ./llvm-c/Core.h:578:7 */
  export declare function GetDiagInfoDescription(DI: LLVM.DiagnosticInfoRef): Pointer<"LLVMGetDiagInfoDescription">;

  /** ./llvm-c/Core.h:585:24 */
  export declare function GetDiagInfoSeverity(DI: LLVM.DiagnosticInfoRef): LLVM.DiagnosticSeverity;

  /** ./llvm-c/Core.h:587:10 */
  export declare function GetMDKindIDInContext(C: LLVM.ContextRef, Name: Pointer<"Name">, SLen: Opaque<number, "SLen">): Opaque<number, "LLVMGetMDKindIDInContext">;

  /** ./llvm-c/Core.h:589:10 */
  export declare function GetMDKindID(Name: Pointer<"Name">, SLen: Opaque<number, "SLen">): Opaque<number, "LLVMGetMDKindID">;

  /** ./llvm-c/Core.h:602:10 */
  export declare function GetEnumAttributeKindForName(Name: Pointer<"Name">, SLen: Opaque<number, "SLen">): Opaque<number, "LLVMGetEnumAttributeKindForName">;

  /** ./llvm-c/Core.h:603:10 */
  export declare function GetLastEnumAttributeKind(): Opaque<number, "LLVMGetLastEnumAttributeKind">;

  /** ./llvm-c/Core.h:608:18 */
  export declare function CreateEnumAttribute(C: LLVM.ContextRef, KindID: Opaque<number, "KindID">, Val: Opaque<bigint, "Val">): LLVM.AttributeRef;

  /** ./llvm-c/Core.h:615:10 */
  export declare function GetEnumAttributeKind(A: LLVM.AttributeRef): Opaque<number, "LLVMGetEnumAttributeKind">;

  /** ./llvm-c/Core.h:620:10 */
  export declare function GetEnumAttributeValue(A: LLVM.AttributeRef): Opaque<bigint, "LLVMGetEnumAttributeValue">;

  /** ./llvm-c/Core.h:625:18 */
  export declare function CreateTypeAttribute(C: LLVM.ContextRef, KindID: Opaque<number, "KindID">, type_ref: LLVM.TypeRef): LLVM.AttributeRef;

  /** ./llvm-c/Core.h:631:13 */
  export declare function GetTypeAttributeValue(A: LLVM.AttributeRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:636:18 */
  export declare function CreateStringAttribute(C: LLVM.ContextRef, K: Pointer<"K">, KLength: Opaque<number, "KLength">, V: Pointer<"V">, VLength: Opaque<number, "VLength">): LLVM.AttributeRef;

  /** ./llvm-c/Core.h:643:13 */
  export declare function GetStringAttributeKind(A: LLVM.AttributeRef, Length: Pointer<"Length">): Pointer<"LLVMGetStringAttributeKind">;

  /** ./llvm-c/Core.h:648:13 */
  export declare function GetStringAttributeValue(A: LLVM.AttributeRef, Length: Pointer<"Length">): Pointer<"LLVMGetStringAttributeValue">;

  /** ./llvm-c/Core.h:653:10 */
  export declare function IsEnumAttribute(A: LLVM.AttributeRef): LLVM.Bool;

  /** ./llvm-c/Core.h:654:10 */
  export declare function IsStringAttribute(A: LLVM.AttributeRef): LLVM.Bool;

  /** ./llvm-c/Core.h:655:10 */
  export declare function IsTypeAttribute(A: LLVM.AttributeRef): LLVM.Bool;

  /** ./llvm-c/Core.h:660:13 */
  export declare function GetTypeByName2(C: LLVM.ContextRef, Name: Pointer<"Name">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:685:15 */
  export declare function ModuleCreateWithName(ModuleID: Pointer<"ModuleID">): LLVM.ModuleRef;

  /** ./llvm-c/Core.h:693:15 */
  export declare function ModuleCreateWithNameInContext(ModuleID: Pointer<"ModuleID">, C: LLVM.ContextRef): LLVM.ModuleRef;

  /** ./llvm-c/Core.h:698:15 */
  export declare function CloneModule(M: LLVM.ModuleRef): LLVM.ModuleRef;

  /** ./llvm-c/Core.h:706:6 */
  export declare function DisposeModule(M: LLVM.ModuleRef): void;

  /** ./llvm-c/Core.h:716:13 */
  export declare function GetModuleIdentifier(M: LLVM.ModuleRef, Len: Pointer<"Len">): Pointer<"LLVMGetModuleIdentifier">;

  /** ./llvm-c/Core.h:726:6 */
  export declare function SetModuleIdentifier(M: LLVM.ModuleRef, Ident: Pointer<"Ident">, Len: Opaque<number, "Len">): void;

  /** ./llvm-c/Core.h:736:13 */
  export declare function GetSourceFileName(M: LLVM.ModuleRef, Len: Pointer<"Len">): Pointer<"LLVMGetSourceFileName">;

  /** ./llvm-c/Core.h:747:6 */
  export declare function SetSourceFileName(M: LLVM.ModuleRef, Name: Pointer<"Name">, Len: Opaque<number, "Len">): void;

  /** ./llvm-c/Core.h:758:13 */
  export declare function GetDataLayoutStr(M: LLVM.ModuleRef): Pointer<"LLVMGetDataLayoutStr">;

  /** ./llvm-c/Core.h:759:13 */
  export declare function GetDataLayout(M: LLVM.ModuleRef): Pointer<"LLVMGetDataLayout">;

  /** ./llvm-c/Core.h:766:6 */
  export declare function SetDataLayout(M: LLVM.ModuleRef, DataLayoutStr: Pointer<"DataLayoutStr">): void;

  /** ./llvm-c/Core.h:773:13 */
  export declare function GetTarget(M: LLVM.ModuleRef): Pointer<"LLVMGetTarget">;

  /** ./llvm-c/Core.h:780:6 */
  export declare function SetTarget(M: LLVM.ModuleRef, Triple: Pointer<"Triple">): void;

  /** ./llvm-c/Core.h:789:22 */
  export declare function CopyModuleFlagsMetadata(M: LLVM.ModuleRef, Len: Pointer<"Len">): Pointer<"LLVMCopyModuleFlagsMetadata">;

  /** ./llvm-c/Core.h:794:6 */
  export declare function DisposeModuleFlagsMetadata(Entries: Pointer<"Entries">): void;

  /** ./llvm-c/Core.h:802:1 */
  export declare function ModuleFlagEntriesGetFlagBehavior(Entries: Pointer<"Entries">, Index: Opaque<number, "Index">): LLVM.ModuleFlagBehavior;

  /** ./llvm-c/Core.h:810:13 */
  export declare function ModuleFlagEntriesGetKey(Entries: Pointer<"Entries">, Index: Opaque<number, "Index">, Len: Pointer<"Len">): Pointer<"LLVMModuleFlagEntriesGetKey">;

  /** ./llvm-c/Core.h:818:17 */
  export declare function ModuleFlagEntriesGetMetadata(Entries: Pointer<"Entries">, Index: Opaque<number, "Index">): LLVM.MetadataRef;

  /** ./llvm-c/Core.h:827:17 */
  export declare function GetModuleFlag(M: LLVM.ModuleRef, Key: Pointer<"Key">, KeyLen: Opaque<number, "KeyLen">): LLVM.MetadataRef;

  /** ./llvm-c/Core.h:836:6 */
  export declare function AddModuleFlag(M: LLVM.ModuleRef, Behavior: LLVM.ModuleFlagBehavior, Key: Pointer<"Key">, KeyLen: Opaque<number, "KeyLen">, Val: LLVM.MetadataRef): void;

  /** ./llvm-c/Core.h:845:6 */
  export declare function DumpModule(M: LLVM.ModuleRef): void;

  /** ./llvm-c/Core.h:853:10 */
  export declare function PrintModuleToFile(M: LLVM.ModuleRef, Filename: Pointer<"Filename">, ErrorMessage: Pointer<"ErrorMessage">): LLVM.Bool;

  /** ./llvm-c/Core.h:862:7 */
  export declare function PrintModuleToString(M: LLVM.ModuleRef): Pointer<"LLVMPrintModuleToString">;

  /** ./llvm-c/Core.h:869:13 */
  export declare function GetModuleInlineAsm(M: LLVM.ModuleRef, Len: Pointer<"Len">): Pointer<"LLVMGetModuleInlineAsm">;

  /** ./llvm-c/Core.h:876:6 */
  export declare function SetModuleInlineAsm2(M: LLVM.ModuleRef, Asm: Pointer<"Asm">, Len: Opaque<number, "Len">): void;

  /** ./llvm-c/Core.h:883:6 */
  export declare function AppendModuleInlineAsm(M: LLVM.ModuleRef, Asm: Pointer<"Asm">, Len: Opaque<number, "Len">): void;

  /** ./llvm-c/Core.h:890:14 */
  export declare function GetInlineAsm(Ty: LLVM.TypeRef, AsmString: Pointer<"AsmString">, AsmStringSize: Opaque<number, "AsmStringSize">, Constraints: Pointer<"Constraints">, ConstraintsSize: Opaque<number, "ConstraintsSize">, HasSideEffects: LLVM.Bool, IsAlignStack: LLVM.Bool, Dialect: LLVM.InlineAsmDialect, CanThrow: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:901:16 */
  export declare function GetModuleContext(M: LLVM.ModuleRef): LLVM.ContextRef;

  /** ./llvm-c/Core.h:904:13 */
  export declare function GetTypeByName(M: LLVM.ModuleRef, Name: Pointer<"Name">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:911:20 */
  export declare function GetFirstNamedMetadata(M: LLVM.ModuleRef): LLVM.NamedMDNodeRef;

  /** ./llvm-c/Core.h:918:20 */
  export declare function GetLastNamedMetadata(M: LLVM.ModuleRef): LLVM.NamedMDNodeRef;

  /** ./llvm-c/Core.h:926:20 */
  export declare function GetNextNamedMetadata(NamedMDNode: LLVM.NamedMDNodeRef): LLVM.NamedMDNodeRef;

  /** ./llvm-c/Core.h:934:20 */
  export declare function GetPreviousNamedMetadata(NamedMDNode: LLVM.NamedMDNodeRef): LLVM.NamedMDNodeRef;

  /** ./llvm-c/Core.h:942:20 */
  export declare function GetNamedMetadata(M: LLVM.ModuleRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">): LLVM.NamedMDNodeRef;

  /** ./llvm-c/Core.h:951:20 */
  export declare function GetOrInsertNamedMetadata(M: LLVM.ModuleRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">): LLVM.NamedMDNodeRef;

  /** ./llvm-c/Core.h:960:13 */
  export declare function GetNamedMetadataName(NamedMD: LLVM.NamedMDNodeRef, NameLen: Pointer<"NameLen">): Pointer<"LLVMGetNamedMetadataName">;

  /** ./llvm-c/Core.h:968:10 */
  export declare function GetNamedMetadataNumOperands(M: LLVM.ModuleRef, Name: Pointer<"Name">): Opaque<number, "LLVMGetNamedMetadataNumOperands">;

  /** ./llvm-c/Core.h:981:6 */
  export declare function GetNamedMetadataOperands(M: LLVM.ModuleRef, Name: Pointer<"Name">, Dest: Pointer<"Dest">): void;

  /** ./llvm-c/Core.h:990:6 */
  export declare function AddNamedMetadataOperand(M: LLVM.ModuleRef, Name: Pointer<"Name">, Val: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:1001:13 */
  export declare function GetDebugLocDirectory(Val: LLVM.ValueRef, Length: Pointer<"Length">): Pointer<"LLVMGetDebugLocDirectory">;

  /** ./llvm-c/Core.h:1011:13 */
  export declare function GetDebugLocFilename(Val: LLVM.ValueRef, Length: Pointer<"Length">): Pointer<"LLVMGetDebugLocFilename">;

  /** ./llvm-c/Core.h:1021:10 */
  export declare function GetDebugLocLine(Val: LLVM.ValueRef): Opaque<number, "LLVMGetDebugLocLine">;

  /** ./llvm-c/Core.h:1029:10 */
  export declare function GetDebugLocColumn(Val: LLVM.ValueRef): Opaque<number, "LLVMGetDebugLocColumn">;

  /** ./llvm-c/Core.h:1036:14 */
  export declare function AddFunction(M: LLVM.ModuleRef, Name: Pointer<"Name">, FunctionTy: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1046:14 */
  export declare function GetNamedFunction(M: LLVM.ModuleRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1053:14 */
  export declare function GetFirstFunction(M: LLVM.ModuleRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1060:14 */
  export declare function GetLastFunction(M: LLVM.ModuleRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1068:14 */
  export declare function GetNextFunction(Fn: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1076:14 */
  export declare function GetPreviousFunction(Fn: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1079:6 */
  export declare function SetModuleInlineAsm(M: LLVM.ModuleRef, Asm: Pointer<"Asm">): void;

  /** ./llvm-c/Core.h:1119:14 */
  export declare function GetTypeKind(Ty: LLVM.TypeRef): LLVM.TypeKind;

  /** ./llvm-c/Core.h:1128:10 */
  export declare function TypeIsSized(Ty: LLVM.TypeRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1135:16 */
  export declare function GetTypeContext(Ty: LLVM.TypeRef): LLVM.ContextRef;

  /** ./llvm-c/Core.h:1142:6 */
  export declare function DumpType(Val: LLVM.TypeRef): void;

  /** ./llvm-c/Core.h:1150:7 */
  export declare function PrintTypeToString(Val: LLVM.TypeRef): Pointer<"LLVMPrintTypeToString">;

  /** ./llvm-c/Core.h:1163:13 */
  export declare function Int1TypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1164:13 */
  export declare function Int8TypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1165:13 */
  export declare function Int16TypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1166:13 */
  export declare function Int32TypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1167:13 */
  export declare function Int64TypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1168:13 */
  export declare function Int128TypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1169:13 */
  export declare function IntTypeInContext(C: LLVM.ContextRef, NumBits: Opaque<number, "NumBits">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1175:13 */
  export declare function Int1Type(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1176:13 */
  export declare function Int8Type(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1177:13 */
  export declare function Int16Type(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1178:13 */
  export declare function Int32Type(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1179:13 */
  export declare function Int64Type(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1180:13 */
  export declare function Int128Type(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1181:13 */
  export declare function IntType(NumBits: Opaque<number, "NumBits">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1182:10 */
  export declare function GetIntTypeWidth(IntegerTy: LLVM.TypeRef): Opaque<number, "LLVMGetIntTypeWidth">;

  /** ./llvm-c/Core.h:1197:13 */
  export declare function HalfTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1202:13 */
  export declare function BFloatTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1207:13 */
  export declare function FloatTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1212:13 */
  export declare function DoubleTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1217:13 */
  export declare function X86FP80TypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1223:13 */
  export declare function FP128TypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1228:13 */
  export declare function PPCFP128TypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1235:13 */
  export declare function HalfType(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1236:13 */
  export declare function BFloatType(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1237:13 */
  export declare function FloatType(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1238:13 */
  export declare function DoubleType(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1239:13 */
  export declare function X86FP80Type(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1240:13 */
  export declare function FP128Type(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1241:13 */
  export declare function PPCFP128Type(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1259:13 */
  export declare function FunctionType(ReturnType: LLVM.TypeRef, ParamTypes: Pointer<"ParamTypes">, ParamCount: Opaque<number, "ParamCount">, IsVarArg: LLVM.Bool): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1266:10 */
  export declare function IsFunctionVarArg(FunctionTy: LLVM.TypeRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1271:13 */
  export declare function GetReturnType(FunctionTy: LLVM.TypeRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1276:10 */
  export declare function CountParamTypes(FunctionTy: LLVM.TypeRef): Opaque<number, "LLVMCountParamTypes">;

  /** ./llvm-c/Core.h:1289:6 */
  export declare function GetParamTypes(FunctionTy: LLVM.TypeRef, Dest: Pointer<"Dest">): void;

  /** ./llvm-c/Core.h:1313:13 */
  export declare function StructTypeInContext(C: LLVM.ContextRef, ElementTypes: Pointer<"ElementTypes">, ElementCount: Opaque<number, "ElementCount">, Packed: LLVM.Bool): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1321:13 */
  export declare function StructType(ElementTypes: Pointer<"ElementTypes">, ElementCount: Opaque<number, "ElementCount">, Packed: LLVM.Bool): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1329:13 */
  export declare function StructCreateNamed(C: LLVM.ContextRef, Name: Pointer<"Name">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1336:13 */
  export declare function GetStructName(Ty: LLVM.TypeRef): Pointer<"LLVMGetStructName">;

  /** ./llvm-c/Core.h:1343:6 */
  export declare function StructSetBody(StructTy: LLVM.TypeRef, ElementTypes: Pointer<"ElementTypes">, ElementCount: Opaque<number, "ElementCount">, Packed: LLVM.Bool): void;

  /** ./llvm-c/Core.h:1351:10 */
  export declare function CountStructElementTypes(StructTy: LLVM.TypeRef): Opaque<number, "LLVMCountStructElementTypes">;

  /** ./llvm-c/Core.h:1363:6 */
  export declare function GetStructElementTypes(StructTy: LLVM.TypeRef, Dest: Pointer<"Dest">): void;

  /** ./llvm-c/Core.h:1370:13 */
  export declare function StructGetTypeAtIndex(StructTy: LLVM.TypeRef, i: Opaque<number, "i">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1377:10 */
  export declare function IsPackedStruct(StructTy: LLVM.TypeRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1384:10 */
  export declare function IsOpaqueStruct(StructTy: LLVM.TypeRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1391:10 */
  export declare function IsLiteralStruct(StructTy: LLVM.TypeRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1413:13 */
  export declare function GetElementType(Ty: LLVM.TypeRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1420:6 */
  export declare function GetSubtypes(Tp: LLVM.TypeRef, Arr: Pointer<"Arr">): void;

  /** ./llvm-c/Core.h:1427:10 */
  export declare function GetNumContainedTypes(Tp: LLVM.TypeRef): Opaque<number, "LLVMGetNumContainedTypes">;

  /** ./llvm-c/Core.h:1437:13 */
  export declare function ArrayType(ElementType: LLVM.TypeRef, ElementCount: Opaque<number, "ElementCount">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1446:10 */
  export declare function GetArrayLength(ArrayTy: LLVM.TypeRef): Opaque<number, "LLVMGetArrayLength">;

  /** ./llvm-c/Core.h:1456:13 */
  export declare function PointerType(ElementType: LLVM.TypeRef, AddressSpace: Opaque<number, "AddressSpace">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1465:10 */
  export declare function PointerTypeIsOpaque(Ty: LLVM.TypeRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1472:13 */
  export declare function PointerTypeInContext(C: LLVM.ContextRef, AddressSpace: Opaque<number, "AddressSpace">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1481:10 */
  export declare function GetPointerAddressSpace(PointerTy: LLVM.TypeRef): Opaque<number, "LLVMGetPointerAddressSpace">;

  /** ./llvm-c/Core.h:1492:13 */
  export declare function VectorType(ElementType: LLVM.TypeRef, ElementCount: Opaque<number, "ElementCount">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1503:13 */
  export declare function ScalableVectorType(ElementType: LLVM.TypeRef, ElementCount: Opaque<number, "ElementCount">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1513:10 */
  export declare function GetVectorSize(VectorTy: LLVM.TypeRef): Opaque<number, "LLVMGetVectorSize">;

  /** ./llvm-c/Core.h:1528:13 */
  export declare function VoidTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1533:13 */
  export declare function LabelTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1538:13 */
  export declare function X86MMXTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1543:13 */
  export declare function X86AMXTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1548:13 */
  export declare function TokenTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1553:13 */
  export declare function MetadataTypeInContext(C: LLVM.ContextRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1559:13 */
  export declare function VoidType(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1560:13 */
  export declare function LabelType(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1561:13 */
  export declare function X86MMXType(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1562:13 */
  export declare function X86AMXType(): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1698:13 */
  export declare function TypeOf(Val: LLVM.ValueRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:1705:15 */
  export declare function GetValueKind(Val: LLVM.ValueRef): LLVM.ValueKind;

  /** ./llvm-c/Core.h:1712:13 */
  export declare function GetValueName2(Val: LLVM.ValueRef, Length: Pointer<"Length">): Pointer<"LLVMGetValueName2">;

  /** ./llvm-c/Core.h:1719:6 */
  export declare function SetValueName2(Val: LLVM.ValueRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">): void;

  /** ./llvm-c/Core.h:1726:6 */
  export declare function DumpValue(Val: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:1734:7 */
  export declare function PrintValueToString(Val: LLVM.ValueRef): Pointer<"LLVMPrintValueToString">;

  /** ./llvm-c/Core.h:1741:6 */
  export declare function ReplaceAllUsesWith(OldVal: LLVM.ValueRef, NewVal: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:1746:10 */
  export declare function IsConstant(Val: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1751:10 */
  export declare function IsUndef(Val: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1756:10 */
  export declare function IsPoison(Val: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAArgument(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsABasicBlock(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAInlineAsm(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAUser(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstant(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsABlockAddress(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantAggregateZero(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantArray(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantDataSequential(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantDataArray(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantDataVector(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantExpr(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantFP(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantInt(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantPointerNull(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantStruct(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantTokenNone(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAConstantVector(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAGlobalValue(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAGlobalAlias(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAGlobalObject(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAFunction(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAGlobalVariable(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAGlobalIFunc(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAUndefValue(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAPoisonValue(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAInstruction(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAUnaryOperator(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsABinaryOperator(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsACallInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAIntrinsicInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsADbgInfoIntrinsic(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsADbgVariableIntrinsic(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsADbgDeclareInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsADbgLabelInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAMemIntrinsic(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAMemCpyInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAMemMoveInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAMemSetInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsACmpInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAFCmpInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAICmpInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAExtractElementInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAGetElementPtrInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAInsertElementInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAInsertValueInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsALandingPadInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAPHINode(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsASelectInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAShuffleVectorInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAStoreInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsABranchInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAIndirectBrInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAInvokeInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAReturnInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsASwitchInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAUnreachableInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAResumeInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsACleanupReturnInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsACatchReturnInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsACatchSwitchInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsACallBrInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAFuncletPadInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsACatchPadInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsACleanupPadInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAUnaryInstruction(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAAllocaInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsACastInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAAddrSpaceCastInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsABitCastInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAFPExtInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAFPToSIInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAFPToUIInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAFPTruncInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAIntToPtrInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAPtrToIntInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsASExtInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsASIToFPInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsATruncInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAUIToFPInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAZExtInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAExtractValueInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsALoadInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAVAArgInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAFreezeInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAAtomicCmpXchgInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAAtomicRMWInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1771:1 */
  export declare function IsAFenceInst(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1773:14 */
  export declare function IsAMDNode(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1774:14 */
  export declare function IsAMDString(Val: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1777:13 */
  export declare function GetValueName(Val: LLVM.ValueRef): Pointer<"LLVMGetValueName">;

  /** ./llvm-c/Core.h:1779:6 */
  export declare function SetValueName(Val: LLVM.ValueRef, Name: Pointer<"Name">): void;

  /** ./llvm-c/Core.h:1808:12 */
  export declare function GetFirstUse(Val: LLVM.ValueRef): LLVM.UseRef;

  /** ./llvm-c/Core.h:1816:12 */
  export declare function GetNextUse(U: LLVM.UseRef): LLVM.UseRef;

  /** ./llvm-c/Core.h:1825:14 */
  export declare function GetUser(U: LLVM.UseRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1832:14 */
  export declare function GetUsedValue(U: LLVM.UseRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1853:14 */
  export declare function GetOperand(Val: LLVM.ValueRef, Index: Opaque<number, "Index">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1860:12 */
  export declare function GetOperandUse(Val: LLVM.ValueRef, Index: Opaque<number, "Index">): LLVM.UseRef;

  /** ./llvm-c/Core.h:1867:6 */
  export declare function SetOperand(User: LLVM.ValueRef, Index: Opaque<number, "Index">, Val: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:1874:5 */
  export declare function GetNumOperands(Val: LLVM.ValueRef): Opaque<number, "LLVMGetNumOperands">;

  /** ./llvm-c/Core.h:1897:14 */
  export declare function ConstNull(Ty: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1907:14 */
  export declare function ConstAllOnes(Ty: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1914:14 */
  export declare function GetUndef(Ty: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1921:14 */
  export declare function GetPoison(Ty: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1928:10 */
  export declare function IsNull(Val: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:1934:14 */
  export declare function ConstPointerNull(Ty: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1963:14 */
  export declare function ConstInt(IntTy: LLVM.TypeRef, N: Opaque<bigint, "N">, SignExtend: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1971:14 */
  export declare function ConstIntOfArbitraryPrecision(IntTy: LLVM.TypeRef, NumWords: Opaque<number, "NumWords">, Words: Pointer<"Words">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1984:14 */
  export declare function ConstIntOfString(IntTy: LLVM.TypeRef, Text: Pointer<"Text">, Radix: Opaque<number, "Radix">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1993:14 */
  export declare function ConstIntOfStringAndSize(IntTy: LLVM.TypeRef, Text: Pointer<"Text">, SLen: Opaque<number, "SLen">, Radix: Opaque<number, "Radix">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:1999:14 */
  export declare function ConstReal(RealTy: LLVM.TypeRef, N: Opaque<bigint, "N">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2007:14 */
  export declare function ConstRealOfString(RealTy: LLVM.TypeRef, Text: Pointer<"Text">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2012:14 */
  export declare function ConstRealOfStringAndSize(RealTy: LLVM.TypeRef, Text: Pointer<"Text">, SLen: Opaque<number, "SLen">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2020:20 */
  export declare function ConstIntGetZExtValue(ConstantVal: LLVM.ValueRef): Opaque<bigint, "LLVMConstIntGetZExtValue">;

  /** ./llvm-c/Core.h:2027:11 */
  export declare function ConstIntGetSExtValue(ConstantVal: LLVM.ValueRef): Opaque<bigint, "LLVMConstIntGetSExtValue">;

  /** ./llvm-c/Core.h:2035:8 */
  export declare function ConstRealGetDouble(ConstantVal: LLVM.ValueRef, losesInfo: Pointer<"losesInfo">): Opaque<bigint, "LLVMConstRealGetDouble">;

  /** ./llvm-c/Core.h:2054:14 */
  export declare function ConstStringInContext(C: LLVM.ContextRef, Str: Pointer<"Str">, Length: Opaque<number, "Length">, DontNullTerminate: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2066:14 */
  export declare function ConstString(Str: Pointer<"Str">, Length: Opaque<number, "Length">, DontNullTerminate: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2074:10 */
  export declare function IsConstantString(c: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:2081:13 */
  export declare function GetAsString(c: LLVM.ValueRef, Length: Pointer<"Length">): Pointer<"LLVMGetAsString">;

  /** ./llvm-c/Core.h:2088:14 */
  export declare function ConstStructInContext(C: LLVM.ContextRef, ConstantVals: Pointer<"ConstantVals">, Count: Opaque<number, "Count">, Packed: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2100:14 */
  export declare function ConstStruct(ConstantVals: Pointer<"ConstantVals">, Count: Opaque<number, "Count">, Packed: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2108:14 */
  export declare function ConstArray(ElementTy: LLVM.TypeRef, ConstantVals: Pointer<"ConstantVals">, Length: Opaque<number, "Length">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2116:14 */
  export declare function ConstNamedStruct(StructTy: LLVM.TypeRef, ConstantVals: Pointer<"ConstantVals">, Count: Opaque<number, "Count">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2128:14 */
  export declare function GetAggregateElement(C: LLVM.ValueRef, Idx: Opaque<number, "Idx">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2135:1 */
  export declare function GetElementAsConstant(C: LLVM.ValueRef, idx: Opaque<number, "idx">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2144:14 */
  export declare function ConstVector(ScalarConstantVals: Pointer<"ScalarConstantVals">, Size: Opaque<number, "Size">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2159:12 */
  export declare function GetConstOpcode(ConstantVal: LLVM.ValueRef): LLVM.Opcode;

  /** ./llvm-c/Core.h:2160:14 */
  export declare function AlignOf(Ty: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2161:14 */
  export declare function SizeOf(Ty: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2162:14 */
  export declare function ConstNeg(ConstantVal: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2163:14 */
  export declare function ConstNSWNeg(ConstantVal: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2164:14 */
  export declare function ConstNUWNeg(ConstantVal: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2165:14 */
  export declare function ConstFNeg(ConstantVal: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2166:14 */
  export declare function ConstNot(ConstantVal: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2167:14 */
  export declare function ConstAdd(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2168:14 */
  export declare function ConstNSWAdd(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2169:14 */
  export declare function ConstNUWAdd(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2170:14 */
  export declare function ConstSub(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2171:14 */
  export declare function ConstNSWSub(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2172:14 */
  export declare function ConstNUWSub(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2173:14 */
  export declare function ConstMul(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2174:14 */
  export declare function ConstNSWMul(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2175:14 */
  export declare function ConstNUWMul(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2176:14 */
  export declare function ConstAnd(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2177:14 */
  export declare function ConstOr(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2178:14 */
  export declare function ConstXor(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2179:14 */
  export declare function ConstICmp(Predicate: LLVM.IntPredicate, LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2181:14 */
  export declare function ConstFCmp(Predicate: LLVM.RealPredicate, LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2183:14 */
  export declare function ConstShl(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2184:14 */
  export declare function ConstLShr(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2185:14 */
  export declare function ConstAShr(LHSConstant: LLVM.ValueRef, RHSConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2186:1 */
  export declare function ConstGEP(ConstantVal: LLVM.ValueRef, ConstantIndices: Pointer<"ConstantIndices">, NumIndices: Opaque<number, "NumIndices">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2191:14 */
  export declare function ConstGEP2(Ty: LLVM.TypeRef, ConstantVal: LLVM.ValueRef, ConstantIndices: Pointer<"ConstantIndices">, NumIndices: Opaque<number, "NumIndices">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2193:1 */
  export declare function ConstInBoundsGEP(ConstantVal: LLVM.ValueRef, ConstantIndices: Pointer<"ConstantIndices">, NumIndices: Opaque<number, "NumIndices">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2198:14 */
  export declare function ConstInBoundsGEP2(Ty: LLVM.TypeRef, ConstantVal: LLVM.ValueRef, ConstantIndices: Pointer<"ConstantIndices">, NumIndices: Opaque<number, "NumIndices">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2201:14 */
  export declare function ConstTrunc(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2202:14 */
  export declare function ConstSExt(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2203:14 */
  export declare function ConstZExt(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2204:14 */
  export declare function ConstFPTrunc(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2205:14 */
  export declare function ConstFPExt(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2206:14 */
  export declare function ConstUIToFP(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2207:14 */
  export declare function ConstSIToFP(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2208:14 */
  export declare function ConstFPToUI(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2209:14 */
  export declare function ConstFPToSI(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2210:14 */
  export declare function ConstPtrToInt(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2211:14 */
  export declare function ConstIntToPtr(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2212:14 */
  export declare function ConstBitCast(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2213:14 */
  export declare function ConstAddrSpaceCast(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2214:14 */
  export declare function ConstZExtOrBitCast(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2216:14 */
  export declare function ConstSExtOrBitCast(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2218:14 */
  export declare function ConstTruncOrBitCast(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2220:14 */
  export declare function ConstPointerCast(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2222:14 */
  export declare function ConstIntCast(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef, isSigned: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2224:14 */
  export declare function ConstFPCast(ConstantVal: LLVM.ValueRef, ToType: LLVM.TypeRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2225:14 */
  export declare function ConstSelect(ConstantCondition: LLVM.ValueRef, ConstantIfTrue: LLVM.ValueRef, ConstantIfFalse: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2228:14 */
  export declare function ConstExtractElement(VectorConstant: LLVM.ValueRef, IndexConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2230:14 */
  export declare function ConstInsertElement(VectorConstant: LLVM.ValueRef, ElementValueConstant: LLVM.ValueRef, IndexConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2233:14 */
  export declare function ConstShuffleVector(VectorAConstant: LLVM.ValueRef, VectorBConstant: LLVM.ValueRef, MaskConstant: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2236:14 */
  export declare function BlockAddress(F: LLVM.ValueRef, BB: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2239:14 */
  export declare function ConstInlineAsm(Ty: LLVM.TypeRef, AsmString: Pointer<"AsmString">, Constraints: Pointer<"Constraints">, HasSideEffects: LLVM.Bool, IsAlignStack: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2258:15 */
  export declare function GetGlobalParent(Global: LLVM.ValueRef): LLVM.ModuleRef;

  /** ./llvm-c/Core.h:2259:10 */
  export declare function IsDeclaration(Global: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:2260:13 */
  export declare function GetLinkage(Global: LLVM.ValueRef): LLVM.Linkage;

  /** ./llvm-c/Core.h:2261:6 */
  export declare function SetLinkage(Global: LLVM.ValueRef, Linkage: LLVM.Linkage): void;

  /** ./llvm-c/Core.h:2262:13 */
  export declare function GetSection(Global: LLVM.ValueRef): Pointer<"LLVMGetSection">;

  /** ./llvm-c/Core.h:2263:6 */
  export declare function SetSection(Global: LLVM.ValueRef, Section: Pointer<"Section">): void;

  /** ./llvm-c/Core.h:2264:16 */
  export declare function GetVisibility(Global: LLVM.ValueRef): LLVM.Visibility;

  /** ./llvm-c/Core.h:2265:6 */
  export declare function SetVisibility(Global: LLVM.ValueRef, Viz: LLVM.Visibility): void;

  /** ./llvm-c/Core.h:2266:21 */
  export declare function GetDLLStorageClass(Global: LLVM.ValueRef): LLVM.DLLStorageClass;

  /** ./llvm-c/Core.h:2267:6 */
  export declare function SetDLLStorageClass(Global: LLVM.ValueRef, Class: LLVM.DLLStorageClass): void;

  /** ./llvm-c/Core.h:2268:17 */
  export declare function GetUnnamedAddress(Global: LLVM.ValueRef): LLVM.UnnamedAddr;

  /** ./llvm-c/Core.h:2269:6 */
  export declare function SetUnnamedAddress(Global: LLVM.ValueRef, UnnamedAddr: LLVM.UnnamedAddr): void;

  /** ./llvm-c/Core.h:2277:13 */
  export declare function GlobalGetValueType(Global: LLVM.ValueRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:2280:10 */
  export declare function HasUnnamedAddr(Global: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:2282:6 */
  export declare function SetUnnamedAddr(Global: LLVM.ValueRef, HasUnnamedAddr: LLVM.Bool): void;

  /** ./llvm-c/Core.h:2300:10 */
  export declare function GetAlignment(V: LLVM.ValueRef): Opaque<number, "LLVMGetAlignment">;

  /** ./llvm-c/Core.h:2311:6 */
  export declare function SetAlignment(V: LLVM.ValueRef, Bytes: Opaque<number, "Bytes">): void;

  /** ./llvm-c/Core.h:2319:6 */
  export declare function GlobalSetMetadata(Global: LLVM.ValueRef, Kind: Opaque<number, "Kind">, MD: LLVM.MetadataRef): void;

  /** ./llvm-c/Core.h:2327:6 */
  export declare function GlobalEraseMetadata(Global: LLVM.ValueRef, Kind: Opaque<number, "Kind">): void;

  /** ./llvm-c/Core.h:2334:6 */
  export declare function GlobalClearMetadata(Global: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:2343:25 */
  export declare function GlobalCopyAllMetadata(Value: LLVM.ValueRef, NumEntries: Pointer<"NumEntries">): Pointer<"LLVMGlobalCopyAllMetadata">;

  /** ./llvm-c/Core.h:2349:6 */
  export declare function DisposeValueMetadataEntries(Entries: Pointer<"Entries">): void;

  /** ./llvm-c/Core.h:2354:10 */
  export declare function ValueMetadataEntriesGetKind(Entries: Pointer<"Entries">, Index: Opaque<number, "Index">): Opaque<number, "LLVMValueMetadataEntriesGetKind">;

  /** ./llvm-c/Core.h:2362:1 */
  export declare function ValueMetadataEntriesGetMetadata(Entries: Pointer<"Entries">, Index: Opaque<number, "Index">): LLVM.MetadataRef;

  /** ./llvm-c/Core.h:2378:14 */
  export declare function AddGlobal(M: LLVM.ModuleRef, Ty: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2379:14 */
  export declare function AddGlobalInAddressSpace(M: LLVM.ModuleRef, Ty: LLVM.TypeRef, Name: Pointer<"Name">, AddressSpace: Opaque<number, "AddressSpace">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2382:14 */
  export declare function GetNamedGlobal(M: LLVM.ModuleRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2383:14 */
  export declare function GetFirstGlobal(M: LLVM.ModuleRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2384:14 */
  export declare function GetLastGlobal(M: LLVM.ModuleRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2385:14 */
  export declare function GetNextGlobal(GlobalVar: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2386:14 */
  export declare function GetPreviousGlobal(GlobalVar: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2387:6 */
  export declare function DeleteGlobal(GlobalVar: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:2388:14 */
  export declare function GetInitializer(GlobalVar: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2389:6 */
  export declare function SetInitializer(GlobalVar: LLVM.ValueRef, ConstantVal: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:2390:10 */
  export declare function IsThreadLocal(GlobalVar: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:2391:6 */
  export declare function SetThreadLocal(GlobalVar: LLVM.ValueRef, IsThreadLocal: LLVM.Bool): void;

  /** ./llvm-c/Core.h:2392:10 */
  export declare function IsGlobalConstant(GlobalVar: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:2393:6 */
  export declare function SetGlobalConstant(GlobalVar: LLVM.ValueRef, IsConstant: LLVM.Bool): void;

  /** ./llvm-c/Core.h:2394:21 */
  export declare function GetThreadLocalMode(GlobalVar: LLVM.ValueRef): LLVM.ThreadLocalMode;

  /** ./llvm-c/Core.h:2395:6 */
  export declare function SetThreadLocalMode(GlobalVar: LLVM.ValueRef, Mode: LLVM.ThreadLocalMode): void;

  /** ./llvm-c/Core.h:2396:10 */
  export declare function IsExternallyInitialized(GlobalVar: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:2397:6 */
  export declare function SetExternallyInitialized(GlobalVar: LLVM.ValueRef, IsExtInit: LLVM.Bool): void;

  /** ./llvm-c/Core.h:2413:1 */
  export declare function AddAlias(M: LLVM.ModuleRef, Ty: LLVM.TypeRef, Aliasee: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2423:14 */
  export declare function AddAlias2(M: LLVM.ModuleRef, ValueTy: LLVM.TypeRef, AddrSpace: Opaque<number, "AddrSpace">, Aliasee: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2434:14 */
  export declare function GetNamedGlobalAlias(M: LLVM.ModuleRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2442:14 */
  export declare function GetFirstGlobalAlias(M: LLVM.ModuleRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2449:14 */
  export declare function GetLastGlobalAlias(M: LLVM.ModuleRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2457:14 */
  export declare function GetNextGlobalAlias(GA: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2465:14 */
  export declare function GetPreviousGlobalAlias(GA: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2470:14 */
  export declare function AliasGetAliasee(Alias: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2475:6 */
  export declare function AliasSetAliasee(Alias: LLVM.ValueRef, Aliasee: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:2497:6 */
  export declare function DeleteFunction(Fn: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:2504:10 */
  export declare function HasPersonalityFn(Fn: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:2511:14 */
  export declare function GetPersonalityFn(Fn: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2518:6 */
  export declare function SetPersonalityFn(Fn: LLVM.ValueRef, PersonalityFn: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:2525:10 */
  export declare function LookupIntrinsicID(Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">): Opaque<number, "LLVMLookupIntrinsicID">;

  /** ./llvm-c/Core.h:2532:10 */
  export declare function GetIntrinsicID(Fn: LLVM.ValueRef): Opaque<number, "LLVMGetIntrinsicID">;

  /** ./llvm-c/Core.h:2540:14 */
  export declare function GetIntrinsicDeclaration(Mod: LLVM.ModuleRef, ID: Opaque<number, "ID">, ParamTypes: Pointer<"ParamTypes">, ParamCount: Opaque<number, "ParamCount">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2551:13 */
  export declare function IntrinsicGetType(Ctx: LLVM.ContextRef, ID: Opaque<number, "ID">, ParamTypes: Pointer<"ParamTypes">, ParamCount: Opaque<number, "ParamCount">): LLVM.TypeRef;

  /** ./llvm-c/Core.h:2559:13 */
  export declare function IntrinsicGetName(ID: Opaque<number, "ID">, NameLength: Pointer<"NameLength">): Pointer<"LLVMIntrinsicGetName">;

  /** ./llvm-c/Core.h:2562:13 */
  export declare function IntrinsicCopyOverloadedName(ID: Opaque<number, "ID">, ParamTypes: Pointer<"ParamTypes">, ParamCount: Opaque<number, "ParamCount">, NameLength: Pointer<"NameLength">): Pointer<"LLVMIntrinsicCopyOverloadedName">;

  /** ./llvm-c/Core.h:2578:13 */
  export declare function IntrinsicCopyOverloadedName2(Mod: LLVM.ModuleRef, ID: Opaque<number, "ID">, ParamTypes: Pointer<"ParamTypes">, ParamCount: Opaque<number, "ParamCount">, NameLength: Pointer<"NameLength">): Pointer<"LLVMIntrinsicCopyOverloadedName2">;

  /** ./llvm-c/Core.h:2588:10 */
  export declare function IntrinsicIsOverloaded(ID: Opaque<number, "ID">): LLVM.Bool;

  /** ./llvm-c/Core.h:2597:10 */
  export declare function GetFunctionCallConv(Fn: LLVM.ValueRef): Opaque<number, "LLVMGetFunctionCallConv">;

  /** ./llvm-c/Core.h:2607:6 */
  export declare function SetFunctionCallConv(Fn: LLVM.ValueRef, CC: Opaque<number, "CC">): void;

  /** ./llvm-c/Core.h:2615:13 */
  export declare function GetGC(Fn: LLVM.ValueRef): Pointer<"LLVMGetGC">;

  /** ./llvm-c/Core.h:2622:6 */
  export declare function SetGC(Fn: LLVM.ValueRef, Name: Pointer<"Name">): void;

  /** ./llvm-c/Core.h:2629:6 */
  export declare function AddAttributeAtIndex(F: LLVM.ValueRef, Idx: LLVM.AttributeIndex, A: LLVM.AttributeRef): void;

  /** ./llvm-c/Core.h:2631:10 */
  export declare function GetAttributeCountAtIndex(F: LLVM.ValueRef, Idx: LLVM.AttributeIndex): Opaque<number, "LLVMGetAttributeCountAtIndex">;

  /** ./llvm-c/Core.h:2632:6 */
  export declare function GetAttributesAtIndex(F: LLVM.ValueRef, Idx: LLVM.AttributeIndex, Attrs: Pointer<"Attrs">): void;

  /** ./llvm-c/Core.h:2634:18 */
  export declare function GetEnumAttributeAtIndex(F: LLVM.ValueRef, Idx: LLVM.AttributeIndex, KindID: Opaque<number, "KindID">): LLVM.AttributeRef;

  /** ./llvm-c/Core.h:2637:18 */
  export declare function GetStringAttributeAtIndex(F: LLVM.ValueRef, Idx: LLVM.AttributeIndex, K: Pointer<"K">, KLen: Opaque<number, "KLen">): LLVM.AttributeRef;

  /** ./llvm-c/Core.h:2640:6 */
  export declare function RemoveEnumAttributeAtIndex(F: LLVM.ValueRef, Idx: LLVM.AttributeIndex, KindID: Opaque<number, "KindID">): void;

  /** ./llvm-c/Core.h:2642:6 */
  export declare function RemoveStringAttributeAtIndex(F: LLVM.ValueRef, Idx: LLVM.AttributeIndex, K: Pointer<"K">, KLen: Opaque<number, "KLen">): void;

  /** ./llvm-c/Core.h:2649:6 */
  export declare function AddTargetDependentFunctionAttr(Fn: LLVM.ValueRef, A: Pointer<"A">, V: Pointer<"V">): void;

  /** ./llvm-c/Core.h:2668:10 */
  export declare function CountParams(Fn: LLVM.ValueRef): Opaque<number, "LLVMCountParams">;

  /** ./llvm-c/Core.h:2681:6 */
  export declare function GetParams(Fn: LLVM.ValueRef, Params: Pointer<"Params">): void;

  /** ./llvm-c/Core.h:2690:14 */
  export declare function GetParam(Fn: LLVM.ValueRef, Index: Opaque<number, "Index">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2701:14 */
  export declare function GetParamParent(Inst: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2708:14 */
  export declare function GetFirstParam(Fn: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2715:14 */
  export declare function GetLastParam(Fn: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2724:14 */
  export declare function GetNextParam(Arg: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2731:14 */
  export declare function GetPreviousParam(Arg: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2739:6 */
  export declare function SetParamAlignment(Arg: LLVM.ValueRef, Align: Opaque<number, "Align">): void;

  /** ./llvm-c/Core.h:2761:14 */
  export declare function AddGlobalIFunc(M: LLVM.ModuleRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">, Ty: LLVM.TypeRef, AddrSpace: Opaque<number, "AddrSpace">, Resolver: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2773:14 */
  export declare function GetNamedGlobalIFunc(M: LLVM.ModuleRef, Name: Pointer<"Name">, NameLen: Opaque<number, "NameLen">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2781:14 */
  export declare function GetFirstGlobalIFunc(M: LLVM.ModuleRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2788:14 */
  export declare function GetLastGlobalIFunc(M: LLVM.ModuleRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2796:14 */
  export declare function GetNextGlobalIFunc(IFunc: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2804:14 */
  export declare function GetPreviousGlobalIFunc(IFunc: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2812:14 */
  export declare function GetGlobalIFuncResolver(IFunc: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2819:6 */
  export declare function SetGlobalIFuncResolver(IFunc: LLVM.ValueRef, Resolver: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:2826:6 */
  export declare function EraseGlobalIFunc(IFunc: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:2836:6 */
  export declare function RemoveGlobalIFunc(IFunc: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:2868:17 */
  export declare function MDStringInContext2(C: LLVM.ContextRef, Str: Pointer<"Str">, SLen: Opaque<number, "SLen">): LLVM.MetadataRef;

  /** ./llvm-c/Core.h:2876:17 */
  export declare function MDNodeInContext2(C: LLVM.ContextRef, MDs: Pointer<"MDs">, Count: Opaque<number, "Count">): LLVM.MetadataRef;

  /** ./llvm-c/Core.h:2882:14 */
  export declare function MetadataAsValue(C: LLVM.ContextRef, MD: LLVM.MetadataRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2887:17 */
  export declare function ValueAsMetadata(Val: LLVM.ValueRef): LLVM.MetadataRef;

  /** ./llvm-c/Core.h:2896:13 */
  export declare function GetMDString(V: LLVM.ValueRef, Length: Pointer<"Length">): Pointer<"LLVMGetMDString">;

  /** ./llvm-c/Core.h:2904:10 */
  export declare function GetMDNodeNumOperands(V: LLVM.ValueRef): Opaque<number, "LLVMGetMDNodeNumOperands">;

  /** ./llvm-c/Core.h:2917:6 */
  export declare function GetMDNodeOperands(V: LLVM.ValueRef, Dest: Pointer<"Dest">): void;

  /** ./llvm-c/Core.h:2920:14 */
  export declare function MDStringInContext(C: LLVM.ContextRef, Str: Pointer<"Str">, SLen: Opaque<number, "SLen">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2923:14 */
  export declare function MDString(Str: Pointer<"Str">, SLen: Opaque<number, "SLen">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2925:14 */
  export declare function MDNodeInContext(C: LLVM.ContextRef, Vals: Pointer<"Vals">, Count: Opaque<number, "Count">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2928:14 */
  export declare function MDNode(Vals: Pointer<"Vals">, Count: Opaque<number, "Count">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2954:14 */
  export declare function BasicBlockAsValue(BB: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2959:10 */
  export declare function ValueIsBasicBlock(Val: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:2964:19 */
  export declare function ValueAsBasicBlock(Val: LLVM.ValueRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:2969:13 */
  export declare function GetBasicBlockName(BB: LLVM.BasicBlockRef): Pointer<"LLVMGetBasicBlockName">;

  /** ./llvm-c/Core.h:2976:14 */
  export declare function GetBasicBlockParent(BB: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2988:14 */
  export declare function GetBasicBlockTerminator(BB: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:2995:10 */
  export declare function CountBasicBlocks(Fn: LLVM.ValueRef): Opaque<number, "LLVMCountBasicBlocks">;

  /** ./llvm-c/Core.h:3005:6 */
  export declare function GetBasicBlocks(Fn: LLVM.ValueRef, BasicBlocks: Pointer<"BasicBlocks">): void;

  /** ./llvm-c/Core.h:3015:19 */
  export declare function GetFirstBasicBlock(Fn: LLVM.ValueRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3022:19 */
  export declare function GetLastBasicBlock(Fn: LLVM.ValueRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3027:19 */
  export declare function GetNextBasicBlock(BB: LLVM.BasicBlockRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3032:19 */
  export declare function GetPreviousBasicBlock(BB: LLVM.BasicBlockRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3040:19 */
  export declare function GetEntryBasicBlock(Fn: LLVM.ValueRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3049:6 */
  export declare function InsertExistingBasicBlockAfterInsertBlock(Builder: LLVM.BuilderRef, BB: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3057:6 */
  export declare function AppendExistingBasicBlock(Fn: LLVM.ValueRef, BB: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3065:19 */
  export declare function CreateBasicBlockInContext(C: LLVM.ContextRef, Name: Pointer<"Name">): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3073:19 */
  export declare function AppendBasicBlockInContext(C: LLVM.ContextRef, Fn: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3083:19 */
  export declare function AppendBasicBlock(Fn: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3093:19 */
  export declare function InsertBasicBlockInContext(C: LLVM.ContextRef, BB: LLVM.BasicBlockRef, Name: Pointer<"Name">): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3102:19 */
  export declare function InsertBasicBlock(InsertBeforeBB: LLVM.BasicBlockRef, Name: Pointer<"Name">): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3113:6 */
  export declare function DeleteBasicBlock(BB: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3123:6 */
  export declare function RemoveBasicBlockFromParent(BB: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3130:6 */
  export declare function MoveBasicBlockBefore(BB: LLVM.BasicBlockRef, MovePos: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3137:6 */
  export declare function MoveBasicBlockAfter(BB: LLVM.BasicBlockRef, MovePos: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3145:14 */
  export declare function GetFirstInstruction(BB: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3152:14 */
  export declare function GetLastInstruction(BB: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3178:5 */
  export declare function HasMetadata(Val: LLVM.ValueRef): Opaque<number, "LLVMHasMetadata">;

  /** ./llvm-c/Core.h:3183:14 */
  export declare function GetMetadata(Val: LLVM.ValueRef, KindID: Opaque<number, "KindID">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3188:6 */
  export declare function SetMetadata(Val: LLVM.ValueRef, KindID: Opaque<number, "KindID">, Node: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3197:1 */
  export declare function InstructionGetAllMetadataOtherThanDebugLoc(Instr: LLVM.ValueRef, NumEntries: Pointer<"NumEntries">): Pointer<"LLVMInstructionGetAllMetadataOtherThanDebugLoc">;

  /** ./llvm-c/Core.h:3205:19 */
  export declare function GetInstructionParent(Inst: LLVM.ValueRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3215:14 */
  export declare function GetNextInstruction(Inst: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3223:14 */
  export declare function GetPreviousInstruction(Inst: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3233:6 */
  export declare function InstructionRemoveFromParent(Inst: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3243:6 */
  export declare function InstructionEraseFromParent(Inst: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3253:6 */
  export declare function DeleteInstruction(Inst: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3260:12 */
  export declare function GetInstructionOpcode(Inst: LLVM.ValueRef): LLVM.Opcode;

  /** ./llvm-c/Core.h:3270:18 */
  export declare function GetICmpPredicate(Inst: LLVM.ValueRef): LLVM.IntPredicate;

  /** ./llvm-c/Core.h:3280:19 */
  export declare function GetFCmpPredicate(Inst: LLVM.ValueRef): LLVM.RealPredicate;

  /** ./llvm-c/Core.h:3290:14 */
  export declare function InstructionClone(Inst: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3299:14 */
  export declare function IsATerminatorInst(Inst: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3321:10 */
  export declare function GetNumArgOperands(Instr: LLVM.ValueRef): Opaque<number, "LLVMGetNumArgOperands">;

  /** ./llvm-c/Core.h:3332:6 */
  export declare function SetInstructionCallConv(Instr: LLVM.ValueRef, CC: Opaque<number, "CC">): void;

  /** ./llvm-c/Core.h:3342:10 */
  export declare function GetInstructionCallConv(Instr: LLVM.ValueRef): Opaque<number, "LLVMGetInstructionCallConv">;

  /** ./llvm-c/Core.h:3344:6 */
  export declare function SetInstrParamAlignment(Instr: LLVM.ValueRef, Idx: LLVM.AttributeIndex, Align: Opaque<number, "Align">): void;

  /** ./llvm-c/Core.h:3347:6 */
  export declare function AddCallSiteAttribute(C: LLVM.ValueRef, Idx: LLVM.AttributeIndex, A: LLVM.AttributeRef): void;

  /** ./llvm-c/Core.h:3349:10 */
  export declare function GetCallSiteAttributeCount(C: LLVM.ValueRef, Idx: LLVM.AttributeIndex): Opaque<number, "LLVMGetCallSiteAttributeCount">;

  /** ./llvm-c/Core.h:3350:6 */
  export declare function GetCallSiteAttributes(C: LLVM.ValueRef, Idx: LLVM.AttributeIndex, Attrs: Pointer<"Attrs">): void;

  /** ./llvm-c/Core.h:3352:18 */
  export declare function GetCallSiteEnumAttribute(C: LLVM.ValueRef, Idx: LLVM.AttributeIndex, KindID: Opaque<number, "KindID">): LLVM.AttributeRef;

  /** ./llvm-c/Core.h:3355:18 */
  export declare function GetCallSiteStringAttribute(C: LLVM.ValueRef, Idx: LLVM.AttributeIndex, K: Pointer<"K">, KLen: Opaque<number, "KLen">): LLVM.AttributeRef;

  /** ./llvm-c/Core.h:3358:6 */
  export declare function RemoveCallSiteEnumAttribute(C: LLVM.ValueRef, Idx: LLVM.AttributeIndex, KindID: Opaque<number, "KindID">): void;

  /** ./llvm-c/Core.h:3360:6 */
  export declare function RemoveCallSiteStringAttribute(C: LLVM.ValueRef, Idx: LLVM.AttributeIndex, K: Pointer<"K">, KLen: Opaque<number, "KLen">): void;

  /** ./llvm-c/Core.h:3368:13 */
  export declare function GetCalledFunctionType(C: LLVM.ValueRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:3379:14 */
  export declare function GetCalledValue(Instr: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3388:10 */
  export declare function IsTailCall(CallInst: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:3397:6 */
  export declare function SetTailCall(CallInst: LLVM.ValueRef, IsTailCall: LLVM.Bool): void;

  /** ./llvm-c/Core.h:3406:19 */
  export declare function GetNormalDest(InvokeInst: LLVM.ValueRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3418:19 */
  export declare function GetUnwindDest(InvokeInst: LLVM.ValueRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3427:6 */
  export declare function SetNormalDest(InvokeInst: LLVM.ValueRef, B: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3439:6 */
  export declare function SetUnwindDest(InvokeInst: LLVM.ValueRef, B: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3459:10 */
  export declare function GetNumSuccessors(Term: LLVM.ValueRef): Opaque<number, "LLVMGetNumSuccessors">;

  /** ./llvm-c/Core.h:3466:19 */
  export declare function GetSuccessor(Term: LLVM.ValueRef, i: Opaque<number, "i">): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3473:6 */
  export declare function SetSuccessor(Term: LLVM.ValueRef, i: Opaque<number, "i">, block: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3482:10 */
  export declare function IsConditional(Branch: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:3491:14 */
  export declare function GetCondition(Branch: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3500:6 */
  export declare function SetCondition(Branch: LLVM.ValueRef, Cond: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3509:19 */
  export declare function GetSwitchDefaultDest(SwitchInstr: LLVM.ValueRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3527:13 */
  export declare function GetAllocatedType(Alloca: LLVM.ValueRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:3545:10 */
  export declare function IsInBounds(GEP: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:3550:6 */
  export declare function SetIsInBounds(GEP: LLVM.ValueRef, InBounds: LLVM.Bool): void;

  /** ./llvm-c/Core.h:3555:13 */
  export declare function GetGEPSourceElementType(GEP: LLVM.ValueRef): LLVM.TypeRef;

  /** ./llvm-c/Core.h:3573:6 */
  export declare function AddIncoming(PhiNode: LLVM.ValueRef, IncomingValues: Pointer<"IncomingValues">, IncomingBlocks: Pointer<"IncomingBlocks">, Count: Opaque<number, "Count">): void;

  /** ./llvm-c/Core.h:3579:10 */
  export declare function CountIncoming(PhiNode: LLVM.ValueRef): Opaque<number, "LLVMCountIncoming">;

  /** ./llvm-c/Core.h:3584:14 */
  export declare function GetIncomingValue(PhiNode: LLVM.ValueRef, Index: Opaque<number, "Index">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3589:19 */
  export declare function GetIncomingBlock(PhiNode: LLVM.ValueRef, Index: Opaque<number, "Index">): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3609:10 */
  export declare function GetNumIndices(Inst: LLVM.ValueRef): Opaque<number, "LLVMGetNumIndices">;

  /** ./llvm-c/Core.h:3614:17 */
  export declare function GetIndices(Inst: LLVM.ValueRef): Pointer<"LLVMGetIndices">;

  /** ./llvm-c/Core.h:3637:16 */
  export declare function CreateBuilderInContext(C: LLVM.ContextRef): LLVM.BuilderRef;

  /** ./llvm-c/Core.h:3638:16 */
  export declare function CreateBuilder(): LLVM.BuilderRef;

  /** ./llvm-c/Core.h:3639:6 */
  export declare function PositionBuilder(Builder: LLVM.BuilderRef, Block: LLVM.BasicBlockRef, Instr: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3641:6 */
  export declare function PositionBuilderBefore(Builder: LLVM.BuilderRef, Instr: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3642:6 */
  export declare function PositionBuilderAtEnd(Builder: LLVM.BuilderRef, Block: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3643:19 */
  export declare function GetInsertBlock(Builder: LLVM.BuilderRef): LLVM.BasicBlockRef;

  /** ./llvm-c/Core.h:3644:6 */
  export declare function ClearInsertionPosition(Builder: LLVM.BuilderRef): void;

  /** ./llvm-c/Core.h:3645:6 */
  export declare function InsertIntoBuilder(Builder: LLVM.BuilderRef, Instr: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3646:6 */
  export declare function InsertIntoBuilderWithName(Builder: LLVM.BuilderRef, Instr: LLVM.ValueRef, Name: Pointer<"Name">): void;

  /** ./llvm-c/Core.h:3648:6 */
  export declare function DisposeBuilder(Builder: LLVM.BuilderRef): void;

  /** ./llvm-c/Core.h:3657:17 */
  export declare function GetCurrentDebugLocation2(Builder: LLVM.BuilderRef): LLVM.MetadataRef;

  /** ./llvm-c/Core.h:3666:6 */
  export declare function SetCurrentDebugLocation2(Builder: LLVM.BuilderRef, Loc: LLVM.MetadataRef): void;

  /** ./llvm-c/Core.h:3678:6 */
  export declare function SetInstDebugLocation(Builder: LLVM.BuilderRef, Inst: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3685:6 */
  export declare function AddMetadataToInst(Builder: LLVM.BuilderRef, Inst: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3692:17 */
  export declare function BuilderGetDefaultFPMathTag(Builder: LLVM.BuilderRef): LLVM.MetadataRef;

  /** ./llvm-c/Core.h:3701:6 */
  export declare function BuilderSetDefaultFPMathTag(Builder: LLVM.BuilderRef, FPMathTag: LLVM.MetadataRef): void;

  /** ./llvm-c/Core.h:3708:6 */
  export declare function SetCurrentDebugLocation(Builder: LLVM.BuilderRef, L: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3713:14 */
  export declare function GetCurrentDebugLocation(Builder: LLVM.BuilderRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3716:14 */
  export declare function BuildRetVoid(_0: LLVM.BuilderRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3717:14 */
  export declare function BuildRet(_0: LLVM.BuilderRef, V: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3718:14 */
  export declare function BuildAggregateRet(_0: LLVM.BuilderRef, RetVals: Pointer<"RetVals">, N: Opaque<number, "N">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3720:14 */
  export declare function BuildBr(_0: LLVM.BuilderRef, Dest: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3721:14 */
  export declare function BuildCondBr(_0: LLVM.BuilderRef, If: LLVM.ValueRef, Then: LLVM.BasicBlockRef, Else: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3723:14 */
  export declare function BuildSwitch(_0: LLVM.BuilderRef, V: LLVM.ValueRef, Else: LLVM.BasicBlockRef, NumCases: Opaque<number, "NumCases">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3725:14 */
  export declare function BuildIndirectBr(B: LLVM.BuilderRef, Addr: LLVM.ValueRef, NumDests: Opaque<number, "NumDests">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3727:1 */
  export declare function BuildInvoke(_0: LLVM.BuilderRef, Fn: LLVM.ValueRef, Args: Pointer<"Args">, NumArgs: Opaque<number, "NumArgs">, Then: LLVM.BasicBlockRef, Catch: LLVM.BasicBlockRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3733:14 */
  export declare function BuildInvoke2(_0: LLVM.BuilderRef, Ty: LLVM.TypeRef, Fn: LLVM.ValueRef, Args: Pointer<"Args">, NumArgs: Opaque<number, "NumArgs">, Then: LLVM.BasicBlockRef, Catch: LLVM.BasicBlockRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3737:14 */
  export declare function BuildUnreachable(_0: LLVM.BuilderRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3740:14 */
  export declare function BuildResume(B: LLVM.BuilderRef, Exn: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3741:14 */
  export declare function BuildLandingPad(B: LLVM.BuilderRef, Ty: LLVM.TypeRef, PersFn: LLVM.ValueRef, NumClauses: Opaque<number, "NumClauses">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3744:14 */
  export declare function BuildCleanupRet(B: LLVM.BuilderRef, CatchPad: LLVM.ValueRef, BB: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3746:14 */
  export declare function BuildCatchRet(B: LLVM.BuilderRef, CatchPad: LLVM.ValueRef, BB: LLVM.BasicBlockRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3748:14 */
  export declare function BuildCatchPad(B: LLVM.BuilderRef, ParentPad: LLVM.ValueRef, Args: Pointer<"Args">, NumArgs: Opaque<number, "NumArgs">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3751:14 */
  export declare function BuildCleanupPad(B: LLVM.BuilderRef, ParentPad: LLVM.ValueRef, Args: Pointer<"Args">, NumArgs: Opaque<number, "NumArgs">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3754:14 */
  export declare function BuildCatchSwitch(B: LLVM.BuilderRef, ParentPad: LLVM.ValueRef, UnwindBB: LLVM.BasicBlockRef, NumHandlers: Opaque<number, "NumHandlers">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3759:6 */
  export declare function AddCase(Switch: LLVM.ValueRef, OnVal: LLVM.ValueRef, Dest: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3763:6 */
  export declare function AddDestination(IndirectBr: LLVM.ValueRef, Dest: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3766:10 */
  export declare function GetNumClauses(LandingPad: LLVM.ValueRef): Opaque<number, "LLVMGetNumClauses">;

  /** ./llvm-c/Core.h:3769:14 */
  export declare function GetClause(LandingPad: LLVM.ValueRef, Idx: Opaque<number, "Idx">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3772:6 */
  export declare function AddClause(LandingPad: LLVM.ValueRef, ClauseVal: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3775:10 */
  export declare function IsCleanup(LandingPad: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:3778:6 */
  export declare function SetCleanup(LandingPad: LLVM.ValueRef, Val: LLVM.Bool): void;

  /** ./llvm-c/Core.h:3781:6 */
  export declare function AddHandler(CatchSwitch: LLVM.ValueRef, Dest: LLVM.BasicBlockRef): void;

  /** ./llvm-c/Core.h:3784:10 */
  export declare function GetNumHandlers(CatchSwitch: LLVM.ValueRef): Opaque<number, "LLVMGetNumHandlers">;

  /** ./llvm-c/Core.h:3797:6 */
  export declare function GetHandlers(CatchSwitch: LLVM.ValueRef, Handlers: Pointer<"Handlers">): void;

  /** ./llvm-c/Core.h:3802:14 */
  export declare function GetArgOperand(Funclet: LLVM.ValueRef, i: Opaque<number, "i">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3805:6 */
  export declare function SetArgOperand(Funclet: LLVM.ValueRef, i: Opaque<number, "i">, value: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3814:14 */
  export declare function GetParentCatchSwitch(CatchPad: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3823:6 */
  export declare function SetParentCatchSwitch(CatchPad: LLVM.ValueRef, CatchSwitch: LLVM.ValueRef): void;

  /** ./llvm-c/Core.h:3826:14 */
  export declare function BuildAdd(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3828:14 */
  export declare function BuildNSWAdd(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3830:14 */
  export declare function BuildNUWAdd(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3832:14 */
  export declare function BuildFAdd(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3834:14 */
  export declare function BuildSub(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3836:14 */
  export declare function BuildNSWSub(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3838:14 */
  export declare function BuildNUWSub(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3840:14 */
  export declare function BuildFSub(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3842:14 */
  export declare function BuildMul(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3844:14 */
  export declare function BuildNSWMul(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3846:14 */
  export declare function BuildNUWMul(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3848:14 */
  export declare function BuildFMul(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3850:14 */
  export declare function BuildUDiv(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3852:14 */
  export declare function BuildExactUDiv(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3854:14 */
  export declare function BuildSDiv(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3856:14 */
  export declare function BuildExactSDiv(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3858:14 */
  export declare function BuildFDiv(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3860:14 */
  export declare function BuildURem(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3862:14 */
  export declare function BuildSRem(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3864:14 */
  export declare function BuildFRem(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3866:14 */
  export declare function BuildShl(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3868:14 */
  export declare function BuildLShr(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3870:14 */
  export declare function BuildAShr(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3872:14 */
  export declare function BuildAnd(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3874:14 */
  export declare function BuildOr(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3876:14 */
  export declare function BuildXor(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3878:14 */
  export declare function BuildBinOp(B: LLVM.BuilderRef, Op: LLVM.Opcode, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3881:14 */
  export declare function BuildNeg(_0: LLVM.BuilderRef, V: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3882:14 */
  export declare function BuildNSWNeg(B: LLVM.BuilderRef, V: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3884:14 */
  export declare function BuildNUWNeg(B: LLVM.BuilderRef, V: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3886:14 */
  export declare function BuildFNeg(_0: LLVM.BuilderRef, V: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3887:14 */
  export declare function BuildNot(_0: LLVM.BuilderRef, V: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3890:14 */
  export declare function BuildMalloc(_0: LLVM.BuilderRef, Ty: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3891:14 */
  export declare function BuildArrayMalloc(_0: LLVM.BuilderRef, Ty: LLVM.TypeRef, Val: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3900:14 */
  export declare function BuildMemSet(B: LLVM.BuilderRef, Ptr: LLVM.ValueRef, Val: LLVM.ValueRef, Len: LLVM.ValueRef, Align: Opaque<number, "Align">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3908:14 */
  export declare function BuildMemCpy(B: LLVM.BuilderRef, Dst: LLVM.ValueRef, DstAlign: Opaque<number, "DstAlign">, Src: LLVM.ValueRef, SrcAlign: Opaque<number, "SrcAlign">, Size: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3917:14 */
  export declare function BuildMemMove(B: LLVM.BuilderRef, Dst: LLVM.ValueRef, DstAlign: Opaque<number, "DstAlign">, Src: LLVM.ValueRef, SrcAlign: Opaque<number, "SrcAlign">, Size: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3922:14 */
  export declare function BuildAlloca(_0: LLVM.BuilderRef, Ty: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3923:14 */
  export declare function BuildArrayAlloca(_0: LLVM.BuilderRef, Ty: LLVM.TypeRef, Val: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3925:14 */
  export declare function BuildFree(_0: LLVM.BuilderRef, PointerVal: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3926:1 */
  export declare function BuildLoad(_0: LLVM.BuilderRef, PointerVal: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3930:14 */
  export declare function BuildLoad2(_0: LLVM.BuilderRef, Ty: LLVM.TypeRef, PointerVal: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3932:14 */
  export declare function BuildStore(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, Ptr: LLVM.ValueRef): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3933:1 */
  export declare function BuildGEP(B: LLVM.BuilderRef, Pointer: LLVM.ValueRef, Indices: Pointer<"Indices">, NumIndices: Opaque<number, "NumIndices">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3938:1 */
  export declare function BuildInBoundsGEP(B: LLVM.BuilderRef, Pointer: LLVM.ValueRef, Indices: Pointer<"Indices">, NumIndices: Opaque<number, "NumIndices">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3943:1 */
  export declare function BuildStructGEP(B: LLVM.BuilderRef, Pointer: LLVM.ValueRef, Idx: Opaque<number, "Idx">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3947:14 */
  export declare function BuildGEP2(B: LLVM.BuilderRef, Ty: LLVM.TypeRef, Pointer: LLVM.ValueRef, Indices: Pointer<"Indices">, NumIndices: Opaque<number, "NumIndices">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3950:14 */
  export declare function BuildInBoundsGEP2(B: LLVM.BuilderRef, Ty: LLVM.TypeRef, Pointer: LLVM.ValueRef, Indices: Pointer<"Indices">, NumIndices: Opaque<number, "NumIndices">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3953:14 */
  export declare function BuildStructGEP2(B: LLVM.BuilderRef, Ty: LLVM.TypeRef, Pointer: LLVM.ValueRef, Idx: Opaque<number, "Idx">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3956:14 */
  export declare function BuildGlobalString(B: LLVM.BuilderRef, Str: Pointer<"Str">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3958:14 */
  export declare function BuildGlobalStringPtr(B: LLVM.BuilderRef, Str: Pointer<"Str">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3960:10 */
  export declare function GetVolatile(MemoryAccessInst: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:3961:6 */
  export declare function SetVolatile(MemoryAccessInst: LLVM.ValueRef, IsVolatile: LLVM.Bool): void;

  /** ./llvm-c/Core.h:3962:10 */
  export declare function GetWeak(CmpXchgInst: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:3963:6 */
  export declare function SetWeak(CmpXchgInst: LLVM.ValueRef, IsWeak: LLVM.Bool): void;

  /** ./llvm-c/Core.h:3964:20 */
  export declare function GetOrdering(MemoryAccessInst: LLVM.ValueRef): LLVM.AtomicOrdering;

  /** ./llvm-c/Core.h:3965:6 */
  export declare function SetOrdering(MemoryAccessInst: LLVM.ValueRef, Ordering: LLVM.AtomicOrdering): void;

  /** ./llvm-c/Core.h:3966:20 */
  export declare function GetAtomicRMWBinOp(AtomicRMWInst: LLVM.ValueRef): LLVM.AtomicRMWBinOp;

  /** ./llvm-c/Core.h:3967:6 */
  export declare function SetAtomicRMWBinOp(AtomicRMWInst: LLVM.ValueRef, BinOp: LLVM.AtomicRMWBinOp): void;

  /** ./llvm-c/Core.h:3970:14 */
  export declare function BuildTrunc(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3972:14 */
  export declare function BuildZExt(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3974:14 */
  export declare function BuildSExt(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3976:14 */
  export declare function BuildFPToUI(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3978:14 */
  export declare function BuildFPToSI(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3980:14 */
  export declare function BuildUIToFP(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3982:14 */
  export declare function BuildSIToFP(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3984:14 */
  export declare function BuildFPTrunc(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3986:14 */
  export declare function BuildFPExt(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3988:14 */
  export declare function BuildPtrToInt(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3990:14 */
  export declare function BuildIntToPtr(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3992:14 */
  export declare function BuildBitCast(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3994:14 */
  export declare function BuildAddrSpaceCast(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3996:14 */
  export declare function BuildZExtOrBitCast(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:3998:14 */
  export declare function BuildSExtOrBitCast(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4000:14 */
  export declare function BuildTruncOrBitCast(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4002:14 */
  export declare function BuildCast(B: LLVM.BuilderRef, Op: LLVM.Opcode, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4004:14 */
  export declare function BuildPointerCast(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4006:14 */
  export declare function BuildIntCast2(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, IsSigned: LLVM.Bool, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4009:14 */
  export declare function BuildFPCast(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4013:14 */
  export declare function BuildIntCast(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, DestTy: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4016:12 */
  export declare function GetCastOpcode(Src: LLVM.ValueRef, SrcIsSigned: LLVM.Bool, DestTy: LLVM.TypeRef, DestIsSigned: LLVM.Bool): LLVM.Opcode;

  /** ./llvm-c/Core.h:4020:14 */
  export declare function BuildICmp(_0: LLVM.BuilderRef, Op: LLVM.IntPredicate, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4023:14 */
  export declare function BuildFCmp(_0: LLVM.BuilderRef, Op: LLVM.RealPredicate, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4028:14 */
  export declare function BuildPhi(_0: LLVM.BuilderRef, Ty: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4029:1 */
  export declare function BuildCall(_0: LLVM.BuilderRef, Fn: LLVM.ValueRef, Args: Pointer<"Args">, NumArgs: Opaque<number, "NumArgs">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4034:14 */
  export declare function BuildCall2(_0: LLVM.BuilderRef, _1: LLVM.TypeRef, Fn: LLVM.ValueRef, Args: Pointer<"Args">, NumArgs: Opaque<number, "NumArgs">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4037:14 */
  export declare function BuildSelect(_0: LLVM.BuilderRef, If: LLVM.ValueRef, Then: LLVM.ValueRef, Else: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4040:14 */
  export declare function BuildVAArg(_0: LLVM.BuilderRef, List: LLVM.ValueRef, Ty: LLVM.TypeRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4042:14 */
  export declare function BuildExtractElement(_0: LLVM.BuilderRef, VecVal: LLVM.ValueRef, Index: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4044:14 */
  export declare function BuildInsertElement(_0: LLVM.BuilderRef, VecVal: LLVM.ValueRef, EltVal: LLVM.ValueRef, Index: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4047:14 */
  export declare function BuildShuffleVector(_0: LLVM.BuilderRef, V1: LLVM.ValueRef, V2: LLVM.ValueRef, Mask: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4050:14 */
  export declare function BuildExtractValue(_0: LLVM.BuilderRef, AggVal: LLVM.ValueRef, Index: Opaque<number, "Index">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4052:14 */
  export declare function BuildInsertValue(_0: LLVM.BuilderRef, AggVal: LLVM.ValueRef, EltVal: LLVM.ValueRef, Index: Opaque<number, "Index">, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4055:14 */
  export declare function BuildFreeze(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4058:14 */
  export declare function BuildIsNull(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4060:14 */
  export declare function BuildIsNotNull(_0: LLVM.BuilderRef, Val: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4062:1 */
  export declare function BuildPtrDiff(_0: LLVM.BuilderRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4066:14 */
  export declare function BuildPtrDiff2(_0: LLVM.BuilderRef, ElemTy: LLVM.TypeRef, LHS: LLVM.ValueRef, RHS: LLVM.ValueRef, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4069:14 */
  export declare function BuildFence(B: LLVM.BuilderRef, ordering: LLVM.AtomicOrdering, singleThread: LLVM.Bool, Name: Pointer<"Name">): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4071:14 */
  export declare function BuildAtomicRMW(B: LLVM.BuilderRef, op: LLVM.AtomicRMWBinOp, PTR: LLVM.ValueRef, Val: LLVM.ValueRef, ordering: LLVM.AtomicOrdering, singleThread: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4075:14 */
  export declare function BuildAtomicCmpXchg(B: LLVM.BuilderRef, Ptr: LLVM.ValueRef, Cmp: LLVM.ValueRef, New: LLVM.ValueRef, SuccessOrdering: LLVM.AtomicOrdering, FailureOrdering: LLVM.AtomicOrdering, SingleThread: LLVM.Bool): LLVM.ValueRef;

  /** ./llvm-c/Core.h:4084:10 */
  export declare function GetNumMaskElements(ShuffleVectorInst: LLVM.ValueRef): Opaque<number, "LLVMGetNumMaskElements">;

  /** ./llvm-c/Core.h:4090:5 */
  export declare function GetUndefMaskElem(): Opaque<number, "LLVMGetUndefMaskElem">;

  /** ./llvm-c/Core.h:4099:5 */
  export declare function GetMaskValue(ShuffleVectorInst: LLVM.ValueRef, Elt: Opaque<number, "Elt">): Opaque<number, "LLVMGetMaskValue">;

  /** ./llvm-c/Core.h:4101:10 */
  export declare function IsAtomicSingleThread(AtomicInst: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:4102:6 */
  export declare function SetAtomicSingleThread(AtomicInst: LLVM.ValueRef, SingleThread: LLVM.Bool): void;

  /** ./llvm-c/Core.h:4104:20 */
  export declare function GetCmpXchgSuccessOrdering(CmpXchgInst: LLVM.ValueRef): LLVM.AtomicOrdering;

  /** ./llvm-c/Core.h:4105:6 */
  export declare function SetCmpXchgSuccessOrdering(CmpXchgInst: LLVM.ValueRef, Ordering: LLVM.AtomicOrdering): void;

  /** ./llvm-c/Core.h:4107:20 */
  export declare function GetCmpXchgFailureOrdering(CmpXchgInst: LLVM.ValueRef): LLVM.AtomicOrdering;

  /** ./llvm-c/Core.h:4108:6 */
  export declare function SetCmpXchgFailureOrdering(CmpXchgInst: LLVM.ValueRef, Ordering: LLVM.AtomicOrdering): void;

  /** ./llvm-c/Core.h:4126:1 */
  export declare function CreateModuleProviderForExistingModule(M: LLVM.ModuleRef): LLVM.ModuleProviderRef;

  /** ./llvm-c/Core.h:4131:6 */
  export declare function DisposeModuleProvider(M: LLVM.ModuleProviderRef): void;

  /** ./llvm-c/Core.h:4143:10 */
  export declare function CreateMemoryBufferWithContentsOfFile(Path: Pointer<"Path">, OutMemBuf: Pointer<"OutMemBuf">, OutMessage: Pointer<"OutMessage">): LLVM.Bool;

  /** ./llvm-c/Core.h:4146:10 */
  export declare function CreateMemoryBufferWithSTDIN(OutMemBuf: Pointer<"OutMemBuf">, OutMessage: Pointer<"OutMessage">): LLVM.Bool;

  /** ./llvm-c/Core.h:4148:21 */
  export declare function CreateMemoryBufferWithMemoryRange(InputData: Pointer<"InputData">, InputDataLength: Opaque<number, "InputDataLength">, BufferName: Pointer<"BufferName">, RequiresNullTerminator: LLVM.Bool): LLVM.MemoryBufferRef;

  /** ./llvm-c/Core.h:4152:21 */
  export declare function CreateMemoryBufferWithMemoryRangeCopy(InputData: Pointer<"InputData">, InputDataLength: Opaque<number, "InputDataLength">, BufferName: Pointer<"BufferName">): LLVM.MemoryBufferRef;

  /** ./llvm-c/Core.h:4155:13 */
  export declare function GetBufferStart(MemBuf: LLVM.MemoryBufferRef): Pointer<"LLVMGetBufferStart">;

  /** ./llvm-c/Core.h:4156:8 */
  export declare function GetBufferSize(MemBuf: LLVM.MemoryBufferRef): Opaque<number, "LLVMGetBufferSize">;

  /** ./llvm-c/Core.h:4157:6 */
  export declare function DisposeMemoryBuffer(MemBuf: LLVM.MemoryBufferRef): void;

  /** ./llvm-c/Core.h:4172:21 */
  export declare function GetGlobalPassRegistry(): LLVM.PassRegistryRef;

  /** ./llvm-c/Core.h:4188:20 */
  export declare function CreatePassManager(): LLVM.PassManagerRef;

  /** ./llvm-c/Core.h:4194:20 */
  export declare function CreateFunctionPassManagerForModule(M: LLVM.ModuleRef): LLVM.PassManagerRef;

  /** ./llvm-c/Core.h:4197:20 */
  export declare function CreateFunctionPassManager(MP: LLVM.ModuleProviderRef): LLVM.PassManagerRef;

  /** ./llvm-c/Core.h:4203:10 */
  export declare function RunPassManager(PM: LLVM.PassManagerRef, M: LLVM.ModuleRef): LLVM.Bool;

  /** ./llvm-c/Core.h:4208:10 */
  export declare function InitializeFunctionPassManager(FPM: LLVM.PassManagerRef): LLVM.Bool;

  /** ./llvm-c/Core.h:4214:10 */
  export declare function RunFunctionPassManager(FPM: LLVM.PassManagerRef, F: LLVM.ValueRef): LLVM.Bool;

  /** ./llvm-c/Core.h:4219:10 */
  export declare function FinalizeFunctionPassManager(FPM: LLVM.PassManagerRef): LLVM.Bool;

  /** ./llvm-c/Core.h:4224:6 */
  export declare function DisposePassManager(PM: LLVM.PassManagerRef): void;

  /** ./llvm-c/Core.h:4241:10 */
  export declare function StartMultithreaded(): LLVM.Bool;

  /** ./llvm-c/Core.h:4245:6 */
  export declare function StopMultithreaded(): void;

  /** ./llvm-c/Core.h:4249:10 */
  export declare function IsMultithreaded(): LLVM.Bool;

  /** ./llvm-c/Object.h:76:15 */
  export declare function CreateBinary(MemBuf: LLVM.MemoryBufferRef, Context: LLVM.ContextRef, ErrorMessage: Pointer<"ErrorMessage">): LLVM.BinaryRef;

  /** ./llvm-c/Object.h:86:6 */
  export declare function DisposeBinary(BR: LLVM.BinaryRef): void;

  /** ./llvm-c/Object.h:97:21 */
  export declare function BinaryCopyMemoryBuffer(BR: LLVM.BinaryRef): LLVM.MemoryBufferRef;

  /** ./llvm-c/Object.h:104:16 */
  export declare function BinaryGetType(BR: LLVM.BinaryRef): LLVM.BinaryType;

  /** ./llvm-c/Object.h:117:15 */
  export declare function MachOUniversalBinaryCopyObjectForArch(BR: LLVM.BinaryRef, Arch: Pointer<"Arch">, ArchLen: Opaque<number, "ArchLen">, ErrorMessage: Pointer<"ErrorMessage">): LLVM.BinaryRef;

  /** ./llvm-c/Object.h:133:24 */
  export declare function ObjectFileCopySectionIterator(BR: LLVM.BinaryRef): LLVM.SectionIteratorRef;

  /** ./llvm-c/Object.h:140:10 */
  export declare function ObjectFileIsSectionIteratorAtEnd(BR: LLVM.BinaryRef, SI: LLVM.SectionIteratorRef): LLVM.Bool;

  /** ./llvm-c/Object.h:154:23 */
  export declare function ObjectFileCopySymbolIterator(BR: LLVM.BinaryRef): LLVM.SymbolIteratorRef;

  /** ./llvm-c/Object.h:161:10 */
  export declare function ObjectFileIsSymbolIteratorAtEnd(BR: LLVM.BinaryRef, SI: LLVM.SymbolIteratorRef): LLVM.Bool;

  /** ./llvm-c/Object.h:164:6 */
  export declare function DisposeSectionIterator(SI: LLVM.SectionIteratorRef): void;

  /** ./llvm-c/Object.h:166:6 */
  export declare function MoveToNextSection(SI: LLVM.SectionIteratorRef): void;

  /** ./llvm-c/Object.h:167:6 */
  export declare function MoveToContainingSection(Sect: LLVM.SectionIteratorRef, Sym: LLVM.SymbolIteratorRef): void;

  /** ./llvm-c/Object.h:171:6 */
  export declare function DisposeSymbolIterator(SI: LLVM.SymbolIteratorRef): void;

  /** ./llvm-c/Object.h:172:6 */
  export declare function MoveToNextSymbol(SI: LLVM.SymbolIteratorRef): void;

  /** ./llvm-c/Object.h:175:13 */
  export declare function GetSectionName(SI: LLVM.SectionIteratorRef): Pointer<"LLVMGetSectionName">;

  /** ./llvm-c/Object.h:176:10 */
  export declare function GetSectionSize(SI: LLVM.SectionIteratorRef): Opaque<bigint, "LLVMGetSectionSize">;

  /** ./llvm-c/Object.h:177:13 */
  export declare function GetSectionContents(SI: LLVM.SectionIteratorRef): Pointer<"LLVMGetSectionContents">;

  /** ./llvm-c/Object.h:178:10 */
  export declare function GetSectionAddress(SI: LLVM.SectionIteratorRef): Opaque<bigint, "LLVMGetSectionAddress">;

  /** ./llvm-c/Object.h:179:10 */
  export declare function GetSectionContainsSymbol(SI: LLVM.SectionIteratorRef, Sym: LLVM.SymbolIteratorRef): LLVM.Bool;

  /** ./llvm-c/Object.h:183:27 */
  export declare function GetRelocations(Section: LLVM.SectionIteratorRef): LLVM.RelocationIteratorRef;

  /** ./llvm-c/Object.h:184:6 */
  export declare function DisposeRelocationIterator(RI: LLVM.RelocationIteratorRef): void;

  /** ./llvm-c/Object.h:185:10 */
  export declare function IsRelocationIteratorAtEnd(Section: LLVM.SectionIteratorRef, RI: LLVM.RelocationIteratorRef): LLVM.Bool;

  /** ./llvm-c/Object.h:187:6 */
  export declare function MoveToNextRelocation(RI: LLVM.RelocationIteratorRef): void;

  /** ./llvm-c/Object.h:191:13 */
  export declare function GetSymbolName(SI: LLVM.SymbolIteratorRef): Pointer<"LLVMGetSymbolName">;

  /** ./llvm-c/Object.h:192:10 */
  export declare function GetSymbolAddress(SI: LLVM.SymbolIteratorRef): Opaque<bigint, "LLVMGetSymbolAddress">;

  /** ./llvm-c/Object.h:193:10 */
  export declare function GetSymbolSize(SI: LLVM.SymbolIteratorRef): Opaque<bigint, "LLVMGetSymbolSize">;

  /** ./llvm-c/Object.h:196:10 */
  export declare function GetRelocationOffset(RI: LLVM.RelocationIteratorRef): Opaque<bigint, "LLVMGetRelocationOffset">;

  /** ./llvm-c/Object.h:197:23 */
  export declare function GetRelocationSymbol(RI: LLVM.RelocationIteratorRef): LLVM.SymbolIteratorRef;

  /** ./llvm-c/Object.h:198:10 */
  export declare function GetRelocationType(RI: LLVM.RelocationIteratorRef): Opaque<bigint, "LLVMGetRelocationType">;

  /** ./llvm-c/Object.h:201:13 */
  export declare function GetRelocationTypeName(RI: LLVM.RelocationIteratorRef): Pointer<"LLVMGetRelocationTypeName">;

  /** ./llvm-c/Object.h:202:13 */
  export declare function GetRelocationValueString(RI: LLVM.RelocationIteratorRef): Pointer<"LLVMGetRelocationValueString">;

  /** ./llvm-c/Object.h:208:19 */
  export declare function CreateObjectFile(MemBuf: LLVM.MemoryBufferRef): LLVM.ObjectFileRef;

  /** ./llvm-c/Object.h:211:6 */
  export declare function DisposeObjectFile(ObjectFile: LLVM.ObjectFileRef): void;

  /** ./llvm-c/Object.h:214:24 */
  export declare function GetSections(ObjectFile: LLVM.ObjectFileRef): LLVM.SectionIteratorRef;

  /** ./llvm-c/Object.h:217:10 */
  export declare function IsSectionIteratorAtEnd(ObjectFile: LLVM.ObjectFileRef, SI: LLVM.SectionIteratorRef): LLVM.Bool;

  /** ./llvm-c/Object.h:221:23 */
  export declare function GetSymbols(ObjectFile: LLVM.ObjectFileRef): LLVM.SymbolIteratorRef;

  /** ./llvm-c/Object.h:224:10 */
  export declare function IsSymbolIteratorAtEnd(ObjectFile: LLVM.ObjectFileRef, SI: LLVM.SymbolIteratorRef): LLVM.Bool;

  /** ./llvm-c/BitWriter.h:37:5 */
  export declare function WriteBitcodeToFile(M: LLVM.ModuleRef, Path: Pointer<"Path">): Opaque<number, "LLVMWriteBitcodeToFile">;

  /** ./llvm-c/BitWriter.h:40:5 */
  export declare function WriteBitcodeToFD(M: LLVM.ModuleRef, FD: Opaque<number, "FD">, ShouldClose: Opaque<number, "ShouldClose">, Unbuffered: Opaque<number, "Unbuffered">): Opaque<number, "LLVMWriteBitcodeToFD">;

  /** ./llvm-c/BitWriter.h:45:5 */
  export declare function WriteBitcodeToFileHandle(M: LLVM.ModuleRef, Handle: Opaque<number, "Handle">): Opaque<number, "LLVMWriteBitcodeToFileHandle">;

  /** ./llvm-c/BitWriter.h:48:21 */
  export declare function WriteBitcodeToMemoryBuffer(M: LLVM.ModuleRef): LLVM.MemoryBufferRef;

  /** ./llvm-c/LLJIT.h:74:24 */
  export declare function OrcCreateLLJITBuilder(): LLVM.OrcLLJITBuilderRef;

  /** ./llvm-c/LLJIT.h:81:6 */
  export declare function OrcDisposeLLJITBuilder(Builder: LLVM.OrcLLJITBuilderRef): void;

  /** ./llvm-c/LLJIT.h:92:6 */
  export declare function OrcLLJITBuilderSetJITTargetMachineBuilder(Builder: LLVM.OrcLLJITBuilderRef, JTMB: LLVM.OrcJITTargetMachineBuilderRef): void;

  /** ./llvm-c/LLJIT.h:98:6 */
  export declare function OrcLLJITBuilderSetObjectLinkingLayerCreator(Builder: LLVM.OrcLLJITBuilderRef, F: LLVM.OrcLLJITBuilderObjectLinkingLayerCreatorFunction, Ctx: Pointer<"Ctx">): void;

  /** ./llvm-c/LLJIT.h:116:14 */
  export declare function OrcCreateLLJIT(Result: Pointer<"Result">, Builder: LLVM.OrcLLJITBuilderRef): LLVM.ErrorRef;

  /** ./llvm-c/LLJIT.h:122:14 */
  export declare function OrcDisposeLLJIT(J: LLVM.OrcLLJITRef): LLVM.ErrorRef;

  /** ./llvm-c/LLJIT.h:130:28 */
  export declare function OrcLLJITGetExecutionSession(J: LLVM.OrcLLJITRef): LLVM.OrcExecutionSessionRef;

  /** ./llvm-c/LLJIT.h:138:20 */
  export declare function OrcLLJITGetMainJITDylib(J: LLVM.OrcLLJITRef): LLVM.OrcJITDylibRef;

  /** ./llvm-c/LLJIT.h:144:13 */
  export declare function OrcLLJITGetTripleString(J: LLVM.OrcLLJITRef): Pointer<"LLVMOrcLLJITGetTripleString">;

  /** ./llvm-c/LLJIT.h:149:6 */
  export declare function OrcLLJITGetGlobalPrefix(J: LLVM.OrcLLJITRef): Opaque<number, "LLVMOrcLLJITGetGlobalPrefix">;

  /** ./llvm-c/LLJIT.h:159:1 */
  export declare function OrcLLJITMangleAndIntern(J: LLVM.OrcLLJITRef, UnmangledName: Pointer<"UnmangledName">): LLVM.OrcSymbolStringPoolEntryRef;

  /** ./llvm-c/LLJIT.h:170:14 */
  export declare function OrcLLJITAddObjectFile(J: LLVM.OrcLLJITRef, JD: LLVM.OrcJITDylibRef, ObjBuffer: LLVM.MemoryBufferRef): LLVM.ErrorRef;

  /** ./llvm-c/LLJIT.h:182:14 */
  export declare function OrcLLJITAddObjectFileWithRT(J: LLVM.OrcLLJITRef, RT: LLVM.OrcResourceTrackerRef, ObjBuffer: LLVM.MemoryBufferRef): LLVM.ErrorRef;

  /** ./llvm-c/LLJIT.h:195:14 */
  export declare function OrcLLJITAddLLVMIRModule(J: LLVM.OrcLLJITRef, JD: LLVM.OrcJITDylibRef, TSM: LLVM.OrcThreadSafeModuleRef): LLVM.ErrorRef;

  /** ./llvm-c/LLJIT.h:208:14 */
  export declare function OrcLLJITAddLLVMIRModuleWithRT(J: LLVM.OrcLLJITRef, JD: LLVM.OrcResourceTrackerRef, TSM: LLVM.OrcThreadSafeModuleRef): LLVM.ErrorRef;

  /** ./llvm-c/LLJIT.h:217:14 */
  export declare function OrcLLJITLookup(J: LLVM.OrcLLJITRef, Result: Pointer<"Result">, Name: Pointer<"Name">): LLVM.ErrorRef;

  /** ./llvm-c/LLJIT.h:224:23 */
  export declare function OrcLLJITGetObjLinkingLayer(J: LLVM.OrcLLJITRef): LLVM.OrcObjectLayerRef;

  /** ./llvm-c/LLJIT.h:230:1 */
  export declare function OrcLLJITGetObjTransformLayer(J: LLVM.OrcLLJITRef): LLVM.OrcObjectTransformLayerRef;

  /** ./llvm-c/LLJIT.h:235:28 */
  export declare function OrcLLJITGetIRTransformLayer(J: LLVM.OrcLLJITRef): LLVM.OrcIRTransformLayerRef;

  /** ./llvm-c/LLJIT.h:243:13 */
  export declare function OrcLLJITGetDataLayoutStr(J: LLVM.OrcLLJITRef): Pointer<"LLVMOrcLLJITGetDataLayoutStr">;

  /** ./llvm-c/Support.h:35:10 */
  export declare function LoadLibraryPermanently(Filename: Pointer<"Filename">): LLVM.Bool;

  /** ./llvm-c/Support.h:45:6 */
  export declare function ParseCommandLineOptions(argc: Opaque<number, "argc">, argv: Pointer<"argv">, Overview: Pointer<"Overview">): void;

  /** ./llvm-c/Support.h:55:7 */
  export declare function SearchForAddressOfSymbol(symbolName: Pointer<"symbolName">): Pointer<"LLVMSearchForAddressOfSymbol">;

  /** ./llvm-c/Support.h:64:6 */
  export declare function AddSymbol(symbolName: Pointer<"symbolName">, symbolValue: Pointer<"symbolValue">): void;

  /** ./llvm-c/Initialization.h:34:6 */
  export declare function InitializeTransformUtils(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:35:6 */
  export declare function InitializeScalarOpts(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:36:6 */
  export declare function InitializeObjCARCOpts(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:37:6 */
  export declare function InitializeVectorization(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:38:6 */
  export declare function InitializeInstCombine(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:39:6 */
  export declare function InitializeAggressiveInstCombiner(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:40:6 */
  export declare function InitializeIPO(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:41:6 */
  export declare function InitializeInstrumentation(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:42:6 */
  export declare function InitializeAnalysis(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:43:6 */
  export declare function InitializeIPA(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:44:6 */
  export declare function InitializeCodeGen(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/Initialization.h:45:6 */
  export declare function InitializeTarget(R: LLVM.PassRegistryRef): void;

  /** ./llvm-c/BitReader.h:39:10 */
  export declare function ParseBitcode(MemBuf: LLVM.MemoryBufferRef, OutModule: Pointer<"OutModule">, OutMessage: Pointer<"OutMessage">): LLVM.Bool;

  /** ./llvm-c/BitReader.h:44:10 */
  export declare function ParseBitcode2(MemBuf: LLVM.MemoryBufferRef, OutModule: Pointer<"OutModule">): LLVM.Bool;

  /** ./llvm-c/BitReader.h:48:10 */
  export declare function ParseBitcodeInContext(ContextRef: LLVM.ContextRef, MemBuf: LLVM.MemoryBufferRef, OutModule: Pointer<"OutModule">, OutMessage: Pointer<"OutMessage">): LLVM.Bool;

  /** ./llvm-c/BitReader.h:52:10 */
  export declare function ParseBitcodeInContext2(ContextRef: LLVM.ContextRef, MemBuf: LLVM.MemoryBufferRef, OutModule: Pointer<"OutModule">): LLVM.Bool;

  /** ./llvm-c/BitReader.h:60:10 */
  export declare function GetBitcodeModuleInContext(ContextRef: LLVM.ContextRef, MemBuf: LLVM.MemoryBufferRef, OutM: Pointer<"OutM">, OutMessage: Pointer<"OutMessage">): LLVM.Bool;

  /** ./llvm-c/BitReader.h:71:10 */
  export declare function GetBitcodeModuleInContext2(ContextRef: LLVM.ContextRef, MemBuf: LLVM.MemoryBufferRef, OutM: Pointer<"OutM">): LLVM.Bool;

  /** ./llvm-c/BitReader.h:76:10 */
  export declare function GetBitcodeModule(MemBuf: LLVM.MemoryBufferRef, OutM: Pointer<"OutM">, OutMessage: Pointer<"OutMessage">): LLVM.Bool;

  /** ./llvm-c/BitReader.h:79:10 */
  export declare function GetBitcodeModule2(MemBuf: LLVM.MemoryBufferRef, OutM: Pointer<"OutM">): LLVM.Bool;

  /** ./llvm-c/Comdat.h:46:15 */
  export declare function GetOrInsertComdat(M: LLVM.ModuleRef, Name: Pointer<"Name">): LLVM.ComdatRef;

  /** ./llvm-c/Comdat.h:53:15 */
  export declare function GetComdat(V: LLVM.ValueRef): LLVM.ComdatRef;

  /** ./llvm-c/Comdat.h:60:6 */
  export declare function SetComdat(V: LLVM.ValueRef, C: LLVM.ComdatRef): void;

  /** ./llvm-c/Comdat.h:67:25 */
  export declare function GetComdatSelectionKind(C: LLVM.ComdatRef): LLVM.ComdatSelectionKind;

  /** ./llvm-c/Comdat.h:74:6 */
  export declare function SetComdatSelectionKind(C: LLVM.ComdatRef, Kind: LLVM.ComdatSelectionKind): void;

  /** ./llvm-c/IRReader.h:38:10 */
  export declare function ParseIRInContext(ContextRef: LLVM.ContextRef, MemBuf: LLVM.MemoryBufferRef, OutM: Pointer<"OutM">, OutMessage: Pointer<"OutMessage">): LLVM.Bool;

  export declare function close(): void;
}
