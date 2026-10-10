# Examples for task 25

## Fixtures

### encodings

```text tree
enc/invalid.txt            2026-10-01 10:00
enc/utf16be.txt            2026-10-01 10:00
enc/utf16le-nobom.txt      2026-10-01 10:00
enc/utf16le.txt            2026-10-01 10:00
enc/utf32be.txt            2026-10-01 10:00
enc/utf32le.txt            2026-10-01 10:00
enc/utf8bom.txt            2026-10-01 10:00
```

```text setup
bytes enc/invalid.txt 68656c6c6f2066726f6d20696e76616c696420ff2062797465730a
bytes enc/utf16be.txt feff00680065006c006c006f002000660072006f006d0020007500740066002d0031003600620065000a
bytes enc/utf16le-nobom.txt 680065006c006c006f002000660072006f006d0020007500740066002d00310036006c006500200077006900740068006f007500740020006100200062006f006d000a00
bytes enc/utf16le.txt fffe680065006c006c006f002000660072006f006d0020007500740066002d00310036006c0065000a00
bytes enc/utf32be.txt 0000feff00000068000000650000006c0000006c0000006f0000002000000066000000720000006f0000006d000000200000007500000074000000660000002d000000330000003200000062000000650000000a
bytes enc/utf32le.txt fffe000068000000650000006c0000006c0000006f0000002000000066000000720000006f0000006d000000200000007500000074000000660000002d00000033000000320000006c000000650000000a000000
bytes enc/utf8bom.txt efbbbf68656c6c6f2066726f6d207574662d382077697468206120626f6d0a
```

The files are created by the test setup: each says `hello from <encoding>` in its encoding, with its BOM, except
`utf16le-nobom.txt`; `invalid.txt` is UTF-8 with an invalid byte `ff`. Times are UTC.

## E25.1 Every encoding is decoded; the output shows the ones which are not UTF-8

A UTF-16 file without a BOM is recognized by its zero bytes and not taken for a binary file. The invalid byte is
replaced with `�` and the file has a `decode_errors` warning.

Fixture: `encodings`

```json request
{ "query": "text has 'hello'" }
```

```text response
files: 7  lines: 7  matches: 7  result: r1
#1 text has 'hello'

enc/
invalid.txt  [#1]  lines: 1  matches: 1
1:⟦#1:hello⟧ from invalid � bytes
utf16be.txt  [#1]  lines: 1  matches: 1  utf-16be
1:⟦#1:hello⟧ from utf-16be
utf16le-nobom.txt  [#1]  lines: 1  matches: 1  utf-16le
1:⟦#1:hello⟧ from utf-16le without a bom
utf16le.txt  [#1]  lines: 1  matches: 1  utf-16le
1:⟦#1:hello⟧ from utf-16le
utf32be.txt  [#1]  lines: 1  matches: 1  utf-32be
1:⟦#1:hello⟧ from utf-32be
utf32le.txt  [#1]  lines: 1  matches: 1  utf-32le
1:⟦#1:hello⟧ from utf-32le
utf8bom.txt  [#1]  lines: 1  matches: 1
1:⟦#1:hello⟧ from utf-8 with a bom

warning: decode_errors (1): enc/invalid.txt
```

## E25.2 A UTF-16 file without a BOM

Fixture: `encodings`

```json request
{ "query": "name ~ 'utf16*' and text has 'without a bom'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name ~ 'utf16*'
#2 text has 'without a bom'

enc/
utf16le-nobom.txt  [#1 #2]  lines: 1  matches: 1  utf-16le
1:hello from utf-16le ⟦#2:without a bom⟧
```
