# DiskDbg

A utility that captures the `dmesg`, `ioreg` and `AGDCDiagnose` output periodically to files in `/Library/Logs`.

You can get a build from the GitHub Actions tab. Quick download [here](https://nightly.link/ChefKissInc/DiskDbg/workflows/main/master/macOS%20Universal%20Release.zip).

## Usage guide

Install the package.

To enable, run the following command:

```sh
sudo launchctl load -w /Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist
```

The service will constantly gather logs in the background. Leave enabled only during debugging.

To disable, run the following command:

```sh
sudo launchctl unload -w /Library/LaunchDaemons/com.ChefKiss.DiskDbg.plist
```

To uninstall, run the following command:

```sh
curl -L https://raw.githubusercontent.com/ChefKissInc/DiskDbg/refs/heads/master/uninstall.sh | sudo bash
```
