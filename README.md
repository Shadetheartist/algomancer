# Algomancer

This project is an unstable work-in-progress and could be subject to major changes in the near future. 

## Overview

Algomancer is a deterministic state machine designed to serve as a reusable 'game rules engine' for the card game 
'Algomancy'.

It is currently tailored for use as a command line tool, handling input state & actions 'statelessly', 
it is not a long-running process, continuous games must be handled in some way by the user.

## Features

- Deterministic Gameplay: Ensures consistent rule enforcement and game progression.
- Efficient Performance: Built in Rust for optimal speed and memory usage.

## Getting Started

###  Prerequisites

To be able to compile from source, you will need to have rust & cargo installed. 

### Installation

Clone the repository:

`git clone https://github.com/Shadetheartist/algomancer`

Navigate to the project directory:

`cd algomancer`

Build the project:

`cargo build --release`

The compiled binary should be in 

`./target/release/algomancer`

## Usage

At any time you can run `algomancer help` to discover info about the current (sub)command.

```
$ algomancer help

The Algomancy Game Rules Engine

Usage: algomancer <COMMAND>

Commands:
  new     Initialize a new game
  action  List actions, Apply an Action
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### New Game

```
Initialize a new game

Usage: algomancer new [OPTIONS] <COMMAND>

Commands:
  live_draft   Create a new game using the Live Draft game mode
  pre_draft    Not Implemented Yet
  team_draft   Not Implemented Yet
  constructed  Not Implemented Yet
  help         Print this message or the help of the given subcommand(s)

Options:
  -s, --seed <SEED>  a 128 bit unsigned integer used as seed for the random number generator [default: 0]
  -h, --help         Print help
```

As a quick start example, the following command creates the initial state for a 'new player mode' game and writes it
to stdout as json.

`$ algomancer new live_draft -f wood -f fire 1v1`

### Actions

```
List actions, Apply an Action

Usage: algomancer action <COMMAND>

Commands:
  ls     List the valid actions for a given state
  apply  Apply an action to a given state and receive the resulting state
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

#### List Actions

You can generate a list of valid actions for a given state with `$ algomancer action ls`

`$ algomancer action ls "$(algomancer new live_draft -f wood -f fire 1v1)"`

#### Apply an Action

and apply an action to the state with something `$ algomancer action apply` 

`$ algomancer action apply "$(algomancer new live_draft -f wood -f fire 1v1)" '{"PassPriority":2}'`





