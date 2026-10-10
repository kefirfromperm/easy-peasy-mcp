# Examples for task 116

Startup examples (AGENTS.md) with JSON-RPC exchanges. The examples use the fixture `funnel` of task 102: eleven
`many/*.txt` files with `x`; `src/a.rs`. With one search thread and slow reads the first search takes a few seconds.

## E116.1 Overlapping searches and an `update_config`

The second search waits for the only search thread, so the first search is answered first and keeps `r1`. The
`update_config` is sent when the first search reports its progress, so it is applied while the first search runs:
the first search uses the configuration of its start (`detail = lines`), the second one the new one
(`detail = count`).

Fixture: `funnel`

Slow reads: `300ms`

```toml conf/o.toml
[server]
threads = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "text has 'x'"}, "_meta": {"progressToken": "p2"}}}
```

```json receive
{"jsonrpc": "2.0", "method": "notifications/progress", "params": {"progressToken": "p2"}}
```

```json send
{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "update_config", "arguments": {"set": "output.detail = \"count\""}}}
```

```json receive
{"jsonrpc": "2.0", "id": 3, "result": {"content": [{"type": "text", "text": "[output]\ndetail = \"count\"    # was \"lines\"\n"}], "isError": false}}
```

```json send
{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "name = 'a.rs'"}}}
```

```json receive
{"jsonrpc": "2.0", "id": 4, "result": {"content": [{"type": "text", "text": "files: 1  lines: 0  matches: 0  result: r2\n"}], "isError": false}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"content": [{"type": "text", "text": "files: 11  lines: 11  matches: 11  result: r1\n#1 text has 'x'\n\nrepeated:\n  ×11 ⟦#1:x⟧    many/f01.txt:1, many/f02.txt:1, many/f03.txt:1, … 8 more\n"}], "isError": false}}
```
