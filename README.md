![](https://github.com/fission-suite/PROJECTNAME/raw/master/assets/logo.png?sanitize=true)

# Interplanetary Image Metadata and Resizer

[![Build Status](https://travis-ci.org/fission-suite/PROJECTNAME.svg?branch=master)](https://travis-ci.org/fission-suite/PROJECTNAME)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/fission-suite/blob/master/LICENSE)
[![Maintainability](https://api.codeclimate.com/v1/badges/44fb6a8a0cfd88bc41ef/maintainability)](https://codeclimate.com/github/fission-suite/PROJECTNAME/maintainability)
[![Built by FISSION](https://img.shields.io/badge/⌘-Built_by_FISSION-purple.svg)](https://fission.codes)
[![Discord](https://img.shields.io/discord/478735028319158273.svg)](https://discord.gg/zAQBDEq)
[![Discourse](https://img.shields.io/discourse/https/talk.fission.codes/topics)](https://talk.fission.codes)

Ymir is a mighty giant from Norse Mythology. Iimir is a tool that smashes images into different sizes, and links them together with metadata on IPFS, the InterPlanetary File System.

# QuickStart
###Installing ipfs

See [here](https://docs.ipfs.io/guides/guides/install/).

###Setting up rust
To install rustup on Linux or MacOS 
```shell
$ curl https://sh.rustup.rs -sSf | sh
```

###Cloning, compiling and running

```shell
$ git clone git@github.com:fission-suite/iimir.git
$ cd iimir
$ cargo run -q -- <path_to_image>
```

`cargo run` builds and runs in one command. You can always find the executable 
at `target/debug/iimir`.

# Known issues

- When original file size is too small, its data gets overwritten in the output dag
by a smaller version. This is due to a [documented go-ipfs issue](https://github.com/ipfs/go-ipfs/issues/7190).
