import { panic } from 'panic-fn';

import {
  chain,
  char,
  either,
  EOF,
  lazy,
  map,
  oneOrMany,
  Parser,
  pattern,
  skip,
  zeroOrMany,
} from './combinator-parser';

// TODO: support comments

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

  if (res.remaining.length > 0) {
    const pos = getPosAtIndex(input, input.length - res.remaining.length);

    panic(new ParseError(`Unexpected character '${res.remaining[0]}'`, pos));
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

const whitespace = map(pattern(/\s*/), (c) => c.length);
const atom = either([
  pattern(/^[^()\d"][\w\/-]*/), // symbol
  pattern(/^\d[\d_]*(?:\.[\d_]*)?/), // number
  pattern(/^"(?:[^"\\]|\\.)*"/), // string
]);
const exprThenWhitespace: Parser<SExpr> = lazy(() =>
  map(chain([expr, skip(whitespace)] as const), ([expr]) => expr),
);
const list = map(
  chain([
    skip(char('(')),
    skip(whitespace),
    zeroOrMany(exprThenWhitespace),
    skip(char(')')),
  ] as const),
  ([content]) => content as SExpr,
);
const expr = either([atom, list]);
const script = map(
  chain([
    skip(whitespace),
    oneOrMany(map(chain([list, skip(whitespace)] as const), ([expr]) => expr)),
    skip(char(EOF)),
  ] as const),
  ([exprs]) => exprs,
);
