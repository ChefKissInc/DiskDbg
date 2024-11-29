launchctl unload -w /Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist
rm -f /Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist
rm -f /Library/Scripts/disk-dbg
pkgutil --forget com.ChefKiss.DiskDbg
