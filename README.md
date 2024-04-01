# minter-daemon

A tool used to start a program, and restart it if the process exits.

Especially for mining.

## First Step

Confirm that on your Linux you don't have to input password when exec `sudo` command.

For example, in Ubuntu you can follow those steps:

1. `sudo visudo`
2. Modify the last third line "%sudo ALL=(ALL:ALL) ALL" to "%sudo ALL=(ALL:ALL) NOPASSWD:ALL"
3. Ctrl+O, Enter, Ctrl+X

## Second Step

Prepare a .sh file(for example it is named "start.sh"), it will be used to execute your real target app, just like:

```sh
#!/bin/bash
sudo some_app
```

## Third Step

Start minter-daemon (assume your .sh file is named "start.sh"):

```sh
./minter-daemon './start.sh'
```

Tips:

1. `minter-daemon` program should be in the same directory with `start.sh`
2. Only executable file path is supported, like './start.sh'. If your argument is like 'bash ./start.sh', an error will occur.
