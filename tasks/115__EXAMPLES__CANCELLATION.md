# Examples for task 115

Startup examples (AGENTS.md) with JSON-RPC exchanges. The examples use the fixture `funnel` of task 102: eleven
`many/*.txt` files with `x`. With one search thread and reads of two seconds a search of the content takes half a
minute.

## E115.1 A cancelled search gets no response

The next request is answered as usual.

Fixture: `funnel`

Slow reads: `2000ms`

```toml conf/o.toml
[server]
threads = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "text has 'x'"}}}
```

```json send
{"jsonrpc": "2.0", "method": "notifications/cancelled", "params": {"requestId": 2, "reason": "user aborted"}}
```

```json send
{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "name = 'a.rs'"}}}
```

```json receive
{"jsonrpc": "2.0", "id": 3, "result": {"content": [{"type": "text", "text": "files: 1  lines: 0  matches: 0  result: r1\n#1 name = 'a.rs'\n\nsrc/\na.rs  [#1]\n"}], "isError": false}}
```

```json never
{"id": 2}
```

## E115.2 An unknown or finished request

A cancellation of a request which is unknown or already answered is ignored.

Fixture: `funnel`

```text command
easy-peasy-mcp
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "name = 'a.rs'", "output": {"detail": "count"}}}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"content": [{"type": "text", "text": "files: 1  lines: 0  matches: 0  result: r1\n"}], "isError": false}}
```

```json send
{"jsonrpc": "2.0", "method": "notifications/cancelled", "params": {"requestId": 2}}
```

```json send
{"jsonrpc": "2.0", "method": "notifications/cancelled", "params": {"requestId": 99}}
```

```json send
{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "name = 'b.rs'", "output": {"detail": "count"}}}}
```

```json receive
{"jsonrpc": "2.0", "id": 3, "result": {"content": [{"type": "text", "text": "files: 1  lines: 0  matches: 0  result: r2\n"}], "isError": false}}
```
