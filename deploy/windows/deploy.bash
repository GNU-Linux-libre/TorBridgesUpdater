#!/bin/bash

if [ ! -f "../../target/release/tor_bridges_updater" ]; then
    echo "No release was built. Please build the release target with \"cargo build -r\"";
fi

if [ -f "../../target/release/tor_bridges_updater" ]; then
    if [ -e "Tor Bridges Updater" ]; then
        rm -rf "Tor Bridges Updater";
    fi
    mkdir "Tor Bridges Updater"
    cp ../../target/release/tor_bridges_updater "Tor Bridges Updater/Tor Bridges Updater.exe"
    for dep in $(/mingw64/bin/ntldd -R "Tor Bridges Updater/Tor Bridges Updater.exe" | grep "mingw.*\.dll" -o); do
        cp /$dep "Tor Bridges Updater"; 
    done;
    
    mkdir "Tor Bridges Updater"/lib
    cp -r /mingw64/lib/gdk-pixbuf-2.0 "Tor Bridges Updater"/lib
    
    mkdir -p "Tor Bridges Updater"/share/glib-2.0/schemas

    /mingw64/bin/glib-compile-schemas /mingw64/share/glib-2.0/schemas --targetdir="Tor Bridges Updater"/share/glib-2.0/schemas

    mkdir -p "Tor Bridges Updater"/share/icons

    cp -r /mingw64/share/icons/Adwaita "Tor Bridges Updater"/share/icons
    cp -r /mingw64/share/icons/hicolor "Tor Bridges Updater"/share/icons

    cp install_script "Tor Bridges Updater"/install.bat
fi
