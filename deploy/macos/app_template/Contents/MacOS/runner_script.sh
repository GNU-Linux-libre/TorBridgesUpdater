#!/bin/sh

app_path="$(dirname "$0")"

cd "$app_path"

export GSETTINGS_SCHEMA_DIR="$app_path/share/glib-2.0/schemas"
export XDG_DATA_DIRS="$app_path/share"
export GDK_PIXBUF_MODULEDIR="$app_path/gdk-pixbuf-2.0/2.10.0/loaders"
export GDK_PIXBUF_MODULE_FILE="$app_path/gdk-pixbuf-2.0/2.10.0/loaders.cache"

./tor_bridges_updater
