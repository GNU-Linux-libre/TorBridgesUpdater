# Prepare for building

To prepare for building, you first need to install **Gtk 4** and it's dependencies on your system.

## Windows

First install [Chocolatey](https://chocolatey.org) package manager. This will make the whole process of installing dependencies way easier.

Install **Rust MINGW** by running

`choco install rust`

Then install **MSYS2**

`choco install msys2`

and **add it's directories to your PATH**

`C:\tools\msys64\mingw64\include`

`C:\tools\msys64\mingw64\bin`

`C:\tools\msys64\mingw64\lib`

After installing everything with Chocolatey, open command line and type `msys2` to launch MSYS2 shell. From within this shell, you need to install **Gtk 4** and some additional dependencies. Run

`pacman -S base-devel mingw-w64-x86_64-toolchain mingw-w64-x86_64-gtk4 mingw-w64-adwaita-icon-theme mingw-w64-hicolor-icon-theme mingw-w64-x86_64-ntldd-git`

**NOTE:** You will have to build the project from the command line, **not from MSYS2**!

## macOS

Install **Homebrew** package manager from [brew.sh](https://brew.sh). Next, just install Rust, Gtk 4 with icons and DylibBundler by running

`brew install rust gtk4 adwaita-icon-theme hicolor-icon-theme dylibbundler`

## Linux

On Arch Linux, install Rust, Gtk 4 and it's dependencies

`pacman -S rust gtk4`

## Now you can build the project with

`cargo build -r`

and continue to [deploying](DEPLOY.md)
