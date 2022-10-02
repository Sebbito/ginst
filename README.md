# FTI

FTI or First Time Install is a tool used by me to install all the command line programms that i usually need with every Linux distro.

## Prerequisites

-  Package manager (apt or pacman)
-  git

Other tools like curl will be installed automatically.

## Usage

Clone this repository and go into the folder to execute the install script.

```bash

apt update
apt install -y git
git clone https://github.com/Sebbito/FTI.git
cd FTI/
./fti.sh

```

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