launchctl unload -w /Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist
rm /Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist
rm /Library/Scripts/disk-dbg
pkgutil --forget com.ChefKiss.DiskDbg
