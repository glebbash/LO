// ./llvm-c/Analysis.h:44:10
export const LLVMVerifyModule = {
  parameters: ["pointer", "i32", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Analysis.h:49:10
export const LLVMVerifyFunction = {
  parameters: ["pointer", "i32"],
  returnType: "i32"
} as const;

// ./llvm-c/Analysis.h:53:6
export const LLVMViewFunctionCFG = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Analysis.h:54:6
export const LLVMViewFunctionCFGOnly = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:26:1 <Spelling=<scratch space>:53:1>
export const LLVMInitializeAArch64TargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:27:1 <Spelling=<scratch space>:55:1>
export const LLVMInitializeAMDGPUTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:28:1 <Spelling=<scratch space>:57:1>
export const LLVMInitializeARMTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:29:1 <Spelling=<scratch space>:59:1>
export const LLVMInitializeAVRTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:30:1 <Spelling=<scratch space>:61:1>
export const LLVMInitializeBPFTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:31:1 <Spelling=<scratch space>:63:1>
export const LLVMInitializeHexagonTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:32:1 <Spelling=<scratch space>:65:1>
export const LLVMInitializeLanaiTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:33:1 <Spelling=<scratch space>:67:1>
export const LLVMInitializeMipsTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:34:1 <Spelling=<scratch space>:69:1>
export const LLVMInitializeMSP430TargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:35:1 <Spelling=<scratch space>:71:1>
export const LLVMInitializeNVPTXTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:36:1 <Spelling=<scratch space>:73:1>
export const LLVMInitializePowerPCTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:37:1 <Spelling=<scratch space>:75:1>
export const LLVMInitializeRISCVTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:38:1 <Spelling=<scratch space>:77:1>
export const LLVMInitializeSparcTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:39:1 <Spelling=<scratch space>:79:1>
export const LLVMInitializeSystemZTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:40:1 <Spelling=<scratch space>:81:1>
export const LLVMInitializeVETargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:83:1>
export const LLVMInitializeWebAssemblyTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:85:1>
export const LLVMInitializeX86TargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:43:1 <Spelling=<scratch space>:87:1>
export const LLVMInitializeXCoreTargetInfo = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:26:1 <Spelling=<scratch space>:89:1>
export const LLVMInitializeAArch64Target = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:27:1 <Spelling=<scratch space>:91:1>
export const LLVMInitializeAMDGPUTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:28:1 <Spelling=<scratch space>:93:1>
export const LLVMInitializeARMTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:29:1 <Spelling=<scratch space>:95:1>
export const LLVMInitializeAVRTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:30:1 <Spelling=<scratch space>:97:1>
export const LLVMInitializeBPFTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:31:1 <Spelling=<scratch space>:99:1>
export const LLVMInitializeHexagonTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:32:1 <Spelling=<scratch space>:101:1>
export const LLVMInitializeLanaiTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:33:1 <Spelling=<scratch space>:103:1>
export const LLVMInitializeMipsTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:34:1 <Spelling=<scratch space>:105:1>
export const LLVMInitializeMSP430Target = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:35:1 <Spelling=<scratch space>:107:1>
export const LLVMInitializeNVPTXTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:36:1 <Spelling=<scratch space>:109:1>
export const LLVMInitializePowerPCTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:37:1 <Spelling=<scratch space>:111:1>
export const LLVMInitializeRISCVTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:38:1 <Spelling=<scratch space>:113:1>
export const LLVMInitializeSparcTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:39:1 <Spelling=<scratch space>:115:1>
export const LLVMInitializeSystemZTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:40:1 <Spelling=<scratch space>:117:1>
export const LLVMInitializeVETarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:119:1>
export const LLVMInitializeWebAssemblyTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:121:1>
export const LLVMInitializeX86Target = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:43:1 <Spelling=<scratch space>:123:1>
export const LLVMInitializeXCoreTarget = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:26:1 <Spelling=<scratch space>:125:1>
export const LLVMInitializeAArch64TargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:27:1 <Spelling=<scratch space>:127:1>
export const LLVMInitializeAMDGPUTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:28:1 <Spelling=<scratch space>:129:1>
export const LLVMInitializeARMTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:29:1 <Spelling=<scratch space>:131:1>
export const LLVMInitializeAVRTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:30:1 <Spelling=<scratch space>:133:1>
export const LLVMInitializeBPFTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:31:1 <Spelling=<scratch space>:135:1>
export const LLVMInitializeHexagonTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:32:1 <Spelling=<scratch space>:137:1>
export const LLVMInitializeLanaiTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:33:1 <Spelling=<scratch space>:139:1>
export const LLVMInitializeMipsTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:34:1 <Spelling=<scratch space>:141:1>
export const LLVMInitializeMSP430TargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:35:1 <Spelling=<scratch space>:143:1>
export const LLVMInitializeNVPTXTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:36:1 <Spelling=<scratch space>:145:1>
export const LLVMInitializePowerPCTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:37:1 <Spelling=<scratch space>:147:1>
export const LLVMInitializeRISCVTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:38:1 <Spelling=<scratch space>:149:1>
export const LLVMInitializeSparcTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:39:1 <Spelling=<scratch space>:151:1>
export const LLVMInitializeSystemZTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:40:1 <Spelling=<scratch space>:153:1>
export const LLVMInitializeVETargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:155:1>
export const LLVMInitializeWebAssemblyTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:157:1>
export const LLVMInitializeX86TargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Targets.def:43:1 <Spelling=<scratch space>:159:1>
export const LLVMInitializeXCoreTargetMC = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:27:1 <Spelling=<scratch space>:161:1>
export const LLVMInitializeAArch64AsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:28:1 <Spelling=<scratch space>:163:1>
export const LLVMInitializeAMDGPUAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:29:1 <Spelling=<scratch space>:165:1>
export const LLVMInitializeARMAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:30:1 <Spelling=<scratch space>:167:1>
export const LLVMInitializeAVRAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:31:1 <Spelling=<scratch space>:169:1>
export const LLVMInitializeBPFAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:32:1 <Spelling=<scratch space>:171:1>
export const LLVMInitializeHexagonAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:33:1 <Spelling=<scratch space>:173:1>
export const LLVMInitializeLanaiAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:34:1 <Spelling=<scratch space>:175:1>
export const LLVMInitializeMipsAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:35:1 <Spelling=<scratch space>:177:1>
export const LLVMInitializeMSP430AsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:36:1 <Spelling=<scratch space>:179:1>
export const LLVMInitializeNVPTXAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:37:1 <Spelling=<scratch space>:181:1>
export const LLVMInitializePowerPCAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:38:1 <Spelling=<scratch space>:183:1>
export const LLVMInitializeRISCVAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:39:1 <Spelling=<scratch space>:185:1>
export const LLVMInitializeSparcAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:40:1 <Spelling=<scratch space>:187:1>
export const LLVMInitializeSystemZAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:41:1 <Spelling=<scratch space>:189:1>
export const LLVMInitializeVEAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:42:1 <Spelling=<scratch space>:2:1>
export const LLVMInitializeWebAssemblyAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:43:1 <Spelling=<scratch space>:4:1>
export const LLVMInitializeX86AsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmPrinters.def:44:1 <Spelling=<scratch space>:6:1>
export const LLVMInitializeXCoreAsmPrinter = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:27:1 <Spelling=<scratch space>:8:1>
export const LLVMInitializeAArch64AsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:28:1 <Spelling=<scratch space>:10:1>
export const LLVMInitializeAMDGPUAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:29:1 <Spelling=<scratch space>:12:1>
export const LLVMInitializeARMAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:30:1 <Spelling=<scratch space>:14:1>
export const LLVMInitializeAVRAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:31:1 <Spelling=<scratch space>:16:1>
export const LLVMInitializeBPFAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:32:1 <Spelling=<scratch space>:18:1>
export const LLVMInitializeHexagonAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:33:1 <Spelling=<scratch space>:20:1>
export const LLVMInitializeLanaiAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:34:1 <Spelling=<scratch space>:22:1>
export const LLVMInitializeMipsAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:35:1 <Spelling=<scratch space>:24:1>
export const LLVMInitializeMSP430AsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:36:1 <Spelling=<scratch space>:26:1>
export const LLVMInitializePowerPCAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:37:1 <Spelling=<scratch space>:28:1>
export const LLVMInitializeRISCVAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:38:1 <Spelling=<scratch space>:30:1>
export const LLVMInitializeSparcAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:39:1 <Spelling=<scratch space>:32:1>
export const LLVMInitializeSystemZAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:40:1 <Spelling=<scratch space>:34:1>
export const LLVMInitializeVEAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:41:1 <Spelling=<scratch space>:36:1>
export const LLVMInitializeWebAssemblyAsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/AsmParsers.def:42:1 <Spelling=<scratch space>:38:1>
export const LLVMInitializeX86AsmParser = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:27:1 <Spelling=<scratch space>:40:1>
export const LLVMInitializeAArch64Disassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:28:1 <Spelling=<scratch space>:42:1>
export const LLVMInitializeAMDGPUDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:29:1 <Spelling=<scratch space>:44:1>
export const LLVMInitializeARMDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:30:1 <Spelling=<scratch space>:46:1>
export const LLVMInitializeAVRDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:31:1 <Spelling=<scratch space>:48:1>
export const LLVMInitializeBPFDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:32:1 <Spelling=<scratch space>:50:1>
export const LLVMInitializeHexagonDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:33:1 <Spelling=<scratch space>:52:1>
export const LLVMInitializeLanaiDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:34:1 <Spelling=<scratch space>:54:1>
export const LLVMInitializeMipsDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:35:1 <Spelling=<scratch space>:56:1>
export const LLVMInitializeMSP430Disassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:36:1 <Spelling=<scratch space>:58:1>
export const LLVMInitializePowerPCDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:37:1 <Spelling=<scratch space>:60:1>
export const LLVMInitializeRISCVDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:38:1 <Spelling=<scratch space>:62:1>
export const LLVMInitializeSparcDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:39:1 <Spelling=<scratch space>:64:1>
export const LLVMInitializeSystemZDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:40:1 <Spelling=<scratch space>:66:1>
export const LLVMInitializeVEDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:41:1 <Spelling=<scratch space>:68:1>
export const LLVMInitializeWebAssemblyDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:42:1 <Spelling=<scratch space>:70:1>
export const LLVMInitializeX86Disassembler = {
  parameters: [],
  returnType: "void"
} as const;

// /usr/include/llvm/Config/Disassemblers.def:43:1 <Spelling=<scratch space>:72:1>
export const LLVMInitializeXCoreDisassembler = {
  parameters: [],
  returnType: "void"
} as const;

// ./llvm-c/Target.h:76:20
export const LLVMInitializeAllTargetInfos = {
  type: "inline"
};

// ./llvm-c/Target.h:85:20
export const LLVMInitializeAllTargets = {
  type: "inline"
};

// ./llvm-c/Target.h:94:20
export const LLVMInitializeAllTargetMCs = {
  type: "inline"
};

// ./llvm-c/Target.h:103:20
export const LLVMInitializeAllAsmPrinters = {
  type: "inline"
};

// ./llvm-c/Target.h:112:20
export const LLVMInitializeAllAsmParsers = {
  type: "inline"
};

// ./llvm-c/Target.h:121:20
export const LLVMInitializeAllDisassemblers = {
  type: "inline"
};

// ./llvm-c/Target.h:131:24
export const LLVMInitializeNativeTarget = {
  type: "inline"
};

// ./llvm-c/Target.h:146:24
export const LLVMInitializeNativeAsmParser = {
  type: "inline"
};

// ./llvm-c/Target.h:158:24
export const LLVMInitializeNativeAsmPrinter = {
  type: "inline"
};

// ./llvm-c/Target.h:170:24
export const LLVMInitializeNativeDisassembler = {
  type: "inline"
};

// ./llvm-c/Target.h:186:19
export const LLVMGetModuleDataLayout = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Target.h:193:6
export const LLVMSetModuleDataLayout = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Target.h:197:19
export const LLVMCreateTargetData = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Target.h:201:6
export const LLVMDisposeTargetData = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Target.h:206:6
export const LLVMAddTargetLibraryInfo = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Target.h:212:7
export const LLVMCopyStringRepOfTargetData = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Target.h:217:23
export const LLVMByteOrder = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Target.h:221:10
export const LLVMPointerSize = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Target.h:226:10
export const LLVMPointerSizeForAS = {
  parameters: ["pointer", "u32"],
  returnType: "u32"
} as const;

// ./llvm-c/Target.h:230:13
export const LLVMIntPtrType = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Target.h:235:13
export const LLVMIntPtrTypeForAS = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Target.h:239:13
export const LLVMIntPtrTypeInContext = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Target.h:244:13
export const LLVMIntPtrTypeForASInContext = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Target.h:249:20
export const LLVMSizeOfTypeInBits = {
  parameters: ["pointer", "pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Target.h:253:20
export const LLVMStoreSizeOfType = {
  parameters: ["pointer", "pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Target.h:257:20
export const LLVMABISizeOfType = {
  parameters: ["pointer", "pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Target.h:261:10
export const LLVMABIAlignmentOfType = {
  parameters: ["pointer", "pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Target.h:265:10
export const LLVMCallFrameAlignmentOfType = {
  parameters: ["pointer", "pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Target.h:269:10
export const LLVMPreferredAlignmentOfType = {
  parameters: ["pointer", "pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Target.h:273:10
export const LLVMPreferredAlignmentOfGlobal = {
  parameters: ["pointer", "pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Target.h:278:10
export const LLVMElementAtOffset = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "u32"
} as const;

// ./llvm-c/Target.h:283:20
export const LLVMOffsetOfElement = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "u64"
} as const;

// ./llvm-c/TargetMachine.h:70:15
export const LLVMGetFirstTarget = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:72:15
export const LLVMGetNextTarget = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:77:15
export const LLVMGetTargetFromName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:82:10
export const LLVMGetTargetFromTriple = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/TargetMachine.h:86:13
export const LLVMGetTargetName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:89:13
export const LLVMGetTargetDescription = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:92:10
export const LLVMTargetHasJIT = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/TargetMachine.h:95:10
export const LLVMTargetHasTargetMachine = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/TargetMachine.h:98:10
export const LLVMTargetHasAsmBackend = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/TargetMachine.h:102:22
export const LLVMCreateTargetMachine = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "i32", "i32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:108:6
export const LLVMDisposeTargetMachine = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/TargetMachine.h:111:15
export const LLVMGetTargetMachineTarget = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:116:7
export const LLVMGetTargetMachineTriple = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:121:7
export const LLVMGetTargetMachineCPU = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:126:7
export const LLVMGetTargetMachineFeatureString = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:129:19
export const LLVMCreateTargetDataLayout = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:132:6
export const LLVMSetTargetMachineAsmVerbosity = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/TargetMachine.h:138:10
export const LLVMTargetMachineEmitToFile = {
  parameters: ["pointer", "pointer", "pointer", "i32", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/TargetMachine.h:142:10
export const LLVMTargetMachineEmitToMemoryBuffer = {
  parameters: ["pointer", "pointer", "i32", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/TargetMachine.h:148:7
export const LLVMGetDefaultTargetTriple = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:152:7
export const LLVMNormalizeTargetTriple = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:156:7
export const LLVMGetHostCPUName = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:160:7
export const LLVMGetHostCPUFeatures = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/TargetMachine.h:163:6
export const LLVMAddAnalysisPasses = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:36:6
export const LLVMLinkInMCJIT = {
  parameters: [],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:37:6
export const LLVMLinkInInterpreter = {
  parameters: [],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:53:21
export const LLVMCreateGenericValueOfInt = {
  parameters: ["pointer", "u64", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:57:21
export const LLVMCreateGenericValueOfPointer = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:59:21
export const LLVMCreateGenericValueOfFloat = {
  parameters: ["pointer", "i64"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:61:10
export const LLVMGenericValueIntWidth = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/ExecutionEngine.h:63:20
export const LLVMGenericValueToInt = {
  parameters: ["pointer", "i32"],
  returnType: "u64"
} as const;

// ./llvm-c/ExecutionEngine.h:66:7
export const LLVMGenericValueToPointer = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:68:8
export const LLVMGenericValueToFloat = {
  parameters: ["pointer", "pointer"],
  returnType: "i64"
} as const;

// ./llvm-c/ExecutionEngine.h:70:6
export const LLVMDisposeGenericValue = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:74:10
export const LLVMCreateExecutionEngineForModule = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/ExecutionEngine.h:78:10
export const LLVMCreateInterpreterForModule = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/ExecutionEngine.h:82:10
export const LLVMCreateJITCompilerForModule = {
  parameters: ["pointer", "pointer", "u32", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/ExecutionEngine.h:87:6
export const LLVMInitializeMCJITCompilerOptions = {
  parameters: ["pointer", "u64"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:107:10
export const LLVMCreateMCJITCompilerForModule = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/ExecutionEngine.h:112:6
export const LLVMDisposeExecutionEngine = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:114:6
export const LLVMRunStaticConstructors = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:116:6
export const LLVMRunStaticDestructors = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:118:5
export const LLVMRunFunctionAsMain = {
  parameters: ["pointer", "pointer", "u32", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/ExecutionEngine.h:122:21
export const LLVMRunFunction = {
  parameters: ["pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:126:6
export const LLVMFreeMachineCodeForFunction = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:128:6
export const LLVMAddModule = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:130:10
export const LLVMRemoveModule = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/ExecutionEngine.h:133:10
export const LLVMFindFunction = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/ExecutionEngine.h:136:7
export const LLVMRecompileAndRelinkFunction = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:139:19
export const LLVMGetExecutionEngineTargetData = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:141:1
export const LLVMGetExecutionEngineTargetMachine = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:143:6
export const LLVMAddGlobalMapping = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:146:7
export const LLVMGetPointerToGlobal = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:148:10
export const LLVMGetGlobalValueAddress = {
  parameters: ["pointer", "pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/ExecutionEngine.h:150:10
export const LLVMGetFunctionAddress = {
  parameters: ["pointer", "pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/ExecutionEngine.h:154:10
export const LLVMExecutionEngineGetErrMsg = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/ExecutionEngine.h:180:27
export const LLVMCreateSimpleMCJITMemoryManager = {
  parameters: ["pointer", "function", "function", "function", "function"],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:187:6
export const LLVMDisposeMCJITMemoryManager = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/ExecutionEngine.h:191:25
export const LLVMCreateGDBRegistrationListener = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:192:25
export const LLVMCreateIntelJITEventListener = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:193:25
export const LLVMCreateOProfileJITEventListener = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/ExecutionEngine.h:194:25
export const LLVMCreatePerfJITEventListener = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Disassembler.h:38:22
export const LLVMCreateDisasm = {
  parameters: ["pointer", "pointer", "i32", "function", "function"],
  returnType: "pointer"
} as const;

// ./llvm-c/Disassembler.h:50:22
export const LLVMCreateDisasmCPU = {
  parameters: ["pointer", "pointer", "pointer", "i32", "function", "function"],
  returnType: "pointer"
} as const;

// ./llvm-c/Disassembler.h:63:1
export const LLVMCreateDisasmCPUFeatures = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "i32", "function", "function"],
  returnType: "pointer"
} as const;

// ./llvm-c/Disassembler.h:72:5
export const LLVMSetDisasmOptions = {
  parameters: ["pointer", "u64"],
  returnType: "i32"
} as const;

// ./llvm-c/Disassembler.h:88:6
export const LLVMDisasmDispose = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Disassembler.h:100:8
export const LLVMDisasmInstruction = {
  parameters: ["pointer", "pointer", "u64", "u64", "pointer", "u64"],
  returnType: "u64"
} as const;

// ./llvm-c/DebugInfo.h:197:10
export const LLVMDebugMetadataVersion = {
  parameters: [],
  returnType: "u32"
} as const;

// ./llvm-c/DebugInfo.h:202:10
export const LLVMGetModuleDebugMetadataVersion = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/DebugInfo.h:210:10
export const LLVMStripModuleDebugInfo = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/DebugInfo.h:216:18
export const LLVMCreateDIBuilderDisallowUnresolved = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:223:18
export const LLVMCreateDIBuilder = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:229:6
export const LLVMDisposeDIBuilder = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/DebugInfo.h:234:6
export const LLVMDIBuilderFinalize = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/DebugInfo.h:240:6
export const LLVMDIBuilderFinalizeSubprogram = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/DebugInfo.h:275:17
export const LLVMDIBuilderCreateCompileUnit = {
  parameters: ["pointer", "i32", "pointer", "pointer", "u64", "i32", "pointer", "u64", "u32", "pointer", "u64", "i32", "u32", "i32", "i32", "pointer", "u64", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:293:1
export const LLVMDIBuilderCreateFile = {
  parameters: ["pointer", "pointer", "u64", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:312:1
export const LLVMDIBuilderCreateModule = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u64", "pointer", "u64", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:328:1
export const LLVMDIBuilderCreateNameSpace = {
  parameters: ["pointer", "pointer", "pointer", "u64", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:351:17
export const LLVMDIBuilderCreateFunction = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u64", "pointer", "u32", "pointer", "i32", "i32", "u32", "i32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:366:17
export const LLVMDIBuilderCreateLexicalBlock = {
  parameters: ["pointer", "pointer", "pointer", "u32", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:378:1
export const LLVMDIBuilderCreateLexicalBlockFile = {
  parameters: ["pointer", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:392:1
export const LLVMDIBuilderCreateImportedModuleFromNamespace = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:409:17
export const LLVMDIBuilderCreateImportedModuleFromAlias = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:424:17
export const LLVMDIBuilderCreateImportedModuleFromModule = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:444:17
export const LLVMDIBuilderCreateImportedDeclaration = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer", "u64", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:460:1
export const LLVMDIBuilderCreateDebugLocation = {
  parameters: ["pointer", "u32", "u32", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:470:10
export const LLVMDILocationGetLine = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/DebugInfo.h:478:10
export const LLVMDILocationGetColumn = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/DebugInfo.h:486:17
export const LLVMDILocationGetScope = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:494:17
export const LLVMDILocationGetInlinedAt = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:502:17
export const LLVMDIScopeGetFile = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:511:13
export const LLVMDIFileGetDirectory = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:520:13
export const LLVMDIFileGetFilename = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:529:13
export const LLVMDIFileGetSource = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:537:17
export const LLVMDIBuilderGetOrCreateTypeArray = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:552:1
export const LLVMDIBuilderCreateSubroutineType = {
  parameters: ["pointer", "pointer", "pointer", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:569:17
export const LLVMDIBuilderCreateMacro = {
  parameters: ["pointer", "pointer", "u32", "i32", "pointer", "u64", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:586:1
export const LLVMDIBuilderCreateTempMacroFile = {
  parameters: ["pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:598:17
export const LLVMDIBuilderCreateEnumerator = {
  parameters: ["pointer", "pointer", "u64", "i64", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:617:17
export const LLVMDIBuilderCreateEnumerationType = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:640:17
export const LLVMDIBuilderCreateUnionType = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "i32", "pointer", "u32", "u32", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:658:1
export const LLVMDIBuilderCreateArrayType = {
  parameters: ["pointer", "u64", "u32", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:673:1
export const LLVMDIBuilderCreateVectorType = {
  parameters: ["pointer", "u64", "u32", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:685:1
export const LLVMDIBuilderCreateUnspecifiedType = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:699:1
export const LLVMDIBuilderCreateBasicType = {
  parameters: ["pointer", "pointer", "u64", "u64", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:714:17
export const LLVMDIBuilderCreatePointerType = {
  parameters: ["pointer", "pointer", "u64", "u32", "u32", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:737:17
export const LLVMDIBuilderCreateStructType = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "i32", "pointer", "pointer", "u32", "u32", "pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:759:17
export const LLVMDIBuilderCreateMemberType = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "u64", "i32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:780:1
export const LLVMDIBuilderCreateStaticMemberType = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "pointer", "i32", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:796:1
export const LLVMDIBuilderCreateMemberPointerType = {
  parameters: ["pointer", "pointer", "pointer", "u64", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:817:1
export const LLVMDIBuilderCreateObjCIVar = {
  parameters: ["pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "u64", "i32", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:839:1
export const LLVMDIBuilderCreateObjCProperty = {
  parameters: ["pointer", "pointer", "u64", "pointer", "u32", "pointer", "u64", "pointer", "u64", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:853:1
export const LLVMDIBuilderCreateObjectPointerType = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:865:1
export const LLVMDIBuilderCreateQualifiedType = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:876:1
export const LLVMDIBuilderCreateReferenceType = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:884:1
export const LLVMDIBuilderCreateNullPtrType = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:896:1
export const LLVMDIBuilderCreateTypedef = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:912:1
export const LLVMDIBuilderCreateInheritance = {
  parameters: ["pointer", "pointer", "pointer", "u64", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:933:17
export const LLVMDIBuilderCreateForwardDecl = {
  parameters: ["pointer", "u32", "pointer", "u64", "pointer", "pointer", "u32", "u32", "u64", "u32", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:957:1
export const LLVMDIBuilderCreateReplaceableCompositeType = {
  parameters: ["pointer", "u32", "pointer", "u64", "pointer", "pointer", "u32", "u32", "u64", "u32", "i32", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:979:1
export const LLVMDIBuilderCreateBitFieldMemberType = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u64", "u64", "i32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1010:17
export const LLVMDIBuilderCreateClassType = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "u64", "u32", "u64", "i32", "pointer", "pointer", "u32", "pointer", "pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1025:1
export const LLVMDIBuilderCreateArtificialType = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1035:13
export const LLVMDITypeGetName = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1043:10
export const LLVMDITypeGetSizeInBits = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/DebugInfo.h:1051:10
export const LLVMDITypeGetOffsetInBits = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/DebugInfo.h:1059:10
export const LLVMDITypeGetAlignInBits = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/DebugInfo.h:1067:10
export const LLVMDITypeGetLine = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/DebugInfo.h:1075:13
export const LLVMDITypeGetFlags = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/DebugInfo.h:1083:17
export const LLVMDIBuilderGetOrCreateSubrange = {
  parameters: ["pointer", "i64", "i64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1093:17
export const LLVMDIBuilderGetOrCreateArray = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1104:17
export const LLVMDIBuilderCreateExpression = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1114:1
export const LLVMDIBuilderCreateConstantValueExpression = {
  parameters: ["pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1136:17
export const LLVMDIBuilderCreateGlobalVariableExpression = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u64", "pointer", "u32", "pointer", "i32", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1148:17
export const LLVMDIGlobalVariableExpressionGetVariable = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1156:17
export const LLVMDIGlobalVariableExpressionGetExpression = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1165:17
export const LLVMDIVariableGetFile = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1173:17
export const LLVMDIVariableGetScope = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1181:10
export const LLVMDIVariableGetLine = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/DebugInfo.h:1191:17
export const LLVMTemporaryMDNode = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1201:6
export const LLVMDisposeTemporaryMDNode = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/DebugInfo.h:1208:6
export const LLVMMetadataReplaceAllUsesWith = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/DebugInfo.h:1228:17
export const LLVMDIBuilderCreateTempGlobalVariableFwdDecl = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u64", "pointer", "u32", "pointer", "i32", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1243:14
export const LLVMDIBuilderInsertDeclareBefore = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1258:14
export const LLVMDIBuilderInsertDeclareAtEnd = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1271:14
export const LLVMDIBuilderInsertDbgValueBefore = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1289:14
export const LLVMDIBuilderInsertDbgValueAtEnd = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1309:17
export const LLVMDIBuilderCreateAutoVariable = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "u32", "pointer", "i32", "i32", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1327:17
export const LLVMDIBuilderCreateParameterVariable = {
  parameters: ["pointer", "pointer", "pointer", "u64", "u32", "pointer", "u32", "pointer", "i32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1337:17
export const LLVMGetSubprogram = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1344:6
export const LLVMSetSubprogram = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/DebugInfo.h:1352:10
export const LLVMDISubprogramGetLine = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/DebugInfo.h:1359:17
export const LLVMInstructionGetDebugLoc = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/DebugInfo.h:1368:6
export const LLVMInstructionSetDebugLoc = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/DebugInfo.h:1375:18
export const LLVMGetMetadataKind = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Error.h:44:17
export const LLVMGetErrorTypeId = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Error.h:52:6
export const LLVMConsumeError = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Error.h:60:7
export const LLVMGetErrorMessage = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Error.h:65:6
export const LLVMDisposeErrorMessage = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Error.h:70:17
export const LLVMGetStringErrorTypeId = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Error.h:75:14
export const LLVMCreateStringError = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:457:6
export const LLVMOrcExecutionSessionSetErrorReporter = {
  parameters: ["pointer", "function", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:468:1
export const LLVMOrcExecutionSessionGetSymbolStringPool = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:480:6
export const LLVMOrcSymbolStringPoolClearDeadEntries = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:495:1
export const LLVMOrcExecutionSessionIntern = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:500:6
export const LLVMOrcRetainSymbolStringPoolEntry = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:505:6
export const LLVMOrcReleaseSymbolStringPoolEntry = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:507:13
export const LLVMOrcSymbolStringPoolEntryStr = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:512:6
export const LLVMOrcReleaseResourceTracker = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:518:6
export const LLVMOrcResourceTrackerTransferTo = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:525:14
export const LLVMOrcResourceTrackerRemove = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:532:6
export const LLVMOrcDisposeDefinitionGenerator = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:537:6
export const LLVMOrcDisposeMaterializationUnit = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:572:31
export const LLVMOrcCreateCustomMaterializationUnit = {
  parameters: ["pointer", "pointer", "pointer", "u64", "pointer", "function", "function", "function"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:601:1
export const LLVMOrcAbsoluteSymbols = {
  parameters: ["pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:624:31
export const LLVMOrcLazyReexports = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:639:6
export const LLVMOrcDisposeMaterializationResponsibility = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:645:20
export const LLVMOrcMaterializationResponsibilityGetTargetDylib = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:652:1
export const LLVMOrcMaterializationResponsibilityGetExecutionSession = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:665:29
export const LLVMOrcMaterializationResponsibilityGetSymbols = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:673:6
export const LLVMOrcDisposeCSymbolFlagsMap = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:684:1
export const LLVMOrcMaterializationResponsibilityGetInitializerSymbol = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:694:1
export const LLVMOrcMaterializationResponsibilityGetRequestedSymbols = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:702:6
export const LLVMOrcDisposeSymbols = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:720:14
export const LLVMOrcMaterializationResponsibilityNotifyResolved = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:737:14
export const LLVMOrcMaterializationResponsibilityNotifyEmitted = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:753:14
export const LLVMOrcMaterializationResponsibilityDefineMaterializing = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:764:6
export const LLVMOrcMaterializationResponsibilityFailMaterialization = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:774:14
export const LLVMOrcMaterializationResponsibilityReplace = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:786:14
export const LLVMOrcMaterializationResponsibilityDelegate = {
  parameters: ["pointer", "pointer", "u64", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:809:6
export const LLVMOrcMaterializationResponsibilityAddDependencies = {
  parameters: ["pointer", "pointer", "pointer", "u64"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:819:6
export const LLVMOrcMaterializationResponsibilityAddDependenciesForAll = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:833:1
export const LLVMOrcExecutionSessionCreateBareJITDylib = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:849:1
export const LLVMOrcExecutionSessionCreateJITDylib = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:858:1
export const LLVMOrcExecutionSessionGetJITDylibByName = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:867:1
export const LLVMOrcJITDylibCreateResourceTracker = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:875:1
export const LLVMOrcJITDylibGetDefaultResourceTracker = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:884:14
export const LLVMOrcJITDylibDefine = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:891:14
export const LLVMOrcJITDylibClear = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:899:6
export const LLVMOrcJITDylibAddGenerator = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:905:31
export const LLVMOrcCreateCustomCAPIDefinitionGenerator = {
  parameters: ["function", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:926:14
export const LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess = {
  parameters: ["pointer", "u8", "function", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:951:14
export const LLVMOrcCreateDynamicLibrarySearchGeneratorForPath = {
  parameters: ["pointer", "pointer", "u8", "function", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:969:14
export const LLVMOrcCreateStaticLibrarySearchGeneratorForPath = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:981:29
export const LLVMOrcCreateNewThreadSafeContext = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:987:1
export const LLVMOrcThreadSafeContextGetContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:992:6
export const LLVMOrcDisposeThreadSafeContext = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1005:1
export const LLVMOrcCreateNewThreadSafeModule = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1013:6
export const LLVMOrcDisposeThreadSafeModule = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1019:1
export const LLVMOrcThreadSafeModuleWithModuleDo = {
  parameters: ["pointer", "function", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1031:14
export const LLVMOrcJITTargetMachineBuilderDetectHost = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1044:1
export const LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1049:6
export const LLVMOrcDisposeJITTargetMachineBuilder = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1058:7
export const LLVMOrcJITTargetMachineBuilderGetTargetTriple = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1065:6
export const LLVMOrcJITTargetMachineBuilderSetTargetTriple = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1079:14
export const LLVMOrcObjectLayerAddObjectFile = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1095:1
export const LLVMOrcObjectLayerAddObjectFileWithRT = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1105:6
export const LLVMOrcObjectLayerEmit = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1112:6
export const LLVMOrcDisposeObjectLayer = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1114:6
export const LLVMOrcIRTransformLayerEmit = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1122:6
export const LLVMOrcIRTransformLayerSetTransform = {
  parameters: ["pointer", "function", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1129:6
export const LLVMOrcObjectTransformLayerSetTransform = {
  parameters: ["pointer", "function", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1140:1
export const LLVMOrcCreateLocalIndirectStubsManager = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1145:6
export const LLVMOrcDisposeIndirectStubsManager = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1147:14
export const LLVMOrcCreateLocalLazyCallThroughManager = {
  parameters: ["pointer", "pointer", "u64", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1155:6
export const LLVMOrcDisposeLazyCallThroughManager = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1172:23
export const LLVMOrcCreateDumpObjects = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Orc.h:1178:6
export const LLVMOrcDisposeDumpObjects = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Orc.h:1183:14
export const LLVMOrcDumpObjects_CallOperator = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/OrcEE.h:47:1
export const LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/OrcEE.h:56:6
export const LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:32:27
export const LLVMPassManagerBuilderCreate = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:33:6
export const LLVMPassManagerBuilderDispose = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:37:1
export const LLVMPassManagerBuilderSetOptLevel = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:42:1
export const LLVMPassManagerBuilderSetSizeLevel = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:47:1
export const LLVMPassManagerBuilderSetDisableUnitAtATime = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:52:1
export const LLVMPassManagerBuilderSetDisableUnrollLoops = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:57:1
export const LLVMPassManagerBuilderSetDisableSimplifyLibCalls = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:62:1
export const LLVMPassManagerBuilderUseInlinerWithThreshold = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:67:1
export const LLVMPassManagerBuilderPopulateFunctionPassManager = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassManagerBuilder.h:72:1
export const LLVMPassManagerBuilderPopulateModulePassManager = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Utils.h:35:6
export const LLVMAddLowerSwitchPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Utils.h:38:6
export const LLVMAddPromoteMemoryToRegisterPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Utils.h:41:6
export const LLVMAddAddDiscriminatorsPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:31:6
export const LLVMAddConstantMergePass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:34:6
export const LLVMAddMergeFunctionsPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:37:6
export const LLVMAddCalledValuePropagationPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:40:6
export const LLVMAddDeadArgEliminationPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:43:6
export const LLVMAddFunctionAttrsPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:46:6
export const LLVMAddFunctionInliningPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:49:6
export const LLVMAddAlwaysInlinerPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:52:6
export const LLVMAddGlobalDCEPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:55:6
export const LLVMAddGlobalOptimizerPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:58:6
export const LLVMAddPruneEHPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:61:6
export const LLVMAddIPSCCPPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:64:6
export const LLVMAddInternalizePass = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:76:6
export const LLVMAddInternalizePassWithMustPreservePredicate = {
  parameters: ["pointer", "pointer", "function"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:82:6
export const LLVMAddStripDeadPrototypesPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/IPO.h:85:6
export const LLVMAddStripSymbolsPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:49:14
export const LLVMRunPasses = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:60:27
export const LLVMCreatePassBuilderOptions = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:66:6
export const LLVMPassBuilderOptionsSetVerifyEach = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:72:6
export const LLVMPassBuilderOptionsSetDebugLogging = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:75:6
export const LLVMPassBuilderOptionsSetLoopInterleaving = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:78:6
export const LLVMPassBuilderOptionsSetLoopVectorization = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:81:6
export const LLVMPassBuilderOptionsSetSLPVectorization = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:84:6
export const LLVMPassBuilderOptionsSetLoopUnrolling = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:87:6
export const LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:90:6
export const LLVMPassBuilderOptionsSetLicmMssaOptCap = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:93:6
export const LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:96:6
export const LLVMPassBuilderOptionsSetCallGraphProfile = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:99:6
export const LLVMPassBuilderOptionsSetMergeFunctions = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/PassBuilder.h:105:6
export const LLVMDisposePassBuilderOptions = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/AggressiveInstCombine.h:31:6
export const LLVMAddAggressiveInstCombinerPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:35:6
export const LLVMAddAggressiveDCEPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:38:6
export const LLVMAddDCEPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:41:6
export const LLVMAddBitTrackingDCEPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:44:6
export const LLVMAddAlignmentFromAssumptionsPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:47:6
export const LLVMAddCFGSimplificationPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:50:6
export const LLVMAddDeadStoreEliminationPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:53:6
export const LLVMAddScalarizerPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:56:6
export const LLVMAddMergedLoadStoreMotionPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:59:6
export const LLVMAddGVNPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:62:6
export const LLVMAddNewGVNPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:65:6
export const LLVMAddIndVarSimplifyPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:68:6
export const LLVMAddInstructionCombiningPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:71:6
export const LLVMAddInstructionSimplifyPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:74:6
export const LLVMAddJumpThreadingPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:77:6
export const LLVMAddLICMPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:80:6
export const LLVMAddLoopDeletionPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:83:6
export const LLVMAddLoopIdiomPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:86:6
export const LLVMAddLoopRotatePass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:89:6
export const LLVMAddLoopRerollPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:92:6
export const LLVMAddLoopUnrollPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:95:6
export const LLVMAddLoopUnrollAndJamPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:98:6
export const LLVMAddLowerAtomicPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:101:6
export const LLVMAddMemCpyOptPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:104:6
export const LLVMAddPartiallyInlineLibCallsPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:107:6
export const LLVMAddReassociatePass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:110:6
export const LLVMAddSCCPPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:113:6
export const LLVMAddScalarReplAggregatesPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:116:6
export const LLVMAddScalarReplAggregatesPassSSA = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:119:6
export const LLVMAddScalarReplAggregatesPassWithThreshold = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:123:6
export const LLVMAddSimplifyLibCallsPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:126:6
export const LLVMAddTailCallEliminationPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:129:6
export const LLVMAddDemoteMemoryToRegisterPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:132:6
export const LLVMAddVerifierPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:135:6
export const LLVMAddCorrelatedValuePropagationPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:138:6
export const LLVMAddEarlyCSEPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:141:6
export const LLVMAddEarlyCSEMemSSAPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:144:6
export const LLVMAddLowerExpectIntrinsicPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:147:6
export const LLVMAddLowerConstantIntrinsicsPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:150:6
export const LLVMAddTypeBasedAliasAnalysisPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:153:6
export const LLVMAddScopedNoAliasAAPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:156:6
export const LLVMAddBasicAliasAnalysisPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Scalar.h:159:6
export const LLVMAddUnifyFunctionExitNodesPass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Vectorize.h:36:6
export const LLVMAddLoopVectorizePass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Transforms/Vectorize.h:39:6
export const LLVMAddSLPVectorizePass = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Linker.h:41:10
export const LLVMLinkModules2 = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Remarks.h:64:20
export const LLVMRemarkStringGetData = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:71:17
export const LLVMRemarkStringGetLen = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Remarks.h:86:1
export const LLVMRemarkDebugLocGetSourceFilePath = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:93:17
export const LLVMRemarkDebugLocGetSourceLine = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Remarks.h:100:17
export const LLVMRemarkDebugLocGetSourceColumn = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Remarks.h:117:28
export const LLVMRemarkArgGetKey = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:124:28
export const LLVMRemarkArgGetValue = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:133:30
export const LLVMRemarkArgGetDebugLoc = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:147:13
export const LLVMRemarkEntryDispose = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Remarks.h:155:28
export const LLVMRemarkEntryGetType = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Remarks.h:163:1
export const LLVMRemarkEntryGetPassName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:171:1
export const LLVMRemarkEntryGetRemarkName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:179:1
export const LLVMRemarkEntryGetFunctionName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:189:1
export const LLVMRemarkEntryGetDebugLoc = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:198:17
export const LLVMRemarkEntryGetHotness = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Remarks.h:205:17
export const LLVMRemarkEntryGetNumArgs = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Remarks.h:216:25
export const LLVMRemarkEntryGetFirstArg = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:227:25
export const LLVMRemarkEntryGetNextArg = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:243:28
export const LLVMRemarkParserCreateYAML = {
  parameters: ["pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:257:28
export const LLVMRemarkParserCreateBitstream = {
  parameters: ["pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:302:27
export const LLVMRemarkParserGetNext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:309:17
export const LLVMRemarkParserHasError = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Remarks.h:322:20
export const LLVMRemarkParserGetErrorMessage = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Remarks.h:329:13
export const LLVMRemarkParserDispose = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Remarks.h:336:17
export const LLVMRemarkVersion = {
  parameters: [],
  returnType: "u32"
} as const;

// ./llvm-c/ErrorHandling.h:36:6
export const LLVMInstallFatalErrorHandler = {
  parameters: ["function"],
  returnType: "void"
} as const;

// ./llvm-c/ErrorHandling.h:42:6
export const LLVMResetFatalErrorHandler = {
  parameters: [],
  returnType: "void"
} as const;

// ./llvm-c/ErrorHandling.h:49:6
export const LLVMEnablePrettyStackTrace = {
  parameters: [],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:475:6
export const LLVMInitializeCore = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:480:6
export const LLVMShutdown = {
  parameters: [],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:484:7
export const LLVMCreateMessage = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:485:6
export const LLVMDisposeMessage = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:508:16
export const LLVMContextCreate = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:513:16
export const LLVMGetGlobalContext = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:518:6
export const LLVMContextSetDiagnosticHandler = {
  parameters: ["pointer", "function", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:525:23
export const LLVMContextGetDiagnosticHandler = {
  parameters: ["pointer"],
  returnType: "function"
} as const;

// ./llvm-c/Core.h:530:7
export const LLVMContextGetDiagnosticContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:537:6
export const LLVMContextSetYieldCallback = {
  parameters: ["pointer", "function", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:545:10
export const LLVMContextShouldDiscardValueNames = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:555:6
export const LLVMContextSetDiscardValueNames = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:562:6
export const LLVMContextSetOpaquePointers = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:570:6
export const LLVMContextDispose = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:578:7
export const LLVMGetDiagInfoDescription = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:585:24
export const LLVMGetDiagInfoSeverity = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:587:10
export const LLVMGetMDKindIDInContext = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:589:10
export const LLVMGetMDKindID = {
  parameters: ["pointer", "u32"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:602:10
export const LLVMGetEnumAttributeKindForName = {
  parameters: ["pointer", "u64"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:603:10
export const LLVMGetLastEnumAttributeKind = {
  parameters: [],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:608:18
export const LLVMCreateEnumAttribute = {
  parameters: ["pointer", "u32", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:615:10
export const LLVMGetEnumAttributeKind = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:620:10
export const LLVMGetEnumAttributeValue = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Core.h:625:18
export const LLVMCreateTypeAttribute = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:631:13
export const LLVMGetTypeAttributeValue = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:636:18
export const LLVMCreateStringAttribute = {
  parameters: ["pointer", "pointer", "u32", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:643:13
export const LLVMGetStringAttributeKind = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:648:13
export const LLVMGetStringAttributeValue = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:653:10
export const LLVMIsEnumAttribute = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:654:10
export const LLVMIsStringAttribute = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:655:10
export const LLVMIsTypeAttribute = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:660:13
export const LLVMGetTypeByName2 = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:685:15
export const LLVMModuleCreateWithName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:693:15
export const LLVMModuleCreateWithNameInContext = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:698:15
export const LLVMCloneModule = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:706:6
export const LLVMDisposeModule = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:716:13
export const LLVMGetModuleIdentifier = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:726:6
export const LLVMSetModuleIdentifier = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:736:13
export const LLVMGetSourceFileName = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:747:6
export const LLVMSetSourceFileName = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:758:13
export const LLVMGetDataLayoutStr = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:759:13
export const LLVMGetDataLayout = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:766:6
export const LLVMSetDataLayout = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:773:13
export const LLVMGetTarget = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:780:6
export const LLVMSetTarget = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:789:22
export const LLVMCopyModuleFlagsMetadata = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:794:6
export const LLVMDisposeModuleFlagsMetadata = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:802:1
export const LLVMModuleFlagEntriesGetFlagBehavior = {
  parameters: ["pointer", "u32"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:810:13
export const LLVMModuleFlagEntriesGetKey = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:818:17
export const LLVMModuleFlagEntriesGetMetadata = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:827:17
export const LLVMGetModuleFlag = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:836:6
export const LLVMAddModuleFlag = {
  parameters: ["pointer", "i32", "pointer", "u64", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:845:6
export const LLVMDumpModule = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:853:10
export const LLVMPrintModuleToFile = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:862:7
export const LLVMPrintModuleToString = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:869:13
export const LLVMGetModuleInlineAsm = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:876:6
export const LLVMSetModuleInlineAsm2 = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:883:6
export const LLVMAppendModuleInlineAsm = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:890:14
export const LLVMGetInlineAsm = {
  parameters: ["pointer", "pointer", "u64", "pointer", "u64", "i32", "i32", "i32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:901:16
export const LLVMGetModuleContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:904:13
export const LLVMGetTypeByName = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:911:20
export const LLVMGetFirstNamedMetadata = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:918:20
export const LLVMGetLastNamedMetadata = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:926:20
export const LLVMGetNextNamedMetadata = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:934:20
export const LLVMGetPreviousNamedMetadata = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:942:20
export const LLVMGetNamedMetadata = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:951:20
export const LLVMGetOrInsertNamedMetadata = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:960:13
export const LLVMGetNamedMetadataName = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:968:10
export const LLVMGetNamedMetadataNumOperands = {
  parameters: ["pointer", "pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:981:6
export const LLVMGetNamedMetadataOperands = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:990:6
export const LLVMAddNamedMetadataOperand = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1001:13
export const LLVMGetDebugLocDirectory = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1011:13
export const LLVMGetDebugLocFilename = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1021:10
export const LLVMGetDebugLocLine = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:1029:10
export const LLVMGetDebugLocColumn = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:1036:14
export const LLVMAddFunction = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1046:14
export const LLVMGetNamedFunction = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1053:14
export const LLVMGetFirstFunction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1060:14
export const LLVMGetLastFunction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1068:14
export const LLVMGetNextFunction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1076:14
export const LLVMGetPreviousFunction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1079:6
export const LLVMSetModuleInlineAsm = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1119:14
export const LLVMGetTypeKind = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1128:10
export const LLVMTypeIsSized = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1135:16
export const LLVMGetTypeContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1142:6
export const LLVMDumpType = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1150:7
export const LLVMPrintTypeToString = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1163:13
export const LLVMInt1TypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1164:13
export const LLVMInt8TypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1165:13
export const LLVMInt16TypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1166:13
export const LLVMInt32TypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1167:13
export const LLVMInt64TypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1168:13
export const LLVMInt128TypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1169:13
export const LLVMIntTypeInContext = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1175:13
export const LLVMInt1Type = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1176:13
export const LLVMInt8Type = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1177:13
export const LLVMInt16Type = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1178:13
export const LLVMInt32Type = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1179:13
export const LLVMInt64Type = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1180:13
export const LLVMInt128Type = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1181:13
export const LLVMIntType = {
  parameters: ["u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1182:10
export const LLVMGetIntTypeWidth = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:1197:13
export const LLVMHalfTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1202:13
export const LLVMBFloatTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1207:13
export const LLVMFloatTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1212:13
export const LLVMDoubleTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1217:13
export const LLVMX86FP80TypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1223:13
export const LLVMFP128TypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1228:13
export const LLVMPPCFP128TypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1235:13
export const LLVMHalfType = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1236:13
export const LLVMBFloatType = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1237:13
export const LLVMFloatType = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1238:13
export const LLVMDoubleType = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1239:13
export const LLVMX86FP80Type = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1240:13
export const LLVMFP128Type = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1241:13
export const LLVMPPCFP128Type = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1259:13
export const LLVMFunctionType = {
  parameters: ["pointer", "pointer", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1266:10
export const LLVMIsFunctionVarArg = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1271:13
export const LLVMGetReturnType = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1276:10
export const LLVMCountParamTypes = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:1289:6
export const LLVMGetParamTypes = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1313:13
export const LLVMStructTypeInContext = {
  parameters: ["pointer", "pointer", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1321:13
export const LLVMStructType = {
  parameters: ["pointer", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1329:13
export const LLVMStructCreateNamed = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1336:13
export const LLVMGetStructName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1343:6
export const LLVMStructSetBody = {
  parameters: ["pointer", "pointer", "u32", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1351:10
export const LLVMCountStructElementTypes = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:1363:6
export const LLVMGetStructElementTypes = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1370:13
export const LLVMStructGetTypeAtIndex = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1377:10
export const LLVMIsPackedStruct = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1384:10
export const LLVMIsOpaqueStruct = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1391:10
export const LLVMIsLiteralStruct = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1413:13
export const LLVMGetElementType = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1420:6
export const LLVMGetSubtypes = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1427:10
export const LLVMGetNumContainedTypes = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:1437:13
export const LLVMArrayType = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1446:10
export const LLVMGetArrayLength = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:1456:13
export const LLVMPointerType = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1465:10
export const LLVMPointerTypeIsOpaque = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1472:13
export const LLVMPointerTypeInContext = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1481:10
export const LLVMGetPointerAddressSpace = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:1492:13
export const LLVMVectorType = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1503:13
export const LLVMScalableVectorType = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1513:10
export const LLVMGetVectorSize = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:1528:13
export const LLVMVoidTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1533:13
export const LLVMLabelTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1538:13
export const LLVMX86MMXTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1543:13
export const LLVMX86AMXTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1548:13
export const LLVMTokenTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1553:13
export const LLVMMetadataTypeInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1559:13
export const LLVMVoidType = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1560:13
export const LLVMLabelType = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1561:13
export const LLVMX86MMXType = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1562:13
export const LLVMX86AMXType = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1698:13
export const LLVMTypeOf = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1705:15
export const LLVMGetValueKind = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1712:13
export const LLVMGetValueName2 = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1719:6
export const LLVMSetValueName2 = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1726:6
export const LLVMDumpValue = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1734:7
export const LLVMPrintValueToString = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1741:6
export const LLVMReplaceAllUsesWith = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1746:10
export const LLVMIsConstant = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1751:10
export const LLVMIsUndef = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1756:10
export const LLVMIsPoison = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:67:1>
export const LLVMIsAArgument = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:68:1>
export const LLVMIsABasicBlock = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:69:1>
export const LLVMIsAInlineAsm = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:70:1>
export const LLVMIsAUser = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:71:1>
export const LLVMIsAConstant = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:72:1>
export const LLVMIsABlockAddress = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:73:1>
export const LLVMIsAConstantAggregateZero = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:74:1>
export const LLVMIsAConstantArray = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:75:1>
export const LLVMIsAConstantDataSequential = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:76:1>
export const LLVMIsAConstantDataArray = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:77:1>
export const LLVMIsAConstantDataVector = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:78:1>
export const LLVMIsAConstantExpr = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:79:1>
export const LLVMIsAConstantFP = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:80:1>
export const LLVMIsAConstantInt = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:81:1>
export const LLVMIsAConstantPointerNull = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:82:1>
export const LLVMIsAConstantStruct = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:83:1>
export const LLVMIsAConstantTokenNone = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:84:1>
export const LLVMIsAConstantVector = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:85:1>
export const LLVMIsAGlobalValue = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:86:1>
export const LLVMIsAGlobalAlias = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:87:1>
export const LLVMIsAGlobalObject = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:88:1>
export const LLVMIsAFunction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:89:1>
export const LLVMIsAGlobalVariable = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:90:1>
export const LLVMIsAGlobalIFunc = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:91:1>
export const LLVMIsAUndefValue = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:92:1>
export const LLVMIsAPoisonValue = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:93:1>
export const LLVMIsAInstruction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:94:1>
export const LLVMIsAUnaryOperator = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:95:1>
export const LLVMIsABinaryOperator = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:96:1>
export const LLVMIsACallInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:97:1>
export const LLVMIsAIntrinsicInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:98:1>
export const LLVMIsADbgInfoIntrinsic = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:99:1>
export const LLVMIsADbgVariableIntrinsic = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:100:1>
export const LLVMIsADbgDeclareInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:101:1>
export const LLVMIsADbgLabelInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:102:1>
export const LLVMIsAMemIntrinsic = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:103:1>
export const LLVMIsAMemCpyInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:104:1>
export const LLVMIsAMemMoveInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:105:1>
export const LLVMIsAMemSetInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:106:1>
export const LLVMIsACmpInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:107:1>
export const LLVMIsAFCmpInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:108:1>
export const LLVMIsAICmpInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:109:1>
export const LLVMIsAExtractElementInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:110:1>
export const LLVMIsAGetElementPtrInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:111:1>
export const LLVMIsAInsertElementInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:112:1>
export const LLVMIsAInsertValueInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:113:1>
export const LLVMIsALandingPadInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:114:1>
export const LLVMIsAPHINode = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:115:1>
export const LLVMIsASelectInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:116:1>
export const LLVMIsAShuffleVectorInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:117:1>
export const LLVMIsAStoreInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:118:1>
export const LLVMIsABranchInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:119:1>
export const LLVMIsAIndirectBrInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:120:1>
export const LLVMIsAInvokeInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:121:1>
export const LLVMIsAReturnInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:122:1>
export const LLVMIsASwitchInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:123:1>
export const LLVMIsAUnreachableInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:124:1>
export const LLVMIsAResumeInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:125:1>
export const LLVMIsACleanupReturnInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:126:1>
export const LLVMIsACatchReturnInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:127:1>
export const LLVMIsACatchSwitchInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:128:1>
export const LLVMIsACallBrInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:129:1>
export const LLVMIsAFuncletPadInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:130:1>
export const LLVMIsACatchPadInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:131:1>
export const LLVMIsACleanupPadInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:132:1>
export const LLVMIsAUnaryInstruction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:133:1>
export const LLVMIsAAllocaInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:134:1>
export const LLVMIsACastInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:135:1>
export const LLVMIsAAddrSpaceCastInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:136:1>
export const LLVMIsABitCastInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:137:1>
export const LLVMIsAFPExtInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:138:1>
export const LLVMIsAFPToSIInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:139:1>
export const LLVMIsAFPToUIInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:140:1>
export const LLVMIsAFPTruncInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:141:1>
export const LLVMIsAIntToPtrInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:142:1>
export const LLVMIsAPtrToIntInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:143:1>
export const LLVMIsASExtInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:144:1>
export const LLVMIsASIToFPInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:145:1>
export const LLVMIsATruncInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:146:1>
export const LLVMIsAUIToFPInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:147:1>
export const LLVMIsAZExtInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:148:1>
export const LLVMIsAExtractValueInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:149:1>
export const LLVMIsALoadInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:150:1>
export const LLVMIsAVAArgInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:151:1>
export const LLVMIsAFreezeInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:152:1>
export const LLVMIsAAtomicCmpXchgInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:153:1>
export const LLVMIsAAtomicRMWInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:154:1>
export const LLVMIsAFenceInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1773:14
export const LLVMIsAMDNode = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1774:14
export const LLVMIsAMDString = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1777:13
export const LLVMGetValueName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1779:6
export const LLVMSetValueName = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1808:12
export const LLVMGetFirstUse = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1816:12
export const LLVMGetNextUse = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1825:14
export const LLVMGetUser = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1832:14
export const LLVMGetUsedValue = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1853:14
export const LLVMGetOperand = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1860:12
export const LLVMGetOperandUse = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1867:6
export const LLVMSetOperand = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:1874:5
export const LLVMGetNumOperands = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1897:14
export const LLVMConstNull = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1907:14
export const LLVMConstAllOnes = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1914:14
export const LLVMGetUndef = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1921:14
export const LLVMGetPoison = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1928:10
export const LLVMIsNull = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:1934:14
export const LLVMConstPointerNull = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1963:14
export const LLVMConstInt = {
  parameters: ["pointer", "u64", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1971:14
export const LLVMConstIntOfArbitraryPrecision = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1984:14
export const LLVMConstIntOfString = {
  parameters: ["pointer", "pointer", "u8"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1993:14
export const LLVMConstIntOfStringAndSize = {
  parameters: ["pointer", "pointer", "u32", "u8"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:1999:14
export const LLVMConstReal = {
  parameters: ["pointer", "i64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2007:14
export const LLVMConstRealOfString = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2012:14
export const LLVMConstRealOfStringAndSize = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2020:20
export const LLVMConstIntGetZExtValue = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Core.h:2027:11
export const LLVMConstIntGetSExtValue = {
  parameters: ["pointer"],
  returnType: "i64"
} as const;

// ./llvm-c/Core.h:2035:8
export const LLVMConstRealGetDouble = {
  parameters: ["pointer", "pointer"],
  returnType: "i64"
} as const;

// ./llvm-c/Core.h:2054:14
export const LLVMConstStringInContext = {
  parameters: ["pointer", "pointer", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2066:14
export const LLVMConstString = {
  parameters: ["pointer", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2074:10
export const LLVMIsConstantString = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2081:13
export const LLVMGetAsString = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2088:14
export const LLVMConstStructInContext = {
  parameters: ["pointer", "pointer", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2100:14
export const LLVMConstStruct = {
  parameters: ["pointer", "u32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2108:14
export const LLVMConstArray = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2116:14
export const LLVMConstNamedStruct = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2128:14
export const LLVMGetAggregateElement = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2135:1 <Spelling=/data/./llvm-c/Core.h:2136:18>
export const LLVMGetElementAsConstant = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2144:14
export const LLVMConstVector = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2159:12
export const LLVMGetConstOpcode = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2160:14
export const LLVMAlignOf = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2161:14
export const LLVMSizeOf = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2162:14
export const LLVMConstNeg = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2163:14
export const LLVMConstNSWNeg = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2164:14
export const LLVMConstNUWNeg = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2165:14
export const LLVMConstFNeg = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2166:14
export const LLVMConstNot = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2167:14
export const LLVMConstAdd = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2168:14
export const LLVMConstNSWAdd = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2169:14
export const LLVMConstNUWAdd = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2170:14
export const LLVMConstSub = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2171:14
export const LLVMConstNSWSub = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2172:14
export const LLVMConstNUWSub = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2173:14
export const LLVMConstMul = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2174:14
export const LLVMConstNSWMul = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2175:14
export const LLVMConstNUWMul = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2176:14
export const LLVMConstAnd = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2177:14
export const LLVMConstOr = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2178:14
export const LLVMConstXor = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2179:14
export const LLVMConstICmp = {
  parameters: ["i32", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2181:14
export const LLVMConstFCmp = {
  parameters: ["i32", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2183:14
export const LLVMConstShl = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2184:14
export const LLVMConstLShr = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2185:14
export const LLVMConstAShr = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2186:1 <Spelling=/data/./llvm-c/Core.h:2187:18>
export const LLVMConstGEP = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2191:14
export const LLVMConstGEP2 = {
  parameters: ["pointer", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2193:1 <Spelling=/data/./llvm-c/Core.h:2194:18>
export const LLVMConstInBoundsGEP = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2198:14
export const LLVMConstInBoundsGEP2 = {
  parameters: ["pointer", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2201:14
export const LLVMConstTrunc = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2202:14
export const LLVMConstSExt = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2203:14
export const LLVMConstZExt = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2204:14
export const LLVMConstFPTrunc = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2205:14
export const LLVMConstFPExt = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2206:14
export const LLVMConstUIToFP = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2207:14
export const LLVMConstSIToFP = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2208:14
export const LLVMConstFPToUI = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2209:14
export const LLVMConstFPToSI = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2210:14
export const LLVMConstPtrToInt = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2211:14
export const LLVMConstIntToPtr = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2212:14
export const LLVMConstBitCast = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2213:14
export const LLVMConstAddrSpaceCast = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2214:14
export const LLVMConstZExtOrBitCast = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2216:14
export const LLVMConstSExtOrBitCast = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2218:14
export const LLVMConstTruncOrBitCast = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2220:14
export const LLVMConstPointerCast = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2222:14
export const LLVMConstIntCast = {
  parameters: ["pointer", "pointer", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2224:14
export const LLVMConstFPCast = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2225:14
export const LLVMConstSelect = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2228:14
export const LLVMConstExtractElement = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2230:14
export const LLVMConstInsertElement = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2233:14
export const LLVMConstShuffleVector = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2236:14
export const LLVMBlockAddress = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2239:14
export const LLVMConstInlineAsm = {
  parameters: ["pointer", "pointer", "pointer", "i32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2258:15
export const LLVMGetGlobalParent = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2259:10
export const LLVMIsDeclaration = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2260:13
export const LLVMGetLinkage = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2261:6
export const LLVMSetLinkage = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2262:13
export const LLVMGetSection = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2263:6
export const LLVMSetSection = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2264:16
export const LLVMGetVisibility = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2265:6
export const LLVMSetVisibility = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2266:21
export const LLVMGetDLLStorageClass = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2267:6
export const LLVMSetDLLStorageClass = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2268:17
export const LLVMGetUnnamedAddress = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2269:6
export const LLVMSetUnnamedAddress = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2277:13
export const LLVMGlobalGetValueType = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2280:10
export const LLVMHasUnnamedAddr = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2282:6
export const LLVMSetUnnamedAddr = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2300:10
export const LLVMGetAlignment = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:2311:6
export const LLVMSetAlignment = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2319:6
export const LLVMGlobalSetMetadata = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2327:6
export const LLVMGlobalEraseMetadata = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2334:6
export const LLVMGlobalClearMetadata = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2343:25
export const LLVMGlobalCopyAllMetadata = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2349:6
export const LLVMDisposeValueMetadataEntries = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2354:10
export const LLVMValueMetadataEntriesGetKind = {
  parameters: ["pointer", "u32"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:2362:1
export const LLVMValueMetadataEntriesGetMetadata = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2378:14
export const LLVMAddGlobal = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2379:14
export const LLVMAddGlobalInAddressSpace = {
  parameters: ["pointer", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2382:14
export const LLVMGetNamedGlobal = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2383:14
export const LLVMGetFirstGlobal = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2384:14
export const LLVMGetLastGlobal = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2385:14
export const LLVMGetNextGlobal = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2386:14
export const LLVMGetPreviousGlobal = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2387:6
export const LLVMDeleteGlobal = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2388:14
export const LLVMGetInitializer = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2389:6
export const LLVMSetInitializer = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2390:10
export const LLVMIsThreadLocal = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2391:6
export const LLVMSetThreadLocal = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2392:10
export const LLVMIsGlobalConstant = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2393:6
export const LLVMSetGlobalConstant = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2394:21
export const LLVMGetThreadLocalMode = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2395:6
export const LLVMSetThreadLocalMode = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2396:10
export const LLVMIsExternallyInitialized = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2397:6
export const LLVMSetExternallyInitialized = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2413:1 <Spelling=/data/./llvm-c/Core.h:2414:18>
export const LLVMAddAlias = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2423:14
export const LLVMAddAlias2 = {
  parameters: ["pointer", "pointer", "u32", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2434:14
export const LLVMGetNamedGlobalAlias = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2442:14
export const LLVMGetFirstGlobalAlias = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2449:14
export const LLVMGetLastGlobalAlias = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2457:14
export const LLVMGetNextGlobalAlias = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2465:14
export const LLVMGetPreviousGlobalAlias = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2470:14
export const LLVMAliasGetAliasee = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2475:6
export const LLVMAliasSetAliasee = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2497:6
export const LLVMDeleteFunction = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2504:10
export const LLVMHasPersonalityFn = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2511:14
export const LLVMGetPersonalityFn = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2518:6
export const LLVMSetPersonalityFn = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2525:10
export const LLVMLookupIntrinsicID = {
  parameters: ["pointer", "u64"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:2532:10
export const LLVMGetIntrinsicID = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:2540:14
export const LLVMGetIntrinsicDeclaration = {
  parameters: ["pointer", "u32", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2551:13
export const LLVMIntrinsicGetType = {
  parameters: ["pointer", "u32", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2559:13
export const LLVMIntrinsicGetName = {
  parameters: ["u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2562:13
export const LLVMIntrinsicCopyOverloadedName = {
  parameters: ["u32", "pointer", "u64", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2578:13
export const LLVMIntrinsicCopyOverloadedName2 = {
  parameters: ["pointer", "u32", "pointer", "u64", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2588:10
export const LLVMIntrinsicIsOverloaded = {
  parameters: ["u32"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2597:10
export const LLVMGetFunctionCallConv = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:2607:6
export const LLVMSetFunctionCallConv = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2615:13
export const LLVMGetGC = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2622:6
export const LLVMSetGC = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2629:6
export const LLVMAddAttributeAtIndex = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2631:10
export const LLVMGetAttributeCountAtIndex = {
  parameters: ["pointer", "u32"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:2632:6
export const LLVMGetAttributesAtIndex = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2634:18
export const LLVMGetEnumAttributeAtIndex = {
  parameters: ["pointer", "u32", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2637:18
export const LLVMGetStringAttributeAtIndex = {
  parameters: ["pointer", "u32", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2640:6
export const LLVMRemoveEnumAttributeAtIndex = {
  parameters: ["pointer", "u32", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2642:6
export const LLVMRemoveStringAttributeAtIndex = {
  parameters: ["pointer", "u32", "pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2649:6
export const LLVMAddTargetDependentFunctionAttr = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2668:10
export const LLVMCountParams = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:2681:6
export const LLVMGetParams = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2690:14
export const LLVMGetParam = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2701:14
export const LLVMGetParamParent = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2708:14
export const LLVMGetFirstParam = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2715:14
export const LLVMGetLastParam = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2724:14
export const LLVMGetNextParam = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2731:14
export const LLVMGetPreviousParam = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2739:6
export const LLVMSetParamAlignment = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2761:14
export const LLVMAddGlobalIFunc = {
  parameters: ["pointer", "pointer", "u64", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2773:14
export const LLVMGetNamedGlobalIFunc = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2781:14
export const LLVMGetFirstGlobalIFunc = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2788:14
export const LLVMGetLastGlobalIFunc = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2796:14
export const LLVMGetNextGlobalIFunc = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2804:14
export const LLVMGetPreviousGlobalIFunc = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2812:14
export const LLVMGetGlobalIFuncResolver = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2819:6
export const LLVMSetGlobalIFuncResolver = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2826:6
export const LLVMEraseGlobalIFunc = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2836:6
export const LLVMRemoveGlobalIFunc = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2868:17
export const LLVMMDStringInContext2 = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2876:17
export const LLVMMDNodeInContext2 = {
  parameters: ["pointer", "pointer", "u64"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2882:14
export const LLVMMetadataAsValue = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2887:17
export const LLVMValueAsMetadata = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2896:13
export const LLVMGetMDString = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2904:10
export const LLVMGetMDNodeNumOperands = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:2917:6
export const LLVMGetMDNodeOperands = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:2920:14
export const LLVMMDStringInContext = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2923:14
export const LLVMMDString = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2925:14
export const LLVMMDNodeInContext = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2928:14
export const LLVMMDNode = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2954:14
export const LLVMBasicBlockAsValue = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2959:10
export const LLVMValueIsBasicBlock = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:2964:19
export const LLVMValueAsBasicBlock = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2969:13
export const LLVMGetBasicBlockName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2976:14
export const LLVMGetBasicBlockParent = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2988:14
export const LLVMGetBasicBlockTerminator = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:2995:10
export const LLVMCountBasicBlocks = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:3005:6
export const LLVMGetBasicBlocks = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3015:19
export const LLVMGetFirstBasicBlock = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3022:19
export const LLVMGetLastBasicBlock = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3027:19
export const LLVMGetNextBasicBlock = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3032:19
export const LLVMGetPreviousBasicBlock = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3040:19
export const LLVMGetEntryBasicBlock = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3049:6
export const LLVMInsertExistingBasicBlockAfterInsertBlock = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3057:6
export const LLVMAppendExistingBasicBlock = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3065:19
export const LLVMCreateBasicBlockInContext = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3073:19
export const LLVMAppendBasicBlockInContext = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3083:19
export const LLVMAppendBasicBlock = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3093:19
export const LLVMInsertBasicBlockInContext = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3102:19
export const LLVMInsertBasicBlock = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3113:6
export const LLVMDeleteBasicBlock = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3123:6
export const LLVMRemoveBasicBlockFromParent = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3130:6
export const LLVMMoveBasicBlockBefore = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3137:6
export const LLVMMoveBasicBlockAfter = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3145:14
export const LLVMGetFirstInstruction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3152:14
export const LLVMGetLastInstruction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3178:5
export const LLVMHasMetadata = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3183:14
export const LLVMGetMetadata = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3188:6
export const LLVMSetMetadata = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3197:1
export const LLVMInstructionGetAllMetadataOtherThanDebugLoc = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3205:19
export const LLVMGetInstructionParent = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3215:14
export const LLVMGetNextInstruction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3223:14
export const LLVMGetPreviousInstruction = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3233:6
export const LLVMInstructionRemoveFromParent = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3243:6
export const LLVMInstructionEraseFromParent = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3253:6
export const LLVMDeleteInstruction = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3260:12
export const LLVMGetInstructionOpcode = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3270:18
export const LLVMGetICmpPredicate = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3280:19
export const LLVMGetFCmpPredicate = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3290:14
export const LLVMInstructionClone = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3299:14
export const LLVMIsATerminatorInst = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3321:10
export const LLVMGetNumArgOperands = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:3332:6
export const LLVMSetInstructionCallConv = {
  parameters: ["pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3342:10
export const LLVMGetInstructionCallConv = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:3344:6
export const LLVMSetInstrParamAlignment = {
  parameters: ["pointer", "u32", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3347:6
export const LLVMAddCallSiteAttribute = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3349:10
export const LLVMGetCallSiteAttributeCount = {
  parameters: ["pointer", "u32"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:3350:6
export const LLVMGetCallSiteAttributes = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3352:18
export const LLVMGetCallSiteEnumAttribute = {
  parameters: ["pointer", "u32", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3355:18
export const LLVMGetCallSiteStringAttribute = {
  parameters: ["pointer", "u32", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3358:6
export const LLVMRemoveCallSiteEnumAttribute = {
  parameters: ["pointer", "u32", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3360:6
export const LLVMRemoveCallSiteStringAttribute = {
  parameters: ["pointer", "u32", "pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3368:13
export const LLVMGetCalledFunctionType = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3379:14
export const LLVMGetCalledValue = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3388:10
export const LLVMIsTailCall = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3397:6
export const LLVMSetTailCall = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3406:19
export const LLVMGetNormalDest = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3418:19
export const LLVMGetUnwindDest = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3427:6
export const LLVMSetNormalDest = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3439:6
export const LLVMSetUnwindDest = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3459:10
export const LLVMGetNumSuccessors = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:3466:19
export const LLVMGetSuccessor = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3473:6
export const LLVMSetSuccessor = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3482:10
export const LLVMIsConditional = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3491:14
export const LLVMGetCondition = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3500:6
export const LLVMSetCondition = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3509:19
export const LLVMGetSwitchDefaultDest = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3527:13
export const LLVMGetAllocatedType = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3545:10
export const LLVMIsInBounds = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3550:6
export const LLVMSetIsInBounds = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3555:13
export const LLVMGetGEPSourceElementType = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3573:6
export const LLVMAddIncoming = {
  parameters: ["pointer", "pointer", "pointer", "u32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3579:10
export const LLVMCountIncoming = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:3584:14
export const LLVMGetIncomingValue = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3589:19
export const LLVMGetIncomingBlock = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3609:10
export const LLVMGetNumIndices = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:3614:17
export const LLVMGetIndices = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3637:16
export const LLVMCreateBuilderInContext = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3638:16
export const LLVMCreateBuilder = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3639:6
export const LLVMPositionBuilder = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3641:6
export const LLVMPositionBuilderBefore = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3642:6
export const LLVMPositionBuilderAtEnd = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3643:19
export const LLVMGetInsertBlock = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3644:6
export const LLVMClearInsertionPosition = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3645:6
export const LLVMInsertIntoBuilder = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3646:6
export const LLVMInsertIntoBuilderWithName = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3648:6
export const LLVMDisposeBuilder = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3657:17
export const LLVMGetCurrentDebugLocation2 = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3666:6
export const LLVMSetCurrentDebugLocation2 = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3678:6
export const LLVMSetInstDebugLocation = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3685:6
export const LLVMAddMetadataToInst = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3692:17
export const LLVMBuilderGetDefaultFPMathTag = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3701:6
export const LLVMBuilderSetDefaultFPMathTag = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3708:6
export const LLVMSetCurrentDebugLocation = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3713:14
export const LLVMGetCurrentDebugLocation = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3716:14
export const LLVMBuildRetVoid = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3717:14
export const LLVMBuildRet = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3718:14
export const LLVMBuildAggregateRet = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3720:14
export const LLVMBuildBr = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3721:14
export const LLVMBuildCondBr = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3723:14
export const LLVMBuildSwitch = {
  parameters: ["pointer", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3725:14
export const LLVMBuildIndirectBr = {
  parameters: ["pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3727:1 <Spelling=/data/./llvm-c/Core.h:3728:18>
export const LLVMBuildInvoke = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3733:14
export const LLVMBuildInvoke2 = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3737:14
export const LLVMBuildUnreachable = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3740:14
export const LLVMBuildResume = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3741:14
export const LLVMBuildLandingPad = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3744:14
export const LLVMBuildCleanupRet = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3746:14
export const LLVMBuildCatchRet = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3748:14
export const LLVMBuildCatchPad = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3751:14
export const LLVMBuildCleanupPad = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3754:14
export const LLVMBuildCatchSwitch = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3759:6
export const LLVMAddCase = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3763:6
export const LLVMAddDestination = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3766:10
export const LLVMGetNumClauses = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:3769:14
export const LLVMGetClause = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3772:6
export const LLVMAddClause = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3775:10
export const LLVMIsCleanup = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3778:6
export const LLVMSetCleanup = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3781:6
export const LLVMAddHandler = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3784:10
export const LLVMGetNumHandlers = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:3797:6
export const LLVMGetHandlers = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3802:14
export const LLVMGetArgOperand = {
  parameters: ["pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3805:6
export const LLVMSetArgOperand = {
  parameters: ["pointer", "u32", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3814:14
export const LLVMGetParentCatchSwitch = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3823:6
export const LLVMSetParentCatchSwitch = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3826:14
export const LLVMBuildAdd = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3828:14
export const LLVMBuildNSWAdd = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3830:14
export const LLVMBuildNUWAdd = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3832:14
export const LLVMBuildFAdd = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3834:14
export const LLVMBuildSub = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3836:14
export const LLVMBuildNSWSub = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3838:14
export const LLVMBuildNUWSub = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3840:14
export const LLVMBuildFSub = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3842:14
export const LLVMBuildMul = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3844:14
export const LLVMBuildNSWMul = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3846:14
export const LLVMBuildNUWMul = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3848:14
export const LLVMBuildFMul = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3850:14
export const LLVMBuildUDiv = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3852:14
export const LLVMBuildExactUDiv = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3854:14
export const LLVMBuildSDiv = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3856:14
export const LLVMBuildExactSDiv = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3858:14
export const LLVMBuildFDiv = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3860:14
export const LLVMBuildURem = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3862:14
export const LLVMBuildSRem = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3864:14
export const LLVMBuildFRem = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3866:14
export const LLVMBuildShl = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3868:14
export const LLVMBuildLShr = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3870:14
export const LLVMBuildAShr = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3872:14
export const LLVMBuildAnd = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3874:14
export const LLVMBuildOr = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3876:14
export const LLVMBuildXor = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3878:14
export const LLVMBuildBinOp = {
  parameters: ["pointer", "i32", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3881:14
export const LLVMBuildNeg = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3882:14
export const LLVMBuildNSWNeg = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3884:14
export const LLVMBuildNUWNeg = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3886:14
export const LLVMBuildFNeg = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3887:14
export const LLVMBuildNot = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3890:14
export const LLVMBuildMalloc = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3891:14
export const LLVMBuildArrayMalloc = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3900:14
export const LLVMBuildMemSet = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3908:14
export const LLVMBuildMemCpy = {
  parameters: ["pointer", "pointer", "u32", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3917:14
export const LLVMBuildMemMove = {
  parameters: ["pointer", "pointer", "u32", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3922:14
export const LLVMBuildAlloca = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3923:14
export const LLVMBuildArrayAlloca = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3925:14
export const LLVMBuildFree = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3926:1 <Spelling=/data/./llvm-c/Core.h:3927:18>
export const LLVMBuildLoad = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3930:14
export const LLVMBuildLoad2 = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3932:14
export const LLVMBuildStore = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3933:1 <Spelling=/data/./llvm-c/Core.h:3934:18>
export const LLVMBuildGEP = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3938:1 <Spelling=/data/./llvm-c/Core.h:3939:18>
export const LLVMBuildInBoundsGEP = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3943:1 <Spelling=/data/./llvm-c/Core.h:3944:18>
export const LLVMBuildStructGEP = {
  parameters: ["pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3947:14
export const LLVMBuildGEP2 = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3950:14
export const LLVMBuildInBoundsGEP2 = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3953:14
export const LLVMBuildStructGEP2 = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3956:14
export const LLVMBuildGlobalString = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3958:14
export const LLVMBuildGlobalStringPtr = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3960:10
export const LLVMGetVolatile = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3961:6
export const LLVMSetVolatile = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3962:10
export const LLVMGetWeak = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3963:6
export const LLVMSetWeak = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3964:20
export const LLVMGetOrdering = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3965:6
export const LLVMSetOrdering = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3966:20
export const LLVMGetAtomicRMWBinOp = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:3967:6
export const LLVMSetAtomicRMWBinOp = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:3970:14
export const LLVMBuildTrunc = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3972:14
export const LLVMBuildZExt = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3974:14
export const LLVMBuildSExt = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3976:14
export const LLVMBuildFPToUI = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3978:14
export const LLVMBuildFPToSI = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3980:14
export const LLVMBuildUIToFP = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3982:14
export const LLVMBuildSIToFP = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3984:14
export const LLVMBuildFPTrunc = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3986:14
export const LLVMBuildFPExt = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3988:14
export const LLVMBuildPtrToInt = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3990:14
export const LLVMBuildIntToPtr = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3992:14
export const LLVMBuildBitCast = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3994:14
export const LLVMBuildAddrSpaceCast = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3996:14
export const LLVMBuildZExtOrBitCast = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:3998:14
export const LLVMBuildSExtOrBitCast = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4000:14
export const LLVMBuildTruncOrBitCast = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4002:14
export const LLVMBuildCast = {
  parameters: ["pointer", "i32", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4004:14
export const LLVMBuildPointerCast = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4006:14
export const LLVMBuildIntCast2 = {
  parameters: ["pointer", "pointer", "pointer", "i32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4009:14
export const LLVMBuildFPCast = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4013:14
export const LLVMBuildIntCast = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4016:12
export const LLVMGetCastOpcode = {
  parameters: ["pointer", "i32", "pointer", "i32"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4020:14
export const LLVMBuildICmp = {
  parameters: ["pointer", "i32", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4023:14
export const LLVMBuildFCmp = {
  parameters: ["pointer", "i32", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4028:14
export const LLVMBuildPhi = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4029:1 <Spelling=/data/./llvm-c/Core.h:4030:18>
export const LLVMBuildCall = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4034:14
export const LLVMBuildCall2 = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4037:14
export const LLVMBuildSelect = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4040:14
export const LLVMBuildVAArg = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4042:14
export const LLVMBuildExtractElement = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4044:14
export const LLVMBuildInsertElement = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4047:14
export const LLVMBuildShuffleVector = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4050:14
export const LLVMBuildExtractValue = {
  parameters: ["pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4052:14
export const LLVMBuildInsertValue = {
  parameters: ["pointer", "pointer", "pointer", "u32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4055:14
export const LLVMBuildFreeze = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4058:14
export const LLVMBuildIsNull = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4060:14
export const LLVMBuildIsNotNull = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4062:1 <Spelling=/data/./llvm-c/Core.h:4063:18>
export const LLVMBuildPtrDiff = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4066:14
export const LLVMBuildPtrDiff2 = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4069:14
export const LLVMBuildFence = {
  parameters: ["pointer", "i32", "i32", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4071:14
export const LLVMBuildAtomicRMW = {
  parameters: ["pointer", "i32", "pointer", "pointer", "i32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4075:14
export const LLVMBuildAtomicCmpXchg = {
  parameters: ["pointer", "pointer", "pointer", "pointer", "i32", "i32", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4084:10
export const LLVMGetNumMaskElements = {
  parameters: ["pointer"],
  returnType: "u32"
} as const;

// ./llvm-c/Core.h:4090:5
export const LLVMGetUndefMaskElem = {
  parameters: [],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4099:5
export const LLVMGetMaskValue = {
  parameters: ["pointer", "u32"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4101:10
export const LLVMIsAtomicSingleThread = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4102:6
export const LLVMSetAtomicSingleThread = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:4104:20
export const LLVMGetCmpXchgSuccessOrdering = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4105:6
export const LLVMSetCmpXchgSuccessOrdering = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:4107:20
export const LLVMGetCmpXchgFailureOrdering = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4108:6
export const LLVMSetCmpXchgFailureOrdering = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:4126:1
export const LLVMCreateModuleProviderForExistingModule = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4131:6
export const LLVMDisposeModuleProvider = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:4143:10
export const LLVMCreateMemoryBufferWithContentsOfFile = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4146:10
export const LLVMCreateMemoryBufferWithSTDIN = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4148:21
export const LLVMCreateMemoryBufferWithMemoryRange = {
  parameters: ["pointer", "u64", "pointer", "i32"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4152:21
export const LLVMCreateMemoryBufferWithMemoryRangeCopy = {
  parameters: ["pointer", "u64", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4155:13
export const LLVMGetBufferStart = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4156:8
export const LLVMGetBufferSize = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Core.h:4157:6
export const LLVMDisposeMemoryBuffer = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:4172:21
export const LLVMGetGlobalPassRegistry = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4188:20
export const LLVMCreatePassManager = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4194:20
export const LLVMCreateFunctionPassManagerForModule = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4197:20
export const LLVMCreateFunctionPassManager = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Core.h:4203:10
export const LLVMRunPassManager = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4208:10
export const LLVMInitializeFunctionPassManager = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4214:10
export const LLVMRunFunctionPassManager = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4219:10
export const LLVMFinalizeFunctionPassManager = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4224:6
export const LLVMDisposePassManager = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:4241:10
export const LLVMStartMultithreaded = {
  parameters: [],
  returnType: "i32"
} as const;

// ./llvm-c/Core.h:4245:6
export const LLVMStopMultithreaded = {
  parameters: [],
  returnType: "void"
} as const;

// ./llvm-c/Core.h:4249:10
export const LLVMIsMultithreaded = {
  parameters: [],
  returnType: "i32"
} as const;

// ./llvm-c/Object.h:76:15
export const LLVMCreateBinary = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:86:6
export const LLVMDisposeBinary = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Object.h:97:21
export const LLVMBinaryCopyMemoryBuffer = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:104:16
export const LLVMBinaryGetType = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Object.h:117:15
export const LLVMMachOUniversalBinaryCopyObjectForArch = {
  parameters: ["pointer", "pointer", "u64", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:133:24
export const LLVMObjectFileCopySectionIterator = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:140:10
export const LLVMObjectFileIsSectionIteratorAtEnd = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Object.h:154:23
export const LLVMObjectFileCopySymbolIterator = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:161:10
export const LLVMObjectFileIsSymbolIteratorAtEnd = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Object.h:164:6
export const LLVMDisposeSectionIterator = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Object.h:166:6
export const LLVMMoveToNextSection = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Object.h:167:6
export const LLVMMoveToContainingSection = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Object.h:171:6
export const LLVMDisposeSymbolIterator = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Object.h:172:6
export const LLVMMoveToNextSymbol = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Object.h:175:13
export const LLVMGetSectionName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:176:10
export const LLVMGetSectionSize = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Object.h:177:13
export const LLVMGetSectionContents = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:178:10
export const LLVMGetSectionAddress = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Object.h:179:10
export const LLVMGetSectionContainsSymbol = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Object.h:183:27
export const LLVMGetRelocations = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:184:6
export const LLVMDisposeRelocationIterator = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Object.h:185:10
export const LLVMIsRelocationIteratorAtEnd = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Object.h:187:6
export const LLVMMoveToNextRelocation = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Object.h:191:13
export const LLVMGetSymbolName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:192:10
export const LLVMGetSymbolAddress = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Object.h:193:10
export const LLVMGetSymbolSize = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Object.h:196:10
export const LLVMGetRelocationOffset = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Object.h:197:23
export const LLVMGetRelocationSymbol = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:198:10
export const LLVMGetRelocationType = {
  parameters: ["pointer"],
  returnType: "u64"
} as const;

// ./llvm-c/Object.h:201:13
export const LLVMGetRelocationTypeName = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:202:13
export const LLVMGetRelocationValueString = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:208:19
export const LLVMCreateObjectFile = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:211:6
export const LLVMDisposeObjectFile = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Object.h:214:24
export const LLVMGetSections = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:217:10
export const LLVMIsSectionIteratorAtEnd = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Object.h:221:23
export const LLVMGetSymbols = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Object.h:224:10
export const LLVMIsSymbolIteratorAtEnd = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/BitWriter.h:37:5
export const LLVMWriteBitcodeToFile = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/BitWriter.h:40:5
export const LLVMWriteBitcodeToFD = {
  parameters: ["pointer", "i32", "i32", "i32"],
  returnType: "i32"
} as const;

// ./llvm-c/BitWriter.h:45:5
export const LLVMWriteBitcodeToFileHandle = {
  parameters: ["pointer", "i32"],
  returnType: "i32"
} as const;

// ./llvm-c/BitWriter.h:48:21
export const LLVMWriteBitcodeToMemoryBuffer = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:74:24
export const LLVMOrcCreateLLJITBuilder = {
  parameters: [],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:81:6
export const LLVMOrcDisposeLLJITBuilder = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/LLJIT.h:92:6
export const LLVMOrcLLJITBuilderSetJITTargetMachineBuilder = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/LLJIT.h:98:6
export const LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator = {
  parameters: ["pointer", "function", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/LLJIT.h:116:14
export const LLVMOrcCreateLLJIT = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:122:14
export const LLVMOrcDisposeLLJIT = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:130:28
export const LLVMOrcLLJITGetExecutionSession = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:138:20
export const LLVMOrcLLJITGetMainJITDylib = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:144:13
export const LLVMOrcLLJITGetTripleString = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:149:6
export const LLVMOrcLLJITGetGlobalPrefix = {
  parameters: ["pointer"],
  returnType: "u8"
} as const;

// ./llvm-c/LLJIT.h:159:1
export const LLVMOrcLLJITMangleAndIntern = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:170:14
export const LLVMOrcLLJITAddObjectFile = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:182:14
export const LLVMOrcLLJITAddObjectFileWithRT = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:195:14
export const LLVMOrcLLJITAddLLVMIRModule = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:208:14
export const LLVMOrcLLJITAddLLVMIRModuleWithRT = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:217:14
export const LLVMOrcLLJITLookup = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:224:23
export const LLVMOrcLLJITGetObjLinkingLayer = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:230:1
export const LLVMOrcLLJITGetObjTransformLayer = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:235:28
export const LLVMOrcLLJITGetIRTransformLayer = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/LLJIT.h:243:13
export const LLVMOrcLLJITGetDataLayoutStr = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Support.h:35:10
export const LLVMLoadLibraryPermanently = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Support.h:45:6
export const LLVMParseCommandLineOptions = {
  parameters: ["i32", "pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Support.h:55:7
export const LLVMSearchForAddressOfSymbol = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Support.h:64:6
export const LLVMAddSymbol = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:34:6
export const LLVMInitializeTransformUtils = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:35:6
export const LLVMInitializeScalarOpts = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:36:6
export const LLVMInitializeObjCARCOpts = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:37:6
export const LLVMInitializeVectorization = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:38:6
export const LLVMInitializeInstCombine = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:39:6
export const LLVMInitializeAggressiveInstCombiner = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:40:6
export const LLVMInitializeIPO = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:41:6
export const LLVMInitializeInstrumentation = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:42:6
export const LLVMInitializeAnalysis = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:43:6
export const LLVMInitializeIPA = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:44:6
export const LLVMInitializeCodeGen = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Initialization.h:45:6
export const LLVMInitializeTarget = {
  parameters: ["pointer"],
  returnType: "void"
} as const;

// ./llvm-c/BitReader.h:39:10
export const LLVMParseBitcode = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/BitReader.h:44:10
export const LLVMParseBitcode2 = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/BitReader.h:48:10
export const LLVMParseBitcodeInContext = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/BitReader.h:52:10
export const LLVMParseBitcodeInContext2 = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/BitReader.h:60:10
export const LLVMGetBitcodeModuleInContext = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/BitReader.h:71:10
export const LLVMGetBitcodeModuleInContext2 = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/BitReader.h:76:10
export const LLVMGetBitcodeModule = {
  parameters: ["pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/BitReader.h:79:10
export const LLVMGetBitcodeModule2 = {
  parameters: ["pointer", "pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Comdat.h:46:15
export const LLVMGetOrInsertComdat = {
  parameters: ["pointer", "pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Comdat.h:53:15
export const LLVMGetComdat = {
  parameters: ["pointer"],
  returnType: "pointer"
} as const;

// ./llvm-c/Comdat.h:60:6
export const LLVMSetComdat = {
  parameters: ["pointer", "pointer"],
  returnType: "void"
} as const;

// ./llvm-c/Comdat.h:67:25
export const LLVMGetComdatSelectionKind = {
  parameters: ["pointer"],
  returnType: "i32"
} as const;

// ./llvm-c/Comdat.h:74:6
export const LLVMSetComdatSelectionKind = {
  parameters: ["pointer", "i32"],
  returnType: "void"
} as const;

// ./llvm-c/IRReader.h:38:10
export const LLVMParseIRInContext = {
  parameters: ["pointer", "pointer", "pointer", "pointer"],
  returnType: "i32"
} as const;
