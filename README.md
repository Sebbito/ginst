# ginst

ginst (short for generic installer) a Tool used to install programs according to self written configurations.

The tool takes a `programs.json` file and tries to execute the commands given.

Note: It's currently only tested on Linux.

## Requirements

-  rust (cargo)
-  git

## Installation (as of now)

```bash
git clone https://github.com/Sebbito/FTI.git
cd ginst/
cargo build
sudo cp ./target/debug/ginst /bin/
```

## Configuring

Right now the only way to configure the tool is through the `programs.json` file (which has to be in the working dir).

There you can specify a program with installations per distribution and dependencies.

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
    "install": {
        "<dist-name1>,<dist-name2>":{
            "steps": [
                "<bash-command1>",
                "<bash-command2>"
            ]
        }
    },
    "dependencies": {
        "programs": [
            ...
        ]
    }
}
```

As you can see, the dependencies attribute is also just a list of programs, meaning you can nest this as deep as you want to.

Also there is the option to specify multiple distributions in one line separated by commas for which the installation instructions should be executed.

You can also specify multiple commands for installing.

## Troubleshooting

Q: The programm doesn't start with the error:

`Could not parse json file. Maybe you forgot a comma somewhere?`

A: Well the answer is right there. The json could not be parsed. Make sure that you follow the outlined structure and have no syntax error.
