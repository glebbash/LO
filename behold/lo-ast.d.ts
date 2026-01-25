// NOTE: this was AI generated to quickly test the idea for the structural editor.
//   If this is to be kept it has to be reviewed and possibly rewritten.

/** TypeScript types for the LO AST (Abstract Syntax Tree) */

/** Root AST file */
export type AstFile = {
    ast: TopLevelExpr[];
};

/** Top-level expressions in a LO file */
export type TopLevelExpr =
    | { kind: "FnDef"; exported: boolean; decl: FnDecl; body: CodeBlock }
    | {
          kind: "Include";
          file_path: string;
          alias: string | null;
          with_extern: boolean;
      }
    | { kind: "Import"; module_name: string; items: ImportItem[] }
    | { kind: "GlobalDef"; global_name: string; global_value: CodeExpr }
    | { kind: "StructDef"; name: string; fields: StructDefField[] }
    | {
          kind: "EnumDef";
          name: string;
          variant_type: TypeExpr | null;
          variants: EnumDefVariant[];
      }
    | { kind: "TypeDef"; name: string; type: TypeExpr }
    | { kind: "ConstDef"; name: string; value: CodeExpr }
    | { kind: "MemoryDef"; memory: MemoryDef }
    | {
          kind: "TryExport";
          in_name: string;
          out_name: string;
          from_root: boolean;
      }
    | {
          kind: "MacroDef";
          name: string;
          type_params: string[];
          params: FnParam[];
          return_type: TypeExpr | null;
          body: CodeBlock;
      };

/** Function declaration */
export type FnDecl = {
    name: string;
    params: FnParam[];
    return_type: TypeExpr | null;
};

/** Function parameter */
export type FnParam = {
    name: string;
    type:
        | "self"
        | "&self"
        | { kind: "Type"; type: TypeExpr }
        | { kind: "Infer"; name: string };
};

/** Import items */
export type ImportItem =
    | { kind: "FnDecl"; decl: FnDecl }
    | { kind: "Memory"; memory: MemoryDef };

/** Struct field definition */
export type StructDefField = {
    name: string;
    type: TypeExpr;
};

/** Enum variant definition */
export type EnumDefVariant = {
    name: string;
    type: TypeExpr | null;
};

/** Memory definition */
export type MemoryDef = {
    exported: boolean;
    params: CodeExprMap;
};

/** Type expressions */
export type TypeExpr =
    | { kind: "Named"; name: string }
    | { kind: "Pointer"; pointee: TypeExpr }
    | { kind: "SequencePointer"; pointee: TypeExpr }
    | { kind: "Container"; container: TypeExpr; items: TypeExpr[] };

/** Code block (scope with multiple expressions) */
export type CodeBlock = {
    expressions: CodeExpr[];
};

/** Code expressions */
export type CodeExpr =
    // Literals
    | { kind: "BoolLiteral"; value: boolean }
    | { kind: "CharLiteral"; value: string }
    | { kind: "NullLiteral" }
    | { kind: "IntLiteral"; value: string; tag: string | null }
    | { kind: "StringLiteral"; value: string }
    | { kind: "ArrayLiteral"; item_type: TypeExpr; items: CodeExpr[] }
    | {
          kind: "ResultLiteral";
          is_ok: boolean;
          result_type: ResultType | null;
          value: CodeExpr | null;
      }
    // Variables
    | { kind: "Ident"; value: string }
    | { kind: "Let"; local_name: string; value: CodeExpr }
    // Operations
    | { kind: "InfixOp"; op: string; lhs: CodeExpr; rhs: CodeExpr }
    | { kind: "ExprPipe"; op: string; lhs: CodeExpr; rhs: CodeExpr }
    | { kind: "PrefixOp"; op: string; operand: CodeExpr }
    | { kind: "Cast"; expr: CodeExpr; casted_to: TypeExpr }
    | { kind: "Assign"; lhs: CodeExpr; rhs: CodeExpr }
    | { kind: "FieldAccess"; lhs: CodeExpr; field_name: string }
    | { kind: "PropagateError"; expr: CodeExpr }
    | { kind: "Sizeof"; type: TypeExpr }
    // Calls
    | { kind: "FnCall"; fn_name: string; args: CodeExprList }
    | {
          kind: "MethodCall";
          lhs: CodeExpr;
          method_name: string;
          args: CodeExprList;
      }
    | {
          kind: "MacroFnCall";
          fn_name: string;
          type_args: TypeExpr[];
          args: CodeExprList;
      }
    | {
          kind: "IntrinsicCall";
          fn_name: string;
          type_args: TypeExpr[];
          args: CodeExprList;
      }
    | {
          kind: "MacroMethodCall";
          lhs: CodeExpr;
          method_name: string;
          type_args: TypeExpr[];
          args: CodeExprList;
      }
    // Control flow
    | { kind: "Return"; value: CodeExpr | null }
    | {
          kind: "If";
          condition: CodeExpr | MatchHeader;
          then_block: CodeBlock;
          else_block: CodeBlock | CodeExpr | null;
      }
    | { kind: "Match"; header: MatchHeader; else_branch: CodeBlock }
    | { kind: "WhileLoop"; condition: CodeExpr | null; body: CodeBlock }
    | {
          kind: "ForLoop";
          counter: string;
          start: CodeExpr;
          end: CodeExpr;
          body: CodeBlock;
      }
    | { kind: "Break" }
    | { kind: "Continue" }
    | { kind: "Defer"; expr: CodeExpr }
    | {
          kind: "Catch";
          lhs: CodeExpr;
          error_bind: string;
          catch_body: CodeBlock;
      }
    // Other
    | { kind: "Paren"; expr: CodeExpr; has_trailing_comma: boolean }
    | { kind: "DoWith"; body: CodeExpr; args: CodeExprList }
    | { kind: "StructLiteral"; struct_name: string; body: CodeExprMap };

/** Result type with ok and err variants */
export type ResultType = {
    ok: TypeExpr;
    err: TypeExpr;
};

/** Match header (for match expressions and if-match conditions) */
export type MatchHeader = {
    variant_name: string;
    variant_bind: string;
    expr: CodeExpr;
};

/** List of code expressions (used for arguments, etc.) */
export type CodeExprList = {
    items: CodeExpr[];
};

/** Map of code expressions (used for struct literals, memory params, etc.) */
export type CodeExprMap = {
    fields: { key: string; value: CodeExpr }[];
};
