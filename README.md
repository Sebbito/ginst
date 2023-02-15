# ginst

**ginst** (short for generic installer) a tool used to install programs according to self written configurations on GNU/Linux.

The tool takes a file and tries to execute the commands given.

## Requirements

-  cargo (rust)
-  bash

## Installation

```bash
curl --proto '=https' -sSf  https://github.com/Sebbito/ginst/blob/main/install.sh | sh
```

## Usage

See the [Wiki](https://github.com/Sebbito/ginst/wiki)

or use

```bash
ginst --help
```

## Configuring

ginst uses files which hold the instructions used to install or configure programs.

There are example files in `example/` which are regularly tested and should be in the correct form for each release.

Note: See the [Wiki](https://github.com/Sebbito/ginst/wiki/Supported-File-Types) on the supported file types.

## Troubleshooting

Q: The program doesn't start and throws an error:

`thread 'main' panicked at 'Error("expected `<char>`", line: ?, column: ?)', src/parser.rs`

A: The file could not be parsed correctly. The error message states the line and column of the unexpected character. Check the example files and see if you've made an error in your file.
