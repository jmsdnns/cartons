# Asynchronous SSH Pools

This project uses pools of SSH connections to issue commands to multiple machines at the same time.

## CLI Commands

A `.justfile` is provided to simplify the CLI. A working install of limavm is required for using it.

In the commands below, a `swarm` is a group of VMs, and `vm` is a single instance of a VM. For example, `mkswarm` works by calling `mkvm` one or more times.

```shell
$ just -l
Available recipes:
    default
    hostlist name
    hostswarm name
    hostvm id
    lsvm
    mkswarm name count
    mkvm id
    rmswarm name
    rmvm id
    shellvm id
    startswarm name
    stopswarm name
```

We'll begin by creating a swarm.

```shell
$ just mkswarm killabeez 2
...
INFO[0019] Run `limactl start killabeez-1` to start the instance.
...
INFO[0014] Run `limactl start killabeez-2` to start the instance.
```

Then start the swarm.

```shell
$ just startswarm killabeez
...
INFO[0045] READY. Run `limactl shell killabeez-1` to open the shell.
...
INFO[0043] READY. Run `limactl shell killabeez-2` to open the shell.
```

Create a new config the pools, based on the new VMs you just started.

```shell
$ just sshconfig killabeez
...
```

Now run the app. It will read your new config for all the SSH details it needs. The ssh pools will run `hostname` on each machine and print that in order of which VM responds first.

```shell
$ cargo run
CONFIG:
- hosts: ["127.0.0.1:52014", "127.0.0.1:52037"]
- key file: /Users/jmsdnns/.lima/_config/user
- username: jmsdnns
RESULTS:
result: lima-killabeez-2
result: lima-killabeez-1
```

At this point, you can stop the swarm and delete its VMs.

```shell
$ just stopswarm killabeez
$ just rmswarm killabeez
```
