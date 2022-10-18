#!/bin/bash

if [ ! -f "../../target/release/tor_bridges_updater" ]; then
    echo "No release was built. Please build the release target with \"cargo build -r\"";
fi

if [ -f "../../target/release/tor_bridges_updater" ]; then
    if [ -e "Tor Bridges Updater.app" ]; then
        rm -rf "Tor Bridges Updater.app";
    fi
    cp -r app_template "Tor Bridges Updater.app"
    cp ../../target/release/tor_bridges_updater "Tor Bridges Updater.app/Contents/MacOS"
  
    cd "Tor Bridges Updater.app/Contents/MacOS"

    mkdir -p share/glib-2.0/schemas

    glib-compile-schemas /usr/local/Cellar/gtk4/*/share/glib-2.0/schemas --targetdir=share/glib-2.0/schemas

    dylibbundler -of -d . -p "@executable_path" -ns -b -x tor_bridges_updater
    mkdir -p share/icons
    cp -r /usr/local/Cellar/adwaita-icon-theme/*/share/icons/ share/icons
    cp -r /usr/local/Cellar/hicolor-icon-theme/*/share/icons/ share/icons
    cp -r /usr/local/Cellar/gdk-pixbuf/*/lib/gdk-pixbuf-2.0 .

    GDK_PIXBUF_MODULEDIR="gdk-pixbuf-2.0/2.10.0/loaders" gdk-pixbuf-query-loaders > gdk-pixbuf-2.0/2.10.0/loaders.cache

    cd gdk-pixbuf-2.0/2.10.0/loaders

    for lib in *.so; do
        dylibbundler -of -d . -p "@executable_path" -ns -b -x "$lib";
    done;
fi
