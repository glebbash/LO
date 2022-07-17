// ./llvm-c/Types.h:48:16
export const LLVMOpaqueMemoryBuffer = { type: "opaque" };

// ./llvm-c/Types.h:53:16
export const LLVMOpaqueContext = { type: "opaque" };

// ./llvm-c/Types.h:61:16
export const LLVMOpaqueModule = { type: "opaque" };

// ./llvm-c/Types.h:68:16
export const LLVMOpaqueType = { type: "opaque" };

// ./llvm-c/Types.h:75:16
export const LLVMOpaqueValue = { type: "opaque" };

// ./llvm-c/Types.h:82:16
export const LLVMOpaqueBasicBlock = { type: "opaque" };

// ./llvm-c/Types.h:89:16
export const LLVMOpaqueMetadata = { type: "opaque" };

// ./llvm-c/Types.h:96:16
export const LLVMOpaqueNamedMDNode = { type: "opaque" };

// ./llvm-c/Types.h:103:16
export const LLVMOpaqueValueMetadataEntry = { type: "opaque" };

// ./llvm-c/Types.h:110:16
export const LLVMOpaqueBuilder = { type: "opaque" };

// ./llvm-c/Types.h:117:16
export const LLVMOpaqueDIBuilder = { type: "opaque" };

// ./llvm-c/Types.h:124:16
export const LLVMOpaqueModuleProvider = { type: "opaque" };

// ./llvm-c/Types.h:127:16
export const LLVMOpaquePassManager = { type: "opaque" };

// ./llvm-c/Types.h:130:16
export const LLVMOpaquePassRegistry = { type: "opaque" };

// ./llvm-c/Types.h:136:16
export const LLVMOpaqueUse = { type: "opaque" };

// ./llvm-c/Types.h:143:16
export const LLVMOpaqueAttributeRef = { type: "opaque" };

// ./llvm-c/Types.h:148:16
export const LLVMOpaqueDiagnosticInfo = { type: "opaque" };

// ./llvm-c/Types.h:153:16
export const LLVMComdat = { type: "opaque" };

// ./llvm-c/Types.h:158:16
export const LLVMOpaqueModuleFlagEntry = { type: "opaque" };

// ./llvm-c/Types.h:163:16
export const LLVMOpaqueJITEventListener = { type: "opaque" };

// ./llvm-c/Types.h:168:16
export const LLVMOpaqueBinary = { type: "opaque" };

// ./llvm-c/Target.h:37:16
export const LLVMOpaqueTargetData = { type: "opaque" };

// ./llvm-c/Target.h:38:16
export const LLVMOpaqueTargetLibraryInfotData = { type: "opaque" };

// ./llvm-c/TargetMachine.h:34:16
export const LLVMOpaqueTargetMachine = { type: "opaque" };

// ./llvm-c/TargetMachine.h:35:16
export const LLVMTarget = { type: "opaque" };

// ./llvm-c/ExecutionEngine.h:39:16
export const LLVMOpaqueGenericValue = { type: "opaque" };

// ./llvm-c/ExecutionEngine.h:40:16
export const LLVMOpaqueExecutionEngine = { type: "opaque" };

// ./llvm-c/ExecutionEngine.h:41:16
export const LLVMOpaqueMCJITMemoryManager = { type: "opaque" };

