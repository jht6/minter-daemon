# minter-daemon

A tool used to start a program, and restart it if the process exits.
Especially for mining.

## First Step

Confirm root permission is given to this program.

```sh
sudo chown root [bin]
sudo chmod a+s [bin]
```

## Second Step

Prepare a .sh file(for example it is named "start.sh"), just like:

```sh
#!/bin/bash
sudo some_app
```

## Third Step

Start minter-daemon:

```sh
./minter-daemon './start.sh'
```
