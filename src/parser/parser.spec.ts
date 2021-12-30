import { m } from 'multiline-str';

import { parse } from './parser';

describe('parser', () => {
  it('parses empty list', () => {
    const res = parse('()');

    expect(res).toStrictEqual([[]]);
  });

  it('parses symbols', () => {
    expect(parse('(a)')).toStrictEqual([['a']]);
    expect(parse('(abc)')).toStrictEqual([['abc']]);
  });

  it('parses numbers', () => {
    expect(parse('(0)')).toStrictEqual([['0']]);
    expect(parse('(10)')).toStrictEqual([['10']]);
    expect(parse('(10.0)')).toStrictEqual([['10.0']]);
    expect(parse('(0.10)')).toStrictEqual([['0.10']]);
    expect(parse('(100_000.000_000)')).toStrictEqual([['100_000.000_000']]);
  });

  it('parses strings', () => {
    expect(
      parse('("The quick brown fox jumps over the lazy dog.")'),
    ).toStrictEqual([['"The quick brown fox jumps over the lazy dog."']]);
    expect(parse('("\\"escaped quotes supported\\"")')).toStrictEqual([
      ['"\\"escaped quotes supported\\""'],
    ]);
  });

  it('parses lists with multiple atoms', () => {
    const res = parse('(abc1 123 "string\\"s")');

    expect(res).toStrictEqual([['abc1', '123', '"string\\"s"']]);
  });

  it('skips whitespace and comments', () => {
    const res = parse(m`
      (a             b               c) ;comment
      ; comment
      (
        1 ; comment
        2 ; comment ; comment ; comment
        3 ;; comment
      )
      `);

    expect(res).toStrictEqual([
      ['a', 'b', 'c'],
      ['1', '2', '3'],
    ]);
  });

  it('parses complex expressions', () => {
    const res = parse('(a (b (c () d e f (g h)))) (i) (j k (l m n))');

    expect(res).toStrictEqual([
      ['a', ['b', ['c', [], 'd', 'e', 'f', ['g', 'h']]]],
      ['i'],
      ['j', 'k', ['l', 'm', 'n']],
    ]);
  });

  it('parses llvm hello world', () => {
    const res = parse(m`
      ;; Hello World example

      (llvm/target-triple "x86_64-pc-linux-gnu")

      (llvm/extern-fn puts (&i8) i32)

      (llvm/define-string-ptr hello-world "Hello World!")

      (llvm/fn main () i32
        (llvm/insert-point "entrypoint")
        (puts hello-world)
        (llvm/ret (i32 0))
      )
      `);

    expect(res).toStrictEqual([
      ['llvm/target-triple', '"x86_64-pc-linux-gnu"'],
      ['llvm/extern-fn', 'puts', ['&i8'], 'i32'],
      ['llvm/define-string-ptr', 'hello-world', '"Hello World!"'],
      [
        'llvm/fn',
        'main',
        [],
        'i32',
        ['llvm/insert-point', '"entrypoint"'],
        ['puts', 'hello-world'],
        ['llvm/ret', ['i32', '0']],
      ],
    ]);
  });

  it('throws error on invalid input', () => {
    expect(() => parse('')).toThrow(
      `Unexpected character: '<EOF>' at line 1, col 1`,
    );
    expect(() => parse('(')).toThrow(
      `Unexpected character: '<EOF>' at line 1, col 2`,
    );
    expect(() => parse('(a ()')).toThrow(
      `Unexpected character: '<EOF>' at line 1, col 6`,
    );
    expect(() => parse('(a ))')).toThrow(
      "Unexpected character: ')' at line 1, col 5",
    );
  });
});
