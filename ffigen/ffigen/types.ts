export type CSymbol = CTypeDef | CEnum | CStruct | CFunction;

export type CTypeDef = {
  tag: "typedef";
  name: string;
  location: string;
  type: CType;
};

export type CEnum = {
  tag: "enum";
  name: string;
  location: string;
  id: number;
  fields: { tag: "field"; name: string; value: number }[];
};

export type CStruct = {
  tag: "struct";
  name: string;
  location: string;
  id: number;
  "bit-size": number;
  "bit-alignment": number;
  fields: {
    tag: "field";
    name: string;
    "bit-size": number;
    "bit-alignment": number;
    type:
      | { tag: string }
      | { tag: string; "bit-size": number; "bit-alignment": number };
  }[];
};

export type CFunction = {
  tag: "function";
  name: string;
  location: string;
  inline: boolean;
  parameters: { name: string; type: CType }[];
  "return-type": CType;
};

export type CType =
  | { tag: ":void" }
  | { tag: ":int"; "bit-size": number; "bit-alignment": number }
  | { tag: ":unsigned-int"; "bit-size": number; "bit-alignment": number }
  | { tag: ":unsigned-long-long"; "bit-size": number; "bit-alignment": number }
  | { tag: ":long-long"; "bit-size": number; "bit-alignment": number }
  | { tag: ":double"; "bit-size": number; "bit-alignment": number }
  | { tag: ":char"; "bit-size": number; "bit-alignment": number }
  | { tag: ":function-pointer" }
  | { tag: ":pointer" }
  | { tag: "struct" }
  | { tag: ":enum"; id: number; name: string }
  | { tag: "size_t" }
  | { tag: "int64_t" }
  | { tag: "uint64_t" }
  | { tag: "uint32_t" }
  | { tag: "uint8_t" };