// ./llvm-c/ExecutionEngine.h:43:8
export const LLVMMCJITCompilerOptions = {
  "bit-size": 192,
  "bit-alignment": 64,
  "fields": {
    "OptLevel": {
      "bit-offset": 0,
      "bit-size": 32,
      "bit-alignment": 32,
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    "CodeModel": {
      "bit-offset": 32,
      "bit-size": 32,
      "bit-alignment": 32,
      "type": {
        "tag": "LLVMCodeModel"
      }
    },
    "NoFramePointerElim": {
      "bit-offset": 64,
      "bit-size": 32,
      "bit-alignment": 32,
      "type": {
        "tag": "LLVMBool"
      }
    },
    "EnableFastISel": {
      "bit-offset": 96,
      "bit-size": 32,
      "bit-alignment": 32,
      "type": {
        "tag": "LLVMBool"
      }
    },
    "MCJMM": {
      "bit-offset": 128,
      "bit-size": 64,
      "bit-alignment": 64,
      "type": {
        "tag": "LLVMMCJITMemoryManagerRef"
      }
    }
  }
} as const;

// ./llvm-c/DisassemblerTypes.h:72:8
export const LLVMOpInfoSymbol1 = {
  "bit-size": 192,
  "bit-alignment": 64,
  "fields": {
    "Present": {
      "bit-offset": 0,
      "bit-size": 64,
      "bit-alignment": 64,
      "type": {
        "tag": "uint64_t"
      }
    },
    "Name": {
      "bit-offset": 64,
      "bit-size": 64,
      "bit-alignment": 64,
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    "Value": {
      "bit-offset": 128,
      "bit-size": 64,
      "bit-alignment": 64,
      "type": {
        "tag": "uint64_t"
      }
    }
  }
} as const;

// ./llvm-c/DisassemblerTypes.h:78:8
export const LLVMOpInfo1 = {
  "bit-size": 512,
  "bit-alignment": 64,
  "fields": {
    "AddSymbol": {
      "bit-offset": 0,
      "bit-size": 192,
      "bit-alignment": 64,
      "type": {
        "tag": ":struct",
        "name": "LLVMOpInfoSymbol1",
        "id": 61
      }
    },
    "SubtractSymbol": {
      "bit-offset": 192,
      "bit-size": 192,
      "bit-alignment": 64,
      "type": {
        "tag": ":struct",
        "name": "LLVMOpInfoSymbol1",
        "id": 61
      }
    },
    "Value": {
      "bit-offset": 384,
      "bit-size": 64,
      "bit-alignment": 64,
      "type": {
        "tag": "uint64_t"
      }
    },
    "VariantKind": {
      "bit-offset": 448,
      "bit-size": 64,
      "bit-alignment": 64,
      "type": {
        "tag": "uint64_t"
      }
    }
  }
} as const;

// ./llvm-c/Error.h:33:16
export const LLVMOpaqueError = { type: "opaque" };

// ./llvm-c/Orc.h:87:16
export const LLVMOrcOpaqueExecutionSession = { type: "opaque" };

// ./llvm-c/Orc.h:97:16
export const LLVMOrcOpaqueSymbolStringPool = { type: "opaque" };

// ./llvm-c/Orc.h:102:16
export const LLVMOrcOpaqueSymbolStringPoolEntry = { type: "opaque" };

// ./llvm-c/Orc.h:158:16
export const LLVMOrcOpaqueJITDylib = { type: "opaque" };

// ./llvm-c/Orc.h:238:16
export const LLVMOrcOpaqueMaterializationUnit = { type: "opaque" };

// ./llvm-c/Orc.h:245:16
export const LLVMOrcOpaqueMaterializationResponsibility = { type: "opaque" };

// ./llvm-c/Orc.h:282:16
export const LLVMOrcOpaqueResourceTracker = { type: "opaque" };

// ./llvm-c/Orc.h:287:16
export const LLVMOrcOpaqueDefinitionGenerator = { type: "opaque" };

// ./llvm-c/Orc.h:302:16
export const LLVMOrcOpaqueLookupState = { type: "opaque" };

// ./llvm-c/Orc.h:352:16
export const LLVMOrcOpaqueThreadSafeContext = { type: "opaque" };

// ./llvm-c/Orc.h:357:16
export const LLVMOrcOpaqueThreadSafeModule = { type: "opaque" };

// ./llvm-c/Orc.h:369:16
export const LLVMOrcOpaqueJITTargetMachineBuilder = { type: "opaque" };

// ./llvm-c/Orc.h:375:16
export const LLVMOrcOpaqueObjectLayer = { type: "opaque" };

// ./llvm-c/Orc.h:380:16
export const LLVMOrcOpaqueObjectLinkingLayer = { type: "opaque" };

// ./llvm-c/Orc.h:385:16
export const LLVMOrcOpaqueIRTransformLayer = { type: "opaque" };

// ./llvm-c/Orc.h:409:16
export const LLVMOrcOpaqueObjectTransformLayer = { type: "opaque" };

// ./llvm-c/Orc.h:431:16
export const LLVMOrcOpaqueIndirectStubsManager = { type: "opaque" };

// ./llvm-c/Orc.h:437:16
export const LLVMOrcOpaqueLazyCallThroughManager = { type: "opaque" };

// ./llvm-c/Orc.h:446:16
export const LLVMOrcOpaqueDumpObjects = { type: "opaque" };

// ./llvm-c/Transforms/PassManagerBuilder.h:20:16
export const LLVMOpaquePassManagerBuilder = { type: "opaque" };

// ./llvm-c/Transforms/PassBuilder.h:38:16
export const LLVMOpaquePassBuilderOptions = { type: "opaque" };

// ./llvm-c/Remarks.h:57:16
export const LLVMRemarkOpaqueString = { type: "opaque" };

// ./llvm-c/Remarks.h:78:16
export const LLVMRemarkOpaqueDebugLoc = { type: "opaque" };

// ./llvm-c/Remarks.h:109:16
export const LLVMRemarkOpaqueArg = { type: "opaque" };

// ./llvm-c/Remarks.h:140:16
export const LLVMRemarkOpaqueEntry = { type: "opaque" };

// ./llvm-c/Remarks.h:230:16
export const LLVMRemarkOpaqueParser = { type: "opaque" };

// ./llvm-c/Object.h:36:16
export const LLVMOpaqueSectionIterator = { type: "opaque" };

// ./llvm-c/Object.h:37:16
export const LLVMOpaqueSymbolIterator = { type: "opaque" };

// ./llvm-c/Object.h:38:16
export const LLVMOpaqueRelocationIterator = { type: "opaque" };

// ./llvm-c/Object.h:205:16
export const LLVMOpaqueObjectFile = { type: "opaque" };

// ./llvm-c/LLJIT.h:61:16
export const LLVMOrcOpaqueLLJITBuilder = { type: "opaque" };

// ./llvm-c/LLJIT.h:66:16
export const LLVMOrcOpaqueLLJIT = { type: "opaque" };

// ./llvm-c/lto.h:95:16
export const LLVMOpaqueLTOModule = { type: "opaque" };

// ./llvm-c/lto.h:98:16
export const LLVMOpaqueLTOCodeGenerator = { type: "opaque" };

// ./llvm-c/lto.h:101:16
export const LLVMOpaqueThinLTOCodeGenerator = { type: "opaque" };

// ./llvm-c/lto.h:608:16
export const LLVMOpaqueLTOInput = { type: "opaque" };
