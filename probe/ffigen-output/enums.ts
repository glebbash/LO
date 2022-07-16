// ./llvm-c/Target.h:35:6
export enum LLVMByteOrdering {
  LLVMBigEndian = 0,
  LLVMLittleEndian = 1
}

// ./llvm-c/Remarks.h:41:6
export enum LLVMRemarkType {
  LLVMRemarkTypeUnknown = 0,
  LLVMRemarkTypePassed = 1,
  LLVMRemarkTypeMissed = 2,
  LLVMRemarkTypeAnalysis = 3,
  LLVMRemarkTypeAnalysisFPCommute = 4,
  LLVMRemarkTypeAnalysisAliasing = 5,
  LLVMRemarkTypeFailure = 6
}
