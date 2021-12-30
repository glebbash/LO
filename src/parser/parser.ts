import { panic } from 'panic-fn';

import {
  chain,
  char,
  either,
  EOF,
  lazy,
  map,
  Nothing,
  optional,
  Parser,
  pattern,
  separatedBy,
  skip,
} from './combinator-parser';

export type SExpr = string | SExpr[];
type Pos = { line: number; col: number };

class ParseError extends Error {
  constructor(message: string, pos: Pos) {
    super(`${message} at line ${pos.line}, col ${pos.col}`);
  }
}

export function parse(input: string): SExpr[] {
  const res = script(input);

  if (!res.ok) {
    const pos = getPosAtIndex(input, input.length - res.remaining.length);

    panic(new ParseError(res.error, pos));
  }

  return res.value;
}

function getPosAtIndex(input: string, index: number): Pos {
  const pos: Pos = { line: 1, col: 1 };

  for (let i = 0; i < index; i++) {
    const char = input[i];

    if (char === '\n') {
      pos.line++;
      pos.col = 0;
    }

    pos.col++;
  }

  return pos;
}

const literallyWhitespace = skip(pattern(/\s*/));
const comment = map(pattern(/^;[^\n]*\n/), (c) => c.slice(1, -1));
const whitespace = skip(
  chain([
    literallyWhitespace,
    optional(separatedBy(comment, literallyWhitespace)),
    literallyWhitespace,
  ] as const),
);
const atom = either([
  pattern(/^[^()\d"][\w\/-]*/), // symbol
  pattern(/^\d[\d_]*(?:\.[\d_]*)?/), // number
  pattern(/^"(?:[^"\\\n]|\\.)*"/), // string
]);
const list: Parser<SExpr> = lazy(() =>
  map(
    chain([
      skip(char('(')),
      whitespace,
      optional(separatedBy(expr, whitespace)),
      whitespace,
      skip(char(')')),
    ] as const),
    ([content]) => (content === Nothing ? [] : content),
  ),
);
const expr = either([atom, list]);
const script = map(
  chain([
    whitespace,
    separatedBy(list, whitespace),
    whitespace,
    skip(char(EOF)),
  ] as const),
  ([exprs]) => exprs,
);
