#! /bin/bash

rm -rf target/DiskDbg.pkg Package/Payload/Library/Scripts/disk-dbg
universal2
cp target/universal2-apple-darwin/release/disk-dbg Package/Payload/Library/Scripts/
pkgbuild --identifier com.ChefKiss.DiskDbg --version 1.0.0 --scripts ./Package/Scripts --root Package/Payload target/DiskDbg.pkg
