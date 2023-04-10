#!/bin/bash

if [ ! -f "../../target/release/tor_bridges_updater" ]; then
    echo "No release was built. Please build the release target with \"cargo build -r\"";
fi

if [ -f "../../target/release/tor_bridges_updater" ]; then
    if [ -e "Tor Bridges Updater" ]; then
        rm -rf "Tor Bridges Updater";
    fi
    mkdir "Tor Bridges Updater"
    cp ../../target/release/tor_bridges_updater "Tor Bridges Updater"
    cp "com.yakovlevegor.TorBridgesUpdater.desktop.template" "Tor Bridges Updater"
    cp ../../icons/logo.svg "Tor Bridges Updater"/icon.svg
    cp install_script "Tor Bridges Updater"/install.bash
    chmod +x "Tor Bridges Updater"/install.bash
    cp uninstall_script "Tor Bridges Updater"/uninstall.bash
    chmod +x "Tor Bridges Updater"/uninstall.bash
fi
