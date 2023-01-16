# FTI

FTI or First Time Install is (currently) a Linux only tool used by me to install all the command line programs that i usually need with every Linux distro.

The tool takes a `programs.json` file and tries to execute the commands given in the configuration

## Prerequisites

-  cargo
-  git

Other tools like curl will be installed automatically.

## Installation (as of now)

```bash
git clone https://github.com/Sebbito/FTI.git
cd FTI/
cargo run
```

## Configuring

Right now the only way to configure the tool is through the `programs.json` file.

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
        "<dist-name>": "<instructions>"
    },
    "dependencies": {
        "programs": [
            ...
        ]
    }
}
```

As you can see, the dependencies attribute is also just a list of programs, meaning you can nest this as deep as you want to.
