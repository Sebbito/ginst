# FTI

FTI or First Time Install is a tool used by me to install all the command line programms that i usually need with every Linux distro.

## JSON Structure

```json
{
    "programm-name": {
        "install": {
            ...
        }
    }
}
```

## Prerequisites

-  Package manager (apt or pacman)
-  git

Other tools like curl will be installed automatically.

## Usage

There are two ways to use this script.

### Clone it with git

Clone this repository and go into the folder to execute the install script.

```bash

apt update
apt install -y git
git clone https://github.com/Sebbito/FTI.git
cd FTI/
./fti.sh all

```

### One time use via curl

You can also use the script without downloading the repository like this:

`curl -sSl https://raw.githubusercontent.com/Sebbito/FTI/main/fti.sh | sh -s -- all `

Note that you can change the option you want. Just change `all` to `install` or to `configure`.
## Installation list

-  git
-  curl
-  make
-  rust suite via rustup
-  neovim
-  fish

Miscelanius installations:

(On Debian based)
-  apt-utils
-  software-properties-common
