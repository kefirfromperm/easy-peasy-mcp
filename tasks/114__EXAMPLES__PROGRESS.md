# Examples for task 114

Startup examples (AGENTS.md) with JSON-RPC exchanges. The examples use the fixture `funnel` of task 102: eleven
`many/*.txt` files with `x` and four other files without one. One search thread (`server.threads = 1`) and slow reads
make the search take a few seconds.

## E114.1 Progress about once a second

Fixture: `funnel`

Slow reads: `400ms`

```toml conf/o.toml
[server]
threads = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "text has 'x'", "output": {"detail": "count"}}, "_meta": {"progressToken": "p7"}}}
```

```json receive
{"jsonrpc": "2.0", "method": "notifications/progress", "params": {"progressToken": "p7", "progress": "*", "message": "scanned * files, * matched, *s"}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"content": [{"type": "text", "text": "files: 11  lines: 11  matches: 11  result: r1\n"}], "isError": false}}
```

## E114.2 No progress for a fast search

Fixture: `funnel`

```text command
easy-peasy-mcp
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "text has 'x'", "output": {"detail": "count"}}, "_meta": {"progressToken": "p8"}}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"content": [{"type": "text", "text": "files: 11  lines: 11  matches: 11  result: r1\n"}], "isError": false}}
```

```json never
{"jsonrpc": "2.0", "method": "notifications/progress"}
```

## E114.3 No progress without a token

Fixture: `funnel`

Slow reads: `400ms`

```toml conf/o.toml
[server]
threads = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "text has 'x'", "output": {"detail": "count"}}}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"content": [{"type": "text", "text": "files: 11  lines: 11  matches: 11  result: r1\n"}], "isError": false}}
```

```json never
{"jsonrpc": "2.0", "method": "notifications/progress"}
```
