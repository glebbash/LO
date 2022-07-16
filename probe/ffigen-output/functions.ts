// ./llvm-c/Analysis.h:44:10
export const LLVMVerifyModule = {
  "tag": "function",
  "name": "LLVMVerifyModule",
  "ns": 0,
  "location": "/data/./llvm-c/Analysis.h:44:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Action",
      "type": {
        "tag": "LLVMVerifierFailureAction"
      }
    },
    {
      "tag": "parameter",
      "name": "OutMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Analysis.h:49:10
export const LLVMVerifyFunction = {
  "tag": "function",
  "name": "LLVMVerifyFunction",
  "ns": 0,
  "location": "/data/./llvm-c/Analysis.h:49:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Action",
      "type": {
        "tag": "LLVMVerifierFailureAction"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Analysis.h:53:6
export const LLVMViewFunctionCFG = {
  "tag": "function",
  "name": "LLVMViewFunctionCFG",
  "ns": 0,
  "location": "/data/./llvm-c/Analysis.h:53:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Analysis.h:54:6
export const LLVMViewFunctionCFGOnly = {
  "tag": "function",
  "name": "LLVMViewFunctionCFGOnly",
  "ns": 0,
  "location": "/data/./llvm-c/Analysis.h:54:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:26:1 <Spelling=<scratch space>:53:1>
export const LLVMInitializeAArch64TargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeAArch64TargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:26:1 <Spelling=<scratch space>:53:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:27:1 <Spelling=<scratch space>:55:1>
export const LLVMInitializeAMDGPUTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeAMDGPUTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:27:1 <Spelling=<scratch space>:55:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:28:1 <Spelling=<scratch space>:57:1>
export const LLVMInitializeARMTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeARMTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:28:1 <Spelling=<scratch space>:57:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:29:1 <Spelling=<scratch space>:59:1>
export const LLVMInitializeAVRTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeAVRTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:29:1 <Spelling=<scratch space>:59:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:30:1 <Spelling=<scratch space>:61:1>
export const LLVMInitializeBPFTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeBPFTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:30:1 <Spelling=<scratch space>:61:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:31:1 <Spelling=<scratch space>:63:1>
export const LLVMInitializeHexagonTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeHexagonTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:31:1 <Spelling=<scratch space>:63:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:32:1 <Spelling=<scratch space>:65:1>
export const LLVMInitializeLanaiTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeLanaiTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:32:1 <Spelling=<scratch space>:65:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:33:1 <Spelling=<scratch space>:67:1>
export const LLVMInitializeMipsTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeMipsTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:33:1 <Spelling=<scratch space>:67:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:34:1 <Spelling=<scratch space>:69:1>
export const LLVMInitializeMSP430TargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeMSP430TargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:34:1 <Spelling=<scratch space>:69:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:35:1 <Spelling=<scratch space>:71:1>
export const LLVMInitializeNVPTXTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeNVPTXTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:35:1 <Spelling=<scratch space>:71:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:36:1 <Spelling=<scratch space>:73:1>
export const LLVMInitializePowerPCTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializePowerPCTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:36:1 <Spelling=<scratch space>:73:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:37:1 <Spelling=<scratch space>:75:1>
export const LLVMInitializeRISCVTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeRISCVTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:37:1 <Spelling=<scratch space>:75:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:38:1 <Spelling=<scratch space>:77:1>
export const LLVMInitializeSparcTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeSparcTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:38:1 <Spelling=<scratch space>:77:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:39:1 <Spelling=<scratch space>:79:1>
export const LLVMInitializeSystemZTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeSystemZTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:39:1 <Spelling=<scratch space>:79:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:40:1 <Spelling=<scratch space>:81:1>
export const LLVMInitializeVETargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeVETargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:40:1 <Spelling=<scratch space>:81:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:83:1>
export const LLVMInitializeWebAssemblyTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeWebAssemblyTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:83:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:85:1>
export const LLVMInitializeX86TargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeX86TargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:85:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:43:1 <Spelling=<scratch space>:87:1>
export const LLVMInitializeXCoreTargetInfo = {
  "tag": "function",
  "name": "LLVMInitializeXCoreTargetInfo",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:43:1 <Spelling=<scratch space>:87:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:26:1 <Spelling=<scratch space>:89:1>
export const LLVMInitializeAArch64Target = {
  "tag": "function",
  "name": "LLVMInitializeAArch64Target",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:26:1 <Spelling=<scratch space>:89:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:27:1 <Spelling=<scratch space>:91:1>
export const LLVMInitializeAMDGPUTarget = {
  "tag": "function",
  "name": "LLVMInitializeAMDGPUTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:27:1 <Spelling=<scratch space>:91:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:28:1 <Spelling=<scratch space>:93:1>
export const LLVMInitializeARMTarget = {
  "tag": "function",
  "name": "LLVMInitializeARMTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:28:1 <Spelling=<scratch space>:93:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:29:1 <Spelling=<scratch space>:95:1>
export const LLVMInitializeAVRTarget = {
  "tag": "function",
  "name": "LLVMInitializeAVRTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:29:1 <Spelling=<scratch space>:95:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:30:1 <Spelling=<scratch space>:97:1>
export const LLVMInitializeBPFTarget = {
  "tag": "function",
  "name": "LLVMInitializeBPFTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:30:1 <Spelling=<scratch space>:97:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:31:1 <Spelling=<scratch space>:99:1>
export const LLVMInitializeHexagonTarget = {
  "tag": "function",
  "name": "LLVMInitializeHexagonTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:31:1 <Spelling=<scratch space>:99:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:32:1 <Spelling=<scratch space>:101:1>
export const LLVMInitializeLanaiTarget = {
  "tag": "function",
  "name": "LLVMInitializeLanaiTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:32:1 <Spelling=<scratch space>:101:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:33:1 <Spelling=<scratch space>:103:1>
export const LLVMInitializeMipsTarget = {
  "tag": "function",
  "name": "LLVMInitializeMipsTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:33:1 <Spelling=<scratch space>:103:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:34:1 <Spelling=<scratch space>:105:1>
export const LLVMInitializeMSP430Target = {
  "tag": "function",
  "name": "LLVMInitializeMSP430Target",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:34:1 <Spelling=<scratch space>:105:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:35:1 <Spelling=<scratch space>:107:1>
export const LLVMInitializeNVPTXTarget = {
  "tag": "function",
  "name": "LLVMInitializeNVPTXTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:35:1 <Spelling=<scratch space>:107:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:36:1 <Spelling=<scratch space>:109:1>
export const LLVMInitializePowerPCTarget = {
  "tag": "function",
  "name": "LLVMInitializePowerPCTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:36:1 <Spelling=<scratch space>:109:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:37:1 <Spelling=<scratch space>:111:1>
export const LLVMInitializeRISCVTarget = {
  "tag": "function",
  "name": "LLVMInitializeRISCVTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:37:1 <Spelling=<scratch space>:111:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:38:1 <Spelling=<scratch space>:113:1>
export const LLVMInitializeSparcTarget = {
  "tag": "function",
  "name": "LLVMInitializeSparcTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:38:1 <Spelling=<scratch space>:113:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:39:1 <Spelling=<scratch space>:115:1>
export const LLVMInitializeSystemZTarget = {
  "tag": "function",
  "name": "LLVMInitializeSystemZTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:39:1 <Spelling=<scratch space>:115:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:40:1 <Spelling=<scratch space>:117:1>
export const LLVMInitializeVETarget = {
  "tag": "function",
  "name": "LLVMInitializeVETarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:40:1 <Spelling=<scratch space>:117:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:119:1>
export const LLVMInitializeWebAssemblyTarget = {
  "tag": "function",
  "name": "LLVMInitializeWebAssemblyTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:119:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:121:1>
export const LLVMInitializeX86Target = {
  "tag": "function",
  "name": "LLVMInitializeX86Target",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:121:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:43:1 <Spelling=<scratch space>:123:1>
export const LLVMInitializeXCoreTarget = {
  "tag": "function",
  "name": "LLVMInitializeXCoreTarget",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:43:1 <Spelling=<scratch space>:123:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:26:1 <Spelling=<scratch space>:125:1>
export const LLVMInitializeAArch64TargetMC = {
  "tag": "function",
  "name": "LLVMInitializeAArch64TargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:26:1 <Spelling=<scratch space>:125:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:27:1 <Spelling=<scratch space>:127:1>
export const LLVMInitializeAMDGPUTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeAMDGPUTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:27:1 <Spelling=<scratch space>:127:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:28:1 <Spelling=<scratch space>:129:1>
export const LLVMInitializeARMTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeARMTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:28:1 <Spelling=<scratch space>:129:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:29:1 <Spelling=<scratch space>:131:1>
export const LLVMInitializeAVRTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeAVRTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:29:1 <Spelling=<scratch space>:131:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:30:1 <Spelling=<scratch space>:133:1>
export const LLVMInitializeBPFTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeBPFTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:30:1 <Spelling=<scratch space>:133:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:31:1 <Spelling=<scratch space>:135:1>
export const LLVMInitializeHexagonTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeHexagonTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:31:1 <Spelling=<scratch space>:135:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:32:1 <Spelling=<scratch space>:137:1>
export const LLVMInitializeLanaiTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeLanaiTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:32:1 <Spelling=<scratch space>:137:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:33:1 <Spelling=<scratch space>:139:1>
export const LLVMInitializeMipsTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeMipsTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:33:1 <Spelling=<scratch space>:139:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:34:1 <Spelling=<scratch space>:141:1>
export const LLVMInitializeMSP430TargetMC = {
  "tag": "function",
  "name": "LLVMInitializeMSP430TargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:34:1 <Spelling=<scratch space>:141:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:35:1 <Spelling=<scratch space>:143:1>
export const LLVMInitializeNVPTXTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeNVPTXTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:35:1 <Spelling=<scratch space>:143:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:36:1 <Spelling=<scratch space>:145:1>
export const LLVMInitializePowerPCTargetMC = {
  "tag": "function",
  "name": "LLVMInitializePowerPCTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:36:1 <Spelling=<scratch space>:145:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:37:1 <Spelling=<scratch space>:147:1>
export const LLVMInitializeRISCVTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeRISCVTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:37:1 <Spelling=<scratch space>:147:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:38:1 <Spelling=<scratch space>:149:1>
export const LLVMInitializeSparcTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeSparcTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:38:1 <Spelling=<scratch space>:149:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:39:1 <Spelling=<scratch space>:151:1>
export const LLVMInitializeSystemZTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeSystemZTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:39:1 <Spelling=<scratch space>:151:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:40:1 <Spelling=<scratch space>:153:1>
export const LLVMInitializeVETargetMC = {
  "tag": "function",
  "name": "LLVMInitializeVETargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:40:1 <Spelling=<scratch space>:153:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:155:1>
export const LLVMInitializeWebAssemblyTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeWebAssemblyTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:41:1 <Spelling=<scratch space>:155:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:157:1>
export const LLVMInitializeX86TargetMC = {
  "tag": "function",
  "name": "LLVMInitializeX86TargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:42:1 <Spelling=<scratch space>:157:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Targets.def:43:1 <Spelling=<scratch space>:159:1>
export const LLVMInitializeXCoreTargetMC = {
  "tag": "function",
  "name": "LLVMInitializeXCoreTargetMC",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Targets.def:43:1 <Spelling=<scratch space>:159:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:27:1 <Spelling=<scratch space>:161:1>
export const LLVMInitializeAArch64AsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeAArch64AsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:27:1 <Spelling=<scratch space>:161:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:28:1 <Spelling=<scratch space>:163:1>
export const LLVMInitializeAMDGPUAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeAMDGPUAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:28:1 <Spelling=<scratch space>:163:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:29:1 <Spelling=<scratch space>:165:1>
export const LLVMInitializeARMAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeARMAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:29:1 <Spelling=<scratch space>:165:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:30:1 <Spelling=<scratch space>:167:1>
export const LLVMInitializeAVRAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeAVRAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:30:1 <Spelling=<scratch space>:167:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:31:1 <Spelling=<scratch space>:169:1>
export const LLVMInitializeBPFAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeBPFAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:31:1 <Spelling=<scratch space>:169:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:32:1 <Spelling=<scratch space>:171:1>
export const LLVMInitializeHexagonAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeHexagonAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:32:1 <Spelling=<scratch space>:171:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:33:1 <Spelling=<scratch space>:173:1>
export const LLVMInitializeLanaiAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeLanaiAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:33:1 <Spelling=<scratch space>:173:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:34:1 <Spelling=<scratch space>:175:1>
export const LLVMInitializeMipsAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeMipsAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:34:1 <Spelling=<scratch space>:175:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:35:1 <Spelling=<scratch space>:177:1>
export const LLVMInitializeMSP430AsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeMSP430AsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:35:1 <Spelling=<scratch space>:177:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:36:1 <Spelling=<scratch space>:179:1>
export const LLVMInitializeNVPTXAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeNVPTXAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:36:1 <Spelling=<scratch space>:179:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:37:1 <Spelling=<scratch space>:181:1>
export const LLVMInitializePowerPCAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializePowerPCAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:37:1 <Spelling=<scratch space>:181:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:38:1 <Spelling=<scratch space>:183:1>
export const LLVMInitializeRISCVAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeRISCVAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:38:1 <Spelling=<scratch space>:183:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:39:1 <Spelling=<scratch space>:185:1>
export const LLVMInitializeSparcAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeSparcAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:39:1 <Spelling=<scratch space>:185:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:40:1 <Spelling=<scratch space>:187:1>
export const LLVMInitializeSystemZAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeSystemZAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:40:1 <Spelling=<scratch space>:187:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:41:1 <Spelling=<scratch space>:189:1>
export const LLVMInitializeVEAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeVEAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:41:1 <Spelling=<scratch space>:189:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:42:1 <Spelling=<scratch space>:2:1>
export const LLVMInitializeWebAssemblyAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeWebAssemblyAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:42:1 <Spelling=<scratch space>:2:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:43:1 <Spelling=<scratch space>:4:1>
export const LLVMInitializeX86AsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeX86AsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:43:1 <Spelling=<scratch space>:4:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmPrinters.def:44:1 <Spelling=<scratch space>:6:1>
export const LLVMInitializeXCoreAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeXCoreAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmPrinters.def:44:1 <Spelling=<scratch space>:6:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:27:1 <Spelling=<scratch space>:8:1>
export const LLVMInitializeAArch64AsmParser = {
  "tag": "function",
  "name": "LLVMInitializeAArch64AsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:27:1 <Spelling=<scratch space>:8:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:28:1 <Spelling=<scratch space>:10:1>
export const LLVMInitializeAMDGPUAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeAMDGPUAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:28:1 <Spelling=<scratch space>:10:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:29:1 <Spelling=<scratch space>:12:1>
export const LLVMInitializeARMAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeARMAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:29:1 <Spelling=<scratch space>:12:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:30:1 <Spelling=<scratch space>:14:1>
export const LLVMInitializeAVRAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeAVRAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:30:1 <Spelling=<scratch space>:14:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:31:1 <Spelling=<scratch space>:16:1>
export const LLVMInitializeBPFAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeBPFAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:31:1 <Spelling=<scratch space>:16:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:32:1 <Spelling=<scratch space>:18:1>
export const LLVMInitializeHexagonAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeHexagonAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:32:1 <Spelling=<scratch space>:18:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:33:1 <Spelling=<scratch space>:20:1>
export const LLVMInitializeLanaiAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeLanaiAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:33:1 <Spelling=<scratch space>:20:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:34:1 <Spelling=<scratch space>:22:1>
export const LLVMInitializeMipsAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeMipsAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:34:1 <Spelling=<scratch space>:22:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:35:1 <Spelling=<scratch space>:24:1>
export const LLVMInitializeMSP430AsmParser = {
  "tag": "function",
  "name": "LLVMInitializeMSP430AsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:35:1 <Spelling=<scratch space>:24:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:36:1 <Spelling=<scratch space>:26:1>
export const LLVMInitializePowerPCAsmParser = {
  "tag": "function",
  "name": "LLVMInitializePowerPCAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:36:1 <Spelling=<scratch space>:26:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:37:1 <Spelling=<scratch space>:28:1>
export const LLVMInitializeRISCVAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeRISCVAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:37:1 <Spelling=<scratch space>:28:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:38:1 <Spelling=<scratch space>:30:1>
export const LLVMInitializeSparcAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeSparcAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:38:1 <Spelling=<scratch space>:30:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:39:1 <Spelling=<scratch space>:32:1>
export const LLVMInitializeSystemZAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeSystemZAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:39:1 <Spelling=<scratch space>:32:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:40:1 <Spelling=<scratch space>:34:1>
export const LLVMInitializeVEAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeVEAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:40:1 <Spelling=<scratch space>:34:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:41:1 <Spelling=<scratch space>:36:1>
export const LLVMInitializeWebAssemblyAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeWebAssemblyAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:41:1 <Spelling=<scratch space>:36:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/AsmParsers.def:42:1 <Spelling=<scratch space>:38:1>
export const LLVMInitializeX86AsmParser = {
  "tag": "function",
  "name": "LLVMInitializeX86AsmParser",
  "ns": 0,
  "location": "/usr/include/llvm/Config/AsmParsers.def:42:1 <Spelling=<scratch space>:38:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:27:1 <Spelling=<scratch space>:40:1>
export const LLVMInitializeAArch64Disassembler = {
  "tag": "function",
  "name": "LLVMInitializeAArch64Disassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:27:1 <Spelling=<scratch space>:40:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:28:1 <Spelling=<scratch space>:42:1>
export const LLVMInitializeAMDGPUDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeAMDGPUDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:28:1 <Spelling=<scratch space>:42:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:29:1 <Spelling=<scratch space>:44:1>
export const LLVMInitializeARMDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeARMDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:29:1 <Spelling=<scratch space>:44:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:30:1 <Spelling=<scratch space>:46:1>
export const LLVMInitializeAVRDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeAVRDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:30:1 <Spelling=<scratch space>:46:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:31:1 <Spelling=<scratch space>:48:1>
export const LLVMInitializeBPFDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeBPFDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:31:1 <Spelling=<scratch space>:48:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:32:1 <Spelling=<scratch space>:50:1>
export const LLVMInitializeHexagonDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeHexagonDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:32:1 <Spelling=<scratch space>:50:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:33:1 <Spelling=<scratch space>:52:1>
export const LLVMInitializeLanaiDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeLanaiDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:33:1 <Spelling=<scratch space>:52:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:34:1 <Spelling=<scratch space>:54:1>
export const LLVMInitializeMipsDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeMipsDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:34:1 <Spelling=<scratch space>:54:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:35:1 <Spelling=<scratch space>:56:1>
export const LLVMInitializeMSP430Disassembler = {
  "tag": "function",
  "name": "LLVMInitializeMSP430Disassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:35:1 <Spelling=<scratch space>:56:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:36:1 <Spelling=<scratch space>:58:1>
export const LLVMInitializePowerPCDisassembler = {
  "tag": "function",
  "name": "LLVMInitializePowerPCDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:36:1 <Spelling=<scratch space>:58:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:37:1 <Spelling=<scratch space>:60:1>
export const LLVMInitializeRISCVDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeRISCVDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:37:1 <Spelling=<scratch space>:60:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:38:1 <Spelling=<scratch space>:62:1>
export const LLVMInitializeSparcDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeSparcDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:38:1 <Spelling=<scratch space>:62:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:39:1 <Spelling=<scratch space>:64:1>
export const LLVMInitializeSystemZDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeSystemZDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:39:1 <Spelling=<scratch space>:64:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:40:1 <Spelling=<scratch space>:66:1>
export const LLVMInitializeVEDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeVEDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:40:1 <Spelling=<scratch space>:66:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:41:1 <Spelling=<scratch space>:68:1>
export const LLVMInitializeWebAssemblyDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeWebAssemblyDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:41:1 <Spelling=<scratch space>:68:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:42:1 <Spelling=<scratch space>:70:1>
export const LLVMInitializeX86Disassembler = {
  "tag": "function",
  "name": "LLVMInitializeX86Disassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:42:1 <Spelling=<scratch space>:70:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// /usr/include/llvm/Config/Disassemblers.def:43:1 <Spelling=<scratch space>:72:1>
export const LLVMInitializeXCoreDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeXCoreDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm/Config/Disassemblers.def:43:1 <Spelling=<scratch space>:72:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:76:20
export const LLVMInitializeAllTargetInfos = {
  "tag": "function",
  "name": "LLVMInitializeAllTargetInfos",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:76:20",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:85:20
export const LLVMInitializeAllTargets = {
  "tag": "function",
  "name": "LLVMInitializeAllTargets",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:85:20",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:94:20
export const LLVMInitializeAllTargetMCs = {
  "tag": "function",
  "name": "LLVMInitializeAllTargetMCs",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:94:20",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:103:20
export const LLVMInitializeAllAsmPrinters = {
  "tag": "function",
  "name": "LLVMInitializeAllAsmPrinters",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:103:20",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:112:20
export const LLVMInitializeAllAsmParsers = {
  "tag": "function",
  "name": "LLVMInitializeAllAsmParsers",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:112:20",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:121:20
export const LLVMInitializeAllDisassemblers = {
  "tag": "function",
  "name": "LLVMInitializeAllDisassemblers",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:121:20",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:131:24
export const LLVMInitializeNativeTarget = {
  "tag": "function",
  "name": "LLVMInitializeNativeTarget",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:131:24",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Target.h:146:24
export const LLVMInitializeNativeAsmParser = {
  "tag": "function",
  "name": "LLVMInitializeNativeAsmParser",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:146:24",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Target.h:158:24
export const LLVMInitializeNativeAsmPrinter = {
  "tag": "function",
  "name": "LLVMInitializeNativeAsmPrinter",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:158:24",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Target.h:170:24
export const LLVMInitializeNativeDisassembler = {
  "tag": "function",
  "name": "LLVMInitializeNativeDisassembler",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:170:24",
  "variadic": false,
  "inline": true,
  "storage-class": "static",
  "parameters": [],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Target.h:186:19
export const LLVMGetModuleDataLayout = {
  "tag": "function",
  "name": "LLVMGetModuleDataLayout",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:186:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTargetDataRef"
  }
};

// ./llvm-c/Target.h:193:6
export const LLVMSetModuleDataLayout = {
  "tag": "function",
  "name": "LLVMSetModuleDataLayout",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:193:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DL",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:197:19
export const LLVMCreateTargetData = {
  "tag": "function",
  "name": "LLVMCreateTargetData",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:197:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "StringRep",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTargetDataRef"
  }
};

// ./llvm-c/Target.h:201:6
export const LLVMDisposeTargetData = {
  "tag": "function",
  "name": "LLVMDisposeTargetData",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:201:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:206:6
export const LLVMAddTargetLibraryInfo = {
  "tag": "function",
  "name": "LLVMAddTargetLibraryInfo",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:206:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TLI",
      "type": {
        "tag": "LLVMTargetLibraryInfoRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Target.h:212:7
export const LLVMCopyStringRepOfTargetData = {
  "tag": "function",
  "name": "LLVMCopyStringRepOfTargetData",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:212:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Target.h:217:23
export const LLVMByteOrder = {
  "tag": "function",
  "name": "LLVMByteOrder",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:217:23",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":enum",
    "name": "LLVMByteOrdering",
    "id": 0
  }
};

// ./llvm-c/Target.h:221:10
export const LLVMPointerSize = {
  "tag": "function",
  "name": "LLVMPointerSize",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:221:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Target.h:226:10
export const LLVMPointerSizeForAS = {
  "tag": "function",
  "name": "LLVMPointerSizeForAS",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:226:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AS",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Target.h:230:13
export const LLVMIntPtrType = {
  "tag": "function",
  "name": "LLVMIntPtrType",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:230:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Target.h:235:13
export const LLVMIntPtrTypeForAS = {
  "tag": "function",
  "name": "LLVMIntPtrTypeForAS",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:235:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AS",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Target.h:239:13
export const LLVMIntPtrTypeInContext = {
  "tag": "function",
  "name": "LLVMIntPtrTypeInContext",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:239:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Target.h:244:13
export const LLVMIntPtrTypeForASInContext = {
  "tag": "function",
  "name": "LLVMIntPtrTypeForASInContext",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:244:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AS",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Target.h:249:20
export const LLVMSizeOfTypeInBits = {
  "tag": "function",
  "name": "LLVMSizeOfTypeInBits",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:249:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-long-long",
    "bit-size": 64,
    "bit-alignment": 64
  }
};

// ./llvm-c/Target.h:253:20
export const LLVMStoreSizeOfType = {
  "tag": "function",
  "name": "LLVMStoreSizeOfType",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:253:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-long-long",
    "bit-size": 64,
    "bit-alignment": 64
  }
};

// ./llvm-c/Target.h:257:20
export const LLVMABISizeOfType = {
  "tag": "function",
  "name": "LLVMABISizeOfType",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:257:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-long-long",
    "bit-size": 64,
    "bit-alignment": 64
  }
};

// ./llvm-c/Target.h:261:10
export const LLVMABIAlignmentOfType = {
  "tag": "function",
  "name": "LLVMABIAlignmentOfType",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:261:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Target.h:265:10
export const LLVMCallFrameAlignmentOfType = {
  "tag": "function",
  "name": "LLVMCallFrameAlignmentOfType",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:265:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Target.h:269:10
export const LLVMPreferredAlignmentOfType = {
  "tag": "function",
  "name": "LLVMPreferredAlignmentOfType",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:269:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Target.h:273:10
export const LLVMPreferredAlignmentOfGlobal = {
  "tag": "function",
  "name": "LLVMPreferredAlignmentOfGlobal",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:273:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Target.h:278:10
export const LLVMElementAtOffset = {
  "tag": "function",
  "name": "LLVMElementAtOffset",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:278:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Offset",
      "type": {
        "tag": ":unsigned-long-long",
        "bit-size": 64,
        "bit-alignment": 64
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Target.h:283:20
export const LLVMOffsetOfElement = {
  "tag": "function",
  "name": "LLVMOffsetOfElement",
  "ns": 0,
  "location": "/usr/include/llvm-c/Target.h:283:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TD",
      "type": {
        "tag": "LLVMTargetDataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Element",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-long-long",
    "bit-size": 64,
    "bit-alignment": 64
  }
};

// ./llvm-c/TargetMachine.h:70:15
export const LLVMGetFirstTarget = {
  "tag": "function",
  "name": "LLVMGetFirstTarget",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:70:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTargetRef"
  }
};

// ./llvm-c/TargetMachine.h:72:15
export const LLVMGetNextTarget = {
  "tag": "function",
  "name": "LLVMGetNextTarget",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:72:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTargetRef"
  }
};

// ./llvm-c/TargetMachine.h:77:15
export const LLVMGetTargetFromName = {
  "tag": "function",
  "name": "LLVMGetTargetFromName",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:77:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTargetRef"
  }
};

// ./llvm-c/TargetMachine.h:82:10
export const LLVMGetTargetFromTriple = {
  "tag": "function",
  "name": "LLVMGetTargetFromTriple",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:82:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Triple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTargetRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ErrorMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/TargetMachine.h:86:13
export const LLVMGetTargetName = {
  "tag": "function",
  "name": "LLVMGetTargetName",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:86:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/TargetMachine.h:89:13
export const LLVMGetTargetDescription = {
  "tag": "function",
  "name": "LLVMGetTargetDescription",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:89:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/TargetMachine.h:92:10
export const LLVMTargetHasJIT = {
  "tag": "function",
  "name": "LLVMTargetHasJIT",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:92:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/TargetMachine.h:95:10
export const LLVMTargetHasTargetMachine = {
  "tag": "function",
  "name": "LLVMTargetHasTargetMachine",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:95:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/TargetMachine.h:98:10
export const LLVMTargetHasAsmBackend = {
  "tag": "function",
  "name": "LLVMTargetHasAsmBackend",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:98:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/TargetMachine.h:102:22
export const LLVMCreateTargetMachine = {
  "tag": "function",
  "name": "LLVMCreateTargetMachine",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:102:22",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Triple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "CPU",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Features",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Level",
      "type": {
        "tag": "LLVMCodeGenOptLevel"
      }
    },
    {
      "tag": "parameter",
      "name": "Reloc",
      "type": {
        "tag": "LLVMRelocMode"
      }
    },
    {
      "tag": "parameter",
      "name": "CodeModel",
      "type": {
        "tag": "LLVMCodeModel"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTargetMachineRef"
  }
};

// ./llvm-c/TargetMachine.h:108:6
export const LLVMDisposeTargetMachine = {
  "tag": "function",
  "name": "LLVMDisposeTargetMachine",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:108:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/TargetMachine.h:111:15
export const LLVMGetTargetMachineTarget = {
  "tag": "function",
  "name": "LLVMGetTargetMachineTarget",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:111:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTargetRef"
  }
};

// ./llvm-c/TargetMachine.h:116:7
export const LLVMGetTargetMachineTriple = {
  "tag": "function",
  "name": "LLVMGetTargetMachineTriple",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:116:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/TargetMachine.h:121:7
export const LLVMGetTargetMachineCPU = {
  "tag": "function",
  "name": "LLVMGetTargetMachineCPU",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:121:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/TargetMachine.h:126:7
export const LLVMGetTargetMachineFeatureString = {
  "tag": "function",
  "name": "LLVMGetTargetMachineFeatureString",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:126:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/TargetMachine.h:129:19
export const LLVMCreateTargetDataLayout = {
  "tag": "function",
  "name": "LLVMCreateTargetDataLayout",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:129:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTargetDataRef"
  }
};

// ./llvm-c/TargetMachine.h:132:6
export const LLVMSetTargetMachineAsmVerbosity = {
  "tag": "function",
  "name": "LLVMSetTargetMachineAsmVerbosity",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:132:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "VerboseAsm",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/TargetMachine.h:138:10
export const LLVMTargetMachineEmitToFile = {
  "tag": "function",
  "name": "LLVMTargetMachineEmitToFile",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:138:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Filename",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "codegen",
      "type": {
        "tag": "LLVMCodeGenFileType"
      }
    },
    {
      "tag": "parameter",
      "name": "ErrorMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/TargetMachine.h:142:10
export const LLVMTargetMachineEmitToMemoryBuffer = {
  "tag": "function",
  "name": "LLVMTargetMachineEmitToMemoryBuffer",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:142:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "codegen",
      "type": {
        "tag": "LLVMCodeGenFileType"
      }
    },
    {
      "tag": "parameter",
      "name": "ErrorMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutMemBuf",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMemoryBufferRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/TargetMachine.h:148:7
export const LLVMGetDefaultTargetTriple = {
  "tag": "function",
  "name": "LLVMGetDefaultTargetTriple",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:148:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/TargetMachine.h:152:7
export const LLVMNormalizeTargetTriple = {
  "tag": "function",
  "name": "LLVMNormalizeTargetTriple",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:152:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "triple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/TargetMachine.h:156:7
export const LLVMGetHostCPUName = {
  "tag": "function",
  "name": "LLVMGetHostCPUName",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:156:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/TargetMachine.h:160:7
export const LLVMGetHostCPUFeatures = {
  "tag": "function",
  "name": "LLVMGetHostCPUFeatures",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:160:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/TargetMachine.h:163:6
export const LLVMAddAnalysisPasses = {
  "tag": "function",
  "name": "LLVMAddAnalysisPasses",
  "ns": 0,
  "location": "/usr/include/llvm-c/TargetMachine.h:163:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "T",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:36:6
export const LLVMLinkInMCJIT = {
  "tag": "function",
  "name": "LLVMLinkInMCJIT",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:36:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:37:6
export const LLVMLinkInInterpreter = {
  "tag": "function",
  "name": "LLVMLinkInInterpreter",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:37:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:53:21
export const LLVMCreateGenericValueOfInt = {
  "tag": "function",
  "name": "LLVMCreateGenericValueOfInt",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:53:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "N",
      "type": {
        "tag": ":unsigned-long-long",
        "bit-size": 64,
        "bit-alignment": 64
      }
    },
    {
      "tag": "parameter",
      "name": "IsSigned",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMGenericValueRef"
  }
};

// ./llvm-c/ExecutionEngine.h:57:21
export const LLVMCreateGenericValueOfPointer = {
  "tag": "function",
  "name": "LLVMCreateGenericValueOfPointer",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:57:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "P",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMGenericValueRef"
  }
};

// ./llvm-c/ExecutionEngine.h:59:21
export const LLVMCreateGenericValueOfFloat = {
  "tag": "function",
  "name": "LLVMCreateGenericValueOfFloat",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:59:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "N",
      "type": {
        "tag": ":double",
        "bit-size": 64,
        "bit-alignment": 64
      }
    }
  ],
  "return-type": {
    "tag": "LLVMGenericValueRef"
  }
};

// ./llvm-c/ExecutionEngine.h:61:10
export const LLVMGenericValueIntWidth = {
  "tag": "function",
  "name": "LLVMGenericValueIntWidth",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:61:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GenValRef",
      "type": {
        "tag": "LLVMGenericValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/ExecutionEngine.h:63:20
export const LLVMGenericValueToInt = {
  "tag": "function",
  "name": "LLVMGenericValueToInt",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:63:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GenVal",
      "type": {
        "tag": "LLVMGenericValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IsSigned",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-long-long",
    "bit-size": 64,
    "bit-alignment": 64
  }
};

// ./llvm-c/ExecutionEngine.h:66:7
export const LLVMGenericValueToPointer = {
  "tag": "function",
  "name": "LLVMGenericValueToPointer",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:66:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GenVal",
      "type": {
        "tag": "LLVMGenericValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":void"
    }
  }
};

// ./llvm-c/ExecutionEngine.h:68:8
export const LLVMGenericValueToFloat = {
  "tag": "function",
  "name": "LLVMGenericValueToFloat",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:68:8",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TyRef",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "GenVal",
      "type": {
        "tag": "LLVMGenericValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":double",
    "bit-size": 64,
    "bit-alignment": 64
  }
};

// ./llvm-c/ExecutionEngine.h:70:6
export const LLVMDisposeGenericValue = {
  "tag": "function",
  "name": "LLVMDisposeGenericValue",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:70:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GenVal",
      "type": {
        "tag": "LLVMGenericValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:74:10
export const LLVMCreateExecutionEngineForModule = {
  "tag": "function",
  "name": "LLVMCreateExecutionEngineForModule",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:74:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "OutEE",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMExecutionEngineRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutError",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/ExecutionEngine.h:78:10
export const LLVMCreateInterpreterForModule = {
  "tag": "function",
  "name": "LLVMCreateInterpreterForModule",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:78:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "OutInterp",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMExecutionEngineRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutError",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/ExecutionEngine.h:82:10
export const LLVMCreateJITCompilerForModule = {
  "tag": "function",
  "name": "LLVMCreateJITCompilerForModule",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:82:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "OutJIT",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMExecutionEngineRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OptLevel",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "OutError",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/ExecutionEngine.h:87:6
export const LLVMInitializeMCJITCompilerOptions = {
  "tag": "function",
  "name": "LLVMInitializeMCJITCompilerOptions",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:87:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":struct",
          "name": "LLVMMCJITCompilerOptions",
          "id": 60
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SizeOfOptions",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:107:10
export const LLVMCreateMCJITCompilerForModule = {
  "tag": "function",
  "name": "LLVMCreateMCJITCompilerForModule",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:107:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "OutJIT",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMExecutionEngineRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":struct",
          "name": "LLVMMCJITCompilerOptions",
          "id": 60
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SizeOfOptions",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "OutError",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/ExecutionEngine.h:112:6
export const LLVMDisposeExecutionEngine = {
  "tag": "function",
  "name": "LLVMDisposeExecutionEngine",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:112:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:114:6
export const LLVMRunStaticConstructors = {
  "tag": "function",
  "name": "LLVMRunStaticConstructors",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:114:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:116:6
export const LLVMRunStaticDestructors = {
  "tag": "function",
  "name": "LLVMRunStaticDestructors",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:116:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:118:5
export const LLVMRunFunctionAsMain = {
  "tag": "function",
  "name": "LLVMRunFunctionAsMain",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:118:5",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ArgC",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "ArgV",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    },
    {
      "tag": "parameter",
      "name": "EnvP",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": ":int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/ExecutionEngine.h:122:21
export const LLVMRunFunction = {
  "tag": "function",
  "name": "LLVMRunFunction",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:122:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumArgs",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Args",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMGenericValueRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMGenericValueRef"
  }
};

// ./llvm-c/ExecutionEngine.h:126:6
export const LLVMFreeMachineCodeForFunction = {
  "tag": "function",
  "name": "LLVMFreeMachineCodeForFunction",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:126:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:128:6
export const LLVMAddModule = {
  "tag": "function",
  "name": "LLVMAddModule",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:128:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:130:10
export const LLVMRemoveModule = {
  "tag": "function",
  "name": "LLVMRemoveModule",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:130:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutMod",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutError",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/ExecutionEngine.h:133:10
export const LLVMFindFunction = {
  "tag": "function",
  "name": "LLVMFindFunction",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:133:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutFn",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/ExecutionEngine.h:136:7
export const LLVMRecompileAndRelinkFunction = {
  "tag": "function",
  "name": "LLVMRecompileAndRelinkFunction",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:136:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":void"
    }
  }
};

// ./llvm-c/ExecutionEngine.h:139:19
export const LLVMGetExecutionEngineTargetData = {
  "tag": "function",
  "name": "LLVMGetExecutionEngineTargetData",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:139:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTargetDataRef"
  }
};

// ./llvm-c/ExecutionEngine.h:141:1
export const LLVMGetExecutionEngineTargetMachine = {
  "tag": "function",
  "name": "LLVMGetExecutionEngineTargetMachine",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:141:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTargetMachineRef"
  }
};

// ./llvm-c/ExecutionEngine.h:143:6
export const LLVMAddGlobalMapping = {
  "tag": "function",
  "name": "LLVMAddGlobalMapping",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:143:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Addr",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:146:7
export const LLVMGetPointerToGlobal = {
  "tag": "function",
  "name": "LLVMGetPointerToGlobal",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:146:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":void"
    }
  }
};

// ./llvm-c/ExecutionEngine.h:148:10
export const LLVMGetGlobalValueAddress = {
  "tag": "function",
  "name": "LLVMGetGlobalValueAddress",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:148:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/ExecutionEngine.h:150:10
export const LLVMGetFunctionAddress = {
  "tag": "function",
  "name": "LLVMGetFunctionAddress",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:150:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/ExecutionEngine.h:154:10
export const LLVMExecutionEngineGetErrMsg = {
  "tag": "function",
  "name": "LLVMExecutionEngineGetErrMsg",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:154:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "EE",
      "type": {
        "tag": "LLVMExecutionEngineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutError",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/ExecutionEngine.h:180:27
export const LLVMCreateSimpleMCJITMemoryManager = {
  "tag": "function",
  "name": "LLVMCreateSimpleMCJITMemoryManager",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:180:27",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Opaque",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "AllocateCodeSection",
      "type": {
        "tag": "LLVMMemoryManagerAllocateCodeSectionCallback"
      }
    },
    {
      "tag": "parameter",
      "name": "AllocateDataSection",
      "type": {
        "tag": "LLVMMemoryManagerAllocateDataSectionCallback"
      }
    },
    {
      "tag": "parameter",
      "name": "FinalizeMemory",
      "type": {
        "tag": "LLVMMemoryManagerFinalizeMemoryCallback"
      }
    },
    {
      "tag": "parameter",
      "name": "Destroy",
      "type": {
        "tag": "LLVMMemoryManagerDestroyCallback"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMCJITMemoryManagerRef"
  }
};

// ./llvm-c/ExecutionEngine.h:187:6
export const LLVMDisposeMCJITMemoryManager = {
  "tag": "function",
  "name": "LLVMDisposeMCJITMemoryManager",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:187:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MM",
      "type": {
        "tag": "LLVMMCJITMemoryManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ExecutionEngine.h:191:25
export const LLVMCreateGDBRegistrationListener = {
  "tag": "function",
  "name": "LLVMCreateGDBRegistrationListener",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:191:25",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMJITEventListenerRef"
  }
};

// ./llvm-c/ExecutionEngine.h:192:25
export const LLVMCreateIntelJITEventListener = {
  "tag": "function",
  "name": "LLVMCreateIntelJITEventListener",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:192:25",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMJITEventListenerRef"
  }
};

// ./llvm-c/ExecutionEngine.h:193:25
export const LLVMCreateOProfileJITEventListener = {
  "tag": "function",
  "name": "LLVMCreateOProfileJITEventListener",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:193:25",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMJITEventListenerRef"
  }
};

// ./llvm-c/ExecutionEngine.h:194:25
export const LLVMCreatePerfJITEventListener = {
  "tag": "function",
  "name": "LLVMCreatePerfJITEventListener",
  "ns": 0,
  "location": "/data/./llvm-c/ExecutionEngine.h:194:25",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMJITEventListenerRef"
  }
};

// ./llvm-c/Disassembler.h:38:22
export const LLVMCreateDisasm = {
  "tag": "function",
  "name": "LLVMCreateDisasm",
  "ns": 0,
  "location": "/data/./llvm-c/Disassembler.h:38:22",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TripleName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "DisInfo",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "TagType",
      "type": {
        "tag": ":int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "GetOpInfo",
      "type": {
        "tag": "LLVMOpInfoCallback"
      }
    },
    {
      "tag": "parameter",
      "name": "SymbolLookUp",
      "type": {
        "tag": "LLVMSymbolLookupCallback"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMDisasmContextRef"
  }
};

// ./llvm-c/Disassembler.h:50:22
export const LLVMCreateDisasmCPU = {
  "tag": "function",
  "name": "LLVMCreateDisasmCPU",
  "ns": 0,
  "location": "/data/./llvm-c/Disassembler.h:50:22",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Triple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "CPU",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "DisInfo",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "TagType",
      "type": {
        "tag": ":int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "GetOpInfo",
      "type": {
        "tag": "LLVMOpInfoCallback"
      }
    },
    {
      "tag": "parameter",
      "name": "SymbolLookUp",
      "type": {
        "tag": "LLVMSymbolLookupCallback"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMDisasmContextRef"
  }
};

// ./llvm-c/Disassembler.h:63:1
export const LLVMCreateDisasmCPUFeatures = {
  "tag": "function",
  "name": "LLVMCreateDisasmCPUFeatures",
  "ns": 0,
  "location": "/data/./llvm-c/Disassembler.h:63:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Triple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "CPU",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Features",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "DisInfo",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "TagType",
      "type": {
        "tag": ":int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "GetOpInfo",
      "type": {
        "tag": "LLVMOpInfoCallback"
      }
    },
    {
      "tag": "parameter",
      "name": "SymbolLookUp",
      "type": {
        "tag": "LLVMSymbolLookupCallback"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMDisasmContextRef"
  }
};

// ./llvm-c/Disassembler.h:72:5
export const LLVMSetDisasmOptions = {
  "tag": "function",
  "name": "LLVMSetDisasmOptions",
  "ns": 0,
  "location": "/data/./llvm-c/Disassembler.h:72:5",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DC",
      "type": {
        "tag": "LLVMDisasmContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "uint64_t"
      }
    }
  ],
  "return-type": {
    "tag": ":int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Disassembler.h:88:6
export const LLVMDisasmDispose = {
  "tag": "function",
  "name": "LLVMDisasmDispose",
  "ns": 0,
  "location": "/data/./llvm-c/Disassembler.h:88:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DC",
      "type": {
        "tag": "LLVMDisasmContextRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Disassembler.h:100:8
export const LLVMDisasmInstruction = {
  "tag": "function",
  "name": "LLVMDisasmInstruction",
  "ns": 0,
  "location": "/data/./llvm-c/Disassembler.h:100:8",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DC",
      "type": {
        "tag": "LLVMDisasmContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Bytes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "uint8_t"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "BytesSize",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "PC",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "OutString",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutStringSize",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "size_t"
  }
};

// ./llvm-c/DebugInfo.h:197:10
export const LLVMDebugMetadataVersion = {
  "tag": "function",
  "name": "LLVMDebugMetadataVersion",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:197:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/DebugInfo.h:202:10
export const LLVMGetModuleDebugMetadataVersion = {
  "tag": "function",
  "name": "LLVMGetModuleDebugMetadataVersion",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:202:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Module",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/DebugInfo.h:210:10
export const LLVMStripModuleDebugInfo = {
  "tag": "function",
  "name": "LLVMStripModuleDebugInfo",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:210:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Module",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/DebugInfo.h:216:18
export const LLVMCreateDIBuilderDisallowUnresolved = {
  "tag": "function",
  "name": "LLVMCreateDIBuilderDisallowUnresolved",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:216:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMDIBuilderRef"
  }
};

// ./llvm-c/DebugInfo.h:223:18
export const LLVMCreateDIBuilder = {
  "tag": "function",
  "name": "LLVMCreateDIBuilder",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:223:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMDIBuilderRef"
  }
};

// ./llvm-c/DebugInfo.h:229:6
export const LLVMDisposeDIBuilder = {
  "tag": "function",
  "name": "LLVMDisposeDIBuilder",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:229:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/DebugInfo.h:234:6
export const LLVMDIBuilderFinalize = {
  "tag": "function",
  "name": "LLVMDIBuilderFinalize",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:234:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/DebugInfo.h:240:6
export const LLVMDIBuilderFinalizeSubprogram = {
  "tag": "function",
  "name": "LLVMDIBuilderFinalizeSubprogram",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:240:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Subprogram",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/DebugInfo.h:275:17
export const LLVMDIBuilderCreateCompileUnit = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateCompileUnit",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:275:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Lang",
      "type": {
        "tag": "LLVMDWARFSourceLanguage"
      }
    },
    {
      "tag": "parameter",
      "name": "FileRef",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Producer",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ProducerLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "isOptimized",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "FlagsLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "RuntimeVer",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SplitName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SplitNameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Kind",
      "type": {
        "tag": "LLVMDWARFEmissionKind"
      }
    },
    {
      "tag": "parameter",
      "name": "DWOId",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SplitDebugInlining",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "DebugInfoForProfiling",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "SysRoot",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SysRootLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "SDK",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SDKLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:293:1
export const LLVMDIBuilderCreateFile = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateFile",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:293:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Filename",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "FilenameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Directory",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "DirectoryLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:312:1
export const LLVMDIBuilderCreateModule = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateModule",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:312:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ParentScope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "ConfigMacros",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ConfigMacrosLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "IncludePath",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "IncludePathLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "APINotesFile",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "APINotesFileLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:328:1
export const LLVMDIBuilderCreateNameSpace = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateNameSpace",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:328:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ParentScope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "ExportSymbols",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:351:17
export const LLVMDIBuilderCreateFunction = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateFunction",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:351:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "LinkageName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "LinkageNameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IsLocalToUnit",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "IsDefinition",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "ScopeLine",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "IsOptimized",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:366:17
export const LLVMDIBuilderCreateLexicalBlock = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateLexicalBlock",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:366:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Column",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:378:1
export const LLVMDIBuilderCreateLexicalBlockFile = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateLexicalBlockFile",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:378:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Discriminator",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:392:1
export const LLVMDIBuilderCreateImportedModuleFromNamespace = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateImportedModuleFromNamespace",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:392:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NS",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:409:17
export const LLVMDIBuilderCreateImportedModuleFromAlias = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateImportedModuleFromAlias",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:409:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ImportedEntity",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Elements",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:424:17
export const LLVMDIBuilderCreateImportedModuleFromModule = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateImportedModuleFromModule",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:424:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Elements",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:444:17
export const LLVMDIBuilderCreateImportedDeclaration = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateImportedDeclaration",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:444:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Decl",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Elements",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:460:1
export const LLVMDIBuilderCreateDebugLocation = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateDebugLocation",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:460:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Column",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "InlinedAt",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:470:10
export const LLVMDILocationGetLine = {
  "tag": "function",
  "name": "LLVMDILocationGetLine",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:470:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Location",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/DebugInfo.h:478:10
export const LLVMDILocationGetColumn = {
  "tag": "function",
  "name": "LLVMDILocationGetColumn",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:478:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Location",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/DebugInfo.h:486:17
export const LLVMDILocationGetScope = {
  "tag": "function",
  "name": "LLVMDILocationGetScope",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:486:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Location",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:494:17
export const LLVMDILocationGetInlinedAt = {
  "tag": "function",
  "name": "LLVMDILocationGetInlinedAt",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:494:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Location",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:502:17
export const LLVMDIScopeGetFile = {
  "tag": "function",
  "name": "LLVMDIScopeGetFile",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:502:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:511:13
export const LLVMDIFileGetDirectory = {
  "tag": "function",
  "name": "LLVMDIFileGetDirectory",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:511:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":unsigned-int",
          "bit-size": 32,
          "bit-alignment": 32
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/DebugInfo.h:520:13
export const LLVMDIFileGetFilename = {
  "tag": "function",
  "name": "LLVMDIFileGetFilename",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:520:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":unsigned-int",
          "bit-size": 32,
          "bit-alignment": 32
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/DebugInfo.h:529:13
export const LLVMDIFileGetSource = {
  "tag": "function",
  "name": "LLVMDIFileGetSource",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:529:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":unsigned-int",
          "bit-size": 32,
          "bit-alignment": 32
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/DebugInfo.h:537:17
export const LLVMDIBuilderGetOrCreateTypeArray = {
  "tag": "function",
  "name": "LLVMDIBuilderGetOrCreateTypeArray",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:537:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Data",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:552:1
export const LLVMDIBuilderCreateSubroutineType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateSubroutineType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:552:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ParameterTypes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumParameterTypes",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:569:17
export const LLVMDIBuilderCreateMacro = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateMacro",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:569:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ParentMacroFile",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "RecordType",
      "type": {
        "tag": "LLVMDWARFMacinfoRecordType"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Value",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ValueLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:586:1
export const LLVMDIBuilderCreateTempMacroFile = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateTempMacroFile",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:586:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ParentMacroFile",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:598:17
export const LLVMDIBuilderCreateEnumerator = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateEnumerator",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:598:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Value",
      "type": {
        "tag": "int64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "IsUnsigned",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:617:17
export const LLVMDIBuilderCreateEnumerationType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateEnumerationType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:617:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNumber",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Elements",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "ClassTy",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:640:17
export const LLVMDIBuilderCreateUnionType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateUnionType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:640:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNumber",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "Elements",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "RunTimeLang",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueId",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueIdLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:658:1
export const LLVMDIBuilderCreateArrayType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateArrayType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:658:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Size",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Subscripts",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumSubscripts",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:673:1
export const LLVMDIBuilderCreateVectorType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateVectorType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:673:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Size",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Subscripts",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumSubscripts",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:685:1
export const LLVMDIBuilderCreateUnspecifiedType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateUnspecifiedType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:685:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:699:1
export const LLVMDIBuilderCreateBasicType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateBasicType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:699:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Encoding",
      "type": {
        "tag": "LLVMDWARFTypeEncoding"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:714:17
export const LLVMDIBuilderCreatePointerType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreatePointerType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:714:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PointeeTy",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AddressSpace",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:737:17
export const LLVMDIBuilderCreateStructType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateStructType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:737:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNumber",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "DerivedFrom",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Elements",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "RunTimeLang",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "VTableHolder",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueId",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueIdLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:759:17
export const LLVMDIBuilderCreateMemberType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateMemberType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:759:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "OffsetInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:780:1
export const LLVMDIBuilderCreateStaticMemberType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateStaticMemberType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:780:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNumber",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Type",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:796:1
export const LLVMDIBuilderCreateMemberPointerType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateMemberPointerType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:796:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PointeeType",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ClassType",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:817:1
export const LLVMDIBuilderCreateObjCIVar = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateObjCIVar",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:817:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "OffsetInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PropertyNode",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:839:1
export const LLVMDIBuilderCreateObjCProperty = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateObjCProperty",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:839:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "GetterName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "GetterNameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "SetterName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SetterNameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "PropertyAttributes",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:853:1
export const LLVMDIBuilderCreateObjectPointerType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateObjectPointerType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:853:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Type",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:865:1
export const LLVMDIBuilderCreateQualifiedType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateQualifiedType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:865:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Tag",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Type",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:876:1
export const LLVMDIBuilderCreateReferenceType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateReferenceType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:876:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Tag",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Type",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:884:1
export const LLVMDIBuilderCreateNullPtrType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateNullPtrType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:884:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:896:1
export const LLVMDIBuilderCreateTypedef = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateTypedef",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:896:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Type",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:912:1
export const LLVMDIBuilderCreateInheritance = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateInheritance",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:912:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BaseTy",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BaseOffset",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "VBPtrOffset",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:933:17
export const LLVMDIBuilderCreateForwardDecl = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateForwardDecl",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:933:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Tag",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "RuntimeLang",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueIdentifier",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueIdentifierLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:957:1
export const LLVMDIBuilderCreateReplaceableCompositeType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateReplaceableCompositeType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:957:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Tag",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Line",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "RuntimeLang",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueIdentifier",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueIdentifierLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:979:1
export const LLVMDIBuilderCreateBitFieldMemberType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateBitFieldMemberType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:979:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNumber",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "OffsetInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "StorageOffsetInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "Type",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1010:17
export const LLVMDIBuilderCreateClassType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateClassType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1010:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNumber",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "SizeInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    },
    {
      "tag": "parameter",
      "name": "OffsetInBits",
      "type": {
        "tag": "uint64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "DerivedFrom",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Elements",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "VTableHolder",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TemplateParamsNode",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueIdentifier",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "UniqueIdentifierLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1025:1
export const LLVMDIBuilderCreateArtificialType = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateArtificialType",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1025:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Type",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1035:13
export const LLVMDITypeGetName = {
  "tag": "function",
  "name": "LLVMDITypeGetName",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1035:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DType",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/DebugInfo.h:1043:10
export const LLVMDITypeGetSizeInBits = {
  "tag": "function",
  "name": "LLVMDITypeGetSizeInBits",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1043:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DType",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/DebugInfo.h:1051:10
export const LLVMDITypeGetOffsetInBits = {
  "tag": "function",
  "name": "LLVMDITypeGetOffsetInBits",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1051:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DType",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/DebugInfo.h:1059:10
export const LLVMDITypeGetAlignInBits = {
  "tag": "function",
  "name": "LLVMDITypeGetAlignInBits",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1059:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DType",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint32_t"
  }
};

// ./llvm-c/DebugInfo.h:1067:10
export const LLVMDITypeGetLine = {
  "tag": "function",
  "name": "LLVMDITypeGetLine",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1067:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DType",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/DebugInfo.h:1075:13
export const LLVMDITypeGetFlags = {
  "tag": "function",
  "name": "LLVMDITypeGetFlags",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1075:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DType",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMDIFlags"
  }
};

// ./llvm-c/DebugInfo.h:1083:17
export const LLVMDIBuilderGetOrCreateSubrange = {
  "tag": "function",
  "name": "LLVMDIBuilderGetOrCreateSubrange",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1083:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LowerBound",
      "type": {
        "tag": "int64_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Count",
      "type": {
        "tag": "int64_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1093:17
export const LLVMDIBuilderGetOrCreateArray = {
  "tag": "function",
  "name": "LLVMDIBuilderGetOrCreateArray",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1093:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Data",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1104:17
export const LLVMDIBuilderCreateExpression = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateExpression",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1104:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Addr",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "uint64_t"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1114:1
export const LLVMDIBuilderCreateConstantValueExpression = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateConstantValueExpression",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1114:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Value",
      "type": {
        "tag": "uint64_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1136:17
export const LLVMDIBuilderCreateGlobalVariableExpression = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateGlobalVariableExpression",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1136:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Linkage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "LinkLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LocalToUnit",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "Expr",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Decl",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1148:17
export const LLVMDIGlobalVariableExpressionGetVariable = {
  "tag": "function",
  "name": "LLVMDIGlobalVariableExpressionGetVariable",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1148:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GVE",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1156:17
export const LLVMDIGlobalVariableExpressionGetExpression = {
  "tag": "function",
  "name": "LLVMDIGlobalVariableExpressionGetExpression",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1156:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GVE",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1165:17
export const LLVMDIVariableGetFile = {
  "tag": "function",
  "name": "LLVMDIVariableGetFile",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1165:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Var",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1173:17
export const LLVMDIVariableGetScope = {
  "tag": "function",
  "name": "LLVMDIVariableGetScope",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1173:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Var",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1181:10
export const LLVMDIVariableGetLine = {
  "tag": "function",
  "name": "LLVMDIVariableGetLine",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1181:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Var",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/DebugInfo.h:1191:17
export const LLVMTemporaryMDNode = {
  "tag": "function",
  "name": "LLVMTemporaryMDNode",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1191:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Data",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumElements",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1201:6
export const LLVMDisposeTemporaryMDNode = {
  "tag": "function",
  "name": "LLVMDisposeTemporaryMDNode",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1201:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TempNode",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/DebugInfo.h:1208:6
export const LLVMMetadataReplaceAllUsesWith = {
  "tag": "function",
  "name": "LLVMMetadataReplaceAllUsesWith",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1208:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TempTargetMetadata",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Replacement",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/DebugInfo.h:1228:17
export const LLVMDIBuilderCreateTempGlobalVariableFwdDecl = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateTempGlobalVariableFwdDecl",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1228:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Linkage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "LnkLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LocalToUnit",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "Decl",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1243:14
export const LLVMDIBuilderInsertDeclareBefore = {
  "tag": "function",
  "name": "LLVMDIBuilderInsertDeclareBefore",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1243:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Storage",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "VarInfo",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Expr",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DebugLoc",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/DebugInfo.h:1258:14
export const LLVMDIBuilderInsertDeclareAtEnd = {
  "tag": "function",
  "name": "LLVMDIBuilderInsertDeclareAtEnd",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1258:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Storage",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "VarInfo",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Expr",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DebugLoc",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Block",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/DebugInfo.h:1271:14
export const LLVMDIBuilderInsertDbgValueBefore = {
  "tag": "function",
  "name": "LLVMDIBuilderInsertDbgValueBefore",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1271:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "VarInfo",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Expr",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DebugLoc",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/DebugInfo.h:1289:14
export const LLVMDIBuilderInsertDbgValueAtEnd = {
  "tag": "function",
  "name": "LLVMDIBuilderInsertDbgValueAtEnd",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1289:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "VarInfo",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Expr",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DebugLoc",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Block",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/DebugInfo.h:1309:17
export const LLVMDIBuilderCreateAutoVariable = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateAutoVariable",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1309:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AlwaysPreserve",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    },
    {
      "tag": "parameter",
      "name": "AlignInBits",
      "type": {
        "tag": "uint32_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1327:17
export const LLVMDIBuilderCreateParameterVariable = {
  "tag": "function",
  "name": "LLVMDIBuilderCreateParameterVariable",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1327:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMDIBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Scope",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "ArgNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "File",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LineNo",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AlwaysPreserve",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "Flags",
      "type": {
        "tag": "LLVMDIFlags"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1337:17
export const LLVMGetSubprogram = {
  "tag": "function",
  "name": "LLVMGetSubprogram",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1337:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Func",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1344:6
export const LLVMSetSubprogram = {
  "tag": "function",
  "name": "LLVMSetSubprogram",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1344:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Func",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SP",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/DebugInfo.h:1352:10
export const LLVMDISubprogramGetLine = {
  "tag": "function",
  "name": "LLVMDISubprogramGetLine",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1352:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Subprogram",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/DebugInfo.h:1359:17
export const LLVMInstructionGetDebugLoc = {
  "tag": "function",
  "name": "LLVMInstructionGetDebugLoc",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1359:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/DebugInfo.h:1368:6
export const LLVMInstructionSetDebugLoc = {
  "tag": "function",
  "name": "LLVMInstructionSetDebugLoc",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1368:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Loc",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/DebugInfo.h:1375:18
export const LLVMGetMetadataKind = {
  "tag": "function",
  "name": "LLVMGetMetadataKind",
  "ns": 0,
  "location": "/data/./llvm-c/DebugInfo.h:1375:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Metadata",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataKind"
  }
};

// ./llvm-c/Error.h:44:17
export const LLVMGetErrorTypeId = {
  "tag": "function",
  "name": "LLVMGetErrorTypeId",
  "ns": 0,
  "location": "/data/./llvm-c/Error.h:44:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Err",
      "type": {
        "tag": "LLVMErrorRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorTypeId"
  }
};

// ./llvm-c/Error.h:52:6
export const LLVMConsumeError = {
  "tag": "function",
  "name": "LLVMConsumeError",
  "ns": 0,
  "location": "/data/./llvm-c/Error.h:52:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Err",
      "type": {
        "tag": "LLVMErrorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Error.h:60:7
export const LLVMGetErrorMessage = {
  "tag": "function",
  "name": "LLVMGetErrorMessage",
  "ns": 0,
  "location": "/data/./llvm-c/Error.h:60:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Err",
      "type": {
        "tag": "LLVMErrorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Error.h:65:6
export const LLVMDisposeErrorMessage = {
  "tag": "function",
  "name": "LLVMDisposeErrorMessage",
  "ns": 0,
  "location": "/data/./llvm-c/Error.h:65:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ErrMsg",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Error.h:70:17
export const LLVMGetStringErrorTypeId = {
  "tag": "function",
  "name": "LLVMGetStringErrorTypeId",
  "ns": 0,
  "location": "/data/./llvm-c/Error.h:70:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMErrorTypeId"
  }
};

// ./llvm-c/Error.h:75:14
export const LLVMCreateStringError = {
  "tag": "function",
  "name": "LLVMCreateStringError",
  "ns": 0,
  "location": "/data/./llvm-c/Error.h:75:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ErrMsg",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:457:6
export const LLVMOrcExecutionSessionSetErrorReporter = {
  "tag": "function",
  "name": "LLVMOrcExecutionSessionSetErrorReporter",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:457:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ES",
      "type": {
        "tag": "LLVMOrcExecutionSessionRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ReportError",
      "type": {
        "tag": "LLVMOrcErrorReporterFunction"
      }
    },
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:468:1
export const LLVMOrcExecutionSessionGetSymbolStringPool = {
  "tag": "function",
  "name": "LLVMOrcExecutionSessionGetSymbolStringPool",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:468:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ES",
      "type": {
        "tag": "LLVMOrcExecutionSessionRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcSymbolStringPoolRef"
  }
};

// ./llvm-c/Orc.h:480:6
export const LLVMOrcSymbolStringPoolClearDeadEntries = {
  "tag": "function",
  "name": "LLVMOrcSymbolStringPoolClearDeadEntries",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:480:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SSP",
      "type": {
        "tag": "LLVMOrcSymbolStringPoolRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:495:1
export const LLVMOrcExecutionSessionIntern = {
  "tag": "function",
  "name": "LLVMOrcExecutionSessionIntern",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:495:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ES",
      "type": {
        "tag": "LLVMOrcExecutionSessionRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcSymbolStringPoolEntryRef"
  }
};

// ./llvm-c/Orc.h:500:6
export const LLVMOrcRetainSymbolStringPoolEntry = {
  "tag": "function",
  "name": "LLVMOrcRetainSymbolStringPoolEntry",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:500:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "S",
      "type": {
        "tag": "LLVMOrcSymbolStringPoolEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:505:6
export const LLVMOrcReleaseSymbolStringPoolEntry = {
  "tag": "function",
  "name": "LLVMOrcReleaseSymbolStringPoolEntry",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:505:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "S",
      "type": {
        "tag": "LLVMOrcSymbolStringPoolEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:507:13
export const LLVMOrcSymbolStringPoolEntryStr = {
  "tag": "function",
  "name": "LLVMOrcSymbolStringPoolEntryStr",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:507:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "S",
      "type": {
        "tag": "LLVMOrcSymbolStringPoolEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Orc.h:512:6
export const LLVMOrcReleaseResourceTracker = {
  "tag": "function",
  "name": "LLVMOrcReleaseResourceTracker",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:512:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RT",
      "type": {
        "tag": "LLVMOrcResourceTrackerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:518:6
export const LLVMOrcResourceTrackerTransferTo = {
  "tag": "function",
  "name": "LLVMOrcResourceTrackerTransferTo",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:518:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SrcRT",
      "type": {
        "tag": "LLVMOrcResourceTrackerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DstRT",
      "type": {
        "tag": "LLVMOrcResourceTrackerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:525:14
export const LLVMOrcResourceTrackerRemove = {
  "tag": "function",
  "name": "LLVMOrcResourceTrackerRemove",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:525:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RT",
      "type": {
        "tag": "LLVMOrcResourceTrackerRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:532:6
export const LLVMOrcDisposeDefinitionGenerator = {
  "tag": "function",
  "name": "LLVMOrcDisposeDefinitionGenerator",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:532:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DG",
      "type": {
        "tag": "LLVMOrcDefinitionGeneratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:537:6
export const LLVMOrcDisposeMaterializationUnit = {
  "tag": "function",
  "name": "LLVMOrcDisposeMaterializationUnit",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:537:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MU",
      "type": {
        "tag": "LLVMOrcMaterializationUnitRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:572:31
export const LLVMOrcCreateCustomMaterializationUnit = {
  "tag": "function",
  "name": "LLVMOrcCreateCustomMaterializationUnit",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:572:31",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Syms",
      "type": {
        "tag": "LLVMOrcCSymbolFlagsMapPairs"
      }
    },
    {
      "tag": "parameter",
      "name": "NumSyms",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "InitSym",
      "type": {
        "tag": "LLVMOrcSymbolStringPoolEntryRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Materialize",
      "type": {
        "tag": "LLVMOrcMaterializationUnitMaterializeFunction"
      }
    },
    {
      "tag": "parameter",
      "name": "Discard",
      "type": {
        "tag": "LLVMOrcMaterializationUnitDiscardFunction"
      }
    },
    {
      "tag": "parameter",
      "name": "Destroy",
      "type": {
        "tag": "LLVMOrcMaterializationUnitDestroyFunction"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcMaterializationUnitRef"
  }
};

// ./llvm-c/Orc.h:601:1
export const LLVMOrcAbsoluteSymbols = {
  "tag": "function",
  "name": "LLVMOrcAbsoluteSymbols",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:601:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Syms",
      "type": {
        "tag": "LLVMOrcCSymbolMapPairs"
      }
    },
    {
      "tag": "parameter",
      "name": "NumPairs",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcMaterializationUnitRef"
  }
};

// ./llvm-c/Orc.h:624:31
export const LLVMOrcLazyReexports = {
  "tag": "function",
  "name": "LLVMOrcLazyReexports",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:624:31",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LCTM",
      "type": {
        "tag": "LLVMOrcLazyCallThroughManagerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ISM",
      "type": {
        "tag": "LLVMOrcIndirectStubsManagerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SourceRef",
      "type": {
        "tag": "LLVMOrcJITDylibRef"
      }
    },
    {
      "tag": "parameter",
      "name": "CallableAliases",
      "type": {
        "tag": "LLVMOrcCSymbolAliasMapPairs"
      }
    },
    {
      "tag": "parameter",
      "name": "NumPairs",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcMaterializationUnitRef"
  }
};

// ./llvm-c/Orc.h:639:6
export const LLVMOrcDisposeMaterializationResponsibility = {
  "tag": "function",
  "name": "LLVMOrcDisposeMaterializationResponsibility",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:639:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:645:20
export const LLVMOrcMaterializationResponsibilityGetTargetDylib = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityGetTargetDylib",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:645:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcJITDylibRef"
  }
};

// ./llvm-c/Orc.h:652:1
export const LLVMOrcMaterializationResponsibilityGetExecutionSession = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityGetExecutionSession",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:652:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcExecutionSessionRef"
  }
};

// ./llvm-c/Orc.h:665:29
export const LLVMOrcMaterializationResponsibilityGetSymbols = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityGetSymbols",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:665:29",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumPairs",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcCSymbolFlagsMapPairs"
  }
};

// ./llvm-c/Orc.h:673:6
export const LLVMOrcDisposeCSymbolFlagsMap = {
  "tag": "function",
  "name": "LLVMOrcDisposeCSymbolFlagsMap",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:673:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Pairs",
      "type": {
        "tag": "LLVMOrcCSymbolFlagsMapPairs"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:684:1
export const LLVMOrcMaterializationResponsibilityGetInitializerSymbol = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityGetInitializerSymbol",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:684:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcSymbolStringPoolEntryRef"
  }
};

// ./llvm-c/Orc.h:694:1
export const LLVMOrcMaterializationResponsibilityGetRequestedSymbols = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityGetRequestedSymbols",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:694:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumSymbols",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": "LLVMOrcSymbolStringPoolEntryRef"
    }
  }
};

// ./llvm-c/Orc.h:702:6
export const LLVMOrcDisposeSymbols = {
  "tag": "function",
  "name": "LLVMOrcDisposeSymbols",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:702:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Symbols",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcSymbolStringPoolEntryRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:720:14
export const LLVMOrcMaterializationResponsibilityNotifyResolved = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityNotifyResolved",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:720:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Symbols",
      "type": {
        "tag": "LLVMOrcCSymbolMapPairs"
      }
    },
    {
      "tag": "parameter",
      "name": "NumPairs",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:737:14
export const LLVMOrcMaterializationResponsibilityNotifyEmitted = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityNotifyEmitted",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:737:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:753:14
export const LLVMOrcMaterializationResponsibilityDefineMaterializing = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityDefineMaterializing",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:753:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Pairs",
      "type": {
        "tag": "LLVMOrcCSymbolFlagsMapPairs"
      }
    },
    {
      "tag": "parameter",
      "name": "NumPairs",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:764:6
export const LLVMOrcMaterializationResponsibilityFailMaterialization = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityFailMaterialization",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:764:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:774:14
export const LLVMOrcMaterializationResponsibilityReplace = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityReplace",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:774:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MU",
      "type": {
        "tag": "LLVMOrcMaterializationUnitRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:786:14
export const LLVMOrcMaterializationResponsibilityDelegate = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityDelegate",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:786:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Symbols",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcSymbolStringPoolEntryRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumSymbols",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Result",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcMaterializationResponsibilityRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:809:6
export const LLVMOrcMaterializationResponsibilityAddDependencies = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityAddDependencies",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:809:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": "LLVMOrcSymbolStringPoolEntryRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dependencies",
      "type": {
        "tag": "LLVMOrcCDependenceMapPairs"
      }
    },
    {
      "tag": "parameter",
      "name": "NumPairs",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:819:6
export const LLVMOrcMaterializationResponsibilityAddDependenciesForAll = {
  "tag": "function",
  "name": "LLVMOrcMaterializationResponsibilityAddDependenciesForAll",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:819:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dependencies",
      "type": {
        "tag": "LLVMOrcCDependenceMapPairs"
      }
    },
    {
      "tag": "parameter",
      "name": "NumPairs",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:833:1
export const LLVMOrcExecutionSessionCreateBareJITDylib = {
  "tag": "function",
  "name": "LLVMOrcExecutionSessionCreateBareJITDylib",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:833:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ES",
      "type": {
        "tag": "LLVMOrcExecutionSessionRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcJITDylibRef"
  }
};

// ./llvm-c/Orc.h:849:1
export const LLVMOrcExecutionSessionCreateJITDylib = {
  "tag": "function",
  "name": "LLVMOrcExecutionSessionCreateJITDylib",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:849:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ES",
      "type": {
        "tag": "LLVMOrcExecutionSessionRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Result",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcJITDylibRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:858:1
export const LLVMOrcExecutionSessionGetJITDylibByName = {
  "tag": "function",
  "name": "LLVMOrcExecutionSessionGetJITDylibByName",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:858:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ES",
      "type": {
        "tag": "LLVMOrcExecutionSessionRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcJITDylibRef"
  }
};

// ./llvm-c/Orc.h:867:1
export const LLVMOrcJITDylibCreateResourceTracker = {
  "tag": "function",
  "name": "LLVMOrcJITDylibCreateResourceTracker",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:867:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "JD",
      "type": {
        "tag": "LLVMOrcJITDylibRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcResourceTrackerRef"
  }
};

// ./llvm-c/Orc.h:875:1
export const LLVMOrcJITDylibGetDefaultResourceTracker = {
  "tag": "function",
  "name": "LLVMOrcJITDylibGetDefaultResourceTracker",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:875:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "JD",
      "type": {
        "tag": "LLVMOrcJITDylibRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcResourceTrackerRef"
  }
};

// ./llvm-c/Orc.h:884:14
export const LLVMOrcJITDylibDefine = {
  "tag": "function",
  "name": "LLVMOrcJITDylibDefine",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:884:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "JD",
      "type": {
        "tag": "LLVMOrcJITDylibRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MU",
      "type": {
        "tag": "LLVMOrcMaterializationUnitRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:891:14
export const LLVMOrcJITDylibClear = {
  "tag": "function",
  "name": "LLVMOrcJITDylibClear",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:891:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "JD",
      "type": {
        "tag": "LLVMOrcJITDylibRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:899:6
export const LLVMOrcJITDylibAddGenerator = {
  "tag": "function",
  "name": "LLVMOrcJITDylibAddGenerator",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:899:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "JD",
      "type": {
        "tag": "LLVMOrcJITDylibRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DG",
      "type": {
        "tag": "LLVMOrcDefinitionGeneratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:905:31
export const LLVMOrcCreateCustomCAPIDefinitionGenerator = {
  "tag": "function",
  "name": "LLVMOrcCreateCustomCAPIDefinitionGenerator",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:905:31",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction"
      }
    },
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcDefinitionGeneratorRef"
  }
};

// ./llvm-c/Orc.h:926:14
export const LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess = {
  "tag": "function",
  "name": "LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:926:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Result",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcDefinitionGeneratorRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "GlobalPrefx",
      "type": {
        "tag": ":char",
        "bit-size": 8,
        "bit-alignment": 8
      }
    },
    {
      "tag": "parameter",
      "name": "Filter",
      "type": {
        "tag": "LLVMOrcSymbolPredicate"
      }
    },
    {
      "tag": "parameter",
      "name": "FilterCtx",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:951:14
export const LLVMOrcCreateDynamicLibrarySearchGeneratorForPath = {
  "tag": "function",
  "name": "LLVMOrcCreateDynamicLibrarySearchGeneratorForPath",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:951:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Result",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcDefinitionGeneratorRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "FileName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "GlobalPrefix",
      "type": {
        "tag": ":char",
        "bit-size": 8,
        "bit-alignment": 8
      }
    },
    {
      "tag": "parameter",
      "name": "Filter",
      "type": {
        "tag": "LLVMOrcSymbolPredicate"
      }
    },
    {
      "tag": "parameter",
      "name": "FilterCtx",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:969:14
export const LLVMOrcCreateStaticLibrarySearchGeneratorForPath = {
  "tag": "function",
  "name": "LLVMOrcCreateStaticLibrarySearchGeneratorForPath",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:969:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Result",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcDefinitionGeneratorRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ObjLayer",
      "type": {
        "tag": "LLVMOrcObjectLayerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "FileName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "TargetTriple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:981:29
export const LLVMOrcCreateNewThreadSafeContext = {
  "tag": "function",
  "name": "LLVMOrcCreateNewThreadSafeContext",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:981:29",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMOrcThreadSafeContextRef"
  }
};

// ./llvm-c/Orc.h:987:1
export const LLVMOrcThreadSafeContextGetContext = {
  "tag": "function",
  "name": "LLVMOrcThreadSafeContextGetContext",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:987:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TSCtx",
      "type": {
        "tag": "LLVMOrcThreadSafeContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMContextRef"
  }
};

// ./llvm-c/Orc.h:992:6
export const LLVMOrcDisposeThreadSafeContext = {
  "tag": "function",
  "name": "LLVMOrcDisposeThreadSafeContext",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:992:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TSCtx",
      "type": {
        "tag": "LLVMOrcThreadSafeContextRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1005:1
export const LLVMOrcCreateNewThreadSafeModule = {
  "tag": "function",
  "name": "LLVMOrcCreateNewThreadSafeModule",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1005:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TSCtx",
      "type": {
        "tag": "LLVMOrcThreadSafeContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcThreadSafeModuleRef"
  }
};

// ./llvm-c/Orc.h:1013:6
export const LLVMOrcDisposeThreadSafeModule = {
  "tag": "function",
  "name": "LLVMOrcDisposeThreadSafeModule",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1013:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TSM",
      "type": {
        "tag": "LLVMOrcThreadSafeModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1019:1
export const LLVMOrcThreadSafeModuleWithModuleDo = {
  "tag": "function",
  "name": "LLVMOrcThreadSafeModuleWithModuleDo",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1019:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TSM",
      "type": {
        "tag": "LLVMOrcThreadSafeModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMOrcGenericIRModuleOperationFunction"
      }
    },
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:1031:14
export const LLVMOrcJITTargetMachineBuilderDetectHost = {
  "tag": "function",
  "name": "LLVMOrcJITTargetMachineBuilderDetectHost",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1031:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Result",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcJITTargetMachineBuilderRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:1044:1
export const LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine = {
  "tag": "function",
  "name": "LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1044:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TM",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcJITTargetMachineBuilderRef"
  }
};

// ./llvm-c/Orc.h:1049:6
export const LLVMOrcDisposeJITTargetMachineBuilder = {
  "tag": "function",
  "name": "LLVMOrcDisposeJITTargetMachineBuilder",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1049:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "JTMB",
      "type": {
        "tag": "LLVMOrcJITTargetMachineBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1058:7
export const LLVMOrcJITTargetMachineBuilderGetTargetTriple = {
  "tag": "function",
  "name": "LLVMOrcJITTargetMachineBuilderGetTargetTriple",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1058:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "JTMB",
      "type": {
        "tag": "LLVMOrcJITTargetMachineBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Orc.h:1065:6
export const LLVMOrcJITTargetMachineBuilderSetTargetTriple = {
  "tag": "function",
  "name": "LLVMOrcJITTargetMachineBuilderSetTargetTriple",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1065:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "JTMB",
      "type": {
        "tag": "LLVMOrcJITTargetMachineBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TargetTriple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1079:14
export const LLVMOrcObjectLayerAddObjectFile = {
  "tag": "function",
  "name": "LLVMOrcObjectLayerAddObjectFile",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1079:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjLayer",
      "type": {
        "tag": "LLVMOrcObjectLayerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "JD",
      "type": {
        "tag": "LLVMOrcJITDylibRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ObjBuffer",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:1095:1
export const LLVMOrcObjectLayerAddObjectFileWithRT = {
  "tag": "function",
  "name": "LLVMOrcObjectLayerAddObjectFileWithRT",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1095:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjLayer",
      "type": {
        "tag": "LLVMOrcObjectLayerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RT",
      "type": {
        "tag": "LLVMOrcResourceTrackerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ObjBuffer",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:1105:6
export const LLVMOrcObjectLayerEmit = {
  "tag": "function",
  "name": "LLVMOrcObjectLayerEmit",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1105:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjLayer",
      "type": {
        "tag": "LLVMOrcObjectLayerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ObjBuffer",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1112:6
export const LLVMOrcDisposeObjectLayer = {
  "tag": "function",
  "name": "LLVMOrcDisposeObjectLayer",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1112:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjLayer",
      "type": {
        "tag": "LLVMOrcObjectLayerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1114:6
export const LLVMOrcIRTransformLayerEmit = {
  "tag": "function",
  "name": "LLVMOrcIRTransformLayerEmit",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1114:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IRTransformLayer",
      "type": {
        "tag": "LLVMOrcIRTransformLayerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MR",
      "type": {
        "tag": "LLVMOrcMaterializationResponsibilityRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TSM",
      "type": {
        "tag": "LLVMOrcThreadSafeModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1122:6
export const LLVMOrcIRTransformLayerSetTransform = {
  "tag": "function",
  "name": "LLVMOrcIRTransformLayerSetTransform",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1122:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IRTransformLayer",
      "type": {
        "tag": "LLVMOrcIRTransformLayerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TransformFunction",
      "type": {
        "tag": "LLVMOrcIRTransformLayerTransformFunction"
      }
    },
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1129:6
export const LLVMOrcObjectTransformLayerSetTransform = {
  "tag": "function",
  "name": "LLVMOrcObjectTransformLayerSetTransform",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1129:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjTransformLayer",
      "type": {
        "tag": "LLVMOrcObjectTransformLayerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TransformFunction",
      "type": {
        "tag": "LLVMOrcObjectTransformLayerTransformFunction"
      }
    },
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1140:1
export const LLVMOrcCreateLocalIndirectStubsManager = {
  "tag": "function",
  "name": "LLVMOrcCreateLocalIndirectStubsManager",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1140:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TargetTriple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcIndirectStubsManagerRef"
  }
};

// ./llvm-c/Orc.h:1145:6
export const LLVMOrcDisposeIndirectStubsManager = {
  "tag": "function",
  "name": "LLVMOrcDisposeIndirectStubsManager",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1145:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ISM",
      "type": {
        "tag": "LLVMOrcIndirectStubsManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1147:14
export const LLVMOrcCreateLocalLazyCallThroughManager = {
  "tag": "function",
  "name": "LLVMOrcCreateLocalLazyCallThroughManager",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1147:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "TargetTriple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ES",
      "type": {
        "tag": "LLVMOrcExecutionSessionRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ErrorHandlerAddr",
      "type": {
        "tag": "LLVMOrcJITTargetAddress"
      }
    },
    {
      "tag": "parameter",
      "name": "LCTM",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcLazyCallThroughManagerRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Orc.h:1155:6
export const LLVMOrcDisposeLazyCallThroughManager = {
  "tag": "function",
  "name": "LLVMOrcDisposeLazyCallThroughManager",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1155:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LCTM",
      "type": {
        "tag": "LLVMOrcLazyCallThroughManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1172:23
export const LLVMOrcCreateDumpObjects = {
  "tag": "function",
  "name": "LLVMOrcCreateDumpObjects",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1172:23",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DumpDir",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "IdentifierOverride",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcDumpObjectsRef"
  }
};

// ./llvm-c/Orc.h:1178:6
export const LLVMOrcDisposeDumpObjects = {
  "tag": "function",
  "name": "LLVMOrcDisposeDumpObjects",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1178:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DumpObjects",
      "type": {
        "tag": "LLVMOrcDumpObjectsRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Orc.h:1183:14
export const LLVMOrcDumpObjects_CallOperator = {
  "tag": "function",
  "name": "LLVMOrcDumpObjects_CallOperator",
  "ns": 0,
  "location": "/usr/include/llvm-c/Orc.h:1183:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DumpObjects",
      "type": {
        "tag": "LLVMOrcDumpObjectsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ObjBuffer",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMemoryBufferRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/OrcEE.h:47:1
export const LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager = {
  "tag": "function",
  "name": "LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager",
  "ns": 0,
  "location": "/data/./llvm-c/OrcEE.h:47:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ES",
      "type": {
        "tag": "LLVMOrcExecutionSessionRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcObjectLayerRef"
  }
};

// ./llvm-c/OrcEE.h:56:6
export const LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener = {
  "tag": "function",
  "name": "LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener",
  "ns": 0,
  "location": "/data/./llvm-c/OrcEE.h:56:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RTDyldObjLinkingLayer",
      "type": {
        "tag": "LLVMOrcObjectLayerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Listener",
      "type": {
        "tag": "LLVMJITEventListenerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:32:27
export const LLVMPassManagerBuilderCreate = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderCreate",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:32:27",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMPassManagerBuilderRef"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:33:6
export const LLVMPassManagerBuilderDispose = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderDispose",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:33:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PMB",
      "type": {
        "tag": "LLVMPassManagerBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:37:1
export const LLVMPassManagerBuilderSetOptLevel = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderSetOptLevel",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:37:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PMB",
      "type": {
        "tag": "LLVMPassManagerBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OptLevel",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:42:1
export const LLVMPassManagerBuilderSetSizeLevel = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderSetSizeLevel",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:42:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PMB",
      "type": {
        "tag": "LLVMPassManagerBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SizeLevel",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:47:1
export const LLVMPassManagerBuilderSetDisableUnitAtATime = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderSetDisableUnitAtATime",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:47:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PMB",
      "type": {
        "tag": "LLVMPassManagerBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Value",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:52:1
export const LLVMPassManagerBuilderSetDisableUnrollLoops = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderSetDisableUnrollLoops",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:52:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PMB",
      "type": {
        "tag": "LLVMPassManagerBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Value",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:57:1
export const LLVMPassManagerBuilderSetDisableSimplifyLibCalls = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderSetDisableSimplifyLibCalls",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:57:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PMB",
      "type": {
        "tag": "LLVMPassManagerBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Value",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:62:1
export const LLVMPassManagerBuilderUseInlinerWithThreshold = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderUseInlinerWithThreshold",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:62:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PMB",
      "type": {
        "tag": "LLVMPassManagerBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Threshold",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:67:1
export const LLVMPassManagerBuilderPopulateFunctionPassManager = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderPopulateFunctionPassManager",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:67:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PMB",
      "type": {
        "tag": "LLVMPassManagerBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassManagerBuilder.h:72:1
export const LLVMPassManagerBuilderPopulateModulePassManager = {
  "tag": "function",
  "name": "LLVMPassManagerBuilderPopulateModulePassManager",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassManagerBuilder.h:72:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PMB",
      "type": {
        "tag": "LLVMPassManagerBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Utils.h:35:6
export const LLVMAddLowerSwitchPass = {
  "tag": "function",
  "name": "LLVMAddLowerSwitchPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Utils.h:35:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Utils.h:38:6
export const LLVMAddPromoteMemoryToRegisterPass = {
  "tag": "function",
  "name": "LLVMAddPromoteMemoryToRegisterPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Utils.h:38:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Utils.h:41:6
export const LLVMAddAddDiscriminatorsPass = {
  "tag": "function",
  "name": "LLVMAddAddDiscriminatorsPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Utils.h:41:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:31:6
export const LLVMAddConstantMergePass = {
  "tag": "function",
  "name": "LLVMAddConstantMergePass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:31:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:34:6
export const LLVMAddMergeFunctionsPass = {
  "tag": "function",
  "name": "LLVMAddMergeFunctionsPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:34:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:37:6
export const LLVMAddCalledValuePropagationPass = {
  "tag": "function",
  "name": "LLVMAddCalledValuePropagationPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:37:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:40:6
export const LLVMAddDeadArgEliminationPass = {
  "tag": "function",
  "name": "LLVMAddDeadArgEliminationPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:40:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:43:6
export const LLVMAddFunctionAttrsPass = {
  "tag": "function",
  "name": "LLVMAddFunctionAttrsPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:43:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:46:6
export const LLVMAddFunctionInliningPass = {
  "tag": "function",
  "name": "LLVMAddFunctionInliningPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:46:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:49:6
export const LLVMAddAlwaysInlinerPass = {
  "tag": "function",
  "name": "LLVMAddAlwaysInlinerPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:49:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:52:6
export const LLVMAddGlobalDCEPass = {
  "tag": "function",
  "name": "LLVMAddGlobalDCEPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:52:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:55:6
export const LLVMAddGlobalOptimizerPass = {
  "tag": "function",
  "name": "LLVMAddGlobalOptimizerPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:55:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:58:6
export const LLVMAddPruneEHPass = {
  "tag": "function",
  "name": "LLVMAddPruneEHPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:58:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:61:6
export const LLVMAddIPSCCPPass = {
  "tag": "function",
  "name": "LLVMAddIPSCCPPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:61:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:64:6
export const LLVMAddInternalizePass = {
  "tag": "function",
  "name": "LLVMAddInternalizePass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:64:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AllButMain",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:76:6
export const LLVMAddInternalizePassWithMustPreservePredicate = {
  "tag": "function",
  "name": "LLVMAddInternalizePassWithMustPreservePredicate",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:76:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Context",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "MustPreserve",
      "type": {
        "tag": ":function-pointer"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:82:6
export const LLVMAddStripDeadPrototypesPass = {
  "tag": "function",
  "name": "LLVMAddStripDeadPrototypesPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:82:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/IPO.h:85:6
export const LLVMAddStripSymbolsPass = {
  "tag": "function",
  "name": "LLVMAddStripSymbolsPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/IPO.h:85:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:49:14
export const LLVMRunPasses = {
  "tag": "function",
  "name": "LLVMRunPasses",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:49:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Passes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "TM",
      "type": {
        "tag": "LLVMTargetMachineRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:60:27
export const LLVMCreatePassBuilderOptions = {
  "tag": "function",
  "name": "LLVMCreatePassBuilderOptions",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:60:27",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMPassBuilderOptionsRef"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:66:6
export const LLVMPassBuilderOptionsSetVerifyEach = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetVerifyEach",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:66:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "VerifyEach",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:72:6
export const LLVMPassBuilderOptionsSetDebugLogging = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetDebugLogging",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:72:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DebugLogging",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:75:6
export const LLVMPassBuilderOptionsSetLoopInterleaving = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetLoopInterleaving",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:75:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LoopInterleaving",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:78:6
export const LLVMPassBuilderOptionsSetLoopVectorization = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetLoopVectorization",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:78:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LoopVectorization",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:81:6
export const LLVMPassBuilderOptionsSetSLPVectorization = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetSLPVectorization",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:81:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SLPVectorization",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:84:6
export const LLVMPassBuilderOptionsSetLoopUnrolling = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetLoopUnrolling",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:84:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LoopUnrolling",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:87:6
export const LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:87:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ForgetAllSCEVInLoopUnroll",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:90:6
export const LLVMPassBuilderOptionsSetLicmMssaOptCap = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetLicmMssaOptCap",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:90:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LicmMssaOptCap",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:93:6
export const LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:93:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LicmMssaNoAccForPromotionCap",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:96:6
export const LLVMPassBuilderOptionsSetCallGraphProfile = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetCallGraphProfile",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:96:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "CallGraphProfile",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:99:6
export const LLVMPassBuilderOptionsSetMergeFunctions = {
  "tag": "function",
  "name": "LLVMPassBuilderOptionsSetMergeFunctions",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:99:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MergeFunctions",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/PassBuilder.h:105:6
export const LLVMDisposePassBuilderOptions = {
  "tag": "function",
  "name": "LLVMDisposePassBuilderOptions",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/PassBuilder.h:105:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Options",
      "type": {
        "tag": "LLVMPassBuilderOptionsRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/AggressiveInstCombine.h:31:6
export const LLVMAddAggressiveInstCombinerPass = {
  "tag": "function",
  "name": "LLVMAddAggressiveInstCombinerPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/AggressiveInstCombine.h:31:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:35:6
export const LLVMAddAggressiveDCEPass = {
  "tag": "function",
  "name": "LLVMAddAggressiveDCEPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:35:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:38:6
export const LLVMAddDCEPass = {
  "tag": "function",
  "name": "LLVMAddDCEPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:38:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:41:6
export const LLVMAddBitTrackingDCEPass = {
  "tag": "function",
  "name": "LLVMAddBitTrackingDCEPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:41:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:44:6
export const LLVMAddAlignmentFromAssumptionsPass = {
  "tag": "function",
  "name": "LLVMAddAlignmentFromAssumptionsPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:44:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:47:6
export const LLVMAddCFGSimplificationPass = {
  "tag": "function",
  "name": "LLVMAddCFGSimplificationPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:47:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:50:6
export const LLVMAddDeadStoreEliminationPass = {
  "tag": "function",
  "name": "LLVMAddDeadStoreEliminationPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:50:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:53:6
export const LLVMAddScalarizerPass = {
  "tag": "function",
  "name": "LLVMAddScalarizerPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:53:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:56:6
export const LLVMAddMergedLoadStoreMotionPass = {
  "tag": "function",
  "name": "LLVMAddMergedLoadStoreMotionPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:56:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:59:6
export const LLVMAddGVNPass = {
  "tag": "function",
  "name": "LLVMAddGVNPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:59:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:62:6
export const LLVMAddNewGVNPass = {
  "tag": "function",
  "name": "LLVMAddNewGVNPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:62:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:65:6
export const LLVMAddIndVarSimplifyPass = {
  "tag": "function",
  "name": "LLVMAddIndVarSimplifyPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:65:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:68:6
export const LLVMAddInstructionCombiningPass = {
  "tag": "function",
  "name": "LLVMAddInstructionCombiningPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:68:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:71:6
export const LLVMAddInstructionSimplifyPass = {
  "tag": "function",
  "name": "LLVMAddInstructionSimplifyPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:71:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:74:6
export const LLVMAddJumpThreadingPass = {
  "tag": "function",
  "name": "LLVMAddJumpThreadingPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:74:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:77:6
export const LLVMAddLICMPass = {
  "tag": "function",
  "name": "LLVMAddLICMPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:77:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:80:6
export const LLVMAddLoopDeletionPass = {
  "tag": "function",
  "name": "LLVMAddLoopDeletionPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:80:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:83:6
export const LLVMAddLoopIdiomPass = {
  "tag": "function",
  "name": "LLVMAddLoopIdiomPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:83:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:86:6
export const LLVMAddLoopRotatePass = {
  "tag": "function",
  "name": "LLVMAddLoopRotatePass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:86:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:89:6
export const LLVMAddLoopRerollPass = {
  "tag": "function",
  "name": "LLVMAddLoopRerollPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:89:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:92:6
export const LLVMAddLoopUnrollPass = {
  "tag": "function",
  "name": "LLVMAddLoopUnrollPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:92:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:95:6
export const LLVMAddLoopUnrollAndJamPass = {
  "tag": "function",
  "name": "LLVMAddLoopUnrollAndJamPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:95:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:98:6
export const LLVMAddLowerAtomicPass = {
  "tag": "function",
  "name": "LLVMAddLowerAtomicPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:98:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:101:6
export const LLVMAddMemCpyOptPass = {
  "tag": "function",
  "name": "LLVMAddMemCpyOptPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:101:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:104:6
export const LLVMAddPartiallyInlineLibCallsPass = {
  "tag": "function",
  "name": "LLVMAddPartiallyInlineLibCallsPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:104:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:107:6
export const LLVMAddReassociatePass = {
  "tag": "function",
  "name": "LLVMAddReassociatePass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:107:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:110:6
export const LLVMAddSCCPPass = {
  "tag": "function",
  "name": "LLVMAddSCCPPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:110:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:113:6
export const LLVMAddScalarReplAggregatesPass = {
  "tag": "function",
  "name": "LLVMAddScalarReplAggregatesPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:113:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:116:6
export const LLVMAddScalarReplAggregatesPassSSA = {
  "tag": "function",
  "name": "LLVMAddScalarReplAggregatesPassSSA",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:116:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:119:6
export const LLVMAddScalarReplAggregatesPassWithThreshold = {
  "tag": "function",
  "name": "LLVMAddScalarReplAggregatesPassWithThreshold",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:119:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Threshold",
      "type": {
        "tag": ":int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:123:6
export const LLVMAddSimplifyLibCallsPass = {
  "tag": "function",
  "name": "LLVMAddSimplifyLibCallsPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:123:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:126:6
export const LLVMAddTailCallEliminationPass = {
  "tag": "function",
  "name": "LLVMAddTailCallEliminationPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:126:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:129:6
export const LLVMAddDemoteMemoryToRegisterPass = {
  "tag": "function",
  "name": "LLVMAddDemoteMemoryToRegisterPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:129:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:132:6
export const LLVMAddVerifierPass = {
  "tag": "function",
  "name": "LLVMAddVerifierPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:132:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:135:6
export const LLVMAddCorrelatedValuePropagationPass = {
  "tag": "function",
  "name": "LLVMAddCorrelatedValuePropagationPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:135:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:138:6
export const LLVMAddEarlyCSEPass = {
  "tag": "function",
  "name": "LLVMAddEarlyCSEPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:138:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:141:6
export const LLVMAddEarlyCSEMemSSAPass = {
  "tag": "function",
  "name": "LLVMAddEarlyCSEMemSSAPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:141:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:144:6
export const LLVMAddLowerExpectIntrinsicPass = {
  "tag": "function",
  "name": "LLVMAddLowerExpectIntrinsicPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:144:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:147:6
export const LLVMAddLowerConstantIntrinsicsPass = {
  "tag": "function",
  "name": "LLVMAddLowerConstantIntrinsicsPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:147:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:150:6
export const LLVMAddTypeBasedAliasAnalysisPass = {
  "tag": "function",
  "name": "LLVMAddTypeBasedAliasAnalysisPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:150:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:153:6
export const LLVMAddScopedNoAliasAAPass = {
  "tag": "function",
  "name": "LLVMAddScopedNoAliasAAPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:153:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:156:6
export const LLVMAddBasicAliasAnalysisPass = {
  "tag": "function",
  "name": "LLVMAddBasicAliasAnalysisPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:156:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Scalar.h:159:6
export const LLVMAddUnifyFunctionExitNodesPass = {
  "tag": "function",
  "name": "LLVMAddUnifyFunctionExitNodesPass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Scalar.h:159:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Vectorize.h:36:6
export const LLVMAddLoopVectorizePass = {
  "tag": "function",
  "name": "LLVMAddLoopVectorizePass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Vectorize.h:36:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Transforms/Vectorize.h:39:6
export const LLVMAddSLPVectorizePass = {
  "tag": "function",
  "name": "LLVMAddSLPVectorizePass",
  "ns": 0,
  "location": "/data/./llvm-c/Transforms/Vectorize.h:39:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Linker.h:41:10
export const LLVMLinkModules2 = {
  "tag": "function",
  "name": "LLVMLinkModules2",
  "ns": 0,
  "location": "/data/./llvm-c/Linker.h:41:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Dest",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Src",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Remarks.h:64:20
export const LLVMRemarkStringGetData = {
  "tag": "function",
  "name": "LLVMRemarkStringGetData",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:64:20",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "String",
      "type": {
        "tag": "LLVMRemarkStringRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Remarks.h:71:17
export const LLVMRemarkStringGetLen = {
  "tag": "function",
  "name": "LLVMRemarkStringGetLen",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:71:17",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "String",
      "type": {
        "tag": "LLVMRemarkStringRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint32_t"
  }
};

// ./llvm-c/Remarks.h:86:1
export const LLVMRemarkDebugLocGetSourceFilePath = {
  "tag": "function",
  "name": "LLVMRemarkDebugLocGetSourceFilePath",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:86:1",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DL",
      "type": {
        "tag": "LLVMRemarkDebugLocRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkStringRef"
  }
};

// ./llvm-c/Remarks.h:93:17
export const LLVMRemarkDebugLocGetSourceLine = {
  "tag": "function",
  "name": "LLVMRemarkDebugLocGetSourceLine",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:93:17",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DL",
      "type": {
        "tag": "LLVMRemarkDebugLocRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint32_t"
  }
};

// ./llvm-c/Remarks.h:100:17
export const LLVMRemarkDebugLocGetSourceColumn = {
  "tag": "function",
  "name": "LLVMRemarkDebugLocGetSourceColumn",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:100:17",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DL",
      "type": {
        "tag": "LLVMRemarkDebugLocRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint32_t"
  }
};

// ./llvm-c/Remarks.h:117:28
export const LLVMRemarkArgGetKey = {
  "tag": "function",
  "name": "LLVMRemarkArgGetKey",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:117:28",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Arg",
      "type": {
        "tag": "LLVMRemarkArgRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkStringRef"
  }
};

// ./llvm-c/Remarks.h:124:28
export const LLVMRemarkArgGetValue = {
  "tag": "function",
  "name": "LLVMRemarkArgGetValue",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:124:28",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Arg",
      "type": {
        "tag": "LLVMRemarkArgRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkStringRef"
  }
};

// ./llvm-c/Remarks.h:133:30
export const LLVMRemarkArgGetDebugLoc = {
  "tag": "function",
  "name": "LLVMRemarkArgGetDebugLoc",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:133:30",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Arg",
      "type": {
        "tag": "LLVMRemarkArgRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkDebugLocRef"
  }
};

// ./llvm-c/Remarks.h:147:13
export const LLVMRemarkEntryDispose = {
  "tag": "function",
  "name": "LLVMRemarkEntryDispose",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:147:13",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Remarks.h:155:28
export const LLVMRemarkEntryGetType = {
  "tag": "function",
  "name": "LLVMRemarkEntryGetType",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:155:28",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":enum",
    "name": "LLVMRemarkType",
    "id": 0
  }
};

// ./llvm-c/Remarks.h:163:1
export const LLVMRemarkEntryGetPassName = {
  "tag": "function",
  "name": "LLVMRemarkEntryGetPassName",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:163:1",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkStringRef"
  }
};

// ./llvm-c/Remarks.h:171:1
export const LLVMRemarkEntryGetRemarkName = {
  "tag": "function",
  "name": "LLVMRemarkEntryGetRemarkName",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:171:1",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkStringRef"
  }
};

// ./llvm-c/Remarks.h:179:1
export const LLVMRemarkEntryGetFunctionName = {
  "tag": "function",
  "name": "LLVMRemarkEntryGetFunctionName",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:179:1",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkStringRef"
  }
};

// ./llvm-c/Remarks.h:189:1
export const LLVMRemarkEntryGetDebugLoc = {
  "tag": "function",
  "name": "LLVMRemarkEntryGetDebugLoc",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:189:1",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkDebugLocRef"
  }
};

// ./llvm-c/Remarks.h:198:17
export const LLVMRemarkEntryGetHotness = {
  "tag": "function",
  "name": "LLVMRemarkEntryGetHotness",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:198:17",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/Remarks.h:205:17
export const LLVMRemarkEntryGetNumArgs = {
  "tag": "function",
  "name": "LLVMRemarkEntryGetNumArgs",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:205:17",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint32_t"
  }
};

// ./llvm-c/Remarks.h:216:25
export const LLVMRemarkEntryGetFirstArg = {
  "tag": "function",
  "name": "LLVMRemarkEntryGetFirstArg",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:216:25",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkArgRef"
  }
};

// ./llvm-c/Remarks.h:227:25
export const LLVMRemarkEntryGetNextArg = {
  "tag": "function",
  "name": "LLVMRemarkEntryGetNextArg",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:227:25",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "It",
      "type": {
        "tag": "LLVMRemarkArgRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Remark",
      "type": {
        "tag": "LLVMRemarkEntryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkArgRef"
  }
};

// ./llvm-c/Remarks.h:243:28
export const LLVMRemarkParserCreateYAML = {
  "tag": "function",
  "name": "LLVMRemarkParserCreateYAML",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:243:28",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Buf",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Size",
      "type": {
        "tag": "uint64_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkParserRef"
  }
};

// ./llvm-c/Remarks.h:257:28
export const LLVMRemarkParserCreateBitstream = {
  "tag": "function",
  "name": "LLVMRemarkParserCreateBitstream",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:257:28",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Buf",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Size",
      "type": {
        "tag": "uint64_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkParserRef"
  }
};

// ./llvm-c/Remarks.h:302:27
export const LLVMRemarkParserGetNext = {
  "tag": "function",
  "name": "LLVMRemarkParserGetNext",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:302:27",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Parser",
      "type": {
        "tag": "LLVMRemarkParserRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRemarkEntryRef"
  }
};

// ./llvm-c/Remarks.h:309:17
export const LLVMRemarkParserHasError = {
  "tag": "function",
  "name": "LLVMRemarkParserHasError",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:309:17",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Parser",
      "type": {
        "tag": "LLVMRemarkParserRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Remarks.h:322:20
export const LLVMRemarkParserGetErrorMessage = {
  "tag": "function",
  "name": "LLVMRemarkParserGetErrorMessage",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:322:20",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Parser",
      "type": {
        "tag": "LLVMRemarkParserRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Remarks.h:329:13
export const LLVMRemarkParserDispose = {
  "tag": "function",
  "name": "LLVMRemarkParserDispose",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:329:13",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Parser",
      "type": {
        "tag": "LLVMRemarkParserRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Remarks.h:336:17
export const LLVMRemarkVersion = {
  "tag": "function",
  "name": "LLVMRemarkVersion",
  "ns": 0,
  "location": "/data/./llvm-c/Remarks.h:336:17",
  "variadic": false,
  "inline": false,
  "storage-class": "extern",
  "parameters": [],
  "return-type": {
    "tag": "uint32_t"
  }
};

// ./llvm-c/ErrorHandling.h:36:6
export const LLVMInstallFatalErrorHandler = {
  "tag": "function",
  "name": "LLVMInstallFatalErrorHandler",
  "ns": 0,
  "location": "/usr/include/llvm-c/ErrorHandling.h:36:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Handler",
      "type": {
        "tag": "LLVMFatalErrorHandler"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ErrorHandling.h:42:6
export const LLVMResetFatalErrorHandler = {
  "tag": "function",
  "name": "LLVMResetFatalErrorHandler",
  "ns": 0,
  "location": "/usr/include/llvm-c/ErrorHandling.h:42:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/ErrorHandling.h:49:6
export const LLVMEnablePrettyStackTrace = {
  "tag": "function",
  "name": "LLVMEnablePrettyStackTrace",
  "ns": 0,
  "location": "/usr/include/llvm-c/ErrorHandling.h:49:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:475:6
export const LLVMInitializeCore = {
  "tag": "function",
  "name": "LLVMInitializeCore",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:475:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:480:6
export const LLVMShutdown = {
  "tag": "function",
  "name": "LLVMShutdown",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:480:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:484:7
export const LLVMCreateMessage = {
  "tag": "function",
  "name": "LLVMCreateMessage",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:484:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Message",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:485:6
export const LLVMDisposeMessage = {
  "tag": "function",
  "name": "LLVMDisposeMessage",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:485:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Message",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:508:16
export const LLVMContextCreate = {
  "tag": "function",
  "name": "LLVMContextCreate",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:508:16",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMContextRef"
  }
};

// ./llvm-c/Core.h:513:16
export const LLVMGetGlobalContext = {
  "tag": "function",
  "name": "LLVMGetGlobalContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:513:16",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMContextRef"
  }
};

// ./llvm-c/Core.h:518:6
export const LLVMContextSetDiagnosticHandler = {
  "tag": "function",
  "name": "LLVMContextSetDiagnosticHandler",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:518:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Handler",
      "type": {
        "tag": "LLVMDiagnosticHandler"
      }
    },
    {
      "tag": "parameter",
      "name": "DiagnosticContext",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:525:23
export const LLVMContextGetDiagnosticHandler = {
  "tag": "function",
  "name": "LLVMContextGetDiagnosticHandler",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:525:23",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMDiagnosticHandler"
  }
};

// ./llvm-c/Core.h:530:7
export const LLVMContextGetDiagnosticContext = {
  "tag": "function",
  "name": "LLVMContextGetDiagnosticContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:530:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":void"
    }
  }
};

// ./llvm-c/Core.h:537:6
export const LLVMContextSetYieldCallback = {
  "tag": "function",
  "name": "LLVMContextSetYieldCallback",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:537:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Callback",
      "type": {
        "tag": "LLVMYieldCallback"
      }
    },
    {
      "tag": "parameter",
      "name": "OpaqueHandle",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:545:10
export const LLVMContextShouldDiscardValueNames = {
  "tag": "function",
  "name": "LLVMContextShouldDiscardValueNames",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:545:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:555:6
export const LLVMContextSetDiscardValueNames = {
  "tag": "function",
  "name": "LLVMContextSetDiscardValueNames",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:555:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Discard",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:562:6
export const LLVMContextSetOpaquePointers = {
  "tag": "function",
  "name": "LLVMContextSetOpaquePointers",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:562:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OpaquePointers",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:570:6
export const LLVMContextDispose = {
  "tag": "function",
  "name": "LLVMContextDispose",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:570:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:578:7
export const LLVMGetDiagInfoDescription = {
  "tag": "function",
  "name": "LLVMGetDiagInfoDescription",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:578:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DI",
      "type": {
        "tag": "LLVMDiagnosticInfoRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:585:24
export const LLVMGetDiagInfoSeverity = {
  "tag": "function",
  "name": "LLVMGetDiagInfoSeverity",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:585:24",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "DI",
      "type": {
        "tag": "LLVMDiagnosticInfoRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMDiagnosticSeverity"
  }
};

// ./llvm-c/Core.h:587:10
export const LLVMGetMDKindIDInContext = {
  "tag": "function",
  "name": "LLVMGetMDKindIDInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:587:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:589:10
export const LLVMGetMDKindID = {
  "tag": "function",
  "name": "LLVMGetMDKindID",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:589:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:602:10
export const LLVMGetEnumAttributeKindForName = {
  "tag": "function",
  "name": "LLVMGetEnumAttributeKindForName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:602:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:603:10
export const LLVMGetLastEnumAttributeKind = {
  "tag": "function",
  "name": "LLVMGetLastEnumAttributeKind",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:603:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:608:18
export const LLVMCreateEnumAttribute = {
  "tag": "function",
  "name": "LLVMCreateEnumAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:608:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "KindID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "uint64_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAttributeRef"
  }
};

// ./llvm-c/Core.h:615:10
export const LLVMGetEnumAttributeKind = {
  "tag": "function",
  "name": "LLVMGetEnumAttributeKind",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:615:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:620:10
export const LLVMGetEnumAttributeValue = {
  "tag": "function",
  "name": "LLVMGetEnumAttributeValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:620:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/Core.h:625:18
export const LLVMCreateTypeAttribute = {
  "tag": "function",
  "name": "LLVMCreateTypeAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:625:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "KindID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "type_ref",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAttributeRef"
  }
};

// ./llvm-c/Core.h:631:13
export const LLVMGetTypeAttributeValue = {
  "tag": "function",
  "name": "LLVMGetTypeAttributeValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:631:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:636:18
export const LLVMCreateStringAttribute = {
  "tag": "function",
  "name": "LLVMCreateStringAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:636:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "K",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "KLength",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "VLength",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAttributeRef"
  }
};

// ./llvm-c/Core.h:643:13
export const LLVMGetStringAttributeKind = {
  "tag": "function",
  "name": "LLVMGetStringAttributeKind",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:643:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":unsigned-int",
          "bit-size": 32,
          "bit-alignment": 32
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:648:13
export const LLVMGetStringAttributeValue = {
  "tag": "function",
  "name": "LLVMGetStringAttributeValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:648:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":unsigned-int",
          "bit-size": 32,
          "bit-alignment": 32
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:653:10
export const LLVMIsEnumAttribute = {
  "tag": "function",
  "name": "LLVMIsEnumAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:653:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:654:10
export const LLVMIsStringAttribute = {
  "tag": "function",
  "name": "LLVMIsStringAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:654:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:655:10
export const LLVMIsTypeAttribute = {
  "tag": "function",
  "name": "LLVMIsTypeAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:655:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:660:13
export const LLVMGetTypeByName2 = {
  "tag": "function",
  "name": "LLVMGetTypeByName2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:660:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:685:15
export const LLVMModuleCreateWithName = {
  "tag": "function",
  "name": "LLVMModuleCreateWithName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:685:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ModuleID",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMModuleRef"
  }
};

// ./llvm-c/Core.h:693:15
export const LLVMModuleCreateWithNameInContext = {
  "tag": "function",
  "name": "LLVMModuleCreateWithNameInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:693:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ModuleID",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMModuleRef"
  }
};

// ./llvm-c/Core.h:698:15
export const LLVMCloneModule = {
  "tag": "function",
  "name": "LLVMCloneModule",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:698:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMModuleRef"
  }
};

// ./llvm-c/Core.h:706:6
export const LLVMDisposeModule = {
  "tag": "function",
  "name": "LLVMDisposeModule",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:706:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:716:13
export const LLVMGetModuleIdentifier = {
  "tag": "function",
  "name": "LLVMGetModuleIdentifier",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:716:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:726:6
export const LLVMSetModuleIdentifier = {
  "tag": "function",
  "name": "LLVMSetModuleIdentifier",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:726:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ident",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:736:13
export const LLVMGetSourceFileName = {
  "tag": "function",
  "name": "LLVMGetSourceFileName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:736:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:747:6
export const LLVMSetSourceFileName = {
  "tag": "function",
  "name": "LLVMSetSourceFileName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:747:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:758:13
export const LLVMGetDataLayoutStr = {
  "tag": "function",
  "name": "LLVMGetDataLayoutStr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:758:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:759:13
export const LLVMGetDataLayout = {
  "tag": "function",
  "name": "LLVMGetDataLayout",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:759:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:766:6
export const LLVMSetDataLayout = {
  "tag": "function",
  "name": "LLVMSetDataLayout",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:766:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DataLayoutStr",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:773:13
export const LLVMGetTarget = {
  "tag": "function",
  "name": "LLVMGetTarget",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:773:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:780:6
export const LLVMSetTarget = {
  "tag": "function",
  "name": "LLVMSetTarget",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:780:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Triple",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:789:22
export const LLVMCopyModuleFlagsMetadata = {
  "tag": "function",
  "name": "LLVMCopyModuleFlagsMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:789:22",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": "LLVMModuleFlagEntry"
    }
  }
};

// ./llvm-c/Core.h:794:6
export const LLVMDisposeModuleFlagsMetadata = {
  "tag": "function",
  "name": "LLVMDisposeModuleFlagsMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:794:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Entries",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleFlagEntry"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:802:1
export const LLVMModuleFlagEntriesGetFlagBehavior = {
  "tag": "function",
  "name": "LLVMModuleFlagEntriesGetFlagBehavior",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:802:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Entries",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleFlagEntry"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMModuleFlagBehavior"
  }
};

// ./llvm-c/Core.h:810:13
export const LLVMModuleFlagEntriesGetKey = {
  "tag": "function",
  "name": "LLVMModuleFlagEntriesGetKey",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:810:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Entries",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleFlagEntry"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:818:17
export const LLVMModuleFlagEntriesGetMetadata = {
  "tag": "function",
  "name": "LLVMModuleFlagEntriesGetMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:818:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Entries",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleFlagEntry"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/Core.h:827:17
export const LLVMGetModuleFlag = {
  "tag": "function",
  "name": "LLVMGetModuleFlag",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:827:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Key",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "KeyLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/Core.h:836:6
export const LLVMAddModuleFlag = {
  "tag": "function",
  "name": "LLVMAddModuleFlag",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:836:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Behavior",
      "type": {
        "tag": "LLVMModuleFlagBehavior"
      }
    },
    {
      "tag": "parameter",
      "name": "Key",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "KeyLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:845:6
export const LLVMDumpModule = {
  "tag": "function",
  "name": "LLVMDumpModule",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:845:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:853:10
export const LLVMPrintModuleToFile = {
  "tag": "function",
  "name": "LLVMPrintModuleToFile",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:853:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Filename",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ErrorMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:862:7
export const LLVMPrintModuleToString = {
  "tag": "function",
  "name": "LLVMPrintModuleToString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:862:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:869:13
export const LLVMGetModuleInlineAsm = {
  "tag": "function",
  "name": "LLVMGetModuleInlineAsm",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:869:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:876:6
export const LLVMSetModuleInlineAsm2 = {
  "tag": "function",
  "name": "LLVMSetModuleInlineAsm2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:876:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Asm",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:883:6
export const LLVMAppendModuleInlineAsm = {
  "tag": "function",
  "name": "LLVMAppendModuleInlineAsm",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:883:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Asm",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:890:14
export const LLVMGetInlineAsm = {
  "tag": "function",
  "name": "LLVMGetInlineAsm",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:890:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AsmString",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "AsmStringSize",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Constraints",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ConstraintsSize",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "HasSideEffects",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "IsAlignStack",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "Dialect",
      "type": {
        "tag": "LLVMInlineAsmDialect"
      }
    },
    {
      "tag": "parameter",
      "name": "CanThrow",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:901:16
export const LLVMGetModuleContext = {
  "tag": "function",
  "name": "LLVMGetModuleContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:901:16",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMContextRef"
  }
};

// ./llvm-c/Core.h:904:13
export const LLVMGetTypeByName = {
  "tag": "function",
  "name": "LLVMGetTypeByName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:904:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:911:20
export const LLVMGetFirstNamedMetadata = {
  "tag": "function",
  "name": "LLVMGetFirstNamedMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:911:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMNamedMDNodeRef"
  }
};

// ./llvm-c/Core.h:918:20
export const LLVMGetLastNamedMetadata = {
  "tag": "function",
  "name": "LLVMGetLastNamedMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:918:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMNamedMDNodeRef"
  }
};

// ./llvm-c/Core.h:926:20
export const LLVMGetNextNamedMetadata = {
  "tag": "function",
  "name": "LLVMGetNextNamedMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:926:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "NamedMDNode",
      "type": {
        "tag": "LLVMNamedMDNodeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMNamedMDNodeRef"
  }
};

// ./llvm-c/Core.h:934:20
export const LLVMGetPreviousNamedMetadata = {
  "tag": "function",
  "name": "LLVMGetPreviousNamedMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:934:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "NamedMDNode",
      "type": {
        "tag": "LLVMNamedMDNodeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMNamedMDNodeRef"
  }
};

// ./llvm-c/Core.h:942:20
export const LLVMGetNamedMetadata = {
  "tag": "function",
  "name": "LLVMGetNamedMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:942:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMNamedMDNodeRef"
  }
};

// ./llvm-c/Core.h:951:20
export const LLVMGetOrInsertNamedMetadata = {
  "tag": "function",
  "name": "LLVMGetOrInsertNamedMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:951:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMNamedMDNodeRef"
  }
};

// ./llvm-c/Core.h:960:13
export const LLVMGetNamedMetadataName = {
  "tag": "function",
  "name": "LLVMGetNamedMetadataName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:960:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "NamedMD",
      "type": {
        "tag": "LLVMNamedMDNodeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:968:10
export const LLVMGetNamedMetadataNumOperands = {
  "tag": "function",
  "name": "LLVMGetNamedMetadataNumOperands",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:968:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:981:6
export const LLVMGetNamedMetadataOperands = {
  "tag": "function",
  "name": "LLVMGetNamedMetadataOperands",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:981:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Dest",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:990:6
export const LLVMAddNamedMetadataOperand = {
  "tag": "function",
  "name": "LLVMAddNamedMetadataOperand",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:990:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1001:13
export const LLVMGetDebugLocDirectory = {
  "tag": "function",
  "name": "LLVMGetDebugLocDirectory",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1001:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":unsigned-int",
          "bit-size": 32,
          "bit-alignment": 32
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:1011:13
export const LLVMGetDebugLocFilename = {
  "tag": "function",
  "name": "LLVMGetDebugLocFilename",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1011:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":unsigned-int",
          "bit-size": 32,
          "bit-alignment": 32
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:1021:10
export const LLVMGetDebugLocLine = {
  "tag": "function",
  "name": "LLVMGetDebugLocLine",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1021:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1029:10
export const LLVMGetDebugLocColumn = {
  "tag": "function",
  "name": "LLVMGetDebugLocColumn",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1029:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1036:14
export const LLVMAddFunction = {
  "tag": "function",
  "name": "LLVMAddFunction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1036:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "FunctionTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1046:14
export const LLVMGetNamedFunction = {
  "tag": "function",
  "name": "LLVMGetNamedFunction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1046:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1053:14
export const LLVMGetFirstFunction = {
  "tag": "function",
  "name": "LLVMGetFirstFunction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1053:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1060:14
export const LLVMGetLastFunction = {
  "tag": "function",
  "name": "LLVMGetLastFunction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1060:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1068:14
export const LLVMGetNextFunction = {
  "tag": "function",
  "name": "LLVMGetNextFunction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1068:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1076:14
export const LLVMGetPreviousFunction = {
  "tag": "function",
  "name": "LLVMGetPreviousFunction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1076:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1079:6
export const LLVMSetModuleInlineAsm = {
  "tag": "function",
  "name": "LLVMSetModuleInlineAsm",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1079:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Asm",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1119:14
export const LLVMGetTypeKind = {
  "tag": "function",
  "name": "LLVMGetTypeKind",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1119:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeKind"
  }
};

// ./llvm-c/Core.h:1128:10
export const LLVMTypeIsSized = {
  "tag": "function",
  "name": "LLVMTypeIsSized",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1128:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1135:16
export const LLVMGetTypeContext = {
  "tag": "function",
  "name": "LLVMGetTypeContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1135:16",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMContextRef"
  }
};

// ./llvm-c/Core.h:1142:6
export const LLVMDumpType = {
  "tag": "function",
  "name": "LLVMDumpType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1142:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1150:7
export const LLVMPrintTypeToString = {
  "tag": "function",
  "name": "LLVMPrintTypeToString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1150:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:1163:13
export const LLVMInt1TypeInContext = {
  "tag": "function",
  "name": "LLVMInt1TypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1163:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1164:13
export const LLVMInt8TypeInContext = {
  "tag": "function",
  "name": "LLVMInt8TypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1164:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1165:13
export const LLVMInt16TypeInContext = {
  "tag": "function",
  "name": "LLVMInt16TypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1165:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1166:13
export const LLVMInt32TypeInContext = {
  "tag": "function",
  "name": "LLVMInt32TypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1166:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1167:13
export const LLVMInt64TypeInContext = {
  "tag": "function",
  "name": "LLVMInt64TypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1167:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1168:13
export const LLVMInt128TypeInContext = {
  "tag": "function",
  "name": "LLVMInt128TypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1168:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1169:13
export const LLVMIntTypeInContext = {
  "tag": "function",
  "name": "LLVMIntTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1169:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumBits",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1175:13
export const LLVMInt1Type = {
  "tag": "function",
  "name": "LLVMInt1Type",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1175:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1176:13
export const LLVMInt8Type = {
  "tag": "function",
  "name": "LLVMInt8Type",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1176:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1177:13
export const LLVMInt16Type = {
  "tag": "function",
  "name": "LLVMInt16Type",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1177:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1178:13
export const LLVMInt32Type = {
  "tag": "function",
  "name": "LLVMInt32Type",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1178:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1179:13
export const LLVMInt64Type = {
  "tag": "function",
  "name": "LLVMInt64Type",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1179:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1180:13
export const LLVMInt128Type = {
  "tag": "function",
  "name": "LLVMInt128Type",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1180:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1181:13
export const LLVMIntType = {
  "tag": "function",
  "name": "LLVMIntType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1181:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "NumBits",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1182:10
export const LLVMGetIntTypeWidth = {
  "tag": "function",
  "name": "LLVMGetIntTypeWidth",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1182:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IntegerTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1197:13
export const LLVMHalfTypeInContext = {
  "tag": "function",
  "name": "LLVMHalfTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1197:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1202:13
export const LLVMBFloatTypeInContext = {
  "tag": "function",
  "name": "LLVMBFloatTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1202:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1207:13
export const LLVMFloatTypeInContext = {
  "tag": "function",
  "name": "LLVMFloatTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1207:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1212:13
export const LLVMDoubleTypeInContext = {
  "tag": "function",
  "name": "LLVMDoubleTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1212:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1217:13
export const LLVMX86FP80TypeInContext = {
  "tag": "function",
  "name": "LLVMX86FP80TypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1217:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1223:13
export const LLVMFP128TypeInContext = {
  "tag": "function",
  "name": "LLVMFP128TypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1223:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1228:13
export const LLVMPPCFP128TypeInContext = {
  "tag": "function",
  "name": "LLVMPPCFP128TypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1228:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1235:13
export const LLVMHalfType = {
  "tag": "function",
  "name": "LLVMHalfType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1235:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1236:13
export const LLVMBFloatType = {
  "tag": "function",
  "name": "LLVMBFloatType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1236:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1237:13
export const LLVMFloatType = {
  "tag": "function",
  "name": "LLVMFloatType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1237:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1238:13
export const LLVMDoubleType = {
  "tag": "function",
  "name": "LLVMDoubleType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1238:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1239:13
export const LLVMX86FP80Type = {
  "tag": "function",
  "name": "LLVMX86FP80Type",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1239:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1240:13
export const LLVMFP128Type = {
  "tag": "function",
  "name": "LLVMFP128Type",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1240:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1241:13
export const LLVMPPCFP128Type = {
  "tag": "function",
  "name": "LLVMPPCFP128Type",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1241:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1259:13
export const LLVMFunctionType = {
  "tag": "function",
  "name": "LLVMFunctionType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1259:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ReturnType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ParamTypes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ParamCount",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "IsVarArg",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1266:10
export const LLVMIsFunctionVarArg = {
  "tag": "function",
  "name": "LLVMIsFunctionVarArg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1266:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "FunctionTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1271:13
export const LLVMGetReturnType = {
  "tag": "function",
  "name": "LLVMGetReturnType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1271:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "FunctionTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1276:10
export const LLVMCountParamTypes = {
  "tag": "function",
  "name": "LLVMCountParamTypes",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1276:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "FunctionTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1289:6
export const LLVMGetParamTypes = {
  "tag": "function",
  "name": "LLVMGetParamTypes",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1289:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "FunctionTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dest",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1313:13
export const LLVMStructTypeInContext = {
  "tag": "function",
  "name": "LLVMStructTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1313:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ElementTypes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ElementCount",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Packed",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1321:13
export const LLVMStructType = {
  "tag": "function",
  "name": "LLVMStructType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1321:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ElementTypes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ElementCount",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Packed",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1329:13
export const LLVMStructCreateNamed = {
  "tag": "function",
  "name": "LLVMStructCreateNamed",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1329:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1336:13
export const LLVMGetStructName = {
  "tag": "function",
  "name": "LLVMGetStructName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1336:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:1343:6
export const LLVMStructSetBody = {
  "tag": "function",
  "name": "LLVMStructSetBody",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1343:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ElementTypes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ElementCount",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Packed",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1351:10
export const LLVMCountStructElementTypes = {
  "tag": "function",
  "name": "LLVMCountStructElementTypes",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1351:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1363:6
export const LLVMGetStructElementTypes = {
  "tag": "function",
  "name": "LLVMGetStructElementTypes",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1363:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dest",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1370:13
export const LLVMStructGetTypeAtIndex = {
  "tag": "function",
  "name": "LLVMStructGetTypeAtIndex",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1370:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "i",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1377:10
export const LLVMIsPackedStruct = {
  "tag": "function",
  "name": "LLVMIsPackedStruct",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1377:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1384:10
export const LLVMIsOpaqueStruct = {
  "tag": "function",
  "name": "LLVMIsOpaqueStruct",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1384:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1391:10
export const LLVMIsLiteralStruct = {
  "tag": "function",
  "name": "LLVMIsLiteralStruct",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1391:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1413:13
export const LLVMGetElementType = {
  "tag": "function",
  "name": "LLVMGetElementType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1413:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1420:6
export const LLVMGetSubtypes = {
  "tag": "function",
  "name": "LLVMGetSubtypes",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1420:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Tp",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Arr",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1427:10
export const LLVMGetNumContainedTypes = {
  "tag": "function",
  "name": "LLVMGetNumContainedTypes",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1427:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Tp",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1437:13
export const LLVMArrayType = {
  "tag": "function",
  "name": "LLVMArrayType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1437:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ElementType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ElementCount",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1446:10
export const LLVMGetArrayLength = {
  "tag": "function",
  "name": "LLVMGetArrayLength",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1446:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ArrayTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1456:13
export const LLVMPointerType = {
  "tag": "function",
  "name": "LLVMPointerType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1456:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ElementType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AddressSpace",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1465:10
export const LLVMPointerTypeIsOpaque = {
  "tag": "function",
  "name": "LLVMPointerTypeIsOpaque",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1465:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1472:13
export const LLVMPointerTypeInContext = {
  "tag": "function",
  "name": "LLVMPointerTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1472:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AddressSpace",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1481:10
export const LLVMGetPointerAddressSpace = {
  "tag": "function",
  "name": "LLVMGetPointerAddressSpace",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1481:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PointerTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1492:13
export const LLVMVectorType = {
  "tag": "function",
  "name": "LLVMVectorType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1492:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ElementType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ElementCount",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1503:13
export const LLVMScalableVectorType = {
  "tag": "function",
  "name": "LLVMScalableVectorType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1503:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ElementType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ElementCount",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1513:10
export const LLVMGetVectorSize = {
  "tag": "function",
  "name": "LLVMGetVectorSize",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1513:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "VectorTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1528:13
export const LLVMVoidTypeInContext = {
  "tag": "function",
  "name": "LLVMVoidTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1528:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1533:13
export const LLVMLabelTypeInContext = {
  "tag": "function",
  "name": "LLVMLabelTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1533:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1538:13
export const LLVMX86MMXTypeInContext = {
  "tag": "function",
  "name": "LLVMX86MMXTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1538:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1543:13
export const LLVMX86AMXTypeInContext = {
  "tag": "function",
  "name": "LLVMX86AMXTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1543:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1548:13
export const LLVMTokenTypeInContext = {
  "tag": "function",
  "name": "LLVMTokenTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1548:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1553:13
export const LLVMMetadataTypeInContext = {
  "tag": "function",
  "name": "LLVMMetadataTypeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1553:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1559:13
export const LLVMVoidType = {
  "tag": "function",
  "name": "LLVMVoidType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1559:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1560:13
export const LLVMLabelType = {
  "tag": "function",
  "name": "LLVMLabelType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1560:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1561:13
export const LLVMX86MMXType = {
  "tag": "function",
  "name": "LLVMX86MMXType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1561:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1562:13
export const LLVMX86AMXType = {
  "tag": "function",
  "name": "LLVMX86AMXType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1562:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1698:13
export const LLVMTypeOf = {
  "tag": "function",
  "name": "LLVMTypeOf",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1698:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:1705:15
export const LLVMGetValueKind = {
  "tag": "function",
  "name": "LLVMGetValueKind",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1705:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueKind"
  }
};

// ./llvm-c/Core.h:1712:13
export const LLVMGetValueName2 = {
  "tag": "function",
  "name": "LLVMGetValueName2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1712:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:1719:6
export const LLVMSetValueName2 = {
  "tag": "function",
  "name": "LLVMSetValueName2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1719:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1726:6
export const LLVMDumpValue = {
  "tag": "function",
  "name": "LLVMDumpValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1726:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1734:7
export const LLVMPrintValueToString = {
  "tag": "function",
  "name": "LLVMPrintValueToString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1734:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:1741:6
export const LLVMReplaceAllUsesWith = {
  "tag": "function",
  "name": "LLVMReplaceAllUsesWith",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1741:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "OldVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NewVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1746:10
export const LLVMIsConstant = {
  "tag": "function",
  "name": "LLVMIsConstant",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1746:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1751:10
export const LLVMIsUndef = {
  "tag": "function",
  "name": "LLVMIsUndef",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1751:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1756:10
export const LLVMIsPoison = {
  "tag": "function",
  "name": "LLVMIsPoison",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1756:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:67:1>
export const LLVMIsAArgument = {
  "tag": "function",
  "name": "LLVMIsAArgument",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:67:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:68:1>
export const LLVMIsABasicBlock = {
  "tag": "function",
  "name": "LLVMIsABasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:68:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:69:1>
export const LLVMIsAInlineAsm = {
  "tag": "function",
  "name": "LLVMIsAInlineAsm",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:69:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:70:1>
export const LLVMIsAUser = {
  "tag": "function",
  "name": "LLVMIsAUser",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:70:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:71:1>
export const LLVMIsAConstant = {
  "tag": "function",
  "name": "LLVMIsAConstant",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:71:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:72:1>
export const LLVMIsABlockAddress = {
  "tag": "function",
  "name": "LLVMIsABlockAddress",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:72:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:73:1>
export const LLVMIsAConstantAggregateZero = {
  "tag": "function",
  "name": "LLVMIsAConstantAggregateZero",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:73:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:74:1>
export const LLVMIsAConstantArray = {
  "tag": "function",
  "name": "LLVMIsAConstantArray",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:74:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:75:1>
export const LLVMIsAConstantDataSequential = {
  "tag": "function",
  "name": "LLVMIsAConstantDataSequential",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:75:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:76:1>
export const LLVMIsAConstantDataArray = {
  "tag": "function",
  "name": "LLVMIsAConstantDataArray",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:76:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:77:1>
export const LLVMIsAConstantDataVector = {
  "tag": "function",
  "name": "LLVMIsAConstantDataVector",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:77:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:78:1>
export const LLVMIsAConstantExpr = {
  "tag": "function",
  "name": "LLVMIsAConstantExpr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:78:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:79:1>
export const LLVMIsAConstantFP = {
  "tag": "function",
  "name": "LLVMIsAConstantFP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:79:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:80:1>
export const LLVMIsAConstantInt = {
  "tag": "function",
  "name": "LLVMIsAConstantInt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:80:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:81:1>
export const LLVMIsAConstantPointerNull = {
  "tag": "function",
  "name": "LLVMIsAConstantPointerNull",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:81:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:82:1>
export const LLVMIsAConstantStruct = {
  "tag": "function",
  "name": "LLVMIsAConstantStruct",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:82:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:83:1>
export const LLVMIsAConstantTokenNone = {
  "tag": "function",
  "name": "LLVMIsAConstantTokenNone",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:83:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:84:1>
export const LLVMIsAConstantVector = {
  "tag": "function",
  "name": "LLVMIsAConstantVector",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:84:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:85:1>
export const LLVMIsAGlobalValue = {
  "tag": "function",
  "name": "LLVMIsAGlobalValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:85:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:86:1>
export const LLVMIsAGlobalAlias = {
  "tag": "function",
  "name": "LLVMIsAGlobalAlias",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:86:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:87:1>
export const LLVMIsAGlobalObject = {
  "tag": "function",
  "name": "LLVMIsAGlobalObject",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:87:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:88:1>
export const LLVMIsAFunction = {
  "tag": "function",
  "name": "LLVMIsAFunction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:88:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:89:1>
export const LLVMIsAGlobalVariable = {
  "tag": "function",
  "name": "LLVMIsAGlobalVariable",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:89:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:90:1>
export const LLVMIsAGlobalIFunc = {
  "tag": "function",
  "name": "LLVMIsAGlobalIFunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:90:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:91:1>
export const LLVMIsAUndefValue = {
  "tag": "function",
  "name": "LLVMIsAUndefValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:91:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:92:1>
export const LLVMIsAPoisonValue = {
  "tag": "function",
  "name": "LLVMIsAPoisonValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:92:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:93:1>
export const LLVMIsAInstruction = {
  "tag": "function",
  "name": "LLVMIsAInstruction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:93:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:94:1>
export const LLVMIsAUnaryOperator = {
  "tag": "function",
  "name": "LLVMIsAUnaryOperator",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:94:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:95:1>
export const LLVMIsABinaryOperator = {
  "tag": "function",
  "name": "LLVMIsABinaryOperator",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:95:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:96:1>
export const LLVMIsACallInst = {
  "tag": "function",
  "name": "LLVMIsACallInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:96:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:97:1>
export const LLVMIsAIntrinsicInst = {
  "tag": "function",
  "name": "LLVMIsAIntrinsicInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:97:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:98:1>
export const LLVMIsADbgInfoIntrinsic = {
  "tag": "function",
  "name": "LLVMIsADbgInfoIntrinsic",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:98:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:99:1>
export const LLVMIsADbgVariableIntrinsic = {
  "tag": "function",
  "name": "LLVMIsADbgVariableIntrinsic",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:99:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:100:1>
export const LLVMIsADbgDeclareInst = {
  "tag": "function",
  "name": "LLVMIsADbgDeclareInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:100:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:101:1>
export const LLVMIsADbgLabelInst = {
  "tag": "function",
  "name": "LLVMIsADbgLabelInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:101:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:102:1>
export const LLVMIsAMemIntrinsic = {
  "tag": "function",
  "name": "LLVMIsAMemIntrinsic",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:102:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:103:1>
export const LLVMIsAMemCpyInst = {
  "tag": "function",
  "name": "LLVMIsAMemCpyInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:103:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:104:1>
export const LLVMIsAMemMoveInst = {
  "tag": "function",
  "name": "LLVMIsAMemMoveInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:104:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:105:1>
export const LLVMIsAMemSetInst = {
  "tag": "function",
  "name": "LLVMIsAMemSetInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:105:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:106:1>
export const LLVMIsACmpInst = {
  "tag": "function",
  "name": "LLVMIsACmpInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:106:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:107:1>
export const LLVMIsAFCmpInst = {
  "tag": "function",
  "name": "LLVMIsAFCmpInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:107:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:108:1>
export const LLVMIsAICmpInst = {
  "tag": "function",
  "name": "LLVMIsAICmpInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:108:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:109:1>
export const LLVMIsAExtractElementInst = {
  "tag": "function",
  "name": "LLVMIsAExtractElementInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:109:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:110:1>
export const LLVMIsAGetElementPtrInst = {
  "tag": "function",
  "name": "LLVMIsAGetElementPtrInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:110:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:111:1>
export const LLVMIsAInsertElementInst = {
  "tag": "function",
  "name": "LLVMIsAInsertElementInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:111:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:112:1>
export const LLVMIsAInsertValueInst = {
  "tag": "function",
  "name": "LLVMIsAInsertValueInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:112:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:113:1>
export const LLVMIsALandingPadInst = {
  "tag": "function",
  "name": "LLVMIsALandingPadInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:113:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:114:1>
export const LLVMIsAPHINode = {
  "tag": "function",
  "name": "LLVMIsAPHINode",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:114:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:115:1>
export const LLVMIsASelectInst = {
  "tag": "function",
  "name": "LLVMIsASelectInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:115:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:116:1>
export const LLVMIsAShuffleVectorInst = {
  "tag": "function",
  "name": "LLVMIsAShuffleVectorInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:116:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:117:1>
export const LLVMIsAStoreInst = {
  "tag": "function",
  "name": "LLVMIsAStoreInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:117:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:118:1>
export const LLVMIsABranchInst = {
  "tag": "function",
  "name": "LLVMIsABranchInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:118:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:119:1>
export const LLVMIsAIndirectBrInst = {
  "tag": "function",
  "name": "LLVMIsAIndirectBrInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:119:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:120:1>
export const LLVMIsAInvokeInst = {
  "tag": "function",
  "name": "LLVMIsAInvokeInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:120:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:121:1>
export const LLVMIsAReturnInst = {
  "tag": "function",
  "name": "LLVMIsAReturnInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:121:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:122:1>
export const LLVMIsASwitchInst = {
  "tag": "function",
  "name": "LLVMIsASwitchInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:122:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:123:1>
export const LLVMIsAUnreachableInst = {
  "tag": "function",
  "name": "LLVMIsAUnreachableInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:123:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:124:1>
export const LLVMIsAResumeInst = {
  "tag": "function",
  "name": "LLVMIsAResumeInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:124:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:125:1>
export const LLVMIsACleanupReturnInst = {
  "tag": "function",
  "name": "LLVMIsACleanupReturnInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:125:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:126:1>
export const LLVMIsACatchReturnInst = {
  "tag": "function",
  "name": "LLVMIsACatchReturnInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:126:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:127:1>
export const LLVMIsACatchSwitchInst = {
  "tag": "function",
  "name": "LLVMIsACatchSwitchInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:127:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:128:1>
export const LLVMIsACallBrInst = {
  "tag": "function",
  "name": "LLVMIsACallBrInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:128:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:129:1>
export const LLVMIsAFuncletPadInst = {
  "tag": "function",
  "name": "LLVMIsAFuncletPadInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:129:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:130:1>
export const LLVMIsACatchPadInst = {
  "tag": "function",
  "name": "LLVMIsACatchPadInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:130:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:131:1>
export const LLVMIsACleanupPadInst = {
  "tag": "function",
  "name": "LLVMIsACleanupPadInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:131:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:132:1>
export const LLVMIsAUnaryInstruction = {
  "tag": "function",
  "name": "LLVMIsAUnaryInstruction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:132:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:133:1>
export const LLVMIsAAllocaInst = {
  "tag": "function",
  "name": "LLVMIsAAllocaInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:133:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:134:1>
export const LLVMIsACastInst = {
  "tag": "function",
  "name": "LLVMIsACastInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:134:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:135:1>
export const LLVMIsAAddrSpaceCastInst = {
  "tag": "function",
  "name": "LLVMIsAAddrSpaceCastInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:135:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:136:1>
export const LLVMIsABitCastInst = {
  "tag": "function",
  "name": "LLVMIsABitCastInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:136:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:137:1>
export const LLVMIsAFPExtInst = {
  "tag": "function",
  "name": "LLVMIsAFPExtInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:137:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:138:1>
export const LLVMIsAFPToSIInst = {
  "tag": "function",
  "name": "LLVMIsAFPToSIInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:138:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:139:1>
export const LLVMIsAFPToUIInst = {
  "tag": "function",
  "name": "LLVMIsAFPToUIInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:139:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:140:1>
export const LLVMIsAFPTruncInst = {
  "tag": "function",
  "name": "LLVMIsAFPTruncInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:140:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:141:1>
export const LLVMIsAIntToPtrInst = {
  "tag": "function",
  "name": "LLVMIsAIntToPtrInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:141:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:142:1>
export const LLVMIsAPtrToIntInst = {
  "tag": "function",
  "name": "LLVMIsAPtrToIntInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:142:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:143:1>
export const LLVMIsASExtInst = {
  "tag": "function",
  "name": "LLVMIsASExtInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:143:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:144:1>
export const LLVMIsASIToFPInst = {
  "tag": "function",
  "name": "LLVMIsASIToFPInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:144:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:145:1>
export const LLVMIsATruncInst = {
  "tag": "function",
  "name": "LLVMIsATruncInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:145:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:146:1>
export const LLVMIsAUIToFPInst = {
  "tag": "function",
  "name": "LLVMIsAUIToFPInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:146:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:147:1>
export const LLVMIsAZExtInst = {
  "tag": "function",
  "name": "LLVMIsAZExtInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:147:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:148:1>
export const LLVMIsAExtractValueInst = {
  "tag": "function",
  "name": "LLVMIsAExtractValueInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:148:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:149:1>
export const LLVMIsALoadInst = {
  "tag": "function",
  "name": "LLVMIsALoadInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:149:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:150:1>
export const LLVMIsAVAArgInst = {
  "tag": "function",
  "name": "LLVMIsAVAArgInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:150:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:151:1>
export const LLVMIsAFreezeInst = {
  "tag": "function",
  "name": "LLVMIsAFreezeInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:151:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:152:1>
export const LLVMIsAAtomicCmpXchgInst = {
  "tag": "function",
  "name": "LLVMIsAAtomicCmpXchgInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:152:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:153:1>
export const LLVMIsAAtomicRMWInst = {
  "tag": "function",
  "name": "LLVMIsAAtomicRMWInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:153:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:154:1>
export const LLVMIsAFenceInst = {
  "tag": "function",
  "name": "LLVMIsAFenceInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1771:1 <Spelling=<scratch space>:154:1>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1773:14
export const LLVMIsAMDNode = {
  "tag": "function",
  "name": "LLVMIsAMDNode",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1773:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1774:14
export const LLVMIsAMDString = {
  "tag": "function",
  "name": "LLVMIsAMDString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1774:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1777:13
export const LLVMGetValueName = {
  "tag": "function",
  "name": "LLVMGetValueName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1777:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:1779:6
export const LLVMSetValueName = {
  "tag": "function",
  "name": "LLVMSetValueName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1779:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1808:12
export const LLVMGetFirstUse = {
  "tag": "function",
  "name": "LLVMGetFirstUse",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1808:12",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMUseRef"
  }
};

// ./llvm-c/Core.h:1816:12
export const LLVMGetNextUse = {
  "tag": "function",
  "name": "LLVMGetNextUse",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1816:12",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "U",
      "type": {
        "tag": "LLVMUseRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMUseRef"
  }
};

// ./llvm-c/Core.h:1825:14
export const LLVMGetUser = {
  "tag": "function",
  "name": "LLVMGetUser",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1825:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "U",
      "type": {
        "tag": "LLVMUseRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1832:14
export const LLVMGetUsedValue = {
  "tag": "function",
  "name": "LLVMGetUsedValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1832:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "U",
      "type": {
        "tag": "LLVMUseRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1853:14
export const LLVMGetOperand = {
  "tag": "function",
  "name": "LLVMGetOperand",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1853:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1860:12
export const LLVMGetOperandUse = {
  "tag": "function",
  "name": "LLVMGetOperandUse",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1860:12",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMUseRef"
  }
};

// ./llvm-c/Core.h:1867:6
export const LLVMSetOperand = {
  "tag": "function",
  "name": "LLVMSetOperand",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1867:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "User",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:1874:5
export const LLVMGetNumOperands = {
  "tag": "function",
  "name": "LLVMGetNumOperands",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1874:5",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:1897:14
export const LLVMConstNull = {
  "tag": "function",
  "name": "LLVMConstNull",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1897:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1907:14
export const LLVMConstAllOnes = {
  "tag": "function",
  "name": "LLVMConstAllOnes",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1907:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1914:14
export const LLVMGetUndef = {
  "tag": "function",
  "name": "LLVMGetUndef",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1914:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1921:14
export const LLVMGetPoison = {
  "tag": "function",
  "name": "LLVMGetPoison",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1921:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1928:10
export const LLVMIsNull = {
  "tag": "function",
  "name": "LLVMIsNull",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1928:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:1934:14
export const LLVMConstPointerNull = {
  "tag": "function",
  "name": "LLVMConstPointerNull",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1934:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1963:14
export const LLVMConstInt = {
  "tag": "function",
  "name": "LLVMConstInt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1963:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IntTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "N",
      "type": {
        "tag": ":unsigned-long-long",
        "bit-size": 64,
        "bit-alignment": 64
      }
    },
    {
      "tag": "parameter",
      "name": "SignExtend",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1971:14
export const LLVMConstIntOfArbitraryPrecision = {
  "tag": "function",
  "name": "LLVMConstIntOfArbitraryPrecision",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1971:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IntTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumWords",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Words",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "uint64_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1984:14
export const LLVMConstIntOfString = {
  "tag": "function",
  "name": "LLVMConstIntOfString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1984:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IntTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Text",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Radix",
      "type": {
        "tag": "uint8_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1993:14
export const LLVMConstIntOfStringAndSize = {
  "tag": "function",
  "name": "LLVMConstIntOfStringAndSize",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1993:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IntTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Text",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Radix",
      "type": {
        "tag": "uint8_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:1999:14
export const LLVMConstReal = {
  "tag": "function",
  "name": "LLVMConstReal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:1999:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RealTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "N",
      "type": {
        "tag": ":double",
        "bit-size": 64,
        "bit-alignment": 64
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2007:14
export const LLVMConstRealOfString = {
  "tag": "function",
  "name": "LLVMConstRealOfString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2007:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RealTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Text",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2012:14
export const LLVMConstRealOfStringAndSize = {
  "tag": "function",
  "name": "LLVMConstRealOfStringAndSize",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2012:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RealTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Text",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2020:20
export const LLVMConstIntGetZExtValue = {
  "tag": "function",
  "name": "LLVMConstIntGetZExtValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2020:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-long-long",
    "bit-size": 64,
    "bit-alignment": 64
  }
};

// ./llvm-c/Core.h:2027:11
export const LLVMConstIntGetSExtValue = {
  "tag": "function",
  "name": "LLVMConstIntGetSExtValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2027:11",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":long-long",
    "bit-size": 64,
    "bit-alignment": 64
  }
};

// ./llvm-c/Core.h:2035:8
export const LLVMConstRealGetDouble = {
  "tag": "function",
  "name": "LLVMConstRealGetDouble",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2035:8",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "losesInfo",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMBool"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":double",
    "bit-size": 64,
    "bit-alignment": 64
  }
};

// ./llvm-c/Core.h:2054:14
export const LLVMConstStringInContext = {
  "tag": "function",
  "name": "LLVMConstStringInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2054:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Str",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "DontNullTerminate",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2066:14
export const LLVMConstString = {
  "tag": "function",
  "name": "LLVMConstString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2066:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Str",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "DontNullTerminate",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2074:10
export const LLVMIsConstantString = {
  "tag": "function",
  "name": "LLVMIsConstantString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2074:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "c",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:2081:13
export const LLVMGetAsString = {
  "tag": "function",
  "name": "LLVMGetAsString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2081:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "c",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:2088:14
export const LLVMConstStructInContext = {
  "tag": "function",
  "name": "LLVMConstStructInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2088:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantVals",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Count",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Packed",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2100:14
export const LLVMConstStruct = {
  "tag": "function",
  "name": "LLVMConstStruct",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2100:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVals",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Count",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Packed",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2108:14
export const LLVMConstArray = {
  "tag": "function",
  "name": "LLVMConstArray",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2108:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ElementTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantVals",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2116:14
export const LLVMConstNamedStruct = {
  "tag": "function",
  "name": "LLVMConstNamedStruct",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2116:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "StructTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantVals",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Count",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2128:14
export const LLVMGetAggregateElement = {
  "tag": "function",
  "name": "LLVMGetAggregateElement",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2128:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2135:1 <Spelling=/data/./llvm-c/Core.h:2136:18>
export const LLVMGetElementAsConstant = {
  "tag": "function",
  "name": "LLVMGetElementAsConstant",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2135:1 <Spelling=/data/./llvm-c/Core.h:2136:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "idx",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2144:14
export const LLVMConstVector = {
  "tag": "function",
  "name": "LLVMConstVector",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2144:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ScalarConstantVals",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Size",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2159:12
export const LLVMGetConstOpcode = {
  "tag": "function",
  "name": "LLVMGetConstOpcode",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2159:12",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOpcode"
  }
};

// ./llvm-c/Core.h:2160:14
export const LLVMAlignOf = {
  "tag": "function",
  "name": "LLVMAlignOf",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2160:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2161:14
export const LLVMSizeOf = {
  "tag": "function",
  "name": "LLVMSizeOf",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2161:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2162:14
export const LLVMConstNeg = {
  "tag": "function",
  "name": "LLVMConstNeg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2162:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2163:14
export const LLVMConstNSWNeg = {
  "tag": "function",
  "name": "LLVMConstNSWNeg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2163:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2164:14
export const LLVMConstNUWNeg = {
  "tag": "function",
  "name": "LLVMConstNUWNeg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2164:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2165:14
export const LLVMConstFNeg = {
  "tag": "function",
  "name": "LLVMConstFNeg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2165:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2166:14
export const LLVMConstNot = {
  "tag": "function",
  "name": "LLVMConstNot",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2166:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2167:14
export const LLVMConstAdd = {
  "tag": "function",
  "name": "LLVMConstAdd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2167:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2168:14
export const LLVMConstNSWAdd = {
  "tag": "function",
  "name": "LLVMConstNSWAdd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2168:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2169:14
export const LLVMConstNUWAdd = {
  "tag": "function",
  "name": "LLVMConstNUWAdd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2169:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2170:14
export const LLVMConstSub = {
  "tag": "function",
  "name": "LLVMConstSub",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2170:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2171:14
export const LLVMConstNSWSub = {
  "tag": "function",
  "name": "LLVMConstNSWSub",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2171:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2172:14
export const LLVMConstNUWSub = {
  "tag": "function",
  "name": "LLVMConstNUWSub",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2172:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2173:14
export const LLVMConstMul = {
  "tag": "function",
  "name": "LLVMConstMul",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2173:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2174:14
export const LLVMConstNSWMul = {
  "tag": "function",
  "name": "LLVMConstNSWMul",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2174:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2175:14
export const LLVMConstNUWMul = {
  "tag": "function",
  "name": "LLVMConstNUWMul",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2175:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2176:14
export const LLVMConstAnd = {
  "tag": "function",
  "name": "LLVMConstAnd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2176:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2177:14
export const LLVMConstOr = {
  "tag": "function",
  "name": "LLVMConstOr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2177:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2178:14
export const LLVMConstXor = {
  "tag": "function",
  "name": "LLVMConstXor",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2178:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2179:14
export const LLVMConstICmp = {
  "tag": "function",
  "name": "LLVMConstICmp",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2179:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Predicate",
      "type": {
        "tag": "LLVMIntPredicate"
      }
    },
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2181:14
export const LLVMConstFCmp = {
  "tag": "function",
  "name": "LLVMConstFCmp",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2181:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Predicate",
      "type": {
        "tag": "LLVMRealPredicate"
      }
    },
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2183:14
export const LLVMConstShl = {
  "tag": "function",
  "name": "LLVMConstShl",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2183:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2184:14
export const LLVMConstLShr = {
  "tag": "function",
  "name": "LLVMConstLShr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2184:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2185:14
export const LLVMConstAShr = {
  "tag": "function",
  "name": "LLVMConstAShr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2185:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHSConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2186:1 <Spelling=/data/./llvm-c/Core.h:2187:18>
export const LLVMConstGEP = {
  "tag": "function",
  "name": "LLVMConstGEP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2186:1 <Spelling=/data/./llvm-c/Core.h:2187:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantIndices",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumIndices",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2191:14
export const LLVMConstGEP2 = {
  "tag": "function",
  "name": "LLVMConstGEP2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2191:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantIndices",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumIndices",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2193:1 <Spelling=/data/./llvm-c/Core.h:2194:18>
export const LLVMConstInBoundsGEP = {
  "tag": "function",
  "name": "LLVMConstInBoundsGEP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2193:1 <Spelling=/data/./llvm-c/Core.h:2194:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantIndices",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumIndices",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2198:14
export const LLVMConstInBoundsGEP2 = {
  "tag": "function",
  "name": "LLVMConstInBoundsGEP2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2198:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantIndices",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumIndices",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2201:14
export const LLVMConstTrunc = {
  "tag": "function",
  "name": "LLVMConstTrunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2201:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2202:14
export const LLVMConstSExt = {
  "tag": "function",
  "name": "LLVMConstSExt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2202:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2203:14
export const LLVMConstZExt = {
  "tag": "function",
  "name": "LLVMConstZExt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2203:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2204:14
export const LLVMConstFPTrunc = {
  "tag": "function",
  "name": "LLVMConstFPTrunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2204:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2205:14
export const LLVMConstFPExt = {
  "tag": "function",
  "name": "LLVMConstFPExt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2205:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2206:14
export const LLVMConstUIToFP = {
  "tag": "function",
  "name": "LLVMConstUIToFP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2206:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2207:14
export const LLVMConstSIToFP = {
  "tag": "function",
  "name": "LLVMConstSIToFP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2207:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2208:14
export const LLVMConstFPToUI = {
  "tag": "function",
  "name": "LLVMConstFPToUI",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2208:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2209:14
export const LLVMConstFPToSI = {
  "tag": "function",
  "name": "LLVMConstFPToSI",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2209:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2210:14
export const LLVMConstPtrToInt = {
  "tag": "function",
  "name": "LLVMConstPtrToInt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2210:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2211:14
export const LLVMConstIntToPtr = {
  "tag": "function",
  "name": "LLVMConstIntToPtr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2211:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2212:14
export const LLVMConstBitCast = {
  "tag": "function",
  "name": "LLVMConstBitCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2212:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2213:14
export const LLVMConstAddrSpaceCast = {
  "tag": "function",
  "name": "LLVMConstAddrSpaceCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2213:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2214:14
export const LLVMConstZExtOrBitCast = {
  "tag": "function",
  "name": "LLVMConstZExtOrBitCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2214:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2216:14
export const LLVMConstSExtOrBitCast = {
  "tag": "function",
  "name": "LLVMConstSExtOrBitCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2216:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2218:14
export const LLVMConstTruncOrBitCast = {
  "tag": "function",
  "name": "LLVMConstTruncOrBitCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2218:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2220:14
export const LLVMConstPointerCast = {
  "tag": "function",
  "name": "LLVMConstPointerCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2220:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2222:14
export const LLVMConstIntCast = {
  "tag": "function",
  "name": "LLVMConstIntCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2222:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "isSigned",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2224:14
export const LLVMConstFPCast = {
  "tag": "function",
  "name": "LLVMConstFPCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2224:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ToType",
      "type": {
        "tag": "LLVMTypeRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2225:14
export const LLVMConstSelect = {
  "tag": "function",
  "name": "LLVMConstSelect",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2225:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ConstantCondition",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantIfTrue",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantIfFalse",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2228:14
export const LLVMConstExtractElement = {
  "tag": "function",
  "name": "LLVMConstExtractElement",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2228:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "VectorConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IndexConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2230:14
export const LLVMConstInsertElement = {
  "tag": "function",
  "name": "LLVMConstInsertElement",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2230:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "VectorConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ElementValueConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IndexConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2233:14
export const LLVMConstShuffleVector = {
  "tag": "function",
  "name": "LLVMConstShuffleVector",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2233:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "VectorAConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "VectorBConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MaskConstant",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2236:14
export const LLVMBlockAddress = {
  "tag": "function",
  "name": "LLVMBlockAddress",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2236:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2239:14
export const LLVMConstInlineAsm = {
  "tag": "function",
  "name": "LLVMConstInlineAsm",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2239:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AsmString",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Constraints",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "HasSideEffects",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "IsAlignStack",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2258:15
export const LLVMGetGlobalParent = {
  "tag": "function",
  "name": "LLVMGetGlobalParent",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2258:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMModuleRef"
  }
};

// ./llvm-c/Core.h:2259:10
export const LLVMIsDeclaration = {
  "tag": "function",
  "name": "LLVMIsDeclaration",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2259:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:2260:13
export const LLVMGetLinkage = {
  "tag": "function",
  "name": "LLVMGetLinkage",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2260:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMLinkage"
  }
};

// ./llvm-c/Core.h:2261:6
export const LLVMSetLinkage = {
  "tag": "function",
  "name": "LLVMSetLinkage",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2261:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Linkage",
      "type": {
        "tag": "LLVMLinkage"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2262:13
export const LLVMGetSection = {
  "tag": "function",
  "name": "LLVMGetSection",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2262:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:2263:6
export const LLVMSetSection = {
  "tag": "function",
  "name": "LLVMSetSection",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2263:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Section",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2264:16
export const LLVMGetVisibility = {
  "tag": "function",
  "name": "LLVMGetVisibility",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2264:16",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMVisibility"
  }
};

// ./llvm-c/Core.h:2265:6
export const LLVMSetVisibility = {
  "tag": "function",
  "name": "LLVMSetVisibility",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2265:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Viz",
      "type": {
        "tag": "LLVMVisibility"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2266:21
export const LLVMGetDLLStorageClass = {
  "tag": "function",
  "name": "LLVMGetDLLStorageClass",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2266:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMDLLStorageClass"
  }
};

// ./llvm-c/Core.h:2267:6
export const LLVMSetDLLStorageClass = {
  "tag": "function",
  "name": "LLVMSetDLLStorageClass",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2267:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Class",
      "type": {
        "tag": "LLVMDLLStorageClass"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2268:17
export const LLVMGetUnnamedAddress = {
  "tag": "function",
  "name": "LLVMGetUnnamedAddress",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2268:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMUnnamedAddr"
  }
};

// ./llvm-c/Core.h:2269:6
export const LLVMSetUnnamedAddress = {
  "tag": "function",
  "name": "LLVMSetUnnamedAddress",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2269:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "UnnamedAddr",
      "type": {
        "tag": "LLVMUnnamedAddr"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2277:13
export const LLVMGlobalGetValueType = {
  "tag": "function",
  "name": "LLVMGlobalGetValueType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2277:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:2280:10
export const LLVMHasUnnamedAddr = {
  "tag": "function",
  "name": "LLVMHasUnnamedAddr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2280:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:2282:6
export const LLVMSetUnnamedAddr = {
  "tag": "function",
  "name": "LLVMSetUnnamedAddr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2282:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "HasUnnamedAddr",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2300:10
export const LLVMGetAlignment = {
  "tag": "function",
  "name": "LLVMGetAlignment",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2300:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:2311:6
export const LLVMSetAlignment = {
  "tag": "function",
  "name": "LLVMSetAlignment",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2311:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Bytes",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2319:6
export const LLVMGlobalSetMetadata = {
  "tag": "function",
  "name": "LLVMGlobalSetMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2319:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Kind",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "MD",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2327:6
export const LLVMGlobalEraseMetadata = {
  "tag": "function",
  "name": "LLVMGlobalEraseMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2327:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Kind",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2334:6
export const LLVMGlobalClearMetadata = {
  "tag": "function",
  "name": "LLVMGlobalClearMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2334:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Global",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2343:25
export const LLVMGlobalCopyAllMetadata = {
  "tag": "function",
  "name": "LLVMGlobalCopyAllMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2343:25",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Value",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumEntries",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": "LLVMValueMetadataEntry"
    }
  }
};

// ./llvm-c/Core.h:2349:6
export const LLVMDisposeValueMetadataEntries = {
  "tag": "function",
  "name": "LLVMDisposeValueMetadataEntries",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2349:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Entries",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueMetadataEntry"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2354:10
export const LLVMValueMetadataEntriesGetKind = {
  "tag": "function",
  "name": "LLVMValueMetadataEntriesGetKind",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2354:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Entries",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueMetadataEntry"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:2362:1
export const LLVMValueMetadataEntriesGetMetadata = {
  "tag": "function",
  "name": "LLVMValueMetadataEntriesGetMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2362:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Entries",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueMetadataEntry"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/Core.h:2378:14
export const LLVMAddGlobal = {
  "tag": "function",
  "name": "LLVMAddGlobal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2378:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2379:14
export const LLVMAddGlobalInAddressSpace = {
  "tag": "function",
  "name": "LLVMAddGlobalInAddressSpace",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2379:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "AddressSpace",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2382:14
export const LLVMGetNamedGlobal = {
  "tag": "function",
  "name": "LLVMGetNamedGlobal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2382:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2383:14
export const LLVMGetFirstGlobal = {
  "tag": "function",
  "name": "LLVMGetFirstGlobal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2383:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2384:14
export const LLVMGetLastGlobal = {
  "tag": "function",
  "name": "LLVMGetLastGlobal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2384:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2385:14
export const LLVMGetNextGlobal = {
  "tag": "function",
  "name": "LLVMGetNextGlobal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2385:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2386:14
export const LLVMGetPreviousGlobal = {
  "tag": "function",
  "name": "LLVMGetPreviousGlobal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2386:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2387:6
export const LLVMDeleteGlobal = {
  "tag": "function",
  "name": "LLVMDeleteGlobal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2387:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2388:14
export const LLVMGetInitializer = {
  "tag": "function",
  "name": "LLVMGetInitializer",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2388:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2389:6
export const LLVMSetInitializer = {
  "tag": "function",
  "name": "LLVMSetInitializer",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2389:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ConstantVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2390:10
export const LLVMIsThreadLocal = {
  "tag": "function",
  "name": "LLVMIsThreadLocal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2390:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:2391:6
export const LLVMSetThreadLocal = {
  "tag": "function",
  "name": "LLVMSetThreadLocal",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2391:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IsThreadLocal",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2392:10
export const LLVMIsGlobalConstant = {
  "tag": "function",
  "name": "LLVMIsGlobalConstant",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2392:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:2393:6
export const LLVMSetGlobalConstant = {
  "tag": "function",
  "name": "LLVMSetGlobalConstant",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2393:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IsConstant",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2394:21
export const LLVMGetThreadLocalMode = {
  "tag": "function",
  "name": "LLVMGetThreadLocalMode",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2394:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMThreadLocalMode"
  }
};

// ./llvm-c/Core.h:2395:6
export const LLVMSetThreadLocalMode = {
  "tag": "function",
  "name": "LLVMSetThreadLocalMode",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2395:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Mode",
      "type": {
        "tag": "LLVMThreadLocalMode"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2396:10
export const LLVMIsExternallyInitialized = {
  "tag": "function",
  "name": "LLVMIsExternallyInitialized",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2396:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:2397:6
export const LLVMSetExternallyInitialized = {
  "tag": "function",
  "name": "LLVMSetExternallyInitialized",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2397:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GlobalVar",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IsExtInit",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2413:1 <Spelling=/data/./llvm-c/Core.h:2414:18>
export const LLVMAddAlias = {
  "tag": "function",
  "name": "LLVMAddAlias",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2413:1 <Spelling=/data/./llvm-c/Core.h:2414:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Aliasee",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2423:14
export const LLVMAddAlias2 = {
  "tag": "function",
  "name": "LLVMAddAlias2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2423:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ValueTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AddrSpace",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Aliasee",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2434:14
export const LLVMGetNamedGlobalAlias = {
  "tag": "function",
  "name": "LLVMGetNamedGlobalAlias",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2434:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2442:14
export const LLVMGetFirstGlobalAlias = {
  "tag": "function",
  "name": "LLVMGetFirstGlobalAlias",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2442:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2449:14
export const LLVMGetLastGlobalAlias = {
  "tag": "function",
  "name": "LLVMGetLastGlobalAlias",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2449:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2457:14
export const LLVMGetNextGlobalAlias = {
  "tag": "function",
  "name": "LLVMGetNextGlobalAlias",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2457:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GA",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2465:14
export const LLVMGetPreviousGlobalAlias = {
  "tag": "function",
  "name": "LLVMGetPreviousGlobalAlias",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2465:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GA",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2470:14
export const LLVMAliasGetAliasee = {
  "tag": "function",
  "name": "LLVMAliasGetAliasee",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2470:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Alias",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2475:6
export const LLVMAliasSetAliasee = {
  "tag": "function",
  "name": "LLVMAliasSetAliasee",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2475:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Alias",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Aliasee",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2497:6
export const LLVMDeleteFunction = {
  "tag": "function",
  "name": "LLVMDeleteFunction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2497:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2504:10
export const LLVMHasPersonalityFn = {
  "tag": "function",
  "name": "LLVMHasPersonalityFn",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2504:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:2511:14
export const LLVMGetPersonalityFn = {
  "tag": "function",
  "name": "LLVMGetPersonalityFn",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2511:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2518:6
export const LLVMSetPersonalityFn = {
  "tag": "function",
  "name": "LLVMSetPersonalityFn",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2518:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PersonalityFn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2525:10
export const LLVMLookupIntrinsicID = {
  "tag": "function",
  "name": "LLVMLookupIntrinsicID",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2525:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:2532:10
export const LLVMGetIntrinsicID = {
  "tag": "function",
  "name": "LLVMGetIntrinsicID",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2532:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:2540:14
export const LLVMGetIntrinsicDeclaration = {
  "tag": "function",
  "name": "LLVMGetIntrinsicDeclaration",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2540:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Mod",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "ParamTypes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ParamCount",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2551:13
export const LLVMIntrinsicGetType = {
  "tag": "function",
  "name": "LLVMIntrinsicGetType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2551:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "ParamTypes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ParamCount",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:2559:13
export const LLVMIntrinsicGetName = {
  "tag": "function",
  "name": "LLVMIntrinsicGetName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2559:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "NameLength",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:2562:13
export const LLVMIntrinsicCopyOverloadedName = {
  "tag": "function",
  "name": "LLVMIntrinsicCopyOverloadedName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2562:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "ParamTypes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ParamCount",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "NameLength",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:2578:13
export const LLVMIntrinsicCopyOverloadedName2 = {
  "tag": "function",
  "name": "LLVMIntrinsicCopyOverloadedName2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2578:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Mod",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "ParamTypes",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMTypeRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ParamCount",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "NameLength",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:2588:10
export const LLVMIntrinsicIsOverloaded = {
  "tag": "function",
  "name": "LLVMIntrinsicIsOverloaded",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2588:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:2597:10
export const LLVMGetFunctionCallConv = {
  "tag": "function",
  "name": "LLVMGetFunctionCallConv",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2597:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:2607:6
export const LLVMSetFunctionCallConv = {
  "tag": "function",
  "name": "LLVMSetFunctionCallConv",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2607:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "CC",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2615:13
export const LLVMGetGC = {
  "tag": "function",
  "name": "LLVMGetGC",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2615:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:2622:6
export const LLVMSetGC = {
  "tag": "function",
  "name": "LLVMSetGC",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2622:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2629:6
export const LLVMAddAttributeAtIndex = {
  "tag": "function",
  "name": "LLVMAddAttributeAtIndex",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2629:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2631:10
export const LLVMGetAttributeCountAtIndex = {
  "tag": "function",
  "name": "LLVMGetAttributeCountAtIndex",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2631:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:2632:6
export const LLVMGetAttributesAtIndex = {
  "tag": "function",
  "name": "LLVMGetAttributesAtIndex",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2632:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "Attrs",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMAttributeRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2634:18
export const LLVMGetEnumAttributeAtIndex = {
  "tag": "function",
  "name": "LLVMGetEnumAttributeAtIndex",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2634:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "KindID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAttributeRef"
  }
};

// ./llvm-c/Core.h:2637:18
export const LLVMGetStringAttributeAtIndex = {
  "tag": "function",
  "name": "LLVMGetStringAttributeAtIndex",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2637:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "K",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "KLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAttributeRef"
  }
};

// ./llvm-c/Core.h:2640:6
export const LLVMRemoveEnumAttributeAtIndex = {
  "tag": "function",
  "name": "LLVMRemoveEnumAttributeAtIndex",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2640:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "KindID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2642:6
export const LLVMRemoveStringAttributeAtIndex = {
  "tag": "function",
  "name": "LLVMRemoveStringAttributeAtIndex",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2642:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "K",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "KLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2649:6
export const LLVMAddTargetDependentFunctionAttr = {
  "tag": "function",
  "name": "LLVMAddTargetDependentFunctionAttr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2649:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2668:10
export const LLVMCountParams = {
  "tag": "function",
  "name": "LLVMCountParams",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2668:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:2681:6
export const LLVMGetParams = {
  "tag": "function",
  "name": "LLVMGetParams",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2681:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Params",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2690:14
export const LLVMGetParam = {
  "tag": "function",
  "name": "LLVMGetParam",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2690:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2701:14
export const LLVMGetParamParent = {
  "tag": "function",
  "name": "LLVMGetParamParent",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2701:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2708:14
export const LLVMGetFirstParam = {
  "tag": "function",
  "name": "LLVMGetFirstParam",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2708:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2715:14
export const LLVMGetLastParam = {
  "tag": "function",
  "name": "LLVMGetLastParam",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2715:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2724:14
export const LLVMGetNextParam = {
  "tag": "function",
  "name": "LLVMGetNextParam",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2724:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Arg",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2731:14
export const LLVMGetPreviousParam = {
  "tag": "function",
  "name": "LLVMGetPreviousParam",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2731:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Arg",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2739:6
export const LLVMSetParamAlignment = {
  "tag": "function",
  "name": "LLVMSetParamAlignment",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2739:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Arg",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Align",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2761:14
export const LLVMAddGlobalIFunc = {
  "tag": "function",
  "name": "LLVMAddGlobalIFunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2761:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AddrSpace",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Resolver",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2773:14
export const LLVMGetNamedGlobalIFunc = {
  "tag": "function",
  "name": "LLVMGetNamedGlobalIFunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2773:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NameLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2781:14
export const LLVMGetFirstGlobalIFunc = {
  "tag": "function",
  "name": "LLVMGetFirstGlobalIFunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2781:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2788:14
export const LLVMGetLastGlobalIFunc = {
  "tag": "function",
  "name": "LLVMGetLastGlobalIFunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2788:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2796:14
export const LLVMGetNextGlobalIFunc = {
  "tag": "function",
  "name": "LLVMGetNextGlobalIFunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2796:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IFunc",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2804:14
export const LLVMGetPreviousGlobalIFunc = {
  "tag": "function",
  "name": "LLVMGetPreviousGlobalIFunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2804:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IFunc",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2812:14
export const LLVMGetGlobalIFuncResolver = {
  "tag": "function",
  "name": "LLVMGetGlobalIFuncResolver",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2812:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IFunc",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2819:6
export const LLVMSetGlobalIFuncResolver = {
  "tag": "function",
  "name": "LLVMSetGlobalIFuncResolver",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2819:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IFunc",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Resolver",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2826:6
export const LLVMEraseGlobalIFunc = {
  "tag": "function",
  "name": "LLVMEraseGlobalIFunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2826:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IFunc",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2836:6
export const LLVMRemoveGlobalIFunc = {
  "tag": "function",
  "name": "LLVMRemoveGlobalIFunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2836:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IFunc",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2868:17
export const LLVMMDStringInContext2 = {
  "tag": "function",
  "name": "LLVMMDStringInContext2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2868:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Str",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SLen",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/Core.h:2876:17
export const LLVMMDNodeInContext2 = {
  "tag": "function",
  "name": "LLVMMDNodeInContext2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2876:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MDs",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMetadataRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Count",
      "type": {
        "tag": "size_t"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/Core.h:2882:14
export const LLVMMetadataAsValue = {
  "tag": "function",
  "name": "LLVMMetadataAsValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2882:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MD",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2887:17
export const LLVMValueAsMetadata = {
  "tag": "function",
  "name": "LLVMValueAsMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2887:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/Core.h:2896:13
export const LLVMGetMDString = {
  "tag": "function",
  "name": "LLVMGetMDString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2896:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Length",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":unsigned-int",
          "bit-size": 32,
          "bit-alignment": 32
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:2904:10
export const LLVMGetMDNodeNumOperands = {
  "tag": "function",
  "name": "LLVMGetMDNodeNumOperands",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2904:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:2917:6
export const LLVMGetMDNodeOperands = {
  "tag": "function",
  "name": "LLVMGetMDNodeOperands",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2917:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dest",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:2920:14
export const LLVMMDStringInContext = {
  "tag": "function",
  "name": "LLVMMDStringInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2920:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Str",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2923:14
export const LLVMMDString = {
  "tag": "function",
  "name": "LLVMMDString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2923:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Str",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "SLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2925:14
export const LLVMMDNodeInContext = {
  "tag": "function",
  "name": "LLVMMDNodeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2925:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Vals",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Count",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2928:14
export const LLVMMDNode = {
  "tag": "function",
  "name": "LLVMMDNode",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2928:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Vals",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Count",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2954:14
export const LLVMBasicBlockAsValue = {
  "tag": "function",
  "name": "LLVMBasicBlockAsValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2954:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2959:10
export const LLVMValueIsBasicBlock = {
  "tag": "function",
  "name": "LLVMValueIsBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2959:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:2964:19
export const LLVMValueAsBasicBlock = {
  "tag": "function",
  "name": "LLVMValueAsBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2964:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:2969:13
export const LLVMGetBasicBlockName = {
  "tag": "function",
  "name": "LLVMGetBasicBlockName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2969:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:2976:14
export const LLVMGetBasicBlockParent = {
  "tag": "function",
  "name": "LLVMGetBasicBlockParent",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2976:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2988:14
export const LLVMGetBasicBlockTerminator = {
  "tag": "function",
  "name": "LLVMGetBasicBlockTerminator",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2988:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:2995:10
export const LLVMCountBasicBlocks = {
  "tag": "function",
  "name": "LLVMCountBasicBlocks",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:2995:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3005:6
export const LLVMGetBasicBlocks = {
  "tag": "function",
  "name": "LLVMGetBasicBlocks",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3005:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BasicBlocks",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMBasicBlockRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3015:19
export const LLVMGetFirstBasicBlock = {
  "tag": "function",
  "name": "LLVMGetFirstBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3015:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3022:19
export const LLVMGetLastBasicBlock = {
  "tag": "function",
  "name": "LLVMGetLastBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3022:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3027:19
export const LLVMGetNextBasicBlock = {
  "tag": "function",
  "name": "LLVMGetNextBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3027:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3032:19
export const LLVMGetPreviousBasicBlock = {
  "tag": "function",
  "name": "LLVMGetPreviousBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3032:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3040:19
export const LLVMGetEntryBasicBlock = {
  "tag": "function",
  "name": "LLVMGetEntryBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3040:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3049:6
export const LLVMInsertExistingBasicBlockAfterInsertBlock = {
  "tag": "function",
  "name": "LLVMInsertExistingBasicBlockAfterInsertBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3049:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3057:6
export const LLVMAppendExistingBasicBlock = {
  "tag": "function",
  "name": "LLVMAppendExistingBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3057:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3065:19
export const LLVMCreateBasicBlockInContext = {
  "tag": "function",
  "name": "LLVMCreateBasicBlockInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3065:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3073:19
export const LLVMAppendBasicBlockInContext = {
  "tag": "function",
  "name": "LLVMAppendBasicBlockInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3073:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3083:19
export const LLVMAppendBasicBlock = {
  "tag": "function",
  "name": "LLVMAppendBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3083:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3093:19
export const LLVMInsertBasicBlockInContext = {
  "tag": "function",
  "name": "LLVMInsertBasicBlockInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3093:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3102:19
export const LLVMInsertBasicBlock = {
  "tag": "function",
  "name": "LLVMInsertBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3102:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "InsertBeforeBB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3113:6
export const LLVMDeleteBasicBlock = {
  "tag": "function",
  "name": "LLVMDeleteBasicBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3113:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3123:6
export const LLVMRemoveBasicBlockFromParent = {
  "tag": "function",
  "name": "LLVMRemoveBasicBlockFromParent",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3123:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3130:6
export const LLVMMoveBasicBlockBefore = {
  "tag": "function",
  "name": "LLVMMoveBasicBlockBefore",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3130:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MovePos",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3137:6
export const LLVMMoveBasicBlockAfter = {
  "tag": "function",
  "name": "LLVMMoveBasicBlockAfter",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3137:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MovePos",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3145:14
export const LLVMGetFirstInstruction = {
  "tag": "function",
  "name": "LLVMGetFirstInstruction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3145:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3152:14
export const LLVMGetLastInstruction = {
  "tag": "function",
  "name": "LLVMGetLastInstruction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3152:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3178:5
export const LLVMHasMetadata = {
  "tag": "function",
  "name": "LLVMHasMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3178:5",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3183:14
export const LLVMGetMetadata = {
  "tag": "function",
  "name": "LLVMGetMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3183:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "KindID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3188:6
export const LLVMSetMetadata = {
  "tag": "function",
  "name": "LLVMSetMetadata",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3188:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "KindID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Node",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3197:1
export const LLVMInstructionGetAllMetadataOtherThanDebugLoc = {
  "tag": "function",
  "name": "LLVMInstructionGetAllMetadataOtherThanDebugLoc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3197:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumEntries",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "size_t"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": "LLVMValueMetadataEntry"
    }
  }
};

// ./llvm-c/Core.h:3205:19
export const LLVMGetInstructionParent = {
  "tag": "function",
  "name": "LLVMGetInstructionParent",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3205:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3215:14
export const LLVMGetNextInstruction = {
  "tag": "function",
  "name": "LLVMGetNextInstruction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3215:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3223:14
export const LLVMGetPreviousInstruction = {
  "tag": "function",
  "name": "LLVMGetPreviousInstruction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3223:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3233:6
export const LLVMInstructionRemoveFromParent = {
  "tag": "function",
  "name": "LLVMInstructionRemoveFromParent",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3233:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3243:6
export const LLVMInstructionEraseFromParent = {
  "tag": "function",
  "name": "LLVMInstructionEraseFromParent",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3243:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3253:6
export const LLVMDeleteInstruction = {
  "tag": "function",
  "name": "LLVMDeleteInstruction",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3253:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3260:12
export const LLVMGetInstructionOpcode = {
  "tag": "function",
  "name": "LLVMGetInstructionOpcode",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3260:12",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOpcode"
  }
};

// ./llvm-c/Core.h:3270:18
export const LLVMGetICmpPredicate = {
  "tag": "function",
  "name": "LLVMGetICmpPredicate",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3270:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMIntPredicate"
  }
};

// ./llvm-c/Core.h:3280:19
export const LLVMGetFCmpPredicate = {
  "tag": "function",
  "name": "LLVMGetFCmpPredicate",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3280:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRealPredicate"
  }
};

// ./llvm-c/Core.h:3290:14
export const LLVMInstructionClone = {
  "tag": "function",
  "name": "LLVMInstructionClone",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3290:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3299:14
export const LLVMIsATerminatorInst = {
  "tag": "function",
  "name": "LLVMIsATerminatorInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3299:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3321:10
export const LLVMGetNumArgOperands = {
  "tag": "function",
  "name": "LLVMGetNumArgOperands",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3321:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3332:6
export const LLVMSetInstructionCallConv = {
  "tag": "function",
  "name": "LLVMSetInstructionCallConv",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3332:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "CC",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3342:10
export const LLVMGetInstructionCallConv = {
  "tag": "function",
  "name": "LLVMGetInstructionCallConv",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3342:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3344:6
export const LLVMSetInstrParamAlignment = {
  "tag": "function",
  "name": "LLVMSetInstrParamAlignment",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3344:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "Align",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3347:6
export const LLVMAddCallSiteAttribute = {
  "tag": "function",
  "name": "LLVMAddCallSiteAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3347:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "A",
      "type": {
        "tag": "LLVMAttributeRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3349:10
export const LLVMGetCallSiteAttributeCount = {
  "tag": "function",
  "name": "LLVMGetCallSiteAttributeCount",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3349:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3350:6
export const LLVMGetCallSiteAttributes = {
  "tag": "function",
  "name": "LLVMGetCallSiteAttributes",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3350:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "Attrs",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMAttributeRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3352:18
export const LLVMGetCallSiteEnumAttribute = {
  "tag": "function",
  "name": "LLVMGetCallSiteEnumAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3352:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "KindID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAttributeRef"
  }
};

// ./llvm-c/Core.h:3355:18
export const LLVMGetCallSiteStringAttribute = {
  "tag": "function",
  "name": "LLVMGetCallSiteStringAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3355:18",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "K",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "KLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAttributeRef"
  }
};

// ./llvm-c/Core.h:3358:6
export const LLVMRemoveCallSiteEnumAttribute = {
  "tag": "function",
  "name": "LLVMRemoveCallSiteEnumAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3358:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "KindID",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3360:6
export const LLVMRemoveCallSiteStringAttribute = {
  "tag": "function",
  "name": "LLVMRemoveCallSiteStringAttribute",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3360:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": "LLVMAttributeIndex"
      }
    },
    {
      "tag": "parameter",
      "name": "K",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "KLen",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3368:13
export const LLVMGetCalledFunctionType = {
  "tag": "function",
  "name": "LLVMGetCalledFunctionType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3368:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:3379:14
export const LLVMGetCalledValue = {
  "tag": "function",
  "name": "LLVMGetCalledValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3379:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3388:10
export const LLVMIsTailCall = {
  "tag": "function",
  "name": "LLVMIsTailCall",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3388:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CallInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:3397:6
export const LLVMSetTailCall = {
  "tag": "function",
  "name": "LLVMSetTailCall",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3397:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CallInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IsTailCall",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3406:19
export const LLVMGetNormalDest = {
  "tag": "function",
  "name": "LLVMGetNormalDest",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3406:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "InvokeInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3418:19
export const LLVMGetUnwindDest = {
  "tag": "function",
  "name": "LLVMGetUnwindDest",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3418:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "InvokeInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3427:6
export const LLVMSetNormalDest = {
  "tag": "function",
  "name": "LLVMSetNormalDest",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3427:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "InvokeInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3439:6
export const LLVMSetUnwindDest = {
  "tag": "function",
  "name": "LLVMSetUnwindDest",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3439:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "InvokeInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3459:10
export const LLVMGetNumSuccessors = {
  "tag": "function",
  "name": "LLVMGetNumSuccessors",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3459:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Term",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3466:19
export const LLVMGetSuccessor = {
  "tag": "function",
  "name": "LLVMGetSuccessor",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3466:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Term",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "i",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3473:6
export const LLVMSetSuccessor = {
  "tag": "function",
  "name": "LLVMSetSuccessor",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3473:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Term",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "i",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "block",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3482:10
export const LLVMIsConditional = {
  "tag": "function",
  "name": "LLVMIsConditional",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3482:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Branch",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:3491:14
export const LLVMGetCondition = {
  "tag": "function",
  "name": "LLVMGetCondition",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3491:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Branch",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3500:6
export const LLVMSetCondition = {
  "tag": "function",
  "name": "LLVMSetCondition",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3500:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Branch",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Cond",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3509:19
export const LLVMGetSwitchDefaultDest = {
  "tag": "function",
  "name": "LLVMGetSwitchDefaultDest",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3509:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SwitchInstr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3527:13
export const LLVMGetAllocatedType = {
  "tag": "function",
  "name": "LLVMGetAllocatedType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3527:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Alloca",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:3545:10
export const LLVMIsInBounds = {
  "tag": "function",
  "name": "LLVMIsInBounds",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3545:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GEP",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:3550:6
export const LLVMSetIsInBounds = {
  "tag": "function",
  "name": "LLVMSetIsInBounds",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3550:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GEP",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "InBounds",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3555:13
export const LLVMGetGEPSourceElementType = {
  "tag": "function",
  "name": "LLVMGetGEPSourceElementType",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3555:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "GEP",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMTypeRef"
  }
};

// ./llvm-c/Core.h:3573:6
export const LLVMAddIncoming = {
  "tag": "function",
  "name": "LLVMAddIncoming",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3573:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PhiNode",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IncomingValues",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "IncomingBlocks",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMBasicBlockRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Count",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3579:10
export const LLVMCountIncoming = {
  "tag": "function",
  "name": "LLVMCountIncoming",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3579:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PhiNode",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3584:14
export const LLVMGetIncomingValue = {
  "tag": "function",
  "name": "LLVMGetIncomingValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3584:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PhiNode",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3589:19
export const LLVMGetIncomingBlock = {
  "tag": "function",
  "name": "LLVMGetIncomingBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3589:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PhiNode",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3609:10
export const LLVMGetNumIndices = {
  "tag": "function",
  "name": "LLVMGetNumIndices",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3609:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3614:17
export const LLVMGetIndices = {
  "tag": "function",
  "name": "LLVMGetIndices",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3614:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":unsigned-int",
      "bit-size": 32,
      "bit-alignment": 32
    }
  }
};

// ./llvm-c/Core.h:3637:16
export const LLVMCreateBuilderInContext = {
  "tag": "function",
  "name": "LLVMCreateBuilderInContext",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3637:16",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMContextRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBuilderRef"
  }
};

// ./llvm-c/Core.h:3638:16
export const LLVMCreateBuilder = {
  "tag": "function",
  "name": "LLVMCreateBuilder",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3638:16",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMBuilderRef"
  }
};

// ./llvm-c/Core.h:3639:6
export const LLVMPositionBuilder = {
  "tag": "function",
  "name": "LLVMPositionBuilder",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3639:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Block",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3641:6
export const LLVMPositionBuilderBefore = {
  "tag": "function",
  "name": "LLVMPositionBuilderBefore",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3641:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3642:6
export const LLVMPositionBuilderAtEnd = {
  "tag": "function",
  "name": "LLVMPositionBuilderAtEnd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3642:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Block",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3643:19
export const LLVMGetInsertBlock = {
  "tag": "function",
  "name": "LLVMGetInsertBlock",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3643:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBasicBlockRef"
  }
};

// ./llvm-c/Core.h:3644:6
export const LLVMClearInsertionPosition = {
  "tag": "function",
  "name": "LLVMClearInsertionPosition",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3644:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3645:6
export const LLVMInsertIntoBuilder = {
  "tag": "function",
  "name": "LLVMInsertIntoBuilder",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3645:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3646:6
export const LLVMInsertIntoBuilderWithName = {
  "tag": "function",
  "name": "LLVMInsertIntoBuilderWithName",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3646:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Instr",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3648:6
export const LLVMDisposeBuilder = {
  "tag": "function",
  "name": "LLVMDisposeBuilder",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3648:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3657:17
export const LLVMGetCurrentDebugLocation2 = {
  "tag": "function",
  "name": "LLVMGetCurrentDebugLocation2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3657:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/Core.h:3666:6
export const LLVMSetCurrentDebugLocation2 = {
  "tag": "function",
  "name": "LLVMSetCurrentDebugLocation2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3666:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Loc",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3678:6
export const LLVMSetInstDebugLocation = {
  "tag": "function",
  "name": "LLVMSetInstDebugLocation",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3678:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3685:6
export const LLVMAddMetadataToInst = {
  "tag": "function",
  "name": "LLVMAddMetadataToInst",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3685:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Inst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3692:17
export const LLVMBuilderGetDefaultFPMathTag = {
  "tag": "function",
  "name": "LLVMBuilderGetDefaultFPMathTag",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3692:17",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMetadataRef"
  }
};

// ./llvm-c/Core.h:3701:6
export const LLVMBuilderSetDefaultFPMathTag = {
  "tag": "function",
  "name": "LLVMBuilderSetDefaultFPMathTag",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3701:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "FPMathTag",
      "type": {
        "tag": "LLVMMetadataRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3708:6
export const LLVMSetCurrentDebugLocation = {
  "tag": "function",
  "name": "LLVMSetCurrentDebugLocation",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3708:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "L",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3713:14
export const LLVMGetCurrentDebugLocation = {
  "tag": "function",
  "name": "LLVMGetCurrentDebugLocation",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3713:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3716:14
export const LLVMBuildRetVoid = {
  "tag": "function",
  "name": "LLVMBuildRetVoid",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3716:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3717:14
export const LLVMBuildRet = {
  "tag": "function",
  "name": "LLVMBuildRet",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3717:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3718:14
export const LLVMBuildAggregateRet = {
  "tag": "function",
  "name": "LLVMBuildAggregateRet",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3718:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RetVals",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "N",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3720:14
export const LLVMBuildBr = {
  "tag": "function",
  "name": "LLVMBuildBr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3720:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dest",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3721:14
export const LLVMBuildCondBr = {
  "tag": "function",
  "name": "LLVMBuildCondBr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3721:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "If",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Then",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Else",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3723:14
export const LLVMBuildSwitch = {
  "tag": "function",
  "name": "LLVMBuildSwitch",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3723:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Else",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumCases",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3725:14
export const LLVMBuildIndirectBr = {
  "tag": "function",
  "name": "LLVMBuildIndirectBr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3725:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Addr",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumDests",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3727:1 <Spelling=/data/./llvm-c/Core.h:3728:18>
export const LLVMBuildInvoke = {
  "tag": "function",
  "name": "LLVMBuildInvoke",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3727:1 <Spelling=/data/./llvm-c/Core.h:3728:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Args",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumArgs",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Then",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Catch",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3733:14
export const LLVMBuildInvoke2 = {
  "tag": "function",
  "name": "LLVMBuildInvoke2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3733:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Args",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumArgs",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Then",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Catch",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3737:14
export const LLVMBuildUnreachable = {
  "tag": "function",
  "name": "LLVMBuildUnreachable",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3737:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3740:14
export const LLVMBuildResume = {
  "tag": "function",
  "name": "LLVMBuildResume",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3740:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Exn",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3741:14
export const LLVMBuildLandingPad = {
  "tag": "function",
  "name": "LLVMBuildLandingPad",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3741:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PersFn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumClauses",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3744:14
export const LLVMBuildCleanupRet = {
  "tag": "function",
  "name": "LLVMBuildCleanupRet",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3744:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "CatchPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3746:14
export const LLVMBuildCatchRet = {
  "tag": "function",
  "name": "LLVMBuildCatchRet",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3746:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "CatchPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3748:14
export const LLVMBuildCatchPad = {
  "tag": "function",
  "name": "LLVMBuildCatchPad",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3748:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ParentPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Args",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumArgs",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3751:14
export const LLVMBuildCleanupPad = {
  "tag": "function",
  "name": "LLVMBuildCleanupPad",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3751:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ParentPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Args",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumArgs",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3754:14
export const LLVMBuildCatchSwitch = {
  "tag": "function",
  "name": "LLVMBuildCatchSwitch",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3754:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ParentPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "UnwindBB",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    },
    {
      "tag": "parameter",
      "name": "NumHandlers",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3759:6
export const LLVMAddCase = {
  "tag": "function",
  "name": "LLVMAddCase",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3759:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Switch",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OnVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dest",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3763:6
export const LLVMAddDestination = {
  "tag": "function",
  "name": "LLVMAddDestination",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3763:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "IndirectBr",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dest",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3766:10
export const LLVMGetNumClauses = {
  "tag": "function",
  "name": "LLVMGetNumClauses",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3766:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LandingPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3769:14
export const LLVMGetClause = {
  "tag": "function",
  "name": "LLVMGetClause",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3769:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LandingPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3772:6
export const LLVMAddClause = {
  "tag": "function",
  "name": "LLVMAddClause",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3772:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LandingPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ClauseVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3775:10
export const LLVMIsCleanup = {
  "tag": "function",
  "name": "LLVMIsCleanup",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3775:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LandingPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:3778:6
export const LLVMSetCleanup = {
  "tag": "function",
  "name": "LLVMSetCleanup",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3778:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "LandingPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3781:6
export const LLVMAddHandler = {
  "tag": "function",
  "name": "LLVMAddHandler",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3781:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CatchSwitch",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dest",
      "type": {
        "tag": "LLVMBasicBlockRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3784:10
export const LLVMGetNumHandlers = {
  "tag": "function",
  "name": "LLVMGetNumHandlers",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3784:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CatchSwitch",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:3797:6
export const LLVMGetHandlers = {
  "tag": "function",
  "name": "LLVMGetHandlers",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3797:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CatchSwitch",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Handlers",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMBasicBlockRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3802:14
export const LLVMGetArgOperand = {
  "tag": "function",
  "name": "LLVMGetArgOperand",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3802:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Funclet",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "i",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3805:6
export const LLVMSetArgOperand = {
  "tag": "function",
  "name": "LLVMSetArgOperand",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3805:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Funclet",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "i",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "value",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3814:14
export const LLVMGetParentCatchSwitch = {
  "tag": "function",
  "name": "LLVMGetParentCatchSwitch",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3814:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CatchPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3823:6
export const LLVMSetParentCatchSwitch = {
  "tag": "function",
  "name": "LLVMSetParentCatchSwitch",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3823:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CatchPad",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "CatchSwitch",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3826:14
export const LLVMBuildAdd = {
  "tag": "function",
  "name": "LLVMBuildAdd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3826:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3828:14
export const LLVMBuildNSWAdd = {
  "tag": "function",
  "name": "LLVMBuildNSWAdd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3828:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3830:14
export const LLVMBuildNUWAdd = {
  "tag": "function",
  "name": "LLVMBuildNUWAdd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3830:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3832:14
export const LLVMBuildFAdd = {
  "tag": "function",
  "name": "LLVMBuildFAdd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3832:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3834:14
export const LLVMBuildSub = {
  "tag": "function",
  "name": "LLVMBuildSub",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3834:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3836:14
export const LLVMBuildNSWSub = {
  "tag": "function",
  "name": "LLVMBuildNSWSub",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3836:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3838:14
export const LLVMBuildNUWSub = {
  "tag": "function",
  "name": "LLVMBuildNUWSub",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3838:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3840:14
export const LLVMBuildFSub = {
  "tag": "function",
  "name": "LLVMBuildFSub",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3840:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3842:14
export const LLVMBuildMul = {
  "tag": "function",
  "name": "LLVMBuildMul",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3842:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3844:14
export const LLVMBuildNSWMul = {
  "tag": "function",
  "name": "LLVMBuildNSWMul",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3844:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3846:14
export const LLVMBuildNUWMul = {
  "tag": "function",
  "name": "LLVMBuildNUWMul",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3846:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3848:14
export const LLVMBuildFMul = {
  "tag": "function",
  "name": "LLVMBuildFMul",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3848:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3850:14
export const LLVMBuildUDiv = {
  "tag": "function",
  "name": "LLVMBuildUDiv",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3850:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3852:14
export const LLVMBuildExactUDiv = {
  "tag": "function",
  "name": "LLVMBuildExactUDiv",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3852:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3854:14
export const LLVMBuildSDiv = {
  "tag": "function",
  "name": "LLVMBuildSDiv",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3854:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3856:14
export const LLVMBuildExactSDiv = {
  "tag": "function",
  "name": "LLVMBuildExactSDiv",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3856:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3858:14
export const LLVMBuildFDiv = {
  "tag": "function",
  "name": "LLVMBuildFDiv",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3858:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3860:14
export const LLVMBuildURem = {
  "tag": "function",
  "name": "LLVMBuildURem",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3860:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3862:14
export const LLVMBuildSRem = {
  "tag": "function",
  "name": "LLVMBuildSRem",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3862:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3864:14
export const LLVMBuildFRem = {
  "tag": "function",
  "name": "LLVMBuildFRem",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3864:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3866:14
export const LLVMBuildShl = {
  "tag": "function",
  "name": "LLVMBuildShl",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3866:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3868:14
export const LLVMBuildLShr = {
  "tag": "function",
  "name": "LLVMBuildLShr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3868:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3870:14
export const LLVMBuildAShr = {
  "tag": "function",
  "name": "LLVMBuildAShr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3870:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3872:14
export const LLVMBuildAnd = {
  "tag": "function",
  "name": "LLVMBuildAnd",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3872:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3874:14
export const LLVMBuildOr = {
  "tag": "function",
  "name": "LLVMBuildOr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3874:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3876:14
export const LLVMBuildXor = {
  "tag": "function",
  "name": "LLVMBuildXor",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3876:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3878:14
export const LLVMBuildBinOp = {
  "tag": "function",
  "name": "LLVMBuildBinOp",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3878:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Op",
      "type": {
        "tag": "LLVMOpcode"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3881:14
export const LLVMBuildNeg = {
  "tag": "function",
  "name": "LLVMBuildNeg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3881:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3882:14
export const LLVMBuildNSWNeg = {
  "tag": "function",
  "name": "LLVMBuildNSWNeg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3882:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3884:14
export const LLVMBuildNUWNeg = {
  "tag": "function",
  "name": "LLVMBuildNUWNeg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3884:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3886:14
export const LLVMBuildFNeg = {
  "tag": "function",
  "name": "LLVMBuildFNeg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3886:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3887:14
export const LLVMBuildNot = {
  "tag": "function",
  "name": "LLVMBuildNot",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3887:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3890:14
export const LLVMBuildMalloc = {
  "tag": "function",
  "name": "LLVMBuildMalloc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3890:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3891:14
export const LLVMBuildArrayMalloc = {
  "tag": "function",
  "name": "LLVMBuildArrayMalloc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3891:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3900:14
export const LLVMBuildMemSet = {
  "tag": "function",
  "name": "LLVMBuildMemSet",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3900:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ptr",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Len",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Align",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3908:14
export const LLVMBuildMemCpy = {
  "tag": "function",
  "name": "LLVMBuildMemCpy",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3908:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DstAlign",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Src",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SrcAlign",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Size",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3917:14
export const LLVMBuildMemMove = {
  "tag": "function",
  "name": "LLVMBuildMemMove",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3917:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Dst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DstAlign",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Src",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SrcAlign",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Size",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3922:14
export const LLVMBuildAlloca = {
  "tag": "function",
  "name": "LLVMBuildAlloca",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3922:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3923:14
export const LLVMBuildArrayAlloca = {
  "tag": "function",
  "name": "LLVMBuildArrayAlloca",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3923:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3925:14
export const LLVMBuildFree = {
  "tag": "function",
  "name": "LLVMBuildFree",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3925:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PointerVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3926:1 <Spelling=/data/./llvm-c/Core.h:3927:18>
export const LLVMBuildLoad = {
  "tag": "function",
  "name": "LLVMBuildLoad",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3926:1 <Spelling=/data/./llvm-c/Core.h:3927:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PointerVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3930:14
export const LLVMBuildLoad2 = {
  "tag": "function",
  "name": "LLVMBuildLoad2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3930:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "PointerVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3932:14
export const LLVMBuildStore = {
  "tag": "function",
  "name": "LLVMBuildStore",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3932:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ptr",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3933:1 <Spelling=/data/./llvm-c/Core.h:3934:18>
export const LLVMBuildGEP = {
  "tag": "function",
  "name": "LLVMBuildGEP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3933:1 <Spelling=/data/./llvm-c/Core.h:3934:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Pointer",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Indices",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumIndices",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3938:1 <Spelling=/data/./llvm-c/Core.h:3939:18>
export const LLVMBuildInBoundsGEP = {
  "tag": "function",
  "name": "LLVMBuildInBoundsGEP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3938:1 <Spelling=/data/./llvm-c/Core.h:3939:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Pointer",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Indices",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumIndices",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3943:1 <Spelling=/data/./llvm-c/Core.h:3944:18>
export const LLVMBuildStructGEP = {
  "tag": "function",
  "name": "LLVMBuildStructGEP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3943:1 <Spelling=/data/./llvm-c/Core.h:3944:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Pointer",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3947:14
export const LLVMBuildGEP2 = {
  "tag": "function",
  "name": "LLVMBuildGEP2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3947:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Pointer",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Indices",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumIndices",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3950:14
export const LLVMBuildInBoundsGEP2 = {
  "tag": "function",
  "name": "LLVMBuildInBoundsGEP2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3950:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Pointer",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Indices",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumIndices",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3953:14
export const LLVMBuildStructGEP2 = {
  "tag": "function",
  "name": "LLVMBuildStructGEP2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3953:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Pointer",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Idx",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3956:14
export const LLVMBuildGlobalString = {
  "tag": "function",
  "name": "LLVMBuildGlobalString",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3956:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Str",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3958:14
export const LLVMBuildGlobalStringPtr = {
  "tag": "function",
  "name": "LLVMBuildGlobalStringPtr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3958:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Str",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3960:10
export const LLVMGetVolatile = {
  "tag": "function",
  "name": "LLVMGetVolatile",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3960:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemoryAccessInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:3961:6
export const LLVMSetVolatile = {
  "tag": "function",
  "name": "LLVMSetVolatile",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3961:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemoryAccessInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IsVolatile",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3962:10
export const LLVMGetWeak = {
  "tag": "function",
  "name": "LLVMGetWeak",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3962:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CmpXchgInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:3963:6
export const LLVMSetWeak = {
  "tag": "function",
  "name": "LLVMSetWeak",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3963:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CmpXchgInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IsWeak",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3964:20
export const LLVMGetOrdering = {
  "tag": "function",
  "name": "LLVMGetOrdering",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3964:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemoryAccessInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAtomicOrdering"
  }
};

// ./llvm-c/Core.h:3965:6
export const LLVMSetOrdering = {
  "tag": "function",
  "name": "LLVMSetOrdering",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3965:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemoryAccessInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ordering",
      "type": {
        "tag": "LLVMAtomicOrdering"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3966:20
export const LLVMGetAtomicRMWBinOp = {
  "tag": "function",
  "name": "LLVMGetAtomicRMWBinOp",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3966:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "AtomicRMWInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAtomicRMWBinOp"
  }
};

// ./llvm-c/Core.h:3967:6
export const LLVMSetAtomicRMWBinOp = {
  "tag": "function",
  "name": "LLVMSetAtomicRMWBinOp",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3967:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "AtomicRMWInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "BinOp",
      "type": {
        "tag": "LLVMAtomicRMWBinOp"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:3970:14
export const LLVMBuildTrunc = {
  "tag": "function",
  "name": "LLVMBuildTrunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3970:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3972:14
export const LLVMBuildZExt = {
  "tag": "function",
  "name": "LLVMBuildZExt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3972:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3974:14
export const LLVMBuildSExt = {
  "tag": "function",
  "name": "LLVMBuildSExt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3974:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3976:14
export const LLVMBuildFPToUI = {
  "tag": "function",
  "name": "LLVMBuildFPToUI",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3976:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3978:14
export const LLVMBuildFPToSI = {
  "tag": "function",
  "name": "LLVMBuildFPToSI",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3978:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3980:14
export const LLVMBuildUIToFP = {
  "tag": "function",
  "name": "LLVMBuildUIToFP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3980:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3982:14
export const LLVMBuildSIToFP = {
  "tag": "function",
  "name": "LLVMBuildSIToFP",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3982:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3984:14
export const LLVMBuildFPTrunc = {
  "tag": "function",
  "name": "LLVMBuildFPTrunc",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3984:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3986:14
export const LLVMBuildFPExt = {
  "tag": "function",
  "name": "LLVMBuildFPExt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3986:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3988:14
export const LLVMBuildPtrToInt = {
  "tag": "function",
  "name": "LLVMBuildPtrToInt",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3988:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3990:14
export const LLVMBuildIntToPtr = {
  "tag": "function",
  "name": "LLVMBuildIntToPtr",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3990:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3992:14
export const LLVMBuildBitCast = {
  "tag": "function",
  "name": "LLVMBuildBitCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3992:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3994:14
export const LLVMBuildAddrSpaceCast = {
  "tag": "function",
  "name": "LLVMBuildAddrSpaceCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3994:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3996:14
export const LLVMBuildZExtOrBitCast = {
  "tag": "function",
  "name": "LLVMBuildZExtOrBitCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3996:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:3998:14
export const LLVMBuildSExtOrBitCast = {
  "tag": "function",
  "name": "LLVMBuildSExtOrBitCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:3998:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4000:14
export const LLVMBuildTruncOrBitCast = {
  "tag": "function",
  "name": "LLVMBuildTruncOrBitCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4000:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4002:14
export const LLVMBuildCast = {
  "tag": "function",
  "name": "LLVMBuildCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4002:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Op",
      "type": {
        "tag": "LLVMOpcode"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4004:14
export const LLVMBuildPointerCast = {
  "tag": "function",
  "name": "LLVMBuildPointerCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4004:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4006:14
export const LLVMBuildIntCast2 = {
  "tag": "function",
  "name": "LLVMBuildIntCast2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4006:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "IsSigned",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4009:14
export const LLVMBuildFPCast = {
  "tag": "function",
  "name": "LLVMBuildFPCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4009:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4013:14
export const LLVMBuildIntCast = {
  "tag": "function",
  "name": "LLVMBuildIntCast",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4013:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4016:12
export const LLVMGetCastOpcode = {
  "tag": "function",
  "name": "LLVMGetCastOpcode",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4016:12",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Src",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SrcIsSigned",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "DestTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "DestIsSigned",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOpcode"
  }
};

// ./llvm-c/Core.h:4020:14
export const LLVMBuildICmp = {
  "tag": "function",
  "name": "LLVMBuildICmp",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4020:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Op",
      "type": {
        "tag": "LLVMIntPredicate"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4023:14
export const LLVMBuildFCmp = {
  "tag": "function",
  "name": "LLVMBuildFCmp",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4023:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Op",
      "type": {
        "tag": "LLVMRealPredicate"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4028:14
export const LLVMBuildPhi = {
  "tag": "function",
  "name": "LLVMBuildPhi",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4028:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4029:1 <Spelling=/data/./llvm-c/Core.h:4030:18>
export const LLVMBuildCall = {
  "tag": "function",
  "name": "LLVMBuildCall",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4029:1 <Spelling=/data/./llvm-c/Core.h:4030:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Args",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumArgs",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4034:14
export const LLVMBuildCall2 = {
  "tag": "function",
  "name": "LLVMBuildCall2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4034:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Fn",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Args",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMValueRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "NumArgs",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4037:14
export const LLVMBuildSelect = {
  "tag": "function",
  "name": "LLVMBuildSelect",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4037:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "If",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Then",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Else",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4040:14
export const LLVMBuildVAArg = {
  "tag": "function",
  "name": "LLVMBuildVAArg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4040:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "List",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ty",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4042:14
export const LLVMBuildExtractElement = {
  "tag": "function",
  "name": "LLVMBuildExtractElement",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4042:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "VecVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4044:14
export const LLVMBuildInsertElement = {
  "tag": "function",
  "name": "LLVMBuildInsertElement",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4044:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "VecVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "EltVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4047:14
export const LLVMBuildShuffleVector = {
  "tag": "function",
  "name": "LLVMBuildShuffleVector",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4047:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "V1",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "V2",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Mask",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4050:14
export const LLVMBuildExtractValue = {
  "tag": "function",
  "name": "LLVMBuildExtractValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4050:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AggVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4052:14
export const LLVMBuildInsertValue = {
  "tag": "function",
  "name": "LLVMBuildInsertValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4052:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "AggVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "EltVal",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Index",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4055:14
export const LLVMBuildFreeze = {
  "tag": "function",
  "name": "LLVMBuildFreeze",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4055:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4058:14
export const LLVMBuildIsNull = {
  "tag": "function",
  "name": "LLVMBuildIsNull",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4058:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4060:14
export const LLVMBuildIsNotNull = {
  "tag": "function",
  "name": "LLVMBuildIsNotNull",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4060:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4062:1 <Spelling=/data/./llvm-c/Core.h:4063:18>
export const LLVMBuildPtrDiff = {
  "tag": "function",
  "name": "LLVMBuildPtrDiff",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4062:1 <Spelling=/data/./llvm-c/Core.h:4063:18>",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4066:14
export const LLVMBuildPtrDiff2 = {
  "tag": "function",
  "name": "LLVMBuildPtrDiff2",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4066:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ElemTy",
      "type": {
        "tag": "LLVMTypeRef"
      }
    },
    {
      "tag": "parameter",
      "name": "LHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RHS",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4069:14
export const LLVMBuildFence = {
  "tag": "function",
  "name": "LLVMBuildFence",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4069:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ordering",
      "type": {
        "tag": "LLVMAtomicOrdering"
      }
    },
    {
      "tag": "parameter",
      "name": "singleThread",
      "type": {
        "tag": "LLVMBool"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4071:14
export const LLVMBuildAtomicRMW = {
  "tag": "function",
  "name": "LLVMBuildAtomicRMW",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4071:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "op",
      "type": {
        "tag": "LLVMAtomicRMWBinOp"
      }
    },
    {
      "tag": "parameter",
      "name": "PTR",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Val",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ordering",
      "type": {
        "tag": "LLVMAtomicOrdering"
      }
    },
    {
      "tag": "parameter",
      "name": "singleThread",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4075:14
export const LLVMBuildAtomicCmpXchg = {
  "tag": "function",
  "name": "LLVMBuildAtomicCmpXchg",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4075:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "B",
      "type": {
        "tag": "LLVMBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ptr",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Cmp",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "New",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SuccessOrdering",
      "type": {
        "tag": "LLVMAtomicOrdering"
      }
    },
    {
      "tag": "parameter",
      "name": "FailureOrdering",
      "type": {
        "tag": "LLVMAtomicOrdering"
      }
    },
    {
      "tag": "parameter",
      "name": "SingleThread",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMValueRef"
  }
};

// ./llvm-c/Core.h:4084:10
export const LLVMGetNumMaskElements = {
  "tag": "function",
  "name": "LLVMGetNumMaskElements",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4084:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ShuffleVectorInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": ":unsigned-int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:4090:5
export const LLVMGetUndefMaskElem = {
  "tag": "function",
  "name": "LLVMGetUndefMaskElem",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4090:5",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:4099:5
export const LLVMGetMaskValue = {
  "tag": "function",
  "name": "LLVMGetMaskValue",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4099:5",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ShuffleVectorInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Elt",
      "type": {
        "tag": ":unsigned-int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/Core.h:4101:10
export const LLVMIsAtomicSingleThread = {
  "tag": "function",
  "name": "LLVMIsAtomicSingleThread",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4101:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "AtomicInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:4102:6
export const LLVMSetAtomicSingleThread = {
  "tag": "function",
  "name": "LLVMSetAtomicSingleThread",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4102:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "AtomicInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SingleThread",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:4104:20
export const LLVMGetCmpXchgSuccessOrdering = {
  "tag": "function",
  "name": "LLVMGetCmpXchgSuccessOrdering",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4104:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CmpXchgInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAtomicOrdering"
  }
};

// ./llvm-c/Core.h:4105:6
export const LLVMSetCmpXchgSuccessOrdering = {
  "tag": "function",
  "name": "LLVMSetCmpXchgSuccessOrdering",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4105:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CmpXchgInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ordering",
      "type": {
        "tag": "LLVMAtomicOrdering"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:4107:20
export const LLVMGetCmpXchgFailureOrdering = {
  "tag": "function",
  "name": "LLVMGetCmpXchgFailureOrdering",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4107:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CmpXchgInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMAtomicOrdering"
  }
};

// ./llvm-c/Core.h:4108:6
export const LLVMSetCmpXchgFailureOrdering = {
  "tag": "function",
  "name": "LLVMSetCmpXchgFailureOrdering",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4108:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "CmpXchgInst",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Ordering",
      "type": {
        "tag": "LLVMAtomicOrdering"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:4126:1
export const LLVMCreateModuleProviderForExistingModule = {
  "tag": "function",
  "name": "LLVMCreateModuleProviderForExistingModule",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4126:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMModuleProviderRef"
  }
};

// ./llvm-c/Core.h:4131:6
export const LLVMDisposeModuleProvider = {
  "tag": "function",
  "name": "LLVMDisposeModuleProvider",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4131:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleProviderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:4143:10
export const LLVMCreateMemoryBufferWithContentsOfFile = {
  "tag": "function",
  "name": "LLVMCreateMemoryBufferWithContentsOfFile",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4143:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Path",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutMemBuf",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMemoryBufferRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:4146:10
export const LLVMCreateMemoryBufferWithSTDIN = {
  "tag": "function",
  "name": "LLVMCreateMemoryBufferWithSTDIN",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4146:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "OutMemBuf",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMMemoryBufferRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:4148:21
export const LLVMCreateMemoryBufferWithMemoryRange = {
  "tag": "function",
  "name": "LLVMCreateMemoryBufferWithMemoryRange",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4148:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "InputData",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "InputDataLength",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "BufferName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "RequiresNullTerminator",
      "type": {
        "tag": "LLVMBool"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMemoryBufferRef"
  }
};

// ./llvm-c/Core.h:4152:21
export const LLVMCreateMemoryBufferWithMemoryRangeCopy = {
  "tag": "function",
  "name": "LLVMCreateMemoryBufferWithMemoryRangeCopy",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4152:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "InputData",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "InputDataLength",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "BufferName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMemoryBufferRef"
  }
};

// ./llvm-c/Core.h:4155:13
export const LLVMGetBufferStart = {
  "tag": "function",
  "name": "LLVMGetBufferStart",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4155:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Core.h:4156:8
export const LLVMGetBufferSize = {
  "tag": "function",
  "name": "LLVMGetBufferSize",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4156:8",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    }
  ],
  "return-type": {
    "tag": "size_t"
  }
};

// ./llvm-c/Core.h:4157:6
export const LLVMDisposeMemoryBuffer = {
  "tag": "function",
  "name": "LLVMDisposeMemoryBuffer",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4157:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:4172:21
export const LLVMGetGlobalPassRegistry = {
  "tag": "function",
  "name": "LLVMGetGlobalPassRegistry",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4172:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMPassRegistryRef"
  }
};

// ./llvm-c/Core.h:4188:20
export const LLVMCreatePassManager = {
  "tag": "function",
  "name": "LLVMCreatePassManager",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4188:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMPassManagerRef"
  }
};

// ./llvm-c/Core.h:4194:20
export const LLVMCreateFunctionPassManagerForModule = {
  "tag": "function",
  "name": "LLVMCreateFunctionPassManagerForModule",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4194:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMPassManagerRef"
  }
};

// ./llvm-c/Core.h:4197:20
export const LLVMCreateFunctionPassManager = {
  "tag": "function",
  "name": "LLVMCreateFunctionPassManager",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4197:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MP",
      "type": {
        "tag": "LLVMModuleProviderRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMPassManagerRef"
  }
};

// ./llvm-c/Core.h:4203:10
export const LLVMRunPassManager = {
  "tag": "function",
  "name": "LLVMRunPassManager",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4203:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:4208:10
export const LLVMInitializeFunctionPassManager = {
  "tag": "function",
  "name": "LLVMInitializeFunctionPassManager",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4208:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "FPM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:4214:10
export const LLVMRunFunctionPassManager = {
  "tag": "function",
  "name": "LLVMRunFunctionPassManager",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4214:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "FPM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:4219:10
export const LLVMFinalizeFunctionPassManager = {
  "tag": "function",
  "name": "LLVMFinalizeFunctionPassManager",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4219:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "FPM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:4224:6
export const LLVMDisposePassManager = {
  "tag": "function",
  "name": "LLVMDisposePassManager",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4224:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "PM",
      "type": {
        "tag": "LLVMPassManagerRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:4241:10
export const LLVMStartMultithreaded = {
  "tag": "function",
  "name": "LLVMStartMultithreaded",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4241:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Core.h:4245:6
export const LLVMStopMultithreaded = {
  "tag": "function",
  "name": "LLVMStopMultithreaded",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4245:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Core.h:4249:10
export const LLVMIsMultithreaded = {
  "tag": "function",
  "name": "LLVMIsMultithreaded",
  "ns": 0,
  "location": "/data/./llvm-c/Core.h:4249:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Object.h:76:15
export const LLVMCreateBinary = {
  "tag": "function",
  "name": "LLVMCreateBinary",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:76:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Context",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ErrorMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBinaryRef"
  }
};

// ./llvm-c/Object.h:86:6
export const LLVMDisposeBinary = {
  "tag": "function",
  "name": "LLVMDisposeBinary",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:86:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BR",
      "type": {
        "tag": "LLVMBinaryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Object.h:97:21
export const LLVMBinaryCopyMemoryBuffer = {
  "tag": "function",
  "name": "LLVMBinaryCopyMemoryBuffer",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:97:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BR",
      "type": {
        "tag": "LLVMBinaryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMemoryBufferRef"
  }
};

// ./llvm-c/Object.h:104:16
export const LLVMBinaryGetType = {
  "tag": "function",
  "name": "LLVMBinaryGetType",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:104:16",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BR",
      "type": {
        "tag": "LLVMBinaryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBinaryType"
  }
};

// ./llvm-c/Object.h:117:15
export const LLVMMachOUniversalBinaryCopyObjectForArch = {
  "tag": "function",
  "name": "LLVMMachOUniversalBinaryCopyObjectForArch",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:117:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BR",
      "type": {
        "tag": "LLVMBinaryRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Arch",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "ArchLen",
      "type": {
        "tag": "size_t"
      }
    },
    {
      "tag": "parameter",
      "name": "ErrorMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBinaryRef"
  }
};

// ./llvm-c/Object.h:133:24
export const LLVMObjectFileCopySectionIterator = {
  "tag": "function",
  "name": "LLVMObjectFileCopySectionIterator",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:133:24",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BR",
      "type": {
        "tag": "LLVMBinaryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMSectionIteratorRef"
  }
};

// ./llvm-c/Object.h:140:10
export const LLVMObjectFileIsSectionIteratorAtEnd = {
  "tag": "function",
  "name": "LLVMObjectFileIsSectionIteratorAtEnd",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:140:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BR",
      "type": {
        "tag": "LLVMBinaryRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Object.h:154:23
export const LLVMObjectFileCopySymbolIterator = {
  "tag": "function",
  "name": "LLVMObjectFileCopySymbolIterator",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:154:23",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BR",
      "type": {
        "tag": "LLVMBinaryRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMSymbolIteratorRef"
  }
};

// ./llvm-c/Object.h:161:10
export const LLVMObjectFileIsSymbolIteratorAtEnd = {
  "tag": "function",
  "name": "LLVMObjectFileIsSymbolIteratorAtEnd",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:161:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "BR",
      "type": {
        "tag": "LLVMBinaryRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSymbolIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Object.h:164:6
export const LLVMDisposeSectionIterator = {
  "tag": "function",
  "name": "LLVMDisposeSectionIterator",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:164:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Object.h:166:6
export const LLVMMoveToNextSection = {
  "tag": "function",
  "name": "LLVMMoveToNextSection",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:166:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Object.h:167:6
export const LLVMMoveToContainingSection = {
  "tag": "function",
  "name": "LLVMMoveToContainingSection",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:167:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Sect",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Sym",
      "type": {
        "tag": "LLVMSymbolIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Object.h:171:6
export const LLVMDisposeSymbolIterator = {
  "tag": "function",
  "name": "LLVMDisposeSymbolIterator",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:171:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSymbolIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Object.h:172:6
export const LLVMMoveToNextSymbol = {
  "tag": "function",
  "name": "LLVMMoveToNextSymbol",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:172:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSymbolIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Object.h:175:13
export const LLVMGetSectionName = {
  "tag": "function",
  "name": "LLVMGetSectionName",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:175:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Object.h:176:10
export const LLVMGetSectionSize = {
  "tag": "function",
  "name": "LLVMGetSectionSize",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:176:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/Object.h:177:13
export const LLVMGetSectionContents = {
  "tag": "function",
  "name": "LLVMGetSectionContents",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:177:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Object.h:178:10
export const LLVMGetSectionAddress = {
  "tag": "function",
  "name": "LLVMGetSectionAddress",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:178:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/Object.h:179:10
export const LLVMGetSectionContainsSymbol = {
  "tag": "function",
  "name": "LLVMGetSectionContainsSymbol",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:179:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Sym",
      "type": {
        "tag": "LLVMSymbolIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Object.h:183:27
export const LLVMGetRelocations = {
  "tag": "function",
  "name": "LLVMGetRelocations",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:183:27",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Section",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMRelocationIteratorRef"
  }
};

// ./llvm-c/Object.h:184:6
export const LLVMDisposeRelocationIterator = {
  "tag": "function",
  "name": "LLVMDisposeRelocationIterator",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:184:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RI",
      "type": {
        "tag": "LLVMRelocationIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Object.h:185:10
export const LLVMIsRelocationIteratorAtEnd = {
  "tag": "function",
  "name": "LLVMIsRelocationIteratorAtEnd",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:185:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Section",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RI",
      "type": {
        "tag": "LLVMRelocationIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Object.h:187:6
export const LLVMMoveToNextRelocation = {
  "tag": "function",
  "name": "LLVMMoveToNextRelocation",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:187:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RI",
      "type": {
        "tag": "LLVMRelocationIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Object.h:191:13
export const LLVMGetSymbolName = {
  "tag": "function",
  "name": "LLVMGetSymbolName",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:191:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSymbolIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Object.h:192:10
export const LLVMGetSymbolAddress = {
  "tag": "function",
  "name": "LLVMGetSymbolAddress",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:192:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSymbolIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/Object.h:193:10
export const LLVMGetSymbolSize = {
  "tag": "function",
  "name": "LLVMGetSymbolSize",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:193:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSymbolIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/Object.h:196:10
export const LLVMGetRelocationOffset = {
  "tag": "function",
  "name": "LLVMGetRelocationOffset",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:196:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RI",
      "type": {
        "tag": "LLVMRelocationIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/Object.h:197:23
export const LLVMGetRelocationSymbol = {
  "tag": "function",
  "name": "LLVMGetRelocationSymbol",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:197:23",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RI",
      "type": {
        "tag": "LLVMRelocationIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMSymbolIteratorRef"
  }
};

// ./llvm-c/Object.h:198:10
export const LLVMGetRelocationType = {
  "tag": "function",
  "name": "LLVMGetRelocationType",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:198:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RI",
      "type": {
        "tag": "LLVMRelocationIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "uint64_t"
  }
};

// ./llvm-c/Object.h:201:13
export const LLVMGetRelocationTypeName = {
  "tag": "function",
  "name": "LLVMGetRelocationTypeName",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:201:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RI",
      "type": {
        "tag": "LLVMRelocationIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Object.h:202:13
export const LLVMGetRelocationValueString = {
  "tag": "function",
  "name": "LLVMGetRelocationValueString",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:202:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "RI",
      "type": {
        "tag": "LLVMRelocationIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Object.h:208:19
export const LLVMCreateObjectFile = {
  "tag": "function",
  "name": "LLVMCreateObjectFile",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:208:19",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMObjectFileRef"
  }
};

// ./llvm-c/Object.h:211:6
export const LLVMDisposeObjectFile = {
  "tag": "function",
  "name": "LLVMDisposeObjectFile",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:211:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjectFile",
      "type": {
        "tag": "LLVMObjectFileRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Object.h:214:24
export const LLVMGetSections = {
  "tag": "function",
  "name": "LLVMGetSections",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:214:24",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjectFile",
      "type": {
        "tag": "LLVMObjectFileRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMSectionIteratorRef"
  }
};

// ./llvm-c/Object.h:217:10
export const LLVMIsSectionIteratorAtEnd = {
  "tag": "function",
  "name": "LLVMIsSectionIteratorAtEnd",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:217:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjectFile",
      "type": {
        "tag": "LLVMObjectFileRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSectionIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Object.h:221:23
export const LLVMGetSymbols = {
  "tag": "function",
  "name": "LLVMGetSymbols",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:221:23",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjectFile",
      "type": {
        "tag": "LLVMObjectFileRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMSymbolIteratorRef"
  }
};

// ./llvm-c/Object.h:224:10
export const LLVMIsSymbolIteratorAtEnd = {
  "tag": "function",
  "name": "LLVMIsSymbolIteratorAtEnd",
  "ns": 0,
  "location": "/data/./llvm-c/Object.h:224:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ObjectFile",
      "type": {
        "tag": "LLVMObjectFileRef"
      }
    },
    {
      "tag": "parameter",
      "name": "SI",
      "type": {
        "tag": "LLVMSymbolIteratorRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/BitWriter.h:37:5
export const LLVMWriteBitcodeToFile = {
  "tag": "function",
  "name": "LLVMWriteBitcodeToFile",
  "ns": 0,
  "location": "/data/./llvm-c/BitWriter.h:37:5",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Path",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/BitWriter.h:40:5
export const LLVMWriteBitcodeToFD = {
  "tag": "function",
  "name": "LLVMWriteBitcodeToFD",
  "ns": 0,
  "location": "/data/./llvm-c/BitWriter.h:40:5",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "FD",
      "type": {
        "tag": ":int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "ShouldClose",
      "type": {
        "tag": ":int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "Unbuffered",
      "type": {
        "tag": ":int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/BitWriter.h:45:5
export const LLVMWriteBitcodeToFileHandle = {
  "tag": "function",
  "name": "LLVMWriteBitcodeToFileHandle",
  "ns": 0,
  "location": "/data/./llvm-c/BitWriter.h:45:5",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Handle",
      "type": {
        "tag": ":int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    }
  ],
  "return-type": {
    "tag": ":int",
    "bit-size": 32,
    "bit-alignment": 32
  }
};

// ./llvm-c/BitWriter.h:48:21
export const LLVMWriteBitcodeToMemoryBuffer = {
  "tag": "function",
  "name": "LLVMWriteBitcodeToMemoryBuffer",
  "ns": 0,
  "location": "/data/./llvm-c/BitWriter.h:48:21",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMMemoryBufferRef"
  }
};

// ./llvm-c/LLJIT.h:74:24
export const LLVMOrcCreateLLJITBuilder = {
  "tag": "function",
  "name": "LLVMOrcCreateLLJITBuilder",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:74:24",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [],
  "return-type": {
    "tag": "LLVMOrcLLJITBuilderRef"
  }
};

// ./llvm-c/LLJIT.h:81:6
export const LLVMOrcDisposeLLJITBuilder = {
  "tag": "function",
  "name": "LLVMOrcDisposeLLJITBuilder",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:81:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMOrcLLJITBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/LLJIT.h:92:6
export const LLVMOrcLLJITBuilderSetJITTargetMachineBuilder = {
  "tag": "function",
  "name": "LLVMOrcLLJITBuilderSetJITTargetMachineBuilder",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:92:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMOrcLLJITBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "JTMB",
      "type": {
        "tag": "LLVMOrcJITTargetMachineBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/LLJIT.h:98:6
export const LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator = {
  "tag": "function",
  "name": "LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:98:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMOrcLLJITBuilderRef"
      }
    },
    {
      "tag": "parameter",
      "name": "F",
      "type": {
        "tag": "LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction"
      }
    },
    {
      "tag": "parameter",
      "name": "Ctx",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/LLJIT.h:116:14
export const LLVMOrcCreateLLJIT = {
  "tag": "function",
  "name": "LLVMOrcCreateLLJIT",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:116:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Result",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcLLJITRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Builder",
      "type": {
        "tag": "LLVMOrcLLJITBuilderRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/LLJIT.h:122:14
export const LLVMOrcDisposeLLJIT = {
  "tag": "function",
  "name": "LLVMOrcDisposeLLJIT",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:122:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/LLJIT.h:130:28
export const LLVMOrcLLJITGetExecutionSession = {
  "tag": "function",
  "name": "LLVMOrcLLJITGetExecutionSession",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:130:28",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcExecutionSessionRef"
  }
};

// ./llvm-c/LLJIT.h:138:20
export const LLVMOrcLLJITGetMainJITDylib = {
  "tag": "function",
  "name": "LLVMOrcLLJITGetMainJITDylib",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:138:20",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcJITDylibRef"
  }
};

// ./llvm-c/LLJIT.h:144:13
export const LLVMOrcLLJITGetTripleString = {
  "tag": "function",
  "name": "LLVMOrcLLJITGetTripleString",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:144:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/LLJIT.h:149:6
export const LLVMOrcLLJITGetGlobalPrefix = {
  "tag": "function",
  "name": "LLVMOrcLLJITGetGlobalPrefix",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:149:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    }
  ],
  "return-type": {
    "tag": ":char",
    "bit-size": 8,
    "bit-alignment": 8
  }
};

// ./llvm-c/LLJIT.h:159:1
export const LLVMOrcLLJITMangleAndIntern = {
  "tag": "function",
  "name": "LLVMOrcLLJITMangleAndIntern",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:159:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    },
    {
      "tag": "parameter",
      "name": "UnmangledName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcSymbolStringPoolEntryRef"
  }
};

// ./llvm-c/LLJIT.h:170:14
export const LLVMOrcLLJITAddObjectFile = {
  "tag": "function",
  "name": "LLVMOrcLLJITAddObjectFile",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:170:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    },
    {
      "tag": "parameter",
      "name": "JD",
      "type": {
        "tag": "LLVMOrcJITDylibRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ObjBuffer",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/LLJIT.h:182:14
export const LLVMOrcLLJITAddObjectFileWithRT = {
  "tag": "function",
  "name": "LLVMOrcLLJITAddObjectFileWithRT",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:182:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    },
    {
      "tag": "parameter",
      "name": "RT",
      "type": {
        "tag": "LLVMOrcResourceTrackerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "ObjBuffer",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/LLJIT.h:195:14
export const LLVMOrcLLJITAddLLVMIRModule = {
  "tag": "function",
  "name": "LLVMOrcLLJITAddLLVMIRModule",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:195:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    },
    {
      "tag": "parameter",
      "name": "JD",
      "type": {
        "tag": "LLVMOrcJITDylibRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TSM",
      "type": {
        "tag": "LLVMOrcThreadSafeModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/LLJIT.h:208:14
export const LLVMOrcLLJITAddLLVMIRModuleWithRT = {
  "tag": "function",
  "name": "LLVMOrcLLJITAddLLVMIRModuleWithRT",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:208:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    },
    {
      "tag": "parameter",
      "name": "JD",
      "type": {
        "tag": "LLVMOrcResourceTrackerRef"
      }
    },
    {
      "tag": "parameter",
      "name": "TSM",
      "type": {
        "tag": "LLVMOrcThreadSafeModuleRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/LLJIT.h:217:14
export const LLVMOrcLLJITLookup = {
  "tag": "function",
  "name": "LLVMOrcLLJITLookup",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:217:14",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Result",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMOrcExecutorAddress"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMErrorRef"
  }
};

// ./llvm-c/LLJIT.h:224:23
export const LLVMOrcLLJITGetObjLinkingLayer = {
  "tag": "function",
  "name": "LLVMOrcLLJITGetObjLinkingLayer",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:224:23",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcObjectLayerRef"
  }
};

// ./llvm-c/LLJIT.h:230:1
export const LLVMOrcLLJITGetObjTransformLayer = {
  "tag": "function",
  "name": "LLVMOrcLLJITGetObjTransformLayer",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:230:1",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcObjectTransformLayerRef"
  }
};

// ./llvm-c/LLJIT.h:235:28
export const LLVMOrcLLJITGetIRTransformLayer = {
  "tag": "function",
  "name": "LLVMOrcLLJITGetIRTransformLayer",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:235:28",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMOrcIRTransformLayerRef"
  }
};

// ./llvm-c/LLJIT.h:243:13
export const LLVMOrcLLJITGetDataLayoutStr = {
  "tag": "function",
  "name": "LLVMOrcLLJITGetDataLayoutStr",
  "ns": 0,
  "location": "/data/./llvm-c/LLJIT.h:243:13",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "J",
      "type": {
        "tag": "LLVMOrcLLJITRef"
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":char",
      "bit-size": 8,
      "bit-alignment": 8
    }
  }
};

// ./llvm-c/Support.h:35:10
export const LLVMLoadLibraryPermanently = {
  "tag": "function",
  "name": "LLVMLoadLibraryPermanently",
  "ns": 0,
  "location": "/data/./llvm-c/Support.h:35:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "Filename",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Support.h:45:6
export const LLVMParseCommandLineOptions = {
  "tag": "function",
  "name": "LLVMParseCommandLineOptions",
  "ns": 0,
  "location": "/data/./llvm-c/Support.h:45:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "argc",
      "type": {
        "tag": ":int",
        "bit-size": 32,
        "bit-alignment": 32
      }
    },
    {
      "tag": "parameter",
      "name": "argv",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    },
    {
      "tag": "parameter",
      "name": "Overview",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Support.h:55:7
export const LLVMSearchForAddressOfSymbol = {
  "tag": "function",
  "name": "LLVMSearchForAddressOfSymbol",
  "ns": 0,
  "location": "/data/./llvm-c/Support.h:55:7",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "symbolName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": ":pointer",
    "type": {
      "tag": ":void"
    }
  }
};

// ./llvm-c/Support.h:64:6
export const LLVMAddSymbol = {
  "tag": "function",
  "name": "LLVMAddSymbol",
  "ns": 0,
  "location": "/data/./llvm-c/Support.h:64:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "symbolName",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    },
    {
      "tag": "parameter",
      "name": "symbolValue",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":void"
        }
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:34:6
export const LLVMInitializeTransformUtils = {
  "tag": "function",
  "name": "LLVMInitializeTransformUtils",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:34:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:35:6
export const LLVMInitializeScalarOpts = {
  "tag": "function",
  "name": "LLVMInitializeScalarOpts",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:35:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:36:6
export const LLVMInitializeObjCARCOpts = {
  "tag": "function",
  "name": "LLVMInitializeObjCARCOpts",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:36:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:37:6
export const LLVMInitializeVectorization = {
  "tag": "function",
  "name": "LLVMInitializeVectorization",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:37:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:38:6
export const LLVMInitializeInstCombine = {
  "tag": "function",
  "name": "LLVMInitializeInstCombine",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:38:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:39:6
export const LLVMInitializeAggressiveInstCombiner = {
  "tag": "function",
  "name": "LLVMInitializeAggressiveInstCombiner",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:39:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:40:6
export const LLVMInitializeIPO = {
  "tag": "function",
  "name": "LLVMInitializeIPO",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:40:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:41:6
export const LLVMInitializeInstrumentation = {
  "tag": "function",
  "name": "LLVMInitializeInstrumentation",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:41:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:42:6
export const LLVMInitializeAnalysis = {
  "tag": "function",
  "name": "LLVMInitializeAnalysis",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:42:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:43:6
export const LLVMInitializeIPA = {
  "tag": "function",
  "name": "LLVMInitializeIPA",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:43:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:44:6
export const LLVMInitializeCodeGen = {
  "tag": "function",
  "name": "LLVMInitializeCodeGen",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:44:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Initialization.h:45:6
export const LLVMInitializeTarget = {
  "tag": "function",
  "name": "LLVMInitializeTarget",
  "ns": 0,
  "location": "/data/./llvm-c/Initialization.h:45:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "R",
      "type": {
        "tag": "LLVMPassRegistryRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/BitReader.h:39:10
export const LLVMParseBitcode = {
  "tag": "function",
  "name": "LLVMParseBitcode",
  "ns": 0,
  "location": "/data/./llvm-c/BitReader.h:39:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutModule",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/BitReader.h:44:10
export const LLVMParseBitcode2 = {
  "tag": "function",
  "name": "LLVMParseBitcode2",
  "ns": 0,
  "location": "/data/./llvm-c/BitReader.h:44:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutModule",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/BitReader.h:48:10
export const LLVMParseBitcodeInContext = {
  "tag": "function",
  "name": "LLVMParseBitcodeInContext",
  "ns": 0,
  "location": "/data/./llvm-c/BitReader.h:48:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ContextRef",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutModule",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/BitReader.h:52:10
export const LLVMParseBitcodeInContext2 = {
  "tag": "function",
  "name": "LLVMParseBitcodeInContext2",
  "ns": 0,
  "location": "/data/./llvm-c/BitReader.h:52:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ContextRef",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutModule",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/BitReader.h:60:10
export const LLVMGetBitcodeModuleInContext = {
  "tag": "function",
  "name": "LLVMGetBitcodeModuleInContext",
  "ns": 0,
  "location": "/data/./llvm-c/BitReader.h:60:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ContextRef",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutM",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/BitReader.h:71:10
export const LLVMGetBitcodeModuleInContext2 = {
  "tag": "function",
  "name": "LLVMGetBitcodeModuleInContext2",
  "ns": 0,
  "location": "/data/./llvm-c/BitReader.h:71:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ContextRef",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutM",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/BitReader.h:76:10
export const LLVMGetBitcodeModule = {
  "tag": "function",
  "name": "LLVMGetBitcodeModule",
  "ns": 0,
  "location": "/data/./llvm-c/BitReader.h:76:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutM",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/BitReader.h:79:10
export const LLVMGetBitcodeModule2 = {
  "tag": "function",
  "name": "LLVMGetBitcodeModule2",
  "ns": 0,
  "location": "/data/./llvm-c/BitReader.h:79:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutM",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};

// ./llvm-c/Comdat.h:46:15
export const LLVMGetOrInsertComdat = {
  "tag": "function",
  "name": "LLVMGetOrInsertComdat",
  "ns": 0,
  "location": "/data/./llvm-c/Comdat.h:46:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "M",
      "type": {
        "tag": "LLVMModuleRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Name",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":char",
          "bit-size": 8,
          "bit-alignment": 8
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMComdatRef"
  }
};

// ./llvm-c/Comdat.h:53:15
export const LLVMGetComdat = {
  "tag": "function",
  "name": "LLVMGetComdat",
  "ns": 0,
  "location": "/data/./llvm-c/Comdat.h:53:15",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMComdatRef"
  }
};

// ./llvm-c/Comdat.h:60:6
export const LLVMSetComdat = {
  "tag": "function",
  "name": "LLVMSetComdat",
  "ns": 0,
  "location": "/data/./llvm-c/Comdat.h:60:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "V",
      "type": {
        "tag": "LLVMValueRef"
      }
    },
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMComdatRef"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/Comdat.h:67:25
export const LLVMGetComdatSelectionKind = {
  "tag": "function",
  "name": "LLVMGetComdatSelectionKind",
  "ns": 0,
  "location": "/data/./llvm-c/Comdat.h:67:25",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMComdatRef"
      }
    }
  ],
  "return-type": {
    "tag": "LLVMComdatSelectionKind"
  }
};

// ./llvm-c/Comdat.h:74:6
export const LLVMSetComdatSelectionKind = {
  "tag": "function",
  "name": "LLVMSetComdatSelectionKind",
  "ns": 0,
  "location": "/data/./llvm-c/Comdat.h:74:6",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "C",
      "type": {
        "tag": "LLVMComdatRef"
      }
    },
    {
      "tag": "parameter",
      "name": "Kind",
      "type": {
        "tag": "LLVMComdatSelectionKind"
      }
    }
  ],
  "return-type": {
    "tag": ":void"
  }
};

// ./llvm-c/IRReader.h:38:10
export const LLVMParseIRInContext = {
  "tag": "function",
  "name": "LLVMParseIRInContext",
  "ns": 0,
  "location": "/data/./llvm-c/IRReader.h:38:10",
  "variadic": false,
  "inline": false,
  "storage-class": "none",
  "parameters": [
    {
      "tag": "parameter",
      "name": "ContextRef",
      "type": {
        "tag": "LLVMContextRef"
      }
    },
    {
      "tag": "parameter",
      "name": "MemBuf",
      "type": {
        "tag": "LLVMMemoryBufferRef"
      }
    },
    {
      "tag": "parameter",
      "name": "OutM",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": "LLVMModuleRef"
        }
      }
    },
    {
      "tag": "parameter",
      "name": "OutMessage",
      "type": {
        "tag": ":pointer",
        "type": {
          "tag": ":pointer",
          "type": {
            "tag": ":char",
            "bit-size": 8,
            "bit-alignment": 8
          }
        }
      }
    }
  ],
  "return-type": {
    "tag": "LLVMBool"
  }
};
