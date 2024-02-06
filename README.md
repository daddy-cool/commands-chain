# commands-chain
Simple program that takes a list of commands and executes them one after the other.


## Usage
`commands-chain --help`

```
Execute a list of commands in sequence

Usage: commands-chain [COMMANDS]...

Arguments:
  [COMMANDS]...  list of commands

Options:
  -h, --help  Print help
```

### Example
If commands contain whitespace they need to be quoted:
```
commands-chain "git branch --list" "git status"
```

If path to executable contains additional whitespace they need to be double-quoted:
```
commands-chain "'/my path/git' branch --list" "'/my path/git' status"
```

## Build
`cargo build --release`
