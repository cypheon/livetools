# livetools: Live-updating Command Line Tools

[![Build Status](https://travis-ci.org/cypheon/livetools.svg?branch=master)](https://travis-ci.org/cypheon/livetools)

This repo provides live-updating versions of the `wc` and `date` binaries,
designed for interactive use, called `lwc` and `ldate`.

The tools perform the same tasks as their standard *nix counterparts, but
provide periodic updates on the terminal.

## lwc

Count bytes/words/lines in standard input.

### Usage

    $ lwc [options]

Options:

    -c, --bytes         Print count of bytes
    -l, --lines         Print count of lines
    -w, --words         Print count of words
    -h, --help          Show usage

## ldate

Display current date and time.

### Usage

    $ ldate

This will display the current time updated once a second.  Currently, the
`ldate` does not support any command line flags.
