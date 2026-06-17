#!/bin/bash

set -euo pipefail

# the old version
if [ -e "/Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist" ]; then
    launchctl unload -w /Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist
    rm /Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist
    rm /Library/Scripts/disk-dbg
    pkgutil --forget com.ChefKiss.DiskDbg
fi

# the new version
if [ -e "/Library/LaunchDaemons/org.ChefKiss.DiskDbg.plist" ]; then
    launchctl unload -w /Library/LaunchDaemons/org.ChefKiss.DiskDbg.plist
    rm /Library/LaunchDaemons/org.ChefKiss.DiskDbg.plist
    rm /Library/Application\ Support/com.ChefKiss.DiskDbg/DiskDbg
    pkgutil --forget org.ChefKiss.DiskDbg
fi
