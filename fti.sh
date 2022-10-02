#!/bin/bash

# Error codes
SUCCESS=0
PACKAGE_MANAGER_ERROR=1
DISTRO_ERROR=2
TOOLCHAIN_ERROR=3
INSTALLATION_ERROR=4
PERMISSON_ERROR=5

dotfiles=https://github.com/Sebbito/dotfiles.git

# function definitions

check_tool() {
	if ! tool_exists "$@"; then # tool not found 
		if ! install_tool "$@"; then # couldn't install tool either
			exit $INSTALLATION_ERROR
		fi
	fi
}

tool_exists() {
	if type "$@"; then
		return $SUCCESS
	else
		return $TOOLCHAIN_ERROR
	fi
}

install_tool() {
	if $pm_install "$@" > /dev/null; then
		echo "$@" "was correctly installed"
		return $SUCCESS
	else
		echo "could not install" "$@"
		return $INSTALLATION_ERROR
	fi
}

install_fish(){
	echo "Installing fish"
	case $OS in
		Ubuntu)
			apt-add-repository --yes ppa:fish-shell/release-3
			$pm_update	
			$pm_install fish
			;;
		Debian)
			echo 'deb http://download.opensuse.org/repositories/shells:/fish:/release:/3/Debian_11/ /' | sudo tee /etc/apt/sources.list.d/shells:fish:release:3.list
			curl -fsSL https://download.opensuse.org/repositories/shells:fish:release:3/Debian_11/Release.key | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/shells_fish_release_3.gpg > /dev/null
			$pm_update	
			$pm_install fish
			;;
	esac
	echo "Fish installed"
}

install_rust() {
	if curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | $SHELL -s -- -y; then
		{ # add it to home bash and fish style
			source "$HOME/.cargo/env"

			# for the default way of adding the rust stuff to $HOME doesn't work so we create the env file
			if tool_exists fish; then
				echo "Adding config to fish aswell"
				file=$HOME/.config/fish/conf.d/env.fish
				touch "$file" 
				echo set -gx PATH "$HOME/.cargo/bin" "$PATH" > "$file"
			fi
			return $SUCCESS
		} || {
		return $INSTALLATION_ERROR
		}
	else 
		echo "Couldn't install Rustup"
		exit $INSTALLATION_ERROR
	fi
}

install_neovim()
{
	sudo add-apt-repository ppa:neovim-ppa/stable
	$pm_update
	$pm_install neovim
}

prep()
{
	case $OS in
		Debian|Ubuntu)
			pm="sudo apt"
			pm_install="sudo apt-get install -y"
			pm_update="sudo apt update -y"
			$pm_update
			$pm_install software-properties-common apt-utils
			;;
		Arch)
			pm="pacman"
			pm_install="pacman -S"
			;;
	esac
}

install()
{
	check_tool git
	check_tool curl
	check_tool make

	# Check if desired programms are installed and if not, install them

	if ! tool_exists nvim; then
		install_neovim
	fi

	# these programms need special treatment
	# IMPORTANT: install fish first, for the config of rust
	if ! tool_exists fish; then
		echo "fish not installed, installing now..."
		install_fish
	fi

	if ! tool_exists rustup; then
		echo "Rust not installed, installing now..."
		install_rust
	else
		echo "Rust installed, proceeding"
	fi
}

configure()
{
	# only configure if neither .config/nvim or .config/fish exist
	if [ ! -d "$HOME"/.config/nvim ] && [ ! -d "$HOME"/.config/fish ]; then
		git clone "$dotfiles"
		mv dotfiles/* "$HOME"/.config/
	else
		echo "Dotfiles already installed. Aborting configuration"
	fi

	# clean up the download
	rm -rf ./dotfiles
}

echo_install_options()
{
	echo "	install		-	Installs programms."
}

echo_config_options()
{
	echo "	configure	-	Installs the configuration for the installed programms."
}

help_function()
{
	echo "Use these options for fti.sh:"
	echo "	all		-	Does installation and full configuration. Adviced for a fresh install."
	echo_install_options
	echo_config_options
}


# Programm start


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

# name cleanup because if it's debian, and we got the info from /etc/os-release it will not be 'Debian' but 'Debian GNU/Linux' 
case $OS in
	"Debian GNU/Linux")
		OS=Debian
		;;
esac

# Check if guessed package manager is installed
if ! tool_exists $pm; then
	echo "Package manager not found. Aborting"
	exit $PACKAGE_MANAGER_ERROR
fi

case $1 in
	all)
		# do everything
		prep
		install
		configure
		;;
	install)
		# check and install tools
		install
		;;
	configure)
		# download and deploy configs only
		configure
		;;
	*)
	help_function
	;;
esac

# Show report

# Exit