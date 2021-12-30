export type Parser<T> = (input: string) => ParseResult<T>;
export type ParseResult<T> =
  | { ok: true; value: T; remaining: string }
  | { ok: false; error: string; remaining: string };

type ValuesOf<P> = {
  [K in keyof P]: P[K] extends Parser<infer V> ? V : never;
};

type OmitItems<Items, O> = Items extends readonly []
  ? []
  : Items extends readonly [infer X, ...infer XS]
  ? X extends O
    ? OmitItems<XS, O>
    : [X, ...OmitItems<XS, O>]
  : never;

export const EOF = undefined;
export const Skip = Symbol('Skip');
export const Nothing = Symbol('Nothing');

const MAP_TO_SELF = <T>(value: T) => value;

export const lazy = <T>(init: () => Parser<T>): Parser<T> => {
  let parse: Parser<T> | undefined;

  return (input) => {
    if (!parse) {
      parse = init();
    }

    return parse(input);
  };
};

export const emit = <T>(value: T): Parser<T> => {
  return (input) => {
    return { ok: true, value, remaining: input };
  };
};

export const optional = <T>(parse: Parser<T>): Parser<T | typeof Nothing> => {
  return either<T | typeof Nothing>([parse, emit(Nothing)]);
};

export const skip = <T>(parse: Parser<T>): Parser<typeof Skip> =>
  map(parse, () => Skip);

export const chain =
  <P extends readonly Parser<unknown>[] = Parser<unknown>[]>(
    parsers: P,
  ): Parser<OmitItems<ValuesOf<P>, typeof Skip>> =>
  (input) => {
    const result = {
      ok: true,
      value: [] as unknown[],
      remaining: input,
    };

    for (const parse of parsers) {
      const itemResult = parse(result.remaining);

      if (!itemResult.ok) {
        return itemResult;
      }

      result.remaining = itemResult.remaining;
      if (itemResult.value !== Skip) {
        result.value.push(itemResult.value);
      }
    }

    return result as unknown as ParseResult<
      OmitItems<ValuesOf<P>, typeof Skip>
    >;
  };

export const either =
  <T>(parsers: Parser<T>[]): Parser<T> =>
  (input) => {
    for (const parse of parsers) {
      const parseRes = parse(input);

      if (parseRes.ok) {
        return parseRes;
      }
    }

    return {
      ok: false,
      error: `Expected either: ${parsers.map((p) => p.name).join(' or ')}`,
      remaining: input,
    };
  };

export const zeroOrMany =
  <T>(parse: Parser<T>): Parser<T[]> =>
  (input) => {
    const result: ParseResult<T[]> = { ok: true, value: [], remaining: input };

    while (result.remaining.length > 0) {
      const repeatingRes = parse(result.remaining);

      if (!repeatingRes.ok) {
        break;
      }

      result.value.push(repeatingRes.value);
      result.remaining = repeatingRes.remaining;
    }

    return result;
  };

export const separatedBy = <T>(
  parse: Parser<T>,
  separator: Parser<typeof Skip>,
): Parser<T[]> => {
  return map(
    chain([
      parse,
      zeroOrMany(
        map(chain([separator, parse] as const), ([item]) => item as T),
      ),
    ] as const),
    ([required, other]) =>
      other === undefined ? (required as T[]) : [required as T, ...other],
  );
};

export const oneOrMany = <T>(parse: Parser<T>): Parser<T[]> =>
  map(chain([parse, zeroOrMany(parse)] as const), ([required, other]) =>
    other === undefined ? (required as T[]) : [required as T, ...other],
  );

export const map =
  <A, B>(
    parse: Parser<A>,
    transformValue: (a: A) => B,
    transformError: (reason: string) => string = MAP_TO_SELF,
  ): Parser<B> =>
  (input) => {
    const res = parse(input);

    return res.ok
      ? { ok: true, value: transformValue(res.value), remaining: res.remaining }
      : {
          ok: false,
          error: transformError(res.error),
          remaining: res.remaining,
        };
  };

export const named = <A>(name: string, parse: Parser<A>): Parser<A> =>
  map(parse, MAP_TO_SELF, () => `Expected ${name}`);

const displayCharacter = (char: string | typeof EOF) => {
  switch (char) {
    case EOF:
      return '<EOF>';
    case '\n':
      return '\\n';
    case "'":
      return "\\'";
    default:
      return char;
  }
};

export const is =
  <T extends string | typeof EOF>(
    predicate: (char: string | typeof EOF) => char is T,
  ): Parser<T> =>
  (input) => {
    const value = input[0];

    return predicate(value)
      ? { ok: true, value, remaining: input.slice(1) }
      : {
          ok: false,
          error: `Unexpected character: '${displayCharacter(value)}'`,
          remaining: input,
        };
  };

export const char = <T extends string | typeof EOF>(char: T): Parser<T> =>
  is((c): c is T => c === char);

export const pattern =
  (pattern: RegExp): Parser<string> =>
  (input) => {
    const match = pattern.exec(input);

    if (!match) {
      return {
        ok: false,
        error: `Expected: ${pattern}`,
        remaining: input,
      };
    }

    const value = match[0];

    return { ok: true, value, remaining: input.slice(value.length) };
  };
