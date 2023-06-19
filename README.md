![Static Badge](https://img.shields.io/badge/rustc-1.69-red)
![Crates.io](https://img.shields.io/crates/d/ginst)
![Crates.io](https://img.shields.io/crates/l/ginst)


# ginst

**ginst** (short for generic installer) a tool used to install programs
according to self written configurations on GNU/Linux.

The tool takes a file and will execute the commands given.

## Why ginst?

While ginst is similar to automation plattforms like ansible, it aims to be
used once when initially installing programs for the first time on a machine.

You should be able to download and execute a binary together with your easy to
use and expand config and lean back.

The configuration is meant to support cross-plattform usage and as such eliminate
complicated shell scripts.

## Features

You can:
- declare dependencies (as many as you want)
- differentiate between Linux distributions
- install and configure programs in a bash based environment
- 

## Requirements

As of now you need cargo and bash on your machine. I'm working on providing binaries
for the biggest architectures as a means to eliminate any dependencies for using ginst.

-  cargo (rust)
-  bash

## Installation

```bash
curl --proto '=https' -sSf  https://raw.githubusercontent.com/Sebbito/ginst/main/install.sh | sh
```

## Usage and help

See the [Wiki](https://github.com/Sebbito/ginst/wiki)

or use

```bash
ginst --help
```

## Resources

### Having trouble?

Open an issue if you have any trouble or read through the official [ginst wiki](https://codeberg.org/Sebito/ginst/wiki).

### Contributing

If you want to contribute, you can open a Pull Request for ginst [here](https://codeberg.org/Sebito/ginst/pulls).

Every constructive feedback, feature request and bug fix is greatly appreciated ❤️
