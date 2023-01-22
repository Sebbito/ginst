# ginst

**ginst** (short for generic installer) a tool used to install programs according to self written configurations.

The tool takes a `.json` file and tries to execute the commands given.

Note: It's currently only tested on Linux.

## Requirements

-  rust (cargo)
-  git

## Installation

```bash
cargo install ginst
```

## Usage

`ginst --file path/to/programs.json`

## Configuring

Right now the only way to configure the tool is through the `programs.json` file (which has to be in the working dir). There you can specify a program with installations per distribution and dependencies.

See the `example.json` on how `ginst` expects the `.json` to look like.

The `.json` file has the following structure:

```json
{
    "programs": [
        ...
    ]
}
```

Where each program has a structure of:

```json
{
    "name": "<name>",
    "installation": {
        "<dist-name1>,<dist-name2>": [
            "<bash-command1>",
            "<bash-command2>"
        ]
    },
    "configuration": {
        "<dist-name1>,<dist-name2>": [
            "<bash-command1>",
            "<bash-command2>"
        ]
    },
    "dependencies": {
        "programs": [
            ...
        ]
    }
}
```

Some things that are worth noting:

-  The dependencies attribute is also just a list of programs, meaning you can add as many dependencies and nest this as deep as you want to.
-  Also there is the option to specify multiple distributions in one line separated by commas **!without spaces!** for which the installation instructions apply. You can also use a '\*' instead of a regular distribution name to execute the steps for all distros. The distribution will be read out of `/etc/os-release` so make sure you name it like you see it in there or like the list further down.
-  You can specify multiple commands for installing or configuring.

## Troubleshooting

Q: The program doesn't start with the error:

`Could not parse json file. Maybe you forgot a comma somewhere?`

A: Well the answer is right there. The json could not be parsed. Make sure that you follow the outlined structure and have no syntax error.

## os-release names

Make sure you use one of these in your `.json`:

- Fedora Linux
- Ubuntu
- Debian GNU/Linux
- openSUSE Tumbleweed
- etc.
