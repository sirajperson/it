#!/bin/bash

# install_deb.sh - Script to add the 'it' Debian repo and install the package

# Check if running as root or with sudo
if [ "$EUID" -ne 0 ]; then
  echo "Please run this script with sudo: sudo bash $0"
  exit 1
fi

# Add the repo to sources.list.d
echo "Adding 'it' repo to APT sources..."
echo "deb [trusted=yes] https://sirajperson.github.io/it/ ./" > /etc/apt/sources.list.d/it.list

# Update APT
echo "Updating APT package list..."
apt update

# Install the 'it' package
echo "Installing 'it'..."
apt install -y it

# Verify installation
if command -v it &> /dev/null; then
  echo "Installation successful! Run 'it --help' for usage."
else
  echo "Installation failed. Check APT logs for errors."
fi