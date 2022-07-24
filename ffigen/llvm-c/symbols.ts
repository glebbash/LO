export const LLVM_SYMBOLS = {
  VerifyModule: {
    name: "LLVMVerifyModule",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  VerifyFunction: {
    name: "LLVMVerifyFunction",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ViewFunctionCFG: {
    name: "LLVMViewFunctionCFG",
    parameters: ["pointer"],
    result: "void"
  },
  ViewFunctionCFGOnly: {
    name: "LLVMViewFunctionCFGOnly",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeWebAssemblyTargetInfo: {
    name: "LLVMInitializeWebAssemblyTargetInfo",
    parameters: [],
    result: "void"
  },
  InitializeX86TargetInfo: {
    name: "LLVMInitializeX86TargetInfo",
    parameters: [],
    result: "void"
  },
  InitializeWebAssemblyTarget: {
    name: "LLVMInitializeWebAssemblyTarget",
    parameters: [],
    result: "void"
  },
  InitializeX86Target: {
    name: "LLVMInitializeX86Target",
    parameters: [],
    result: "void"
  },
  InitializeWebAssemblyTargetMC: {
    name: "LLVMInitializeWebAssemblyTargetMC",
    parameters: [],
    result: "void"
  },
  InitializeX86TargetMC: {
    name: "LLVMInitializeX86TargetMC",
    parameters: [],
    result: "void"
  },
  InitializeWebAssemblyAsmPrinter: {
    name: "LLVMInitializeWebAssemblyAsmPrinter",
    parameters: [],
    result: "void"
  },
  InitializeX86AsmPrinter: {
    name: "LLVMInitializeX86AsmPrinter",
    parameters: [],
    result: "void"
  },
  InitializeWebAssemblyAsmParser: {
    name: "LLVMInitializeWebAssemblyAsmParser",
    parameters: [],
    result: "void"
  },
  InitializeX86AsmParser: {
    name: "LLVMInitializeX86AsmParser",
    parameters: [],
    result: "void"
  },
  InitializeWebAssemblyDisassembler: {
    name: "LLVMInitializeWebAssemblyDisassembler",
    parameters: [],
    result: "void"
  },
  InitializeX86Disassembler: {
    name: "LLVMInitializeX86Disassembler",
    parameters: [],
    result: "void"
  },
  GetModuleDataLayout: {
    name: "LLVMGetModuleDataLayout",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetModuleDataLayout: {
    name: "LLVMSetModuleDataLayout",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  CreateTargetData: {
    name: "LLVMCreateTargetData",
    parameters: ["pointer"],
    result: "pointer"
  },
  DisposeTargetData: {
    name: "LLVMDisposeTargetData",
    parameters: ["pointer"],
    result: "void"
  },
  AddTargetLibraryInfo: {
    name: "LLVMAddTargetLibraryInfo",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  CopyStringRepOfTargetData: {
    name: "LLVMCopyStringRepOfTargetData",
    parameters: ["pointer"],
    result: "pointer"
  },
  ByteOrder: {
    name: "LLVMByteOrder",
    parameters: ["pointer"],
    result: "i32"
  },
  PointerSize: {
    name: "LLVMPointerSize",
    parameters: ["pointer"],
    result: "u32"
  },
  PointerSizeForAS: {
    name: "LLVMPointerSizeForAS",
    parameters: ["pointer", "u32"],
    result: "u32"
  },
  IntPtrType: {
    name: "LLVMIntPtrType",
    parameters: ["pointer"],
    result: "pointer"
  },
  IntPtrTypeForAS: {
    name: "LLVMIntPtrTypeForAS",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  IntPtrTypeInContext: {
    name: "LLVMIntPtrTypeInContext",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  IntPtrTypeForASInContext: {
    name: "LLVMIntPtrTypeForASInContext",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  SizeOfTypeInBits: {
    name: "LLVMSizeOfTypeInBits",
    parameters: ["pointer", "pointer"],
    result: "u64"
  },
  StoreSizeOfType: {
    name: "LLVMStoreSizeOfType",
    parameters: ["pointer", "pointer"],
    result: "u64"
  },
  ABISizeOfType: {
    name: "LLVMABISizeOfType",
    parameters: ["pointer", "pointer"],
    result: "u64"
  },
  ABIAlignmentOfType: {
    name: "LLVMABIAlignmentOfType",
    parameters: ["pointer", "pointer"],
    result: "u32"
  },
  CallFrameAlignmentOfType: {
    name: "LLVMCallFrameAlignmentOfType",
    parameters: ["pointer", "pointer"],
    result: "u32"
  },
  PreferredAlignmentOfType: {
    name: "LLVMPreferredAlignmentOfType",
    parameters: ["pointer", "pointer"],
    result: "u32"
  },
  PreferredAlignmentOfGlobal: {
    name: "LLVMPreferredAlignmentOfGlobal",
    parameters: ["pointer", "pointer"],
    result: "u32"
  },
  ElementAtOffset: {
    name: "LLVMElementAtOffset",
    parameters: ["pointer", "pointer", "u64"],
    result: "u32"
  },
  OffsetOfElement: {
    name: "LLVMOffsetOfElement",
    parameters: ["pointer", "pointer", "u32"],
    result: "u64"
  },
  GetFirstTarget: {
    name: "LLVMGetFirstTarget",
    parameters: [],
    result: "pointer"
  },
  GetNextTarget: {
    name: "LLVMGetNextTarget",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetTargetFromName: {
    name: "LLVMGetTargetFromName",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetTargetFromTriple: {
    name: "LLVMGetTargetFromTriple",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetTargetName: {
    name: "LLVMGetTargetName",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetTargetDescription: {
    name: "LLVMGetTargetDescription",
    parameters: ["pointer"],
    result: "pointer"
  },
  TargetHasJIT: {
    name: "LLVMTargetHasJIT",
    parameters: ["pointer"],
    result: "pointer"
  },
  TargetHasTargetMachine: {
    name: "LLVMTargetHasTargetMachine",
    parameters: ["pointer"],
    result: "pointer"
  },
  TargetHasAsmBackend: {
    name: "LLVMTargetHasAsmBackend",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateTargetMachine: {
    name: "LLVMCreateTargetMachine",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  DisposeTargetMachine: {
    name: "LLVMDisposeTargetMachine",
    parameters: ["pointer"],
    result: "void"
  },
  GetTargetMachineTarget: {
    name: "LLVMGetTargetMachineTarget",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetTargetMachineTriple: {
    name: "LLVMGetTargetMachineTriple",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetTargetMachineCPU: {
    name: "LLVMGetTargetMachineCPU",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetTargetMachineFeatureString: {
    name: "LLVMGetTargetMachineFeatureString",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateTargetDataLayout: {
    name: "LLVMCreateTargetDataLayout",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetTargetMachineAsmVerbosity: {
    name: "LLVMSetTargetMachineAsmVerbosity",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  TargetMachineEmitToFile: {
    name: "LLVMTargetMachineEmitToFile",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  TargetMachineEmitToMemoryBuffer: {
    name: "LLVMTargetMachineEmitToMemoryBuffer",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetDefaultTargetTriple: {
    name: "LLVMGetDefaultTargetTriple",
    parameters: [],
    result: "pointer"
  },
  NormalizeTargetTriple: {
    name: "LLVMNormalizeTargetTriple",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetHostCPUName: {
    name: "LLVMGetHostCPUName",
    parameters: [],
    result: "pointer"
  },
  GetHostCPUFeatures: {
    name: "LLVMGetHostCPUFeatures",
    parameters: [],
    result: "pointer"
  },
  AddAnalysisPasses: {
    name: "LLVMAddAnalysisPasses",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  LinkInMCJIT: {
    name: "LLVMLinkInMCJIT",
    parameters: [],
    result: "void"
  },
  LinkInInterpreter: {
    name: "LLVMLinkInInterpreter",
    parameters: [],
    result: "void"
  },
  CreateGenericValueOfInt: {
    name: "LLVMCreateGenericValueOfInt",
    parameters: ["pointer", "u64", "pointer"],
    result: "pointer"
  },
  CreateGenericValueOfPointer: {
    name: "LLVMCreateGenericValueOfPointer",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateGenericValueOfFloat: {
    name: "LLVMCreateGenericValueOfFloat",
    parameters: ["pointer", "f64"],
    result: "pointer"
  },
  GenericValueIntWidth: {
    name: "LLVMGenericValueIntWidth",
    parameters: ["pointer"],
    result: "u32"
  },
  GenericValueToInt: {
    name: "LLVMGenericValueToInt",
    parameters: ["pointer", "pointer"],
    result: "u64"
  },
  GenericValueToPointer: {
    name: "LLVMGenericValueToPointer",
    parameters: ["pointer"],
    result: "pointer"
  },
  GenericValueToFloat: {
    name: "LLVMGenericValueToFloat",
    parameters: ["pointer", "pointer"],
    result: "f64"
  },
  DisposeGenericValue: {
    name: "LLVMDisposeGenericValue",
    parameters: ["pointer"],
    result: "void"
  },
  CreateExecutionEngineForModule: {
    name: "LLVMCreateExecutionEngineForModule",
    parameters: ["pointer", "pointer", "pointer"],
    result: "i32"
  },
  CreateInterpreterForModule: {
    name: "LLVMCreateInterpreterForModule",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  CreateJITCompilerForModule: {
    name: "LLVMCreateJITCompilerForModule",
    parameters: ["pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  InitializeMCJITCompilerOptions: {
    name: "LLVMInitializeMCJITCompilerOptions",
    parameters: ["pointer", "u64"],
    result: "void"
  },
  CreateMCJITCompilerForModule: {
    name: "LLVMCreateMCJITCompilerForModule",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer"],
    result: "pointer"
  },
  DisposeExecutionEngine: {
    name: "LLVMDisposeExecutionEngine",
    parameters: ["pointer"],
    result: "void"
  },
  RunStaticConstructors: {
    name: "LLVMRunStaticConstructors",
    parameters: ["pointer"],
    result: "void"
  },
  RunStaticDestructors: {
    name: "LLVMRunStaticDestructors",
    parameters: ["pointer"],
    result: "void"
  },
  RunFunctionAsMain: {
    name: "LLVMRunFunctionAsMain",
    parameters: ["pointer", "pointer", "u32", "pointer", "pointer"],
    result: "i32"
  },
  RunFunction: {
    name: "LLVMRunFunction",
    parameters: ["pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  FreeMachineCodeForFunction: {
    name: "LLVMFreeMachineCodeForFunction",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  AddModule: {
    name: "LLVMAddModule",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  RemoveModule: {
    name: "LLVMRemoveModule",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  FindFunction: {
    name: "LLVMFindFunction",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  RecompileAndRelinkFunction: {
    name: "LLVMRecompileAndRelinkFunction",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetExecutionEngineTargetData: {
    name: "LLVMGetExecutionEngineTargetData",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetExecutionEngineTargetMachine: {
    name: "LLVMGetExecutionEngineTargetMachine",
    parameters: ["pointer"],
    result: "pointer"
  },
  AddGlobalMapping: {
    name: "LLVMAddGlobalMapping",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  GetPointerToGlobal: {
    name: "LLVMGetPointerToGlobal",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetGlobalValueAddress: {
    name: "LLVMGetGlobalValueAddress",
    parameters: ["pointer", "pointer"],
    result: "u64"
  },
  GetFunctionAddress: {
    name: "LLVMGetFunctionAddress",
    parameters: ["pointer", "pointer"],
    result: "u64"
  },
  ExecutionEngineGetErrMsg: {
    name: "LLVMExecutionEngineGetErrMsg",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  CreateSimpleMCJITMemoryManager: {
    name: "LLVMCreateSimpleMCJITMemoryManager",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  DisposeMCJITMemoryManager: {
    name: "LLVMDisposeMCJITMemoryManager",
    parameters: ["pointer"],
    result: "void"
  },
  CreateGDBRegistrationListener: {
    name: "LLVMCreateGDBRegistrationListener",
    parameters: [],
    result: "pointer"
  },
  CreateIntelJITEventListener: {
    name: "LLVMCreateIntelJITEventListener",
    parameters: [],
    result: "pointer"
  },
  CreateOProfileJITEventListener: {
    name: "LLVMCreateOProfileJITEventListener",
    parameters: [],
    result: "pointer"
  },
  CreatePerfJITEventListener: {
    name: "LLVMCreatePerfJITEventListener",
    parameters: [],
    result: "pointer"
  },
  CreateDisasm: {
    name: "LLVMCreateDisasm",
    parameters: ["pointer", "pointer", "i32", "pointer", "pointer"],
    result: "pointer"
  },
  CreateDisasmCPU: {
    name: "LLVMCreateDisasmCPU",
    parameters: ["pointer", "pointer", "pointer", "i32", "pointer", "pointer"],
    result: "pointer"
  },
  CreateDisasmCPUFeatures: {
    name: "LLVMCreateDisasmCPUFeatures",
    parameters: ["pointer", "pointer", "pointer", "pointer", "i32", "pointer", "pointer"],
    result: "pointer"
  },
  SetDisasmOptions: {
    name: "LLVMSetDisasmOptions",
    parameters: ["pointer", "u64"],
    result: "i32"
  },
  DisasmDispose: {
    name: "LLVMDisasmDispose",
    parameters: ["pointer"],
    result: "void"
  },
  DisasmInstruction: {
    name: "LLVMDisasmInstruction",
    parameters: ["pointer", "pointer", "u64", "u64", "pointer", "u64"],
    result: "u64"
  },
  DebugMetadataVersion: {
    name: "LLVMDebugMetadataVersion",
    parameters: [],
    result: "u32"
  },
  GetModuleDebugMetadataVersion: {
    name: "LLVMGetModuleDebugMetadataVersion",
    parameters: ["pointer"],
    result: "u32"
  },
  StripModuleDebugInfo: {
    name: "LLVMStripModuleDebugInfo",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateDIBuilderDisallowUnresolved: {
    name: "LLVMCreateDIBuilderDisallowUnresolved",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateDIBuilder: {
    name: "LLVMCreateDIBuilder",
    parameters: ["pointer"],
    result: "pointer"
  },
  DisposeDIBuilder: {
    name: "LLVMDisposeDIBuilder",
    parameters: ["pointer"],
    result: "void"
  },
  DIBuilderFinalize: {
    name: "LLVMDIBuilderFinalize",
    parameters: ["pointer"],
    result: "void"
  },
  DIBuilderFinalizeSubprogram: {
    name: "LLVMDIBuilderFinalizeSubprogram",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  DIBuilderCreateCompileUnit: {
    name: "LLVMDIBuilderCreateCompileUnit",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u64", "pointer", "pointer", "u64", "u32", "pointer", "u64", "pointer", "u32", "pointer", "pointer", "pointer", "u64", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateFile: {
    name: "LLVMDIBuilderCreateFile",
    parameters: ["pointer", "pointer", "u64", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateModule: {
    name: "LLVMDIBuilderCreateModule",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u64", "pointer", "u64", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateNameSpace: {
    name: "LLVMDIBuilderCreateNameSpace",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateFunction: {
    name: "LLVMDIBuilderCreateFunction",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u64", "pointer", "u32", "pointer", "pointer", "pointer", "u32", "pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateLexicalBlock: {
    name: "LLVMDIBuilderCreateLexicalBlock",
    parameters: ["pointer", "pointer", "pointer", "u32", "u32"],
    result: "pointer"
  },
  DIBuilderCreateLexicalBlockFile: {
    name: "LLVMDIBuilderCreateLexicalBlockFile",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateImportedModuleFromNamespace: {
    name: "LLVMDIBuilderCreateImportedModuleFromNamespace",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateImportedModuleFromAlias: {
    name: "LLVMDIBuilderCreateImportedModuleFromAlias",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateImportedModuleFromModule: {
    name: "LLVMDIBuilderCreateImportedModuleFromModule",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateImportedDeclaration: {
    name: "LLVMDIBuilderCreateImportedDeclaration",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer", "u64", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateDebugLocation: {
    name: "LLVMDIBuilderCreateDebugLocation",
    parameters: ["pointer", "u32", "u32", "pointer", "pointer"],
    result: "pointer"
  },
  DILocationGetLine: {
    name: "LLVMDILocationGetLine",
    parameters: ["pointer"],
    result: "u32"
  },
  DILocationGetColumn: {
    name: "LLVMDILocationGetColumn",
    parameters: ["pointer"],
    result: "u32"
  },
  DILocationGetScope: {
    name: "LLVMDILocationGetScope",
    parameters: ["pointer"],
    result: "pointer"
  },
  DILocationGetInlinedAt: {
    name: "LLVMDILocationGetInlinedAt",
    parameters: ["pointer"],
    result: "pointer"
  },
  DIScopeGetFile: {
    name: "LLVMDIScopeGetFile",
    parameters: ["pointer"],
    result: "pointer"
  },
  DIFileGetDirectory: {
    name: "LLVMDIFileGetDirectory",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DIFileGetFilename: {
    name: "LLVMDIFileGetFilename",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DIFileGetSource: {
    name: "LLVMDIFileGetSource",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderGetOrCreateTypeArray: {
    name: "LLVMDIBuilderGetOrCreateTypeArray",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateSubroutineType: {
    name: "LLVMDIBuilderCreateSubroutineType",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateMacro: {
    name: "LLVMDIBuilderCreateMacro",
    parameters: ["pointer", "pointer", "u32", "pointer", "pointer", "u64", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateTempMacroFile: {
    name: "LLVMDIBuilderCreateTempMacroFile",
    parameters: ["pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateEnumerator: {
    name: "LLVMDIBuilderCreateEnumerator",
    parameters: ["pointer", "pointer", "u64", "i64", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateEnumerationType: {
    name: "LLVMDIBuilderCreateEnumerationType",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateUnionType: {
    name: "LLVMDIBuilderCreateUnionType",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "pointer", "pointer", "u32", "u32", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateArrayType: {
    name: "LLVMDIBuilderCreateArrayType",
    parameters: ["pointer", "u64", "u32", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateVectorType: {
    name: "LLVMDIBuilderCreateVectorType",
    parameters: ["pointer", "u64", "u32", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateUnspecifiedType: {
    name: "LLVMDIBuilderCreateUnspecifiedType",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateBasicType: {
    name: "LLVMDIBuilderCreateBasicType",
    parameters: ["pointer", "pointer", "u64", "u64", "pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderCreatePointerType: {
    name: "LLVMDIBuilderCreatePointerType",
    parameters: ["pointer", "pointer", "u64", "u32", "u32", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateStructType: {
    name: "LLVMDIBuilderCreateStructType",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "pointer", "pointer", "pointer", "u32", "u32", "pointer", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateMemberType: {
    name: "LLVMDIBuilderCreateMemberType",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "u64", "pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateStaticMemberType: {
    name: "LLVMDIBuilderCreateStaticMemberType",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateMemberPointerType: {
    name: "LLVMDIBuilderCreateMemberPointerType",
    parameters: ["pointer", "pointer", "pointer", "u64", "u32", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateObjCIVar: {
    name: "LLVMDIBuilderCreateObjCIVar",
    parameters: ["pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "u64", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateObjCProperty: {
    name: "LLVMDIBuilderCreateObjCProperty",
    parameters: ["pointer", "pointer", "u64", "pointer", "u32", "pointer", "u64", "pointer", "u64", "u32", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateObjectPointerType: {
    name: "LLVMDIBuilderCreateObjectPointerType",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateQualifiedType: {
    name: "LLVMDIBuilderCreateQualifiedType",
    parameters: ["pointer", "u32", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateReferenceType: {
    name: "LLVMDIBuilderCreateReferenceType",
    parameters: ["pointer", "u32", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateNullPtrType: {
    name: "LLVMDIBuilderCreateNullPtrType",
    parameters: ["pointer"],
    result: "pointer"
  },
  DIBuilderCreateTypedef: {
    name: "LLVMDIBuilderCreateTypedef",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateInheritance: {
    name: "LLVMDIBuilderCreateInheritance",
    parameters: ["pointer", "pointer", "pointer", "u64", "u32", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateForwardDecl: {
    name: "LLVMDIBuilderCreateForwardDecl",
    parameters: ["pointer", "u32", "pointer", "u64", "pointer", "pointer", "u32", "u32", "u64", "u32", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateReplaceableCompositeType: {
    name: "LLVMDIBuilderCreateReplaceableCompositeType",
    parameters: ["pointer", "u32", "pointer", "u64", "pointer", "pointer", "u32", "u32", "u64", "u32", "pointer", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateBitFieldMemberType: {
    name: "LLVMDIBuilderCreateBitFieldMemberType",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u64", "u64", "pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateClassType: {
    name: "LLVMDIBuilderCreateClassType",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "u64", "pointer", "pointer", "pointer", "u32", "pointer", "pointer", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateArtificialType: {
    name: "LLVMDIBuilderCreateArtificialType",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DITypeGetName: {
    name: "LLVMDITypeGetName",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DITypeGetSizeInBits: {
    name: "LLVMDITypeGetSizeInBits",
    parameters: ["pointer"],
    result: "u64"
  },
  DITypeGetOffsetInBits: {
    name: "LLVMDITypeGetOffsetInBits",
    parameters: ["pointer"],
    result: "u64"
  },
  DITypeGetAlignInBits: {
    name: "LLVMDITypeGetAlignInBits",
    parameters: ["pointer"],
    result: "u32"
  },
  DITypeGetLine: {
    name: "LLVMDITypeGetLine",
    parameters: ["pointer"],
    result: "u32"
  },
  DITypeGetFlags: {
    name: "LLVMDITypeGetFlags",
    parameters: ["pointer"],
    result: "pointer"
  },
  DIBuilderGetOrCreateSubrange: {
    name: "LLVMDIBuilderGetOrCreateSubrange",
    parameters: ["pointer", "i64", "i64"],
    result: "pointer"
  },
  DIBuilderGetOrCreateArray: {
    name: "LLVMDIBuilderGetOrCreateArray",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateExpression: {
    name: "LLVMDIBuilderCreateExpression",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateConstantValueExpression: {
    name: "LLVMDIBuilderCreateConstantValueExpression",
    parameters: ["pointer", "u64"],
    result: "pointer"
  },
  DIBuilderCreateGlobalVariableExpression: {
    name: "LLVMDIBuilderCreateGlobalVariableExpression",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u64", "pointer", "u32", "pointer", "pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  DIGlobalVariableExpressionGetVariable: {
    name: "LLVMDIGlobalVariableExpressionGetVariable",
    parameters: ["pointer"],
    result: "pointer"
  },
  DIGlobalVariableExpressionGetExpression: {
    name: "LLVMDIGlobalVariableExpressionGetExpression",
    parameters: ["pointer"],
    result: "pointer"
  },
  DIVariableGetFile: {
    name: "LLVMDIVariableGetFile",
    parameters: ["pointer"],
    result: "pointer"
  },
  DIVariableGetScope: {
    name: "LLVMDIVariableGetScope",
    parameters: ["pointer"],
    result: "pointer"
  },
  DIVariableGetLine: {
    name: "LLVMDIVariableGetLine",
    parameters: ["pointer"],
    result: "u32"
  },
  TemporaryMDNode: {
    name: "LLVMTemporaryMDNode",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  DisposeTemporaryMDNode: {
    name: "LLVMDisposeTemporaryMDNode",
    parameters: ["pointer"],
    result: "void"
  },
  MetadataReplaceAllUsesWith: {
    name: "LLVMMetadataReplaceAllUsesWith",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  DIBuilderCreateTempGlobalVariableFwdDecl: {
    name: "LLVMDIBuilderCreateTempGlobalVariableFwdDecl",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u64", "pointer", "u32", "pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderInsertDeclareBefore: {
    name: "LLVMDIBuilderInsertDeclareBefore",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderInsertDeclareAtEnd: {
    name: "LLVMDIBuilderInsertDeclareAtEnd",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderInsertDbgValueBefore: {
    name: "LLVMDIBuilderInsertDbgValueBefore",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderInsertDbgValueAtEnd: {
    name: "LLVMDIBuilderInsertDbgValueAtEnd",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  DIBuilderCreateAutoVariable: {
    name: "LLVMDIBuilderCreateAutoVariable",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  DIBuilderCreateParameterVariable: {
    name: "LLVMDIBuilderCreateParameterVariable",
    parameters: ["pointer", "pointer", "pointer", "u64", "u32", "pointer", "u32", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetSubprogram: {
    name: "LLVMGetSubprogram",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetSubprogram: {
    name: "LLVMSetSubprogram",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  DISubprogramGetLine: {
    name: "LLVMDISubprogramGetLine",
    parameters: ["pointer"],
    result: "u32"
  },
  InstructionGetDebugLoc: {
    name: "LLVMInstructionGetDebugLoc",
    parameters: ["pointer"],
    result: "pointer"
  },
  InstructionSetDebugLoc: {
    name: "LLVMInstructionSetDebugLoc",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetMetadataKind: {
    name: "LLVMGetMetadataKind",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetErrorTypeId: {
    name: "LLVMGetErrorTypeId",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConsumeError: {
    name: "LLVMConsumeError",
    parameters: ["pointer"],
    result: "void"
  },
  GetErrorMessage: {
    name: "LLVMGetErrorMessage",
    parameters: ["pointer"],
    result: "pointer"
  },
  DisposeErrorMessage: {
    name: "LLVMDisposeErrorMessage",
    parameters: ["pointer"],
    result: "void"
  },
  GetStringErrorTypeId: {
    name: "LLVMGetStringErrorTypeId",
    parameters: [],
    result: "pointer"
  },
  CreateStringError: {
    name: "LLVMCreateStringError",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcExecutionSessionSetErrorReporter: {
    name: "LLVMOrcExecutionSessionSetErrorReporter",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  OrcExecutionSessionGetSymbolStringPool: {
    name: "LLVMOrcExecutionSessionGetSymbolStringPool",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcSymbolStringPoolClearDeadEntries: {
    name: "LLVMOrcSymbolStringPoolClearDeadEntries",
    parameters: ["pointer"],
    result: "void"
  },
  OrcExecutionSessionIntern: {
    name: "LLVMOrcExecutionSessionIntern",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcRetainSymbolStringPoolEntry: {
    name: "LLVMOrcRetainSymbolStringPoolEntry",
    parameters: ["pointer"],
    result: "void"
  },
  OrcReleaseSymbolStringPoolEntry: {
    name: "LLVMOrcReleaseSymbolStringPoolEntry",
    parameters: ["pointer"],
    result: "void"
  },
  OrcSymbolStringPoolEntryStr: {
    name: "LLVMOrcSymbolStringPoolEntryStr",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcReleaseResourceTracker: {
    name: "LLVMOrcReleaseResourceTracker",
    parameters: ["pointer"],
    result: "void"
  },
  OrcResourceTrackerTransferTo: {
    name: "LLVMOrcResourceTrackerTransferTo",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  OrcResourceTrackerRemove: {
    name: "LLVMOrcResourceTrackerRemove",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcDisposeDefinitionGenerator: {
    name: "LLVMOrcDisposeDefinitionGenerator",
    parameters: ["pointer"],
    result: "void"
  },
  OrcDisposeMaterializationUnit: {
    name: "LLVMOrcDisposeMaterializationUnit",
    parameters: ["pointer"],
    result: "void"
  },
  OrcCreateCustomMaterializationUnit: {
    name: "LLVMOrcCreateCustomMaterializationUnit",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcAbsoluteSymbols: {
    name: "LLVMOrcAbsoluteSymbols",
    parameters: ["pointer", "u64"],
    result: "pointer"
  },
  OrcLazyReexports: {
    name: "LLVMOrcLazyReexports",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u64"],
    result: "pointer"
  },
  OrcDisposeMaterializationResponsibility: {
    name: "LLVMOrcDisposeMaterializationResponsibility",
    parameters: ["pointer"],
    result: "void"
  },
  OrcMaterializationResponsibilityGetTargetDylib: {
    name: "LLVMOrcMaterializationResponsibilityGetTargetDylib",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcMaterializationResponsibilityGetExecutionSession: {
    name: "LLVMOrcMaterializationResponsibilityGetExecutionSession",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcMaterializationResponsibilityGetSymbols: {
    name: "LLVMOrcMaterializationResponsibilityGetSymbols",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcDisposeCSymbolFlagsMap: {
    name: "LLVMOrcDisposeCSymbolFlagsMap",
    parameters: ["pointer"],
    result: "void"
  },
  OrcMaterializationResponsibilityGetInitializerSymbol: {
    name: "LLVMOrcMaterializationResponsibilityGetInitializerSymbol",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcMaterializationResponsibilityGetRequestedSymbols: {
    name: "LLVMOrcMaterializationResponsibilityGetRequestedSymbols",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcDisposeSymbols: {
    name: "LLVMOrcDisposeSymbols",
    parameters: ["pointer"],
    result: "void"
  },
  OrcMaterializationResponsibilityNotifyResolved: {
    name: "LLVMOrcMaterializationResponsibilityNotifyResolved",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  OrcMaterializationResponsibilityNotifyEmitted: {
    name: "LLVMOrcMaterializationResponsibilityNotifyEmitted",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcMaterializationResponsibilityDefineMaterializing: {
    name: "LLVMOrcMaterializationResponsibilityDefineMaterializing",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  OrcMaterializationResponsibilityFailMaterialization: {
    name: "LLVMOrcMaterializationResponsibilityFailMaterialization",
    parameters: ["pointer"],
    result: "void"
  },
  OrcMaterializationResponsibilityReplace: {
    name: "LLVMOrcMaterializationResponsibilityReplace",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcMaterializationResponsibilityDelegate: {
    name: "LLVMOrcMaterializationResponsibilityDelegate",
    parameters: ["pointer", "pointer", "u64", "pointer"],
    result: "pointer"
  },
  OrcMaterializationResponsibilityAddDependencies: {
    name: "LLVMOrcMaterializationResponsibilityAddDependencies",
    parameters: ["pointer", "pointer", "pointer", "u64"],
    result: "void"
  },
  OrcMaterializationResponsibilityAddDependenciesForAll: {
    name: "LLVMOrcMaterializationResponsibilityAddDependenciesForAll",
    parameters: ["pointer", "pointer", "u64"],
    result: "void"
  },
  OrcExecutionSessionCreateBareJITDylib: {
    name: "LLVMOrcExecutionSessionCreateBareJITDylib",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcExecutionSessionCreateJITDylib: {
    name: "LLVMOrcExecutionSessionCreateJITDylib",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcExecutionSessionGetJITDylibByName: {
    name: "LLVMOrcExecutionSessionGetJITDylibByName",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcJITDylibCreateResourceTracker: {
    name: "LLVMOrcJITDylibCreateResourceTracker",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcJITDylibGetDefaultResourceTracker: {
    name: "LLVMOrcJITDylibGetDefaultResourceTracker",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcJITDylibDefine: {
    name: "LLVMOrcJITDylibDefine",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcJITDylibClear: {
    name: "LLVMOrcJITDylibClear",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcJITDylibAddGenerator: {
    name: "LLVMOrcJITDylibAddGenerator",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  OrcCreateCustomCAPIDefinitionGenerator: {
    name: "LLVMOrcCreateCustomCAPIDefinitionGenerator",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcCreateDynamicLibrarySearchGeneratorForProcess: {
    name: "LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess",
    parameters: ["pointer", "u8", "pointer", "pointer"],
    result: "pointer"
  },
  OrcCreateDynamicLibrarySearchGeneratorForPath: {
    name: "LLVMOrcCreateDynamicLibrarySearchGeneratorForPath",
    parameters: ["pointer", "pointer", "u8", "pointer", "pointer"],
    result: "pointer"
  },
  OrcCreateStaticLibrarySearchGeneratorForPath: {
    name: "LLVMOrcCreateStaticLibrarySearchGeneratorForPath",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcCreateNewThreadSafeContext: {
    name: "LLVMOrcCreateNewThreadSafeContext",
    parameters: [],
    result: "pointer"
  },
  OrcThreadSafeContextGetContext: {
    name: "LLVMOrcThreadSafeContextGetContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcDisposeThreadSafeContext: {
    name: "LLVMOrcDisposeThreadSafeContext",
    parameters: ["pointer"],
    result: "void"
  },
  OrcCreateNewThreadSafeModule: {
    name: "LLVMOrcCreateNewThreadSafeModule",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcDisposeThreadSafeModule: {
    name: "LLVMOrcDisposeThreadSafeModule",
    parameters: ["pointer"],
    result: "void"
  },
  OrcThreadSafeModuleWithModuleDo: {
    name: "LLVMOrcThreadSafeModuleWithModuleDo",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcJITTargetMachineBuilderDetectHost: {
    name: "LLVMOrcJITTargetMachineBuilderDetectHost",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcJITTargetMachineBuilderCreateFromTargetMachine: {
    name: "LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcDisposeJITTargetMachineBuilder: {
    name: "LLVMOrcDisposeJITTargetMachineBuilder",
    parameters: ["pointer"],
    result: "void"
  },
  OrcJITTargetMachineBuilderGetTargetTriple: {
    name: "LLVMOrcJITTargetMachineBuilderGetTargetTriple",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcJITTargetMachineBuilderSetTargetTriple: {
    name: "LLVMOrcJITTargetMachineBuilderSetTargetTriple",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  OrcObjectLayerAddObjectFile: {
    name: "LLVMOrcObjectLayerAddObjectFile",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcObjectLayerEmit: {
    name: "LLVMOrcObjectLayerEmit",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  OrcDisposeObjectLayer: {
    name: "LLVMOrcDisposeObjectLayer",
    parameters: ["pointer"],
    result: "void"
  },
  OrcIRTransformLayerEmit: {
    name: "LLVMOrcIRTransformLayerEmit",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  OrcIRTransformLayerSetTransform: {
    name: "LLVMOrcIRTransformLayerSetTransform",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  OrcObjectTransformLayerSetTransform: {
    name: "LLVMOrcObjectTransformLayerSetTransform",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  OrcCreateLocalIndirectStubsManager: {
    name: "LLVMOrcCreateLocalIndirectStubsManager",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcDisposeIndirectStubsManager: {
    name: "LLVMOrcDisposeIndirectStubsManager",
    parameters: ["pointer"],
    result: "void"
  },
  OrcCreateLocalLazyCallThroughManager: {
    name: "LLVMOrcCreateLocalLazyCallThroughManager",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcDisposeLazyCallThroughManager: {
    name: "LLVMOrcDisposeLazyCallThroughManager",
    parameters: ["pointer"],
    result: "void"
  },
  OrcCreateDumpObjects: {
    name: "LLVMOrcCreateDumpObjects",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcDisposeDumpObjects: {
    name: "LLVMOrcDisposeDumpObjects",
    parameters: ["pointer"],
    result: "void"
  },
  OrcDumpObjects_CallOperator: {
    name: "LLVMOrcDumpObjects_CallOperator",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager: {
    name: "LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcRTDyldObjectLinkingLayerRegisterJITEventListener: {
    name: "LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassManagerBuilderCreate: {
    name: "LLVMPassManagerBuilderCreate",
    parameters: [],
    result: "pointer"
  },
  PassManagerBuilderDispose: {
    name: "LLVMPassManagerBuilderDispose",
    parameters: ["pointer"],
    result: "void"
  },
  PassManagerBuilderSetOptLevel: {
    name: "LLVMPassManagerBuilderSetOptLevel",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  PassManagerBuilderSetSizeLevel: {
    name: "LLVMPassManagerBuilderSetSizeLevel",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  PassManagerBuilderSetDisableUnitAtATime: {
    name: "LLVMPassManagerBuilderSetDisableUnitAtATime",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassManagerBuilderSetDisableUnrollLoops: {
    name: "LLVMPassManagerBuilderSetDisableUnrollLoops",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassManagerBuilderSetDisableSimplifyLibCalls: {
    name: "LLVMPassManagerBuilderSetDisableSimplifyLibCalls",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassManagerBuilderUseInlinerWithThreshold: {
    name: "LLVMPassManagerBuilderUseInlinerWithThreshold",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  PassManagerBuilderPopulateFunctionPassManager: {
    name: "LLVMPassManagerBuilderPopulateFunctionPassManager",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassManagerBuilderPopulateModulePassManager: {
    name: "LLVMPassManagerBuilderPopulateModulePassManager",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  AddLowerSwitchPass: {
    name: "LLVMAddLowerSwitchPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddPromoteMemoryToRegisterPass: {
    name: "LLVMAddPromoteMemoryToRegisterPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddAddDiscriminatorsPass: {
    name: "LLVMAddAddDiscriminatorsPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddConstantMergePass: {
    name: "LLVMAddConstantMergePass",
    parameters: ["pointer"],
    result: "void"
  },
  AddMergeFunctionsPass: {
    name: "LLVMAddMergeFunctionsPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddCalledValuePropagationPass: {
    name: "LLVMAddCalledValuePropagationPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddDeadArgEliminationPass: {
    name: "LLVMAddDeadArgEliminationPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddFunctionAttrsPass: {
    name: "LLVMAddFunctionAttrsPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddFunctionInliningPass: {
    name: "LLVMAddFunctionInliningPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddAlwaysInlinerPass: {
    name: "LLVMAddAlwaysInlinerPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddGlobalDCEPass: {
    name: "LLVMAddGlobalDCEPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddGlobalOptimizerPass: {
    name: "LLVMAddGlobalOptimizerPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddPruneEHPass: {
    name: "LLVMAddPruneEHPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddIPSCCPPass: {
    name: "LLVMAddIPSCCPPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddInternalizePass: {
    name: "LLVMAddInternalizePass",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  AddInternalizePassWithMustPreservePredicate: {
    name: "LLVMAddInternalizePassWithMustPreservePredicate",
    parameters: ["pointer", "pointer", "function"],
    result: "void"
  },
  AddStripDeadPrototypesPass: {
    name: "LLVMAddStripDeadPrototypesPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddStripSymbolsPass: {
    name: "LLVMAddStripSymbolsPass",
    parameters: ["pointer"],
    result: "void"
  },
  RunPasses: {
    name: "LLVMRunPasses",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  CreatePassBuilderOptions: {
    name: "LLVMCreatePassBuilderOptions",
    parameters: [],
    result: "pointer"
  },
  PassBuilderOptionsSetVerifyEach: {
    name: "LLVMPassBuilderOptionsSetVerifyEach",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassBuilderOptionsSetDebugLogging: {
    name: "LLVMPassBuilderOptionsSetDebugLogging",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassBuilderOptionsSetLoopInterleaving: {
    name: "LLVMPassBuilderOptionsSetLoopInterleaving",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassBuilderOptionsSetLoopVectorization: {
    name: "LLVMPassBuilderOptionsSetLoopVectorization",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassBuilderOptionsSetSLPVectorization: {
    name: "LLVMPassBuilderOptionsSetSLPVectorization",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassBuilderOptionsSetLoopUnrolling: {
    name: "LLVMPassBuilderOptionsSetLoopUnrolling",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassBuilderOptionsSetForgetAllSCEVInLoopUnroll: {
    name: "LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassBuilderOptionsSetLicmMssaOptCap: {
    name: "LLVMPassBuilderOptionsSetLicmMssaOptCap",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  PassBuilderOptionsSetLicmMssaNoAccForPromotionCap: {
    name: "LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  PassBuilderOptionsSetCallGraphProfile: {
    name: "LLVMPassBuilderOptionsSetCallGraphProfile",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PassBuilderOptionsSetMergeFunctions: {
    name: "LLVMPassBuilderOptionsSetMergeFunctions",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  DisposePassBuilderOptions: {
    name: "LLVMDisposePassBuilderOptions",
    parameters: ["pointer"],
    result: "void"
  },
  AddAggressiveInstCombinerPass: {
    name: "LLVMAddAggressiveInstCombinerPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddAggressiveDCEPass: {
    name: "LLVMAddAggressiveDCEPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddDCEPass: {
    name: "LLVMAddDCEPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddBitTrackingDCEPass: {
    name: "LLVMAddBitTrackingDCEPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddAlignmentFromAssumptionsPass: {
    name: "LLVMAddAlignmentFromAssumptionsPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddCFGSimplificationPass: {
    name: "LLVMAddCFGSimplificationPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddDeadStoreEliminationPass: {
    name: "LLVMAddDeadStoreEliminationPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddScalarizerPass: {
    name: "LLVMAddScalarizerPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddMergedLoadStoreMotionPass: {
    name: "LLVMAddMergedLoadStoreMotionPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddGVNPass: {
    name: "LLVMAddGVNPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddNewGVNPass: {
    name: "LLVMAddNewGVNPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddIndVarSimplifyPass: {
    name: "LLVMAddIndVarSimplifyPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddInstructionCombiningPass: {
    name: "LLVMAddInstructionCombiningPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddInstructionSimplifyPass: {
    name: "LLVMAddInstructionSimplifyPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddJumpThreadingPass: {
    name: "LLVMAddJumpThreadingPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLICMPass: {
    name: "LLVMAddLICMPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLoopDeletionPass: {
    name: "LLVMAddLoopDeletionPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLoopIdiomPass: {
    name: "LLVMAddLoopIdiomPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLoopRotatePass: {
    name: "LLVMAddLoopRotatePass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLoopRerollPass: {
    name: "LLVMAddLoopRerollPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLoopUnrollPass: {
    name: "LLVMAddLoopUnrollPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLoopUnrollAndJamPass: {
    name: "LLVMAddLoopUnrollAndJamPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLowerAtomicPass: {
    name: "LLVMAddLowerAtomicPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddMemCpyOptPass: {
    name: "LLVMAddMemCpyOptPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddPartiallyInlineLibCallsPass: {
    name: "LLVMAddPartiallyInlineLibCallsPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddReassociatePass: {
    name: "LLVMAddReassociatePass",
    parameters: ["pointer"],
    result: "void"
  },
  AddSCCPPass: {
    name: "LLVMAddSCCPPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddScalarReplAggregatesPass: {
    name: "LLVMAddScalarReplAggregatesPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddScalarReplAggregatesPassSSA: {
    name: "LLVMAddScalarReplAggregatesPassSSA",
    parameters: ["pointer"],
    result: "void"
  },
  AddScalarReplAggregatesPassWithThreshold: {
    name: "LLVMAddScalarReplAggregatesPassWithThreshold",
    parameters: ["pointer", "i32"],
    result: "void"
  },
  AddSimplifyLibCallsPass: {
    name: "LLVMAddSimplifyLibCallsPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddTailCallEliminationPass: {
    name: "LLVMAddTailCallEliminationPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddDemoteMemoryToRegisterPass: {
    name: "LLVMAddDemoteMemoryToRegisterPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddVerifierPass: {
    name: "LLVMAddVerifierPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddCorrelatedValuePropagationPass: {
    name: "LLVMAddCorrelatedValuePropagationPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddEarlyCSEPass: {
    name: "LLVMAddEarlyCSEPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddEarlyCSEMemSSAPass: {
    name: "LLVMAddEarlyCSEMemSSAPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLowerExpectIntrinsicPass: {
    name: "LLVMAddLowerExpectIntrinsicPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLowerConstantIntrinsicsPass: {
    name: "LLVMAddLowerConstantIntrinsicsPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddTypeBasedAliasAnalysisPass: {
    name: "LLVMAddTypeBasedAliasAnalysisPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddScopedNoAliasAAPass: {
    name: "LLVMAddScopedNoAliasAAPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddBasicAliasAnalysisPass: {
    name: "LLVMAddBasicAliasAnalysisPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddUnifyFunctionExitNodesPass: {
    name: "LLVMAddUnifyFunctionExitNodesPass",
    parameters: ["pointer"],
    result: "void"
  },
  AddLoopVectorizePass: {
    name: "LLVMAddLoopVectorizePass",
    parameters: ["pointer"],
    result: "void"
  },
  AddSLPVectorizePass: {
    name: "LLVMAddSLPVectorizePass",
    parameters: ["pointer"],
    result: "void"
  },
  LinkModules2: {
    name: "LLVMLinkModules2",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  RemarkStringGetData: {
    name: "LLVMRemarkStringGetData",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkStringGetLen: {
    name: "LLVMRemarkStringGetLen",
    parameters: ["pointer"],
    result: "u32"
  },
  RemarkDebugLocGetSourceFilePath: {
    name: "LLVMRemarkDebugLocGetSourceFilePath",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkDebugLocGetSourceLine: {
    name: "LLVMRemarkDebugLocGetSourceLine",
    parameters: ["pointer"],
    result: "u32"
  },
  RemarkDebugLocGetSourceColumn: {
    name: "LLVMRemarkDebugLocGetSourceColumn",
    parameters: ["pointer"],
    result: "u32"
  },
  RemarkArgGetKey: {
    name: "LLVMRemarkArgGetKey",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkArgGetValue: {
    name: "LLVMRemarkArgGetValue",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkArgGetDebugLoc: {
    name: "LLVMRemarkArgGetDebugLoc",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkEntryDispose: {
    name: "LLVMRemarkEntryDispose",
    parameters: ["pointer"],
    result: "void"
  },
  RemarkEntryGetType: {
    name: "LLVMRemarkEntryGetType",
    parameters: ["pointer"],
    result: "i32"
  },
  RemarkEntryGetPassName: {
    name: "LLVMRemarkEntryGetPassName",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkEntryGetRemarkName: {
    name: "LLVMRemarkEntryGetRemarkName",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkEntryGetFunctionName: {
    name: "LLVMRemarkEntryGetFunctionName",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkEntryGetDebugLoc: {
    name: "LLVMRemarkEntryGetDebugLoc",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkEntryGetHotness: {
    name: "LLVMRemarkEntryGetHotness",
    parameters: ["pointer"],
    result: "u64"
  },
  RemarkEntryGetNumArgs: {
    name: "LLVMRemarkEntryGetNumArgs",
    parameters: ["pointer"],
    result: "u32"
  },
  RemarkEntryGetFirstArg: {
    name: "LLVMRemarkEntryGetFirstArg",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkEntryGetNextArg: {
    name: "LLVMRemarkEntryGetNextArg",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  RemarkParserCreateYAML: {
    name: "LLVMRemarkParserCreateYAML",
    parameters: ["pointer", "u64"],
    result: "pointer"
  },
  RemarkParserCreateBitstream: {
    name: "LLVMRemarkParserCreateBitstream",
    parameters: ["pointer", "u64"],
    result: "pointer"
  },
  RemarkParserGetNext: {
    name: "LLVMRemarkParserGetNext",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkParserHasError: {
    name: "LLVMRemarkParserHasError",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkParserGetErrorMessage: {
    name: "LLVMRemarkParserGetErrorMessage",
    parameters: ["pointer"],
    result: "pointer"
  },
  RemarkParserDispose: {
    name: "LLVMRemarkParserDispose",
    parameters: ["pointer"],
    result: "void"
  },
  InstallFatalErrorHandler: {
    name: "LLVMInstallFatalErrorHandler",
    parameters: ["pointer"],
    result: "void"
  },
  ResetFatalErrorHandler: {
    name: "LLVMResetFatalErrorHandler",
    parameters: [],
    result: "void"
  },
  EnablePrettyStackTrace: {
    name: "LLVMEnablePrettyStackTrace",
    parameters: [],
    result: "void"
  },
  InitializeCore: {
    name: "LLVMInitializeCore",
    parameters: ["pointer"],
    result: "void"
  },
  Shutdown: {
    name: "LLVMShutdown",
    parameters: [],
    result: "void"
  },
  CreateMessage: {
    name: "LLVMCreateMessage",
    parameters: ["pointer"],
    result: "pointer"
  },
  DisposeMessage: {
    name: "LLVMDisposeMessage",
    parameters: ["pointer"],
    result: "void"
  },
  ContextCreate: {
    name: "LLVMContextCreate",
    parameters: [],
    result: "pointer"
  },
  GetGlobalContext: {
    name: "LLVMGetGlobalContext",
    parameters: [],
    result: "pointer"
  },
  ContextSetDiagnosticHandler: {
    name: "LLVMContextSetDiagnosticHandler",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  ContextGetDiagnosticHandler: {
    name: "LLVMContextGetDiagnosticHandler",
    parameters: ["pointer"],
    result: "pointer"
  },
  ContextGetDiagnosticContext: {
    name: "LLVMContextGetDiagnosticContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  ContextSetYieldCallback: {
    name: "LLVMContextSetYieldCallback",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  ContextShouldDiscardValueNames: {
    name: "LLVMContextShouldDiscardValueNames",
    parameters: ["pointer"],
    result: "pointer"
  },
  ContextSetDiscardValueNames: {
    name: "LLVMContextSetDiscardValueNames",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  ContextSetOpaquePointers: {
    name: "LLVMContextSetOpaquePointers",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  ContextDispose: {
    name: "LLVMContextDispose",
    parameters: ["pointer"],
    result: "void"
  },
  GetDiagInfoDescription: {
    name: "LLVMGetDiagInfoDescription",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetDiagInfoSeverity: {
    name: "LLVMGetDiagInfoSeverity",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetMDKindIDInContext: {
    name: "LLVMGetMDKindIDInContext",
    parameters: ["pointer", "pointer", "u32"],
    result: "u32"
  },
  GetMDKindID: {
    name: "LLVMGetMDKindID",
    parameters: ["pointer", "u32"],
    result: "u32"
  },
  GetEnumAttributeKindForName: {
    name: "LLVMGetEnumAttributeKindForName",
    parameters: ["pointer", "u64"],
    result: "u32"
  },
  GetLastEnumAttributeKind: {
    name: "LLVMGetLastEnumAttributeKind",
    parameters: [],
    result: "u32"
  },
  CreateEnumAttribute: {
    name: "LLVMCreateEnumAttribute",
    parameters: ["pointer", "u32", "u64"],
    result: "pointer"
  },
  GetEnumAttributeKind: {
    name: "LLVMGetEnumAttributeKind",
    parameters: ["pointer"],
    result: "u32"
  },
  GetEnumAttributeValue: {
    name: "LLVMGetEnumAttributeValue",
    parameters: ["pointer"],
    result: "u64"
  },
  CreateTypeAttribute: {
    name: "LLVMCreateTypeAttribute",
    parameters: ["pointer", "u32", "pointer"],
    result: "pointer"
  },
  GetTypeAttributeValue: {
    name: "LLVMGetTypeAttributeValue",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateStringAttribute: {
    name: "LLVMCreateStringAttribute",
    parameters: ["pointer", "pointer", "u32", "pointer", "u32"],
    result: "pointer"
  },
  GetStringAttributeKind: {
    name: "LLVMGetStringAttributeKind",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetStringAttributeValue: {
    name: "LLVMGetStringAttributeValue",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  IsEnumAttribute: {
    name: "LLVMIsEnumAttribute",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsStringAttribute: {
    name: "LLVMIsStringAttribute",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsTypeAttribute: {
    name: "LLVMIsTypeAttribute",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetTypeByName2: {
    name: "LLVMGetTypeByName2",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ModuleCreateWithName: {
    name: "LLVMModuleCreateWithName",
    parameters: ["pointer"],
    result: "pointer"
  },
  ModuleCreateWithNameInContext: {
    name: "LLVMModuleCreateWithNameInContext",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  CloneModule: {
    name: "LLVMCloneModule",
    parameters: ["pointer"],
    result: "pointer"
  },
  DisposeModule: {
    name: "LLVMDisposeModule",
    parameters: ["pointer"],
    result: "void"
  },
  GetModuleIdentifier: {
    name: "LLVMGetModuleIdentifier",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  SetModuleIdentifier: {
    name: "LLVMSetModuleIdentifier",
    parameters: ["pointer", "pointer", "u64"],
    result: "void"
  },
  GetSourceFileName: {
    name: "LLVMGetSourceFileName",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  SetSourceFileName: {
    name: "LLVMSetSourceFileName",
    parameters: ["pointer", "pointer", "u64"],
    result: "void"
  },
  GetDataLayoutStr: {
    name: "LLVMGetDataLayoutStr",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetDataLayout: {
    name: "LLVMGetDataLayout",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetDataLayout: {
    name: "LLVMSetDataLayout",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetTarget: {
    name: "LLVMGetTarget",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetTarget: {
    name: "LLVMSetTarget",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  CopyModuleFlagsMetadata: {
    name: "LLVMCopyModuleFlagsMetadata",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DisposeModuleFlagsMetadata: {
    name: "LLVMDisposeModuleFlagsMetadata",
    parameters: ["pointer"],
    result: "void"
  },
  ModuleFlagEntriesGetFlagBehavior: {
    name: "LLVMModuleFlagEntriesGetFlagBehavior",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  ModuleFlagEntriesGetKey: {
    name: "LLVMModuleFlagEntriesGetKey",
    parameters: ["pointer", "u32", "pointer"],
    result: "pointer"
  },
  ModuleFlagEntriesGetMetadata: {
    name: "LLVMModuleFlagEntriesGetMetadata",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetModuleFlag: {
    name: "LLVMGetModuleFlag",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  AddModuleFlag: {
    name: "LLVMAddModuleFlag",
    parameters: ["pointer", "pointer", "pointer", "u64", "pointer"],
    result: "void"
  },
  DumpModule: {
    name: "LLVMDumpModule",
    parameters: ["pointer"],
    result: "void"
  },
  PrintModuleToFile: {
    name: "LLVMPrintModuleToFile",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  PrintModuleToString: {
    name: "LLVMPrintModuleToString",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetModuleInlineAsm: {
    name: "LLVMGetModuleInlineAsm",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  SetModuleInlineAsm2: {
    name: "LLVMSetModuleInlineAsm2",
    parameters: ["pointer", "pointer", "u64"],
    result: "void"
  },
  AppendModuleInlineAsm: {
    name: "LLVMAppendModuleInlineAsm",
    parameters: ["pointer", "pointer", "u64"],
    result: "void"
  },
  GetInlineAsm: {
    name: "LLVMGetInlineAsm",
    parameters: ["pointer", "pointer", "u64", "pointer", "u64", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetModuleContext: {
    name: "LLVMGetModuleContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetTypeByName: {
    name: "LLVMGetTypeByName",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetFirstNamedMetadata: {
    name: "LLVMGetFirstNamedMetadata",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetLastNamedMetadata: {
    name: "LLVMGetLastNamedMetadata",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNextNamedMetadata: {
    name: "LLVMGetNextNamedMetadata",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPreviousNamedMetadata: {
    name: "LLVMGetPreviousNamedMetadata",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNamedMetadata: {
    name: "LLVMGetNamedMetadata",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  GetOrInsertNamedMetadata: {
    name: "LLVMGetOrInsertNamedMetadata",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  GetNamedMetadataName: {
    name: "LLVMGetNamedMetadataName",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetNamedMetadataNumOperands: {
    name: "LLVMGetNamedMetadataNumOperands",
    parameters: ["pointer", "pointer"],
    result: "u32"
  },
  GetNamedMetadataOperands: {
    name: "LLVMGetNamedMetadataOperands",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  AddNamedMetadataOperand: {
    name: "LLVMAddNamedMetadataOperand",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  GetDebugLocDirectory: {
    name: "LLVMGetDebugLocDirectory",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetDebugLocFilename: {
    name: "LLVMGetDebugLocFilename",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetDebugLocLine: {
    name: "LLVMGetDebugLocLine",
    parameters: ["pointer"],
    result: "u32"
  },
  GetDebugLocColumn: {
    name: "LLVMGetDebugLocColumn",
    parameters: ["pointer"],
    result: "u32"
  },
  AddFunction: {
    name: "LLVMAddFunction",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetNamedFunction: {
    name: "LLVMGetNamedFunction",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetFirstFunction: {
    name: "LLVMGetFirstFunction",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetLastFunction: {
    name: "LLVMGetLastFunction",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNextFunction: {
    name: "LLVMGetNextFunction",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPreviousFunction: {
    name: "LLVMGetPreviousFunction",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetModuleInlineAsm: {
    name: "LLVMSetModuleInlineAsm",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetTypeKind: {
    name: "LLVMGetTypeKind",
    parameters: ["pointer"],
    result: "pointer"
  },
  TypeIsSized: {
    name: "LLVMTypeIsSized",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetTypeContext: {
    name: "LLVMGetTypeContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  DumpType: {
    name: "LLVMDumpType",
    parameters: ["pointer"],
    result: "void"
  },
  PrintTypeToString: {
    name: "LLVMPrintTypeToString",
    parameters: ["pointer"],
    result: "pointer"
  },
  Int1TypeInContext: {
    name: "LLVMInt1TypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  Int8TypeInContext: {
    name: "LLVMInt8TypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  Int16TypeInContext: {
    name: "LLVMInt16TypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  Int32TypeInContext: {
    name: "LLVMInt32TypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  Int64TypeInContext: {
    name: "LLVMInt64TypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  Int128TypeInContext: {
    name: "LLVMInt128TypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  IntTypeInContext: {
    name: "LLVMIntTypeInContext",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  Int1Type: {
    name: "LLVMInt1Type",
    parameters: [],
    result: "pointer"
  },
  Int8Type: {
    name: "LLVMInt8Type",
    parameters: [],
    result: "pointer"
  },
  Int16Type: {
    name: "LLVMInt16Type",
    parameters: [],
    result: "pointer"
  },
  Int32Type: {
    name: "LLVMInt32Type",
    parameters: [],
    result: "pointer"
  },
  Int64Type: {
    name: "LLVMInt64Type",
    parameters: [],
    result: "pointer"
  },
  Int128Type: {
    name: "LLVMInt128Type",
    parameters: [],
    result: "pointer"
  },
  IntType: {
    name: "LLVMIntType",
    parameters: ["u32"],
    result: "pointer"
  },
  GetIntTypeWidth: {
    name: "LLVMGetIntTypeWidth",
    parameters: ["pointer"],
    result: "u32"
  },
  HalfTypeInContext: {
    name: "LLVMHalfTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  BFloatTypeInContext: {
    name: "LLVMBFloatTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  FloatTypeInContext: {
    name: "LLVMFloatTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  DoubleTypeInContext: {
    name: "LLVMDoubleTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  X86FP80TypeInContext: {
    name: "LLVMX86FP80TypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  FP128TypeInContext: {
    name: "LLVMFP128TypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  PPCFP128TypeInContext: {
    name: "LLVMPPCFP128TypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  HalfType: {
    name: "LLVMHalfType",
    parameters: [],
    result: "pointer"
  },
  BFloatType: {
    name: "LLVMBFloatType",
    parameters: [],
    result: "pointer"
  },
  FloatType: {
    name: "LLVMFloatType",
    parameters: [],
    result: "pointer"
  },
  DoubleType: {
    name: "LLVMDoubleType",
    parameters: [],
    result: "pointer"
  },
  X86FP80Type: {
    name: "LLVMX86FP80Type",
    parameters: [],
    result: "pointer"
  },
  FP128Type: {
    name: "LLVMFP128Type",
    parameters: [],
    result: "pointer"
  },
  PPCFP128Type: {
    name: "LLVMPPCFP128Type",
    parameters: [],
    result: "pointer"
  },
  FunctionType: {
    name: "LLVMFunctionType",
    parameters: ["pointer", "pointer", "u32", "i32"],
    result: "pointer"
  },
  IsFunctionVarArg: {
    name: "LLVMIsFunctionVarArg",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetReturnType: {
    name: "LLVMGetReturnType",
    parameters: ["pointer"],
    result: "pointer"
  },
  CountParamTypes: {
    name: "LLVMCountParamTypes",
    parameters: ["pointer"],
    result: "u32"
  },
  GetParamTypes: {
    name: "LLVMGetParamTypes",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  StructTypeInContext: {
    name: "LLVMStructTypeInContext",
    parameters: ["pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  StructType: {
    name: "LLVMStructType",
    parameters: ["pointer", "u32", "pointer"],
    result: "pointer"
  },
  StructCreateNamed: {
    name: "LLVMStructCreateNamed",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetStructName: {
    name: "LLVMGetStructName",
    parameters: ["pointer"],
    result: "pointer"
  },
  StructSetBody: {
    name: "LLVMStructSetBody",
    parameters: ["pointer", "pointer", "u32", "pointer"],
    result: "void"
  },
  CountStructElementTypes: {
    name: "LLVMCountStructElementTypes",
    parameters: ["pointer"],
    result: "u32"
  },
  GetStructElementTypes: {
    name: "LLVMGetStructElementTypes",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  StructGetTypeAtIndex: {
    name: "LLVMStructGetTypeAtIndex",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  IsPackedStruct: {
    name: "LLVMIsPackedStruct",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsOpaqueStruct: {
    name: "LLVMIsOpaqueStruct",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsLiteralStruct: {
    name: "LLVMIsLiteralStruct",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetElementType: {
    name: "LLVMGetElementType",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetSubtypes: {
    name: "LLVMGetSubtypes",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetNumContainedTypes: {
    name: "LLVMGetNumContainedTypes",
    parameters: ["pointer"],
    result: "u32"
  },
  ArrayType: {
    name: "LLVMArrayType",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetArrayLength: {
    name: "LLVMGetArrayLength",
    parameters: ["pointer"],
    result: "u32"
  },
  PointerType: {
    name: "LLVMPointerType",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  PointerTypeIsOpaque: {
    name: "LLVMPointerTypeIsOpaque",
    parameters: ["pointer"],
    result: "pointer"
  },
  PointerTypeInContext: {
    name: "LLVMPointerTypeInContext",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetPointerAddressSpace: {
    name: "LLVMGetPointerAddressSpace",
    parameters: ["pointer"],
    result: "u32"
  },
  VectorType: {
    name: "LLVMVectorType",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  ScalableVectorType: {
    name: "LLVMScalableVectorType",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetVectorSize: {
    name: "LLVMGetVectorSize",
    parameters: ["pointer"],
    result: "u32"
  },
  VoidTypeInContext: {
    name: "LLVMVoidTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  LabelTypeInContext: {
    name: "LLVMLabelTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  X86MMXTypeInContext: {
    name: "LLVMX86MMXTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  X86AMXTypeInContext: {
    name: "LLVMX86AMXTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  TokenTypeInContext: {
    name: "LLVMTokenTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  MetadataTypeInContext: {
    name: "LLVMMetadataTypeInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  VoidType: {
    name: "LLVMVoidType",
    parameters: [],
    result: "pointer"
  },
  LabelType: {
    name: "LLVMLabelType",
    parameters: [],
    result: "pointer"
  },
  X86MMXType: {
    name: "LLVMX86MMXType",
    parameters: [],
    result: "pointer"
  },
  X86AMXType: {
    name: "LLVMX86AMXType",
    parameters: [],
    result: "pointer"
  },
  TypeOf: {
    name: "LLVMTypeOf",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetValueKind: {
    name: "LLVMGetValueKind",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetValueName2: {
    name: "LLVMGetValueName2",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  SetValueName2: {
    name: "LLVMSetValueName2",
    parameters: ["pointer", "pointer", "u64"],
    result: "void"
  },
  DumpValue: {
    name: "LLVMDumpValue",
    parameters: ["pointer"],
    result: "void"
  },
  PrintValueToString: {
    name: "LLVMPrintValueToString",
    parameters: ["pointer"],
    result: "pointer"
  },
  ReplaceAllUsesWith: {
    name: "LLVMReplaceAllUsesWith",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  IsConstant: {
    name: "LLVMIsConstant",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsUndef: {
    name: "LLVMIsUndef",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsPoison: {
    name: "LLVMIsPoison",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAArgument: {
    name: "LLVMIsAArgument",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsABasicBlock: {
    name: "LLVMIsABasicBlock",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAInlineAsm: {
    name: "LLVMIsAInlineAsm",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAUser: {
    name: "LLVMIsAUser",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstant: {
    name: "LLVMIsAConstant",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsABlockAddress: {
    name: "LLVMIsABlockAddress",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantAggregateZero: {
    name: "LLVMIsAConstantAggregateZero",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantArray: {
    name: "LLVMIsAConstantArray",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantDataSequential: {
    name: "LLVMIsAConstantDataSequential",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantDataArray: {
    name: "LLVMIsAConstantDataArray",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantDataVector: {
    name: "LLVMIsAConstantDataVector",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantExpr: {
    name: "LLVMIsAConstantExpr",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantFP: {
    name: "LLVMIsAConstantFP",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantInt: {
    name: "LLVMIsAConstantInt",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantPointerNull: {
    name: "LLVMIsAConstantPointerNull",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantStruct: {
    name: "LLVMIsAConstantStruct",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantTokenNone: {
    name: "LLVMIsAConstantTokenNone",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAConstantVector: {
    name: "LLVMIsAConstantVector",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAGlobalValue: {
    name: "LLVMIsAGlobalValue",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAGlobalAlias: {
    name: "LLVMIsAGlobalAlias",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAGlobalObject: {
    name: "LLVMIsAGlobalObject",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAFunction: {
    name: "LLVMIsAFunction",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAGlobalVariable: {
    name: "LLVMIsAGlobalVariable",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAGlobalIFunc: {
    name: "LLVMIsAGlobalIFunc",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAUndefValue: {
    name: "LLVMIsAUndefValue",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAPoisonValue: {
    name: "LLVMIsAPoisonValue",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAInstruction: {
    name: "LLVMIsAInstruction",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAUnaryOperator: {
    name: "LLVMIsAUnaryOperator",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsABinaryOperator: {
    name: "LLVMIsABinaryOperator",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsACallInst: {
    name: "LLVMIsACallInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAIntrinsicInst: {
    name: "LLVMIsAIntrinsicInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsADbgInfoIntrinsic: {
    name: "LLVMIsADbgInfoIntrinsic",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsADbgVariableIntrinsic: {
    name: "LLVMIsADbgVariableIntrinsic",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsADbgDeclareInst: {
    name: "LLVMIsADbgDeclareInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsADbgLabelInst: {
    name: "LLVMIsADbgLabelInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAMemIntrinsic: {
    name: "LLVMIsAMemIntrinsic",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAMemCpyInst: {
    name: "LLVMIsAMemCpyInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAMemMoveInst: {
    name: "LLVMIsAMemMoveInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAMemSetInst: {
    name: "LLVMIsAMemSetInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsACmpInst: {
    name: "LLVMIsACmpInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAFCmpInst: {
    name: "LLVMIsAFCmpInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAICmpInst: {
    name: "LLVMIsAICmpInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAExtractElementInst: {
    name: "LLVMIsAExtractElementInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAGetElementPtrInst: {
    name: "LLVMIsAGetElementPtrInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAInsertElementInst: {
    name: "LLVMIsAInsertElementInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAInsertValueInst: {
    name: "LLVMIsAInsertValueInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsALandingPadInst: {
    name: "LLVMIsALandingPadInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAPHINode: {
    name: "LLVMIsAPHINode",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsASelectInst: {
    name: "LLVMIsASelectInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAShuffleVectorInst: {
    name: "LLVMIsAShuffleVectorInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAStoreInst: {
    name: "LLVMIsAStoreInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsABranchInst: {
    name: "LLVMIsABranchInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAIndirectBrInst: {
    name: "LLVMIsAIndirectBrInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAInvokeInst: {
    name: "LLVMIsAInvokeInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAReturnInst: {
    name: "LLVMIsAReturnInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsASwitchInst: {
    name: "LLVMIsASwitchInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAUnreachableInst: {
    name: "LLVMIsAUnreachableInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAResumeInst: {
    name: "LLVMIsAResumeInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsACleanupReturnInst: {
    name: "LLVMIsACleanupReturnInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsACatchReturnInst: {
    name: "LLVMIsACatchReturnInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsACatchSwitchInst: {
    name: "LLVMIsACatchSwitchInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsACallBrInst: {
    name: "LLVMIsACallBrInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAFuncletPadInst: {
    name: "LLVMIsAFuncletPadInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsACatchPadInst: {
    name: "LLVMIsACatchPadInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsACleanupPadInst: {
    name: "LLVMIsACleanupPadInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAUnaryInstruction: {
    name: "LLVMIsAUnaryInstruction",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAAllocaInst: {
    name: "LLVMIsAAllocaInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsACastInst: {
    name: "LLVMIsACastInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAAddrSpaceCastInst: {
    name: "LLVMIsAAddrSpaceCastInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsABitCastInst: {
    name: "LLVMIsABitCastInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAFPExtInst: {
    name: "LLVMIsAFPExtInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAFPToSIInst: {
    name: "LLVMIsAFPToSIInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAFPToUIInst: {
    name: "LLVMIsAFPToUIInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAFPTruncInst: {
    name: "LLVMIsAFPTruncInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAIntToPtrInst: {
    name: "LLVMIsAIntToPtrInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAPtrToIntInst: {
    name: "LLVMIsAPtrToIntInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsASExtInst: {
    name: "LLVMIsASExtInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsASIToFPInst: {
    name: "LLVMIsASIToFPInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsATruncInst: {
    name: "LLVMIsATruncInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAUIToFPInst: {
    name: "LLVMIsAUIToFPInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAZExtInst: {
    name: "LLVMIsAZExtInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAExtractValueInst: {
    name: "LLVMIsAExtractValueInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsALoadInst: {
    name: "LLVMIsALoadInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAVAArgInst: {
    name: "LLVMIsAVAArgInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAFreezeInst: {
    name: "LLVMIsAFreezeInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAAtomicCmpXchgInst: {
    name: "LLVMIsAAtomicCmpXchgInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAAtomicRMWInst: {
    name: "LLVMIsAAtomicRMWInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAFenceInst: {
    name: "LLVMIsAFenceInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAMDNode: {
    name: "LLVMIsAMDNode",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsAMDString: {
    name: "LLVMIsAMDString",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetValueName: {
    name: "LLVMGetValueName",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetValueName: {
    name: "LLVMSetValueName",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetFirstUse: {
    name: "LLVMGetFirstUse",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNextUse: {
    name: "LLVMGetNextUse",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetUser: {
    name: "LLVMGetUser",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetUsedValue: {
    name: "LLVMGetUsedValue",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetOperand: {
    name: "LLVMGetOperand",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetOperandUse: {
    name: "LLVMGetOperandUse",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  SetOperand: {
    name: "LLVMSetOperand",
    parameters: ["pointer", "u32", "pointer"],
    result: "void"
  },
  GetNumOperands: {
    name: "LLVMGetNumOperands",
    parameters: ["pointer"],
    result: "i32"
  },
  ConstNull: {
    name: "LLVMConstNull",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConstAllOnes: {
    name: "LLVMConstAllOnes",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetUndef: {
    name: "LLVMGetUndef",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPoison: {
    name: "LLVMGetPoison",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsNull: {
    name: "LLVMIsNull",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConstPointerNull: {
    name: "LLVMConstPointerNull",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConstInt: {
    name: "LLVMConstInt",
    parameters: ["pointer", "u64", "i32"],
    result: "pointer"
  },
  ConstIntOfArbitraryPrecision: {
    name: "LLVMConstIntOfArbitraryPrecision",
    parameters: ["pointer", "u32", "pointer"],
    result: "pointer"
  },
  ConstIntOfString: {
    name: "LLVMConstIntOfString",
    parameters: ["pointer", "pointer", "u8"],
    result: "pointer"
  },
  ConstIntOfStringAndSize: {
    name: "LLVMConstIntOfStringAndSize",
    parameters: ["pointer", "pointer", "u32", "u8"],
    result: "pointer"
  },
  ConstReal: {
    name: "LLVMConstReal",
    parameters: ["pointer", "f64"],
    result: "pointer"
  },
  ConstRealOfString: {
    name: "LLVMConstRealOfString",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstRealOfStringAndSize: {
    name: "LLVMConstRealOfStringAndSize",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  ConstIntGetZExtValue: {
    name: "LLVMConstIntGetZExtValue",
    parameters: ["pointer"],
    result: "u64"
  },
  ConstIntGetSExtValue: {
    name: "LLVMConstIntGetSExtValue",
    parameters: ["pointer"],
    result: "i64"
  },
  ConstRealGetDouble: {
    name: "LLVMConstRealGetDouble",
    parameters: ["pointer", "pointer"],
    result: "f64"
  },
  ConstStringInContext: {
    name: "LLVMConstStringInContext",
    parameters: ["pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  ConstString: {
    name: "LLVMConstString",
    parameters: ["pointer", "u32", "pointer"],
    result: "pointer"
  },
  IsConstantString: {
    name: "LLVMIsConstantString",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetAsString: {
    name: "LLVMGetAsString",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstStructInContext: {
    name: "LLVMConstStructInContext",
    parameters: ["pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  ConstStruct: {
    name: "LLVMConstStruct",
    parameters: ["pointer", "u32", "pointer"],
    result: "pointer"
  },
  ConstArray: {
    name: "LLVMConstArray",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  ConstNamedStruct: {
    name: "LLVMConstNamedStruct",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  GetAggregateElement: {
    name: "LLVMGetAggregateElement",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetElementAsConstant: {
    name: "LLVMGetElementAsConstant",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  ConstVector: {
    name: "LLVMConstVector",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetConstOpcode: {
    name: "LLVMGetConstOpcode",
    parameters: ["pointer"],
    result: "pointer"
  },
  AlignOf: {
    name: "LLVMAlignOf",
    parameters: ["pointer"],
    result: "pointer"
  },
  SizeOf: {
    name: "LLVMSizeOf",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConstNeg: {
    name: "LLVMConstNeg",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConstNSWNeg: {
    name: "LLVMConstNSWNeg",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConstNUWNeg: {
    name: "LLVMConstNUWNeg",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConstFNeg: {
    name: "LLVMConstFNeg",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConstNot: {
    name: "LLVMConstNot",
    parameters: ["pointer"],
    result: "pointer"
  },
  ConstAdd: {
    name: "LLVMConstAdd",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstNSWAdd: {
    name: "LLVMConstNSWAdd",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstNUWAdd: {
    name: "LLVMConstNUWAdd",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstSub: {
    name: "LLVMConstSub",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstNSWSub: {
    name: "LLVMConstNSWSub",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstNUWSub: {
    name: "LLVMConstNUWSub",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstMul: {
    name: "LLVMConstMul",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstNSWMul: {
    name: "LLVMConstNSWMul",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstNUWMul: {
    name: "LLVMConstNUWMul",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstAnd: {
    name: "LLVMConstAnd",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstOr: {
    name: "LLVMConstOr",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstXor: {
    name: "LLVMConstXor",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstICmp: {
    name: "LLVMConstICmp",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  ConstFCmp: {
    name: "LLVMConstFCmp",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  ConstShl: {
    name: "LLVMConstShl",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstLShr: {
    name: "LLVMConstLShr",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstAShr: {
    name: "LLVMConstAShr",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstGEP: {
    name: "LLVMConstGEP",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  ConstGEP2: {
    name: "LLVMConstGEP2",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  ConstInBoundsGEP: {
    name: "LLVMConstInBoundsGEP",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  ConstInBoundsGEP2: {
    name: "LLVMConstInBoundsGEP2",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  ConstTrunc: {
    name: "LLVMConstTrunc",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstSExt: {
    name: "LLVMConstSExt",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstZExt: {
    name: "LLVMConstZExt",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstFPTrunc: {
    name: "LLVMConstFPTrunc",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstFPExt: {
    name: "LLVMConstFPExt",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstUIToFP: {
    name: "LLVMConstUIToFP",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstSIToFP: {
    name: "LLVMConstSIToFP",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstFPToUI: {
    name: "LLVMConstFPToUI",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstFPToSI: {
    name: "LLVMConstFPToSI",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstPtrToInt: {
    name: "LLVMConstPtrToInt",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstIntToPtr: {
    name: "LLVMConstIntToPtr",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstBitCast: {
    name: "LLVMConstBitCast",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstAddrSpaceCast: {
    name: "LLVMConstAddrSpaceCast",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstZExtOrBitCast: {
    name: "LLVMConstZExtOrBitCast",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstSExtOrBitCast: {
    name: "LLVMConstSExtOrBitCast",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstTruncOrBitCast: {
    name: "LLVMConstTruncOrBitCast",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstPointerCast: {
    name: "LLVMConstPointerCast",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstIntCast: {
    name: "LLVMConstIntCast",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  ConstFPCast: {
    name: "LLVMConstFPCast",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstSelect: {
    name: "LLVMConstSelect",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  ConstExtractElement: {
    name: "LLVMConstExtractElement",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstInsertElement: {
    name: "LLVMConstInsertElement",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  ConstShuffleVector: {
    name: "LLVMConstShuffleVector",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BlockAddress: {
    name: "LLVMBlockAddress",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ConstInlineAsm: {
    name: "LLVMConstInlineAsm",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetGlobalParent: {
    name: "LLVMGetGlobalParent",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsDeclaration: {
    name: "LLVMIsDeclaration",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetLinkage: {
    name: "LLVMGetLinkage",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetLinkage: {
    name: "LLVMSetLinkage",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetSection: {
    name: "LLVMGetSection",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetSection: {
    name: "LLVMSetSection",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetVisibility: {
    name: "LLVMGetVisibility",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetVisibility: {
    name: "LLVMSetVisibility",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetDLLStorageClass: {
    name: "LLVMGetDLLStorageClass",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetDLLStorageClass: {
    name: "LLVMSetDLLStorageClass",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetUnnamedAddress: {
    name: "LLVMGetUnnamedAddress",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetUnnamedAddress: {
    name: "LLVMSetUnnamedAddress",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GlobalGetValueType: {
    name: "LLVMGlobalGetValueType",
    parameters: ["pointer"],
    result: "pointer"
  },
  HasUnnamedAddr: {
    name: "LLVMHasUnnamedAddr",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetUnnamedAddr: {
    name: "LLVMSetUnnamedAddr",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetAlignment: {
    name: "LLVMGetAlignment",
    parameters: ["pointer"],
    result: "u32"
  },
  SetAlignment: {
    name: "LLVMSetAlignment",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  GlobalSetMetadata: {
    name: "LLVMGlobalSetMetadata",
    parameters: ["pointer", "u32", "pointer"],
    result: "void"
  },
  GlobalEraseMetadata: {
    name: "LLVMGlobalEraseMetadata",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  GlobalClearMetadata: {
    name: "LLVMGlobalClearMetadata",
    parameters: ["pointer"],
    result: "void"
  },
  GlobalCopyAllMetadata: {
    name: "LLVMGlobalCopyAllMetadata",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DisposeValueMetadataEntries: {
    name: "LLVMDisposeValueMetadataEntries",
    parameters: ["pointer"],
    result: "void"
  },
  ValueMetadataEntriesGetKind: {
    name: "LLVMValueMetadataEntriesGetKind",
    parameters: ["pointer", "u32"],
    result: "u32"
  },
  ValueMetadataEntriesGetMetadata: {
    name: "LLVMValueMetadataEntriesGetMetadata",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  AddGlobal: {
    name: "LLVMAddGlobal",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  AddGlobalInAddressSpace: {
    name: "LLVMAddGlobalInAddressSpace",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  GetNamedGlobal: {
    name: "LLVMGetNamedGlobal",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetFirstGlobal: {
    name: "LLVMGetFirstGlobal",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetLastGlobal: {
    name: "LLVMGetLastGlobal",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNextGlobal: {
    name: "LLVMGetNextGlobal",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPreviousGlobal: {
    name: "LLVMGetPreviousGlobal",
    parameters: ["pointer"],
    result: "pointer"
  },
  DeleteGlobal: {
    name: "LLVMDeleteGlobal",
    parameters: ["pointer"],
    result: "void"
  },
  GetInitializer: {
    name: "LLVMGetInitializer",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetInitializer: {
    name: "LLVMSetInitializer",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  IsThreadLocal: {
    name: "LLVMIsThreadLocal",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetThreadLocal: {
    name: "LLVMSetThreadLocal",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  IsGlobalConstant: {
    name: "LLVMIsGlobalConstant",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetGlobalConstant: {
    name: "LLVMSetGlobalConstant",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetThreadLocalMode: {
    name: "LLVMGetThreadLocalMode",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetThreadLocalMode: {
    name: "LLVMSetThreadLocalMode",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  IsExternallyInitialized: {
    name: "LLVMIsExternallyInitialized",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetExternallyInitialized: {
    name: "LLVMSetExternallyInitialized",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  AddAlias: {
    name: "LLVMAddAlias",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  AddAlias2: {
    name: "LLVMAddAlias2",
    parameters: ["pointer", "pointer", "u32", "pointer", "pointer"],
    result: "pointer"
  },
  GetNamedGlobalAlias: {
    name: "LLVMGetNamedGlobalAlias",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  GetFirstGlobalAlias: {
    name: "LLVMGetFirstGlobalAlias",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetLastGlobalAlias: {
    name: "LLVMGetLastGlobalAlias",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNextGlobalAlias: {
    name: "LLVMGetNextGlobalAlias",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPreviousGlobalAlias: {
    name: "LLVMGetPreviousGlobalAlias",
    parameters: ["pointer"],
    result: "pointer"
  },
  AliasGetAliasee: {
    name: "LLVMAliasGetAliasee",
    parameters: ["pointer"],
    result: "pointer"
  },
  AliasSetAliasee: {
    name: "LLVMAliasSetAliasee",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  DeleteFunction: {
    name: "LLVMDeleteFunction",
    parameters: ["pointer"],
    result: "void"
  },
  HasPersonalityFn: {
    name: "LLVMHasPersonalityFn",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPersonalityFn: {
    name: "LLVMGetPersonalityFn",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetPersonalityFn: {
    name: "LLVMSetPersonalityFn",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  LookupIntrinsicID: {
    name: "LLVMLookupIntrinsicID",
    parameters: ["pointer", "u64"],
    result: "u32"
  },
  GetIntrinsicID: {
    name: "LLVMGetIntrinsicID",
    parameters: ["pointer"],
    result: "u32"
  },
  GetIntrinsicDeclaration: {
    name: "LLVMGetIntrinsicDeclaration",
    parameters: ["pointer", "u32", "pointer", "u64"],
    result: "pointer"
  },
  IntrinsicGetType: {
    name: "LLVMIntrinsicGetType",
    parameters: ["pointer", "u32", "pointer", "u64"],
    result: "pointer"
  },
  IntrinsicGetName: {
    name: "LLVMIntrinsicGetName",
    parameters: ["u32", "pointer"],
    result: "pointer"
  },
  IntrinsicCopyOverloadedName: {
    name: "LLVMIntrinsicCopyOverloadedName",
    parameters: ["u32", "pointer", "u64", "pointer"],
    result: "pointer"
  },
  IntrinsicCopyOverloadedName2: {
    name: "LLVMIntrinsicCopyOverloadedName2",
    parameters: ["pointer", "u32", "pointer", "u64", "pointer"],
    result: "pointer"
  },
  IntrinsicIsOverloaded: {
    name: "LLVMIntrinsicIsOverloaded",
    parameters: ["u32"],
    result: "pointer"
  },
  GetFunctionCallConv: {
    name: "LLVMGetFunctionCallConv",
    parameters: ["pointer"],
    result: "u32"
  },
  SetFunctionCallConv: {
    name: "LLVMSetFunctionCallConv",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  GetGC: {
    name: "LLVMGetGC",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetGC: {
    name: "LLVMSetGC",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  AddAttributeAtIndex: {
    name: "LLVMAddAttributeAtIndex",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  GetAttributeCountAtIndex: {
    name: "LLVMGetAttributeCountAtIndex",
    parameters: ["pointer", "pointer"],
    result: "u32"
  },
  GetAttributesAtIndex: {
    name: "LLVMGetAttributesAtIndex",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  GetEnumAttributeAtIndex: {
    name: "LLVMGetEnumAttributeAtIndex",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  GetStringAttributeAtIndex: {
    name: "LLVMGetStringAttributeAtIndex",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  RemoveEnumAttributeAtIndex: {
    name: "LLVMRemoveEnumAttributeAtIndex",
    parameters: ["pointer", "pointer", "u32"],
    result: "void"
  },
  RemoveStringAttributeAtIndex: {
    name: "LLVMRemoveStringAttributeAtIndex",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "void"
  },
  AddTargetDependentFunctionAttr: {
    name: "LLVMAddTargetDependentFunctionAttr",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  CountParams: {
    name: "LLVMCountParams",
    parameters: ["pointer"],
    result: "u32"
  },
  GetParams: {
    name: "LLVMGetParams",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetParam: {
    name: "LLVMGetParam",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetParamParent: {
    name: "LLVMGetParamParent",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetFirstParam: {
    name: "LLVMGetFirstParam",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetLastParam: {
    name: "LLVMGetLastParam",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNextParam: {
    name: "LLVMGetNextParam",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPreviousParam: {
    name: "LLVMGetPreviousParam",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetParamAlignment: {
    name: "LLVMSetParamAlignment",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  AddGlobalIFunc: {
    name: "LLVMAddGlobalIFunc",
    parameters: ["pointer", "pointer", "u64", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  GetNamedGlobalIFunc: {
    name: "LLVMGetNamedGlobalIFunc",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  GetFirstGlobalIFunc: {
    name: "LLVMGetFirstGlobalIFunc",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetLastGlobalIFunc: {
    name: "LLVMGetLastGlobalIFunc",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNextGlobalIFunc: {
    name: "LLVMGetNextGlobalIFunc",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPreviousGlobalIFunc: {
    name: "LLVMGetPreviousGlobalIFunc",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetGlobalIFuncResolver: {
    name: "LLVMGetGlobalIFuncResolver",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetGlobalIFuncResolver: {
    name: "LLVMSetGlobalIFuncResolver",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  EraseGlobalIFunc: {
    name: "LLVMEraseGlobalIFunc",
    parameters: ["pointer"],
    result: "void"
  },
  RemoveGlobalIFunc: {
    name: "LLVMRemoveGlobalIFunc",
    parameters: ["pointer"],
    result: "void"
  },
  MDStringInContext2: {
    name: "LLVMMDStringInContext2",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  MDNodeInContext2: {
    name: "LLVMMDNodeInContext2",
    parameters: ["pointer", "pointer", "u64"],
    result: "pointer"
  },
  MetadataAsValue: {
    name: "LLVMMetadataAsValue",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ValueAsMetadata: {
    name: "LLVMValueAsMetadata",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetMDString: {
    name: "LLVMGetMDString",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetMDNodeNumOperands: {
    name: "LLVMGetMDNodeNumOperands",
    parameters: ["pointer"],
    result: "u32"
  },
  GetMDNodeOperands: {
    name: "LLVMGetMDNodeOperands",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  MDStringInContext: {
    name: "LLVMMDStringInContext",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  MDString: {
    name: "LLVMMDString",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  MDNodeInContext: {
    name: "LLVMMDNodeInContext",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  MDNode: {
    name: "LLVMMDNode",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  BasicBlockAsValue: {
    name: "LLVMBasicBlockAsValue",
    parameters: ["pointer"],
    result: "pointer"
  },
  ValueIsBasicBlock: {
    name: "LLVMValueIsBasicBlock",
    parameters: ["pointer"],
    result: "pointer"
  },
  ValueAsBasicBlock: {
    name: "LLVMValueAsBasicBlock",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetBasicBlockName: {
    name: "LLVMGetBasicBlockName",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetBasicBlockParent: {
    name: "LLVMGetBasicBlockParent",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetBasicBlockTerminator: {
    name: "LLVMGetBasicBlockTerminator",
    parameters: ["pointer"],
    result: "pointer"
  },
  CountBasicBlocks: {
    name: "LLVMCountBasicBlocks",
    parameters: ["pointer"],
    result: "u32"
  },
  GetBasicBlocks: {
    name: "LLVMGetBasicBlocks",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetFirstBasicBlock: {
    name: "LLVMGetFirstBasicBlock",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetLastBasicBlock: {
    name: "LLVMGetLastBasicBlock",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNextBasicBlock: {
    name: "LLVMGetNextBasicBlock",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPreviousBasicBlock: {
    name: "LLVMGetPreviousBasicBlock",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetEntryBasicBlock: {
    name: "LLVMGetEntryBasicBlock",
    parameters: ["pointer"],
    result: "pointer"
  },
  InsertExistingBasicBlockAfterInsertBlock: {
    name: "LLVMInsertExistingBasicBlockAfterInsertBlock",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  AppendExistingBasicBlock: {
    name: "LLVMAppendExistingBasicBlock",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  CreateBasicBlockInContext: {
    name: "LLVMCreateBasicBlockInContext",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  AppendBasicBlockInContext: {
    name: "LLVMAppendBasicBlockInContext",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  AppendBasicBlock: {
    name: "LLVMAppendBasicBlock",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  InsertBasicBlockInContext: {
    name: "LLVMInsertBasicBlockInContext",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  InsertBasicBlock: {
    name: "LLVMInsertBasicBlock",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DeleteBasicBlock: {
    name: "LLVMDeleteBasicBlock",
    parameters: ["pointer"],
    result: "void"
  },
  RemoveBasicBlockFromParent: {
    name: "LLVMRemoveBasicBlockFromParent",
    parameters: ["pointer"],
    result: "void"
  },
  MoveBasicBlockBefore: {
    name: "LLVMMoveBasicBlockBefore",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  MoveBasicBlockAfter: {
    name: "LLVMMoveBasicBlockAfter",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetFirstInstruction: {
    name: "LLVMGetFirstInstruction",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetLastInstruction: {
    name: "LLVMGetLastInstruction",
    parameters: ["pointer"],
    result: "pointer"
  },
  HasMetadata: {
    name: "LLVMHasMetadata",
    parameters: ["pointer"],
    result: "i32"
  },
  GetMetadata: {
    name: "LLVMGetMetadata",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  SetMetadata: {
    name: "LLVMSetMetadata",
    parameters: ["pointer", "u32", "pointer"],
    result: "void"
  },
  InstructionGetAllMetadataOtherThanDebugLoc: {
    name: "LLVMInstructionGetAllMetadataOtherThanDebugLoc",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetInstructionParent: {
    name: "LLVMGetInstructionParent",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNextInstruction: {
    name: "LLVMGetNextInstruction",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetPreviousInstruction: {
    name: "LLVMGetPreviousInstruction",
    parameters: ["pointer"],
    result: "pointer"
  },
  InstructionRemoveFromParent: {
    name: "LLVMInstructionRemoveFromParent",
    parameters: ["pointer"],
    result: "void"
  },
  InstructionEraseFromParent: {
    name: "LLVMInstructionEraseFromParent",
    parameters: ["pointer"],
    result: "void"
  },
  DeleteInstruction: {
    name: "LLVMDeleteInstruction",
    parameters: ["pointer"],
    result: "void"
  },
  GetInstructionOpcode: {
    name: "LLVMGetInstructionOpcode",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetICmpPredicate: {
    name: "LLVMGetICmpPredicate",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetFCmpPredicate: {
    name: "LLVMGetFCmpPredicate",
    parameters: ["pointer"],
    result: "pointer"
  },
  InstructionClone: {
    name: "LLVMInstructionClone",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsATerminatorInst: {
    name: "LLVMIsATerminatorInst",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetNumArgOperands: {
    name: "LLVMGetNumArgOperands",
    parameters: ["pointer"],
    result: "u32"
  },
  SetInstructionCallConv: {
    name: "LLVMSetInstructionCallConv",
    parameters: ["pointer", "u32"],
    result: "void"
  },
  GetInstructionCallConv: {
    name: "LLVMGetInstructionCallConv",
    parameters: ["pointer"],
    result: "u32"
  },
  SetInstrParamAlignment: {
    name: "LLVMSetInstrParamAlignment",
    parameters: ["pointer", "pointer", "u32"],
    result: "void"
  },
  AddCallSiteAttribute: {
    name: "LLVMAddCallSiteAttribute",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  GetCallSiteAttributeCount: {
    name: "LLVMGetCallSiteAttributeCount",
    parameters: ["pointer", "pointer"],
    result: "u32"
  },
  GetCallSiteAttributes: {
    name: "LLVMGetCallSiteAttributes",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  GetCallSiteEnumAttribute: {
    name: "LLVMGetCallSiteEnumAttribute",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  GetCallSiteStringAttribute: {
    name: "LLVMGetCallSiteStringAttribute",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  RemoveCallSiteEnumAttribute: {
    name: "LLVMRemoveCallSiteEnumAttribute",
    parameters: ["pointer", "pointer", "u32"],
    result: "void"
  },
  RemoveCallSiteStringAttribute: {
    name: "LLVMRemoveCallSiteStringAttribute",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "void"
  },
  GetCalledFunctionType: {
    name: "LLVMGetCalledFunctionType",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetCalledValue: {
    name: "LLVMGetCalledValue",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsTailCall: {
    name: "LLVMIsTailCall",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetTailCall: {
    name: "LLVMSetTailCall",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetNormalDest: {
    name: "LLVMGetNormalDest",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetUnwindDest: {
    name: "LLVMGetUnwindDest",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetNormalDest: {
    name: "LLVMSetNormalDest",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  SetUnwindDest: {
    name: "LLVMSetUnwindDest",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetNumSuccessors: {
    name: "LLVMGetNumSuccessors",
    parameters: ["pointer"],
    result: "u32"
  },
  GetSuccessor: {
    name: "LLVMGetSuccessor",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  SetSuccessor: {
    name: "LLVMSetSuccessor",
    parameters: ["pointer", "u32", "pointer"],
    result: "void"
  },
  IsConditional: {
    name: "LLVMIsConditional",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetCondition: {
    name: "LLVMGetCondition",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetCondition: {
    name: "LLVMSetCondition",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetSwitchDefaultDest: {
    name: "LLVMGetSwitchDefaultDest",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetAllocatedType: {
    name: "LLVMGetAllocatedType",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsInBounds: {
    name: "LLVMIsInBounds",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetIsInBounds: {
    name: "LLVMSetIsInBounds",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetGEPSourceElementType: {
    name: "LLVMGetGEPSourceElementType",
    parameters: ["pointer"],
    result: "pointer"
  },
  AddIncoming: {
    name: "LLVMAddIncoming",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "void"
  },
  CountIncoming: {
    name: "LLVMCountIncoming",
    parameters: ["pointer"],
    result: "u32"
  },
  GetIncomingValue: {
    name: "LLVMGetIncomingValue",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetIncomingBlock: {
    name: "LLVMGetIncomingBlock",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  GetNumIndices: {
    name: "LLVMGetNumIndices",
    parameters: ["pointer"],
    result: "u32"
  },
  GetIndices: {
    name: "LLVMGetIndices",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateBuilderInContext: {
    name: "LLVMCreateBuilderInContext",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateBuilder: {
    name: "LLVMCreateBuilder",
    parameters: [],
    result: "pointer"
  },
  PositionBuilder: {
    name: "LLVMPositionBuilder",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  PositionBuilderBefore: {
    name: "LLVMPositionBuilderBefore",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  PositionBuilderAtEnd: {
    name: "LLVMPositionBuilderAtEnd",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetInsertBlock: {
    name: "LLVMGetInsertBlock",
    parameters: ["pointer"],
    result: "pointer"
  },
  ClearInsertionPosition: {
    name: "LLVMClearInsertionPosition",
    parameters: ["pointer"],
    result: "void"
  },
  InsertIntoBuilder: {
    name: "LLVMInsertIntoBuilder",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  InsertIntoBuilderWithName: {
    name: "LLVMInsertIntoBuilderWithName",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  DisposeBuilder: {
    name: "LLVMDisposeBuilder",
    parameters: ["pointer"],
    result: "void"
  },
  GetCurrentDebugLocation2: {
    name: "LLVMGetCurrentDebugLocation2",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetCurrentDebugLocation2: {
    name: "LLVMSetCurrentDebugLocation2",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  SetInstDebugLocation: {
    name: "LLVMSetInstDebugLocation",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  AddMetadataToInst: {
    name: "LLVMAddMetadataToInst",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  BuilderGetDefaultFPMathTag: {
    name: "LLVMBuilderGetDefaultFPMathTag",
    parameters: ["pointer"],
    result: "pointer"
  },
  BuilderSetDefaultFPMathTag: {
    name: "LLVMBuilderSetDefaultFPMathTag",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  SetCurrentDebugLocation: {
    name: "LLVMSetCurrentDebugLocation",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetCurrentDebugLocation: {
    name: "LLVMGetCurrentDebugLocation",
    parameters: ["pointer"],
    result: "pointer"
  },
  BuildRetVoid: {
    name: "LLVMBuildRetVoid",
    parameters: ["pointer"],
    result: "pointer"
  },
  BuildRet: {
    name: "LLVMBuildRet",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  BuildAggregateRet: {
    name: "LLVMBuildAggregateRet",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  BuildBr: {
    name: "LLVMBuildBr",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  BuildCondBr: {
    name: "LLVMBuildCondBr",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildSwitch: {
    name: "LLVMBuildSwitch",
    parameters: ["pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  BuildIndirectBr: {
    name: "LLVMBuildIndirectBr",
    parameters: ["pointer", "pointer", "u32"],
    result: "pointer"
  },
  BuildInvoke: {
    name: "LLVMBuildInvoke",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildInvoke2: {
    name: "LLVMBuildInvoke2",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildUnreachable: {
    name: "LLVMBuildUnreachable",
    parameters: ["pointer"],
    result: "pointer"
  },
  BuildResume: {
    name: "LLVMBuildResume",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  BuildLandingPad: {
    name: "LLVMBuildLandingPad",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildCleanupRet: {
    name: "LLVMBuildCleanupRet",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildCatchRet: {
    name: "LLVMBuildCatchRet",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildCatchPad: {
    name: "LLVMBuildCatchPad",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildCleanupPad: {
    name: "LLVMBuildCleanupPad",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildCatchSwitch: {
    name: "LLVMBuildCatchSwitch",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  AddCase: {
    name: "LLVMAddCase",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  AddDestination: {
    name: "LLVMAddDestination",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetNumClauses: {
    name: "LLVMGetNumClauses",
    parameters: ["pointer"],
    result: "u32"
  },
  GetClause: {
    name: "LLVMGetClause",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  AddClause: {
    name: "LLVMAddClause",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  IsCleanup: {
    name: "LLVMIsCleanup",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetCleanup: {
    name: "LLVMSetCleanup",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  AddHandler: {
    name: "LLVMAddHandler",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetNumHandlers: {
    name: "LLVMGetNumHandlers",
    parameters: ["pointer"],
    result: "u32"
  },
  GetHandlers: {
    name: "LLVMGetHandlers",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetArgOperand: {
    name: "LLVMGetArgOperand",
    parameters: ["pointer", "u32"],
    result: "pointer"
  },
  SetArgOperand: {
    name: "LLVMSetArgOperand",
    parameters: ["pointer", "u32", "pointer"],
    result: "void"
  },
  GetParentCatchSwitch: {
    name: "LLVMGetParentCatchSwitch",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetParentCatchSwitch: {
    name: "LLVMSetParentCatchSwitch",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  BuildAdd: {
    name: "LLVMBuildAdd",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNSWAdd: {
    name: "LLVMBuildNSWAdd",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNUWAdd: {
    name: "LLVMBuildNUWAdd",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFAdd: {
    name: "LLVMBuildFAdd",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildSub: {
    name: "LLVMBuildSub",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNSWSub: {
    name: "LLVMBuildNSWSub",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNUWSub: {
    name: "LLVMBuildNUWSub",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFSub: {
    name: "LLVMBuildFSub",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildMul: {
    name: "LLVMBuildMul",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNSWMul: {
    name: "LLVMBuildNSWMul",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNUWMul: {
    name: "LLVMBuildNUWMul",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFMul: {
    name: "LLVMBuildFMul",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildUDiv: {
    name: "LLVMBuildUDiv",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildExactUDiv: {
    name: "LLVMBuildExactUDiv",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildSDiv: {
    name: "LLVMBuildSDiv",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildExactSDiv: {
    name: "LLVMBuildExactSDiv",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFDiv: {
    name: "LLVMBuildFDiv",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildURem: {
    name: "LLVMBuildURem",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildSRem: {
    name: "LLVMBuildSRem",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFRem: {
    name: "LLVMBuildFRem",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildShl: {
    name: "LLVMBuildShl",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildLShr: {
    name: "LLVMBuildLShr",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildAShr: {
    name: "LLVMBuildAShr",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildAnd: {
    name: "LLVMBuildAnd",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildOr: {
    name: "LLVMBuildOr",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildXor: {
    name: "LLVMBuildXor",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildBinOp: {
    name: "LLVMBuildBinOp",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNeg: {
    name: "LLVMBuildNeg",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNSWNeg: {
    name: "LLVMBuildNSWNeg",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNUWNeg: {
    name: "LLVMBuildNUWNeg",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFNeg: {
    name: "LLVMBuildFNeg",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildNot: {
    name: "LLVMBuildNot",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildMalloc: {
    name: "LLVMBuildMalloc",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildArrayMalloc: {
    name: "LLVMBuildArrayMalloc",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildMemSet: {
    name: "LLVMBuildMemSet",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u32"],
    result: "pointer"
  },
  BuildMemCpy: {
    name: "LLVMBuildMemCpy",
    parameters: ["pointer", "pointer", "u32", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildMemMove: {
    name: "LLVMBuildMemMove",
    parameters: ["pointer", "pointer", "u32", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildAlloca: {
    name: "LLVMBuildAlloca",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildArrayAlloca: {
    name: "LLVMBuildArrayAlloca",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFree: {
    name: "LLVMBuildFree",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  BuildLoad: {
    name: "LLVMBuildLoad",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildLoad2: {
    name: "LLVMBuildLoad2",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildStore: {
    name: "LLVMBuildStore",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildGEP: {
    name: "LLVMBuildGEP",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildInBoundsGEP: {
    name: "LLVMBuildInBoundsGEP",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildStructGEP: {
    name: "LLVMBuildStructGEP",
    parameters: ["pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildGEP2: {
    name: "LLVMBuildGEP2",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildInBoundsGEP2: {
    name: "LLVMBuildInBoundsGEP2",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildStructGEP2: {
    name: "LLVMBuildStructGEP2",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildGlobalString: {
    name: "LLVMBuildGlobalString",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildGlobalStringPtr: {
    name: "LLVMBuildGlobalStringPtr",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetVolatile: {
    name: "LLVMGetVolatile",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetVolatile: {
    name: "LLVMSetVolatile",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetWeak: {
    name: "LLVMGetWeak",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetWeak: {
    name: "LLVMSetWeak",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetOrdering: {
    name: "LLVMGetOrdering",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetOrdering: {
    name: "LLVMSetOrdering",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetAtomicRMWBinOp: {
    name: "LLVMGetAtomicRMWBinOp",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetAtomicRMWBinOp: {
    name: "LLVMSetAtomicRMWBinOp",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  BuildTrunc: {
    name: "LLVMBuildTrunc",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildZExt: {
    name: "LLVMBuildZExt",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildSExt: {
    name: "LLVMBuildSExt",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFPToUI: {
    name: "LLVMBuildFPToUI",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFPToSI: {
    name: "LLVMBuildFPToSI",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildUIToFP: {
    name: "LLVMBuildUIToFP",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildSIToFP: {
    name: "LLVMBuildSIToFP",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFPTrunc: {
    name: "LLVMBuildFPTrunc",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFPExt: {
    name: "LLVMBuildFPExt",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildPtrToInt: {
    name: "LLVMBuildPtrToInt",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildIntToPtr: {
    name: "LLVMBuildIntToPtr",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildBitCast: {
    name: "LLVMBuildBitCast",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildAddrSpaceCast: {
    name: "LLVMBuildAddrSpaceCast",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildZExtOrBitCast: {
    name: "LLVMBuildZExtOrBitCast",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildSExtOrBitCast: {
    name: "LLVMBuildSExtOrBitCast",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildTruncOrBitCast: {
    name: "LLVMBuildTruncOrBitCast",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildCast: {
    name: "LLVMBuildCast",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildPointerCast: {
    name: "LLVMBuildPointerCast",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildIntCast2: {
    name: "LLVMBuildIntCast2",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFPCast: {
    name: "LLVMBuildFPCast",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildIntCast: {
    name: "LLVMBuildIntCast",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetCastOpcode: {
    name: "LLVMGetCastOpcode",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildICmp: {
    name: "LLVMBuildICmp",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFCmp: {
    name: "LLVMBuildFCmp",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildPhi: {
    name: "LLVMBuildPhi",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildCall: {
    name: "LLVMBuildCall",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildCall2: {
    name: "LLVMBuildCall2",
    parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildSelect: {
    name: "LLVMBuildSelect",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildVAArg: {
    name: "LLVMBuildVAArg",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildExtractElement: {
    name: "LLVMBuildExtractElement",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildInsertElement: {
    name: "LLVMBuildInsertElement",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildShuffleVector: {
    name: "LLVMBuildShuffleVector",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildExtractValue: {
    name: "LLVMBuildExtractValue",
    parameters: ["pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildInsertValue: {
    name: "LLVMBuildInsertValue",
    parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
    result: "pointer"
  },
  BuildFreeze: {
    name: "LLVMBuildFreeze",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildIsNull: {
    name: "LLVMBuildIsNull",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildIsNotNull: {
    name: "LLVMBuildIsNotNull",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildPtrDiff: {
    name: "LLVMBuildPtrDiff",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildPtrDiff2: {
    name: "LLVMBuildPtrDiff2",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildFence: {
    name: "LLVMBuildFence",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildAtomicRMW: {
    name: "LLVMBuildAtomicRMW",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  BuildAtomicCmpXchg: {
    name: "LLVMBuildAtomicCmpXchg",
    parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetNumMaskElements: {
    name: "LLVMGetNumMaskElements",
    parameters: ["pointer"],
    result: "u32"
  },
  GetUndefMaskElem: {
    name: "LLVMGetUndefMaskElem",
    parameters: [],
    result: "i32"
  },
  GetMaskValue: {
    name: "LLVMGetMaskValue",
    parameters: ["pointer", "u32"],
    result: "i32"
  },
  IsAtomicSingleThread: {
    name: "LLVMIsAtomicSingleThread",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetAtomicSingleThread: {
    name: "LLVMSetAtomicSingleThread",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetCmpXchgSuccessOrdering: {
    name: "LLVMGetCmpXchgSuccessOrdering",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetCmpXchgSuccessOrdering: {
    name: "LLVMSetCmpXchgSuccessOrdering",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetCmpXchgFailureOrdering: {
    name: "LLVMGetCmpXchgFailureOrdering",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetCmpXchgFailureOrdering: {
    name: "LLVMSetCmpXchgFailureOrdering",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  CreateModuleProviderForExistingModule: {
    name: "LLVMCreateModuleProviderForExistingModule",
    parameters: ["pointer"],
    result: "pointer"
  },
  DisposeModuleProvider: {
    name: "LLVMDisposeModuleProvider",
    parameters: ["pointer"],
    result: "void"
  },
  CreateMemoryBufferWithContentsOfFile: {
    name: "LLVMCreateMemoryBufferWithContentsOfFile",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  CreateMemoryBufferWithSTDIN: {
    name: "LLVMCreateMemoryBufferWithSTDIN",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  CreateMemoryBufferWithMemoryRange: {
    name: "LLVMCreateMemoryBufferWithMemoryRange",
    parameters: ["pointer", "u64", "pointer", "pointer"],
    result: "pointer"
  },
  CreateMemoryBufferWithMemoryRangeCopy: {
    name: "LLVMCreateMemoryBufferWithMemoryRangeCopy",
    parameters: ["pointer", "u64", "pointer"],
    result: "pointer"
  },
  GetBufferStart: {
    name: "LLVMGetBufferStart",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetBufferSize: {
    name: "LLVMGetBufferSize",
    parameters: ["pointer"],
    result: "u64"
  },
  DisposeMemoryBuffer: {
    name: "LLVMDisposeMemoryBuffer",
    parameters: ["pointer"],
    result: "void"
  },
  GetGlobalPassRegistry: {
    name: "LLVMGetGlobalPassRegistry",
    parameters: [],
    result: "pointer"
  },
  CreatePassManager: {
    name: "LLVMCreatePassManager",
    parameters: [],
    result: "pointer"
  },
  CreateFunctionPassManagerForModule: {
    name: "LLVMCreateFunctionPassManagerForModule",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateFunctionPassManager: {
    name: "LLVMCreateFunctionPassManager",
    parameters: ["pointer"],
    result: "pointer"
  },
  RunPassManager: {
    name: "LLVMRunPassManager",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  InitializeFunctionPassManager: {
    name: "LLVMInitializeFunctionPassManager",
    parameters: ["pointer"],
    result: "pointer"
  },
  RunFunctionPassManager: {
    name: "LLVMRunFunctionPassManager",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  FinalizeFunctionPassManager: {
    name: "LLVMFinalizeFunctionPassManager",
    parameters: ["pointer"],
    result: "pointer"
  },
  DisposePassManager: {
    name: "LLVMDisposePassManager",
    parameters: ["pointer"],
    result: "void"
  },
  StartMultithreaded: {
    name: "LLVMStartMultithreaded",
    parameters: [],
    result: "pointer"
  },
  StopMultithreaded: {
    name: "LLVMStopMultithreaded",
    parameters: [],
    result: "void"
  },
  IsMultithreaded: {
    name: "LLVMIsMultithreaded",
    parameters: [],
    result: "pointer"
  },
  CreateBinary: {
    name: "LLVMCreateBinary",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  DisposeBinary: {
    name: "LLVMDisposeBinary",
    parameters: ["pointer"],
    result: "void"
  },
  BinaryCopyMemoryBuffer: {
    name: "LLVMBinaryCopyMemoryBuffer",
    parameters: ["pointer"],
    result: "pointer"
  },
  BinaryGetType: {
    name: "LLVMBinaryGetType",
    parameters: ["pointer"],
    result: "pointer"
  },
  MachOUniversalBinaryCopyObjectForArch: {
    name: "LLVMMachOUniversalBinaryCopyObjectForArch",
    parameters: ["pointer", "pointer", "u64", "pointer"],
    result: "pointer"
  },
  ObjectFileCopySectionIterator: {
    name: "LLVMObjectFileCopySectionIterator",
    parameters: ["pointer"],
    result: "pointer"
  },
  ObjectFileIsSectionIteratorAtEnd: {
    name: "LLVMObjectFileIsSectionIteratorAtEnd",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ObjectFileCopySymbolIterator: {
    name: "LLVMObjectFileCopySymbolIterator",
    parameters: ["pointer"],
    result: "pointer"
  },
  ObjectFileIsSymbolIteratorAtEnd: {
    name: "LLVMObjectFileIsSymbolIteratorAtEnd",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  DisposeSectionIterator: {
    name: "LLVMDisposeSectionIterator",
    parameters: ["pointer"],
    result: "void"
  },
  MoveToNextSection: {
    name: "LLVMMoveToNextSection",
    parameters: ["pointer"],
    result: "void"
  },
  MoveToContainingSection: {
    name: "LLVMMoveToContainingSection",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  DisposeSymbolIterator: {
    name: "LLVMDisposeSymbolIterator",
    parameters: ["pointer"],
    result: "void"
  },
  MoveToNextSymbol: {
    name: "LLVMMoveToNextSymbol",
    parameters: ["pointer"],
    result: "void"
  },
  GetSectionName: {
    name: "LLVMGetSectionName",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetSectionSize: {
    name: "LLVMGetSectionSize",
    parameters: ["pointer"],
    result: "u64"
  },
  GetSectionContents: {
    name: "LLVMGetSectionContents",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetSectionAddress: {
    name: "LLVMGetSectionAddress",
    parameters: ["pointer"],
    result: "u64"
  },
  GetSectionContainsSymbol: {
    name: "LLVMGetSectionContainsSymbol",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetRelocations: {
    name: "LLVMGetRelocations",
    parameters: ["pointer"],
    result: "pointer"
  },
  DisposeRelocationIterator: {
    name: "LLVMDisposeRelocationIterator",
    parameters: ["pointer"],
    result: "void"
  },
  IsRelocationIteratorAtEnd: {
    name: "LLVMIsRelocationIteratorAtEnd",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  MoveToNextRelocation: {
    name: "LLVMMoveToNextRelocation",
    parameters: ["pointer"],
    result: "void"
  },
  GetSymbolName: {
    name: "LLVMGetSymbolName",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetSymbolAddress: {
    name: "LLVMGetSymbolAddress",
    parameters: ["pointer"],
    result: "u64"
  },
  GetSymbolSize: {
    name: "LLVMGetSymbolSize",
    parameters: ["pointer"],
    result: "u64"
  },
  GetRelocationOffset: {
    name: "LLVMGetRelocationOffset",
    parameters: ["pointer"],
    result: "u64"
  },
  GetRelocationSymbol: {
    name: "LLVMGetRelocationSymbol",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetRelocationType: {
    name: "LLVMGetRelocationType",
    parameters: ["pointer"],
    result: "u64"
  },
  GetRelocationTypeName: {
    name: "LLVMGetRelocationTypeName",
    parameters: ["pointer"],
    result: "pointer"
  },
  GetRelocationValueString: {
    name: "LLVMGetRelocationValueString",
    parameters: ["pointer"],
    result: "pointer"
  },
  CreateObjectFile: {
    name: "LLVMCreateObjectFile",
    parameters: ["pointer"],
    result: "pointer"
  },
  DisposeObjectFile: {
    name: "LLVMDisposeObjectFile",
    parameters: ["pointer"],
    result: "void"
  },
  GetSections: {
    name: "LLVMGetSections",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsSectionIteratorAtEnd: {
    name: "LLVMIsSectionIteratorAtEnd",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetSymbols: {
    name: "LLVMGetSymbols",
    parameters: ["pointer"],
    result: "pointer"
  },
  IsSymbolIteratorAtEnd: {
    name: "LLVMIsSymbolIteratorAtEnd",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  WriteBitcodeToFile: {
    name: "LLVMWriteBitcodeToFile",
    parameters: ["pointer", "pointer"],
    result: "i32"
  },
  WriteBitcodeToFD: {
    name: "LLVMWriteBitcodeToFD",
    parameters: ["pointer", "i32", "i32", "i32"],
    result: "i32"
  },
  WriteBitcodeToFileHandle: {
    name: "LLVMWriteBitcodeToFileHandle",
    parameters: ["pointer", "i32"],
    result: "i32"
  },
  WriteBitcodeToMemoryBuffer: {
    name: "LLVMWriteBitcodeToMemoryBuffer",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcCreateLLJITBuilder: {
    name: "LLVMOrcCreateLLJITBuilder",
    parameters: [],
    result: "pointer"
  },
  OrcDisposeLLJITBuilder: {
    name: "LLVMOrcDisposeLLJITBuilder",
    parameters: ["pointer"],
    result: "void"
  },
  OrcLLJITBuilderSetJITTargetMachineBuilder: {
    name: "LLVMOrcLLJITBuilderSetJITTargetMachineBuilder",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  OrcLLJITBuilderSetObjectLinkingLayerCreator: {
    name: "LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator",
    parameters: ["pointer", "pointer", "pointer"],
    result: "void"
  },
  OrcCreateLLJIT: {
    name: "LLVMOrcCreateLLJIT",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcDisposeLLJIT: {
    name: "LLVMOrcDisposeLLJIT",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcLLJITGetExecutionSession: {
    name: "LLVMOrcLLJITGetExecutionSession",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcLLJITGetMainJITDylib: {
    name: "LLVMOrcLLJITGetMainJITDylib",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcLLJITGetTripleString: {
    name: "LLVMOrcLLJITGetTripleString",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcLLJITGetGlobalPrefix: {
    name: "LLVMOrcLLJITGetGlobalPrefix",
    parameters: ["pointer"],
    result: "u8"
  },
  OrcLLJITMangleAndIntern: {
    name: "LLVMOrcLLJITMangleAndIntern",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  OrcLLJITAddObjectFile: {
    name: "LLVMOrcLLJITAddObjectFile",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcLLJITAddObjectFileWithRT: {
    name: "LLVMOrcLLJITAddObjectFileWithRT",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcLLJITAddLLVMIRModule: {
    name: "LLVMOrcLLJITAddLLVMIRModule",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcLLJITAddLLVMIRModuleWithRT: {
    name: "LLVMOrcLLJITAddLLVMIRModuleWithRT",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcLLJITLookup: {
    name: "LLVMOrcLLJITLookup",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  OrcLLJITGetObjLinkingLayer: {
    name: "LLVMOrcLLJITGetObjLinkingLayer",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcLLJITGetObjTransformLayer: {
    name: "LLVMOrcLLJITGetObjTransformLayer",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcLLJITGetIRTransformLayer: {
    name: "LLVMOrcLLJITGetIRTransformLayer",
    parameters: ["pointer"],
    result: "pointer"
  },
  OrcLLJITGetDataLayoutStr: {
    name: "LLVMOrcLLJITGetDataLayoutStr",
    parameters: ["pointer"],
    result: "pointer"
  },
  LoadLibraryPermanently: {
    name: "LLVMLoadLibraryPermanently",
    parameters: ["pointer"],
    result: "pointer"
  },
  ParseCommandLineOptions: {
    name: "LLVMParseCommandLineOptions",
    parameters: ["i32", "pointer", "pointer"],
    result: "void"
  },
  SearchForAddressOfSymbol: {
    name: "LLVMSearchForAddressOfSymbol",
    parameters: ["pointer"],
    result: "pointer"
  },
  AddSymbol: {
    name: "LLVMAddSymbol",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  InitializeTransformUtils: {
    name: "LLVMInitializeTransformUtils",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeScalarOpts: {
    name: "LLVMInitializeScalarOpts",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeObjCARCOpts: {
    name: "LLVMInitializeObjCARCOpts",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeVectorization: {
    name: "LLVMInitializeVectorization",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeInstCombine: {
    name: "LLVMInitializeInstCombine",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeAggressiveInstCombiner: {
    name: "LLVMInitializeAggressiveInstCombiner",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeIPO: {
    name: "LLVMInitializeIPO",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeInstrumentation: {
    name: "LLVMInitializeInstrumentation",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeAnalysis: {
    name: "LLVMInitializeAnalysis",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeIPA: {
    name: "LLVMInitializeIPA",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeCodeGen: {
    name: "LLVMInitializeCodeGen",
    parameters: ["pointer"],
    result: "void"
  },
  InitializeTarget: {
    name: "LLVMInitializeTarget",
    parameters: ["pointer"],
    result: "void"
  },
  ParseBitcode: {
    name: "LLVMParseBitcode",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  ParseBitcode2: {
    name: "LLVMParseBitcode2",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  ParseBitcodeInContext: {
    name: "LLVMParseBitcodeInContext",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  ParseBitcodeInContext2: {
    name: "LLVMParseBitcodeInContext2",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetBitcodeModuleInContext: {
    name: "LLVMGetBitcodeModuleInContext",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetBitcodeModuleInContext2: {
    name: "LLVMGetBitcodeModuleInContext2",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetBitcodeModule: {
    name: "LLVMGetBitcodeModule",
    parameters: ["pointer", "pointer", "pointer"],
    result: "pointer"
  },
  GetBitcodeModule2: {
    name: "LLVMGetBitcodeModule2",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetOrInsertComdat: {
    name: "LLVMGetOrInsertComdat",
    parameters: ["pointer", "pointer"],
    result: "pointer"
  },
  GetComdat: {
    name: "LLVMGetComdat",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetComdat: {
    name: "LLVMSetComdat",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  GetComdatSelectionKind: {
    name: "LLVMGetComdatSelectionKind",
    parameters: ["pointer"],
    result: "pointer"
  },
  SetComdatSelectionKind: {
    name: "LLVMSetComdatSelectionKind",
    parameters: ["pointer", "pointer"],
    result: "void"
  },
  ParseIRInContext: {
    name: "LLVMParseIRInContext",
    parameters: ["pointer", "pointer", "pointer", "pointer"],
    result: "pointer"
  }
} as const;
