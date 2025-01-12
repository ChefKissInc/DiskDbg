# DiskDbg

A utility that captures the `dmesg`, `ioreg` and `AGDCDiagnose` of a system in realtime, saving them to `/Library/Logs`.

You can get a build from the GitHub Actions tab. Or click [here](https://nightly.link/ChefKissInc/DiskDbg/workflows/main/master/macOS%20Universal%20Release.zip) for a quick download.

All you need to do is install the package.

The service is always active, dumping logs. To disable, run the following command:

```sh
launchctl unload -w /Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist
```

To completely uninstall, run the following command:

```sh
curl -L https://raw.githubusercontent.com/ChefKissInc/DiskDbg/refs/heads/master/uninstall.sh | sudo bash
```
