# Installation

easy-peasy-mcp is one binary without runtime dependencies. It runs locally as an MCP server over stdio: the MCP client
starts it, nothing listens on the network. Install it, register it in an MCP client, and optionally write a config file.

- [Install scripts](#install-scripts)
- [By hand](#by-hand)
- [Register in an MCP client](#register-in-an-mcp-client)
- [Write tools](#write-tools)
- [Config file](#config-file)
- [Update and uninstall](#update-and-uninstall)

## Install scripts

| Platform      | Script        | Default install directory                    |
|---------------|---------------|----------------------------------------------|
| Linux, macOS  | `install.sh`  | `~/.local/bin`                               |
| Windows       | `install.bat` | `%LOCALAPPDATA%\Programs\easy-peasy-mcp`     |

Both scripts get the binary in one of two ways:

- **build** — from the source checkout the script lives in, with `cargo build --release --locked`; needs Rust, see
  [rustup.rs](https://rustup.rs) (the toolchain version is pinned in `rust-toolchain.toml` and installed by rustup)
- **download** — a release binary from [GitHub releases](https://github.com/kefirfromperm/easy-peasy-mcp/releases);
  needs `curl` or `wget` on Linux and macOS, PowerShell on Windows

Without `--build` or `--download` a script builds when it runs from a source checkout and `cargo` is installed, and
downloads otherwise.

From a source checkout:

```sh
git clone https://github.com/kefirfromperm/easy-peasy-mcp.git
cd easy-peasy-mcp
./install.sh
```

```bat
git clone https://github.com/kefirfromperm/easy-peasy-mcp.git
cd easy-peasy-mcp
install.bat
```

Without a checkout, a release binary:

```sh
curl -fsSL https://raw.githubusercontent.com/kefirfromperm/easy-peasy-mcp/master/install.sh | sh
```

Options:

| Option            | Meaning                                                                   |
|-------------------|---------------------------------------------------------------------------|
| `--dir <dir>`     | install directory; also `$EASY_PEASY_INSTALL_DIR`                         |
| `--build`         | build from the source checkout                                            |
| `--download`      | download a release binary                                                 |
| `--version <tag>` | release to download, e.g. `v0.121.0`; default: the latest release        |
| `--no-path`       | `install.bat` only: don't add the install directory to the user `PATH`    |
| `--help`          | usage                                                                     |

```sh
./install.sh --dir /usr/local/bin          # may need sudo
curl -fsSL https://raw.githubusercontent.com/kefirfromperm/easy-peasy-mcp/master/install.sh | sh -s -- --version v0.121.0
```

`install.sh` tells when the install directory is not in `PATH`; `install.bat` adds it to the user `PATH` (open a new
terminal afterwards). At the end both print the installed version and a registration command.

Release binaries exist for Linux x86-64, macOS (Apple silicon and Intel) and Windows x86-64 (also run by Windows on
ARM). On other platforms, e.g. Linux on ARM, build from source.

## By hand

Build:

```sh
cargo build --release --locked
```

and copy `target/release/easy-peasy-mcp` (`easy-peasy-mcp.exe` on Windows) into a directory of your choice. Or take a
release archive, `easy-peasy-mcp-<version>-<target>.tar.gz` (`.zip` for Windows), and copy the binary out of it.

Check:

```sh
easy-peasy-mcp --version
```

```
easy-peasy-mcp 0.135.0, spec 2
```

## Register in an MCP client

The client starts the server itself, so it needs the command and the arguments. `--root` sets the directories the
server may search (`server.allowed_roots`); every other option is in [CONFIG.md](CONFIG.md#command-line).

Claude Code:

```sh
claude mcp add easy-peasy -- easy-peasy-mcp --root ~/projects
```

Other clients, e.g. `.mcp.json` of a project in Claude Code, or the config of Claude Desktop, Cursor and others:

```json
{
  "mcpServers": {
    "easy-peasy": {
      "command": "easy-peasy-mcp",
      "args": ["--root", "~/projects"]
    }
  }
}
```

If the install directory is not in the `PATH` the client sees, write the full path as `command`, e.g.
`/home/me/.local/bin/easy-peasy-mcp` or `C:\\Users\\me\\AppData\\Local\\Programs\\easy-peasy-mcp\\easy-peasy-mcp.exe`
(`\` doubled in JSON).

To check the registration, ask the agent to call `get_config`: the first line is the server version, then the
allowed roots and the defaults.

## Write tools

By default the server only reads: the agent gets `find`, `get_config` and `update_config`. The write tools `delete`,
`rename`, `copy` and `replace` act on the files a `find` query selects; they are off until you turn them on with
`--allow-write` (or `allow_write = true` in the [config file](#config-file)). The agent can't turn them on itself.

```sh
claude mcp add easy-peasy -- easy-peasy-mcp --root ~/projects --allow-write
```

```json
{
  "mcpServers": {
    "easy-peasy": {
      "command": "easy-peasy-mcp",
      "args": ["--root", "~/projects", "--allow-write"]
    }
  }
}
```

An MCP client which was registered without the flag needs it added and a restart; `get_config` then shows
`allow_write = true` and `tools/list` the four tools.

What they do, so you know what you allow:

- they change only files inside the allowed roots, never an allowed root itself, and never follow symlinks
- `delete`, and `rename` / `copy` with `on_conflict = "overwrite"`, move files to the trash of the system (the
  freedesktop trash on Linux, the Trash on macOS, the Recycle Bin on Windows), so a mistake can be undone; an entry
  on a file system without a trash is skipped, never deleted for good, unless the agent asks for `permanent: true`
- `dry_run: true` shows what would be done without changing anything, and `expect: N` makes a call fail unchanged
  when the selection has another number of files; the descriptions of the tools tell the agent to use them
- `replace` keeps the encoding, the BOM, the line breaks and the permissions of a file

The details are in [TOOLS.md](TOOLS.md#write-tools).

## Config file

Optional. Instead of command line arguments the roots and the defaults of every request can be kept in a TOML file:

| Platform | Path                                                         |
|----------|--------------------------------------------------------------|
| Linux    | `~/.config/easy-peasy-mcp/config.toml`                       |
| macOS    | `~/Library/Application Support/easy-peasy-mcp/config.toml`   |
| Windows  | `%APPDATA%\easy-peasy-mcp\config.toml`                       |

```toml
[server]
allowed_roots = ["~/projects"]
# allow_write = true            # the write tools: delete, rename, copy, replace

[traversal]
exclude = ["**/target/**", "**/node_modules/**"]
```

The install scripts never write it. Its keys and the other ways to name the file (`--config`, `$EASY_PEASY_CONFIG`)
are in [CONFIG.md](CONFIG.md#config-file).

## Update and uninstall

Update: run the install script again, it replaces the binary. On Windows a running binary can't be replaced: stop the
MCP client first. The client starts the new version at its next start.

Uninstall: delete the binary from the install directory, remove the server from the MCP client (`claude mcp remove
easy-peasy`), and delete the config directory if you made one. On Windows also remove the install directory from the
user `PATH` (Settings → System → About → Advanced system settings → Environment Variables).
