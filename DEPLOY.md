# Deploy for a platform

After building the release target, you can then deploy the project to a package.

## Windows

Run `msys2` from your command line, and `cd` to the project's directory.

`cd "C:\Users\username\path\to\project"`

and then cd into the project's deploy folder

`cd "deploy\windows"`

Run the deploy script

`bash deploy.bash`

After the script finishes, you can copy freshly created folder **"Tor Bridges Updater"** and distribute it.

## macOS or Linux

Open terminal, `cd` to your project's directory, and then to `deploy/macos` if you're on **macOS**, or `deploy/linux` if you're on **Linux**.

Run 

`bash deploy.bash`

You can then copy freshly created app **"Tor Bridges Updater.app"** if you're on **macOS**, or a directory **"Tor Bridges Updater"** if you're on **Linux**, and distribute it.
