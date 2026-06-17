# DiskDbg

A utility that captures the `dmesg`, `ioreg` and `AGDCDiagnose` output periodically to files in `/Library/Logs`.

## Usage guide

Install the package.

To enable, run the following command:

```sh
sudo launchctl load -w /Library/LaunchDaemons/org.ChefKiss.DiskDbg.plist
```

The service will constantly gather logs in the background; leave enabled ONLY during debugging.

To disable, run the following command:

```sh
sudo launchctl unload -w /Library/LaunchDaemons/org.ChefKiss.DiskDbg.plist
```

To uninstall, run the following command:

```sh
curl -L https://raw.githubusercontent.com/ChefKissInc/DiskDbg/refs/heads/master/uninstall.sh | sudo bash
```
