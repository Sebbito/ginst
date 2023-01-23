# ginst

**ginst** (short for generic installer) a tool used to install programs according to self written configurations.

The tool takes a `.json` file and tries to execute the commands given.

Note: It's currently only tested on Linux.

## Requirements

-  rust (cargo)
-  bash

## Installation

Ez mode

```bash
cargo install ginst
```

or if you want to build it yourself:

```bash
git clone https://github.com/Sebbito/ginst.git
cd ginst
carbo build -r
cp target/release/ginst ~/.local/bin
```

on the last step you can copy it to any directory that is included in PATH but i suggest you use `.local/bin` for that.

## Usage

`ginst --file path/to/programs.json`

Use
```
  k
h   l
  j
```
or 

```
  ˄
˂   ˃
  ˅
```
to move through the menu.

The `l` or `˃` to go a level down and view the dependencies and `h` or `˂` to go one level back up.

Further keybinds are:

-  `i` or `enter` to execute the installation instructions for the program
-  `c` to execute the configuration steps
-  `q` to quit (goes up one level if you're in a submenu)

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
-  Also there is the option to specify multiple distributions in one line separated by commas **!without spaces!** for which the installation instructions apply
-  You can also use a '\*' instead of a regular distribution name to execute the steps for all distros
-  The distribution will be read out of `/etc/os-release` so make sure you name it like you see it in there

## Troubleshooting

Q: The program doesn't start with the error:

`Could not parse json file. Maybe you forgot a comma somewhere?`

A: Well the answer is right there. The json could not be parsed. Make sure that you follow the outlined structure and have no syntax error.

## Further Help

To get more information, see the [Wiki](https://github.com/Sebbito/ginst/wiki) or see

```bash
ginst --help
```
