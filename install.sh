#!/bin/bash

# Error codes
SUCCESS=0
PACKAGE_MANAGER_ERROR=1
DISTRO_ERROR=2
TOOLCHAIN_ERROR=3
INSTALLATION_ERROR=4
PERMISSON_ERROR=5

# function definitions
check_permission() {
	if [ $EUID -ne 0 ]; then
		echo "This script needs super user rights. Please start this script again with sudo ./install.sh"
		exit $PERMISSON_ERROR
	fi
}

check_tool() {
	if ! tool_exists $1; then # tool not found 
		if ! install_tool $1; then # couldn't install tool either
			exit $INSTALLATION_ERROR
		fi
	fi
}

tool_exists() {
	if type "$1" > /dev/null; then
		echo "$1" "is installed"
		return $SUCCESS
	else
		return $TOOLCHAIN_ERROR
	fi
}

install_tool() {
	if $pm_install $1 > /dev/null; then
		echo $1 " was correctly installed"
		return $SUCCESS
	else
		return $INSTALLATION_ERROR
	fi
}

install_fish(){
	case $OS in
		Ubuntu)
			apt-add-repository ppa:fish-shell/release-3
			apt update
			apt install fish
	esac
}

install_rust() {
	if curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; then
		case $SHELL in
			/usr/bin/fish)
				echo "big fish detected blub blub"
				set -gx PATH "$HOME/.cargo/bin" $PATH;
				;;
			*)
				source "$HOME/.cargo/env"
		esac
		return $SUCCESS
	else 
		echo "Couldn't install Rustup"
		exit $INSTALLATION_ERROR
	fi
}

# Programm start
check_permission

# check for distribution

if [ -f /etc/os-release ]; then
    # freedesktop.org and systemd
    . /etc/os-release
    OS=$NAME
    VER=$VERSION_ID
elif type lsb_release >/dev/null 2>&1; then
    # linuxbase.org
    OS=$(lsb_release -si)
    VER=$(lsb_release -sr)
elif [ -f /etc/lsb-release ]; then
    # For some versions of Debian/Ubuntu without lsb_release command
    . /etc/lsb-release
    OS=$DISTRIB_ID
    VER=$DISTRIB_RELEASE
elif [ -f /etc/debian_version ]; then
    # Older Debian/Ubuntu/etc.
    OS=Debian
    VER=$(cat /etc/debian_version)
elif [ -f /etc/SuSe-release ]; then
    # Older SuSE/etc.
    exit $DISTRO_ERROR;
elif [ -f /etc/redhat-release ]; then
    # Older Red Hat, CentOS, etc.
    exit $DISTRO_ERROR;
else
    # Fall back to uname, e.g. "Linux <version>", also works for BSD, etc.
    OS=$(uname -s)
    VER=$(uname -r)
fi

case $OS in
	Ubuntu) 
		pm="apt"
		pm_install="apt-get install -y"
		;;
	Arch)
		pm="pacman"
		pm_install="pacman -S"
		;;
esac

# Check if nescessary tools are installed

if ! tool_exists $pm; then
	echo "Package manager not found. Aborting"
	exit $PACKAGE_MANAGER_ERROR
fi

check_tool git
check_tool curl
check_tool make

# Check if desired programms are installed and if not, install them

check_tool neovim

# these programms need special treatment
if ! tool_exists rustup; then
	echo "Rust not installed, installing now..."
	install_rust
else
	echo "Rust installed, proceeding"
fi

if ! tool_exists fish; then
	echo "fish not installed, installing now..."
	install_fish
fi

# Config



# Show report


# Exit