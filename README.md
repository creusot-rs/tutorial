# Creusot Tutorial

Learn how to formally verify Rust programs with [Creusot](https://github.com/creusot-rs/creusot).

## Contents

- [Examples](src/ex0_examples.rs) from the [slides of the POPL 2026 tutorial presentation][slides]
- [Exercise 1: Gnome sort](https://creusot-rs.github.io/creusot/guide/tutorial/gnome_sort.html)
- [Exercise 2: Linked list](https://creusot-rs.github.io/creusot/guide/tutorial/linked_list.html)

[slides]: https://docs.google.com/presentation/d/e/2PACX-1vQVC3kV9ZRg7KBRFGxKuCDwLQIyi9OpZJTRbIG3tMm1dLoNYi7VSqM163l4k2zqicWd5WiyTV1MRWiw/pub?start=false&loop=false&delayms=3000

## Installation

### Try Creusot online

You can try Creusot in your browser with Codespaces.
In a few clicks, you get a ready-made VS Code session running
on Github's servers, with a free quota of 120h monthly per user.

1. Click the button below, select "Machine type: 4-core", and click on "Create codespace". This will open a VS Code session in your browser.

    [![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new/creusot-rs/tutorial?skip_quickstart=true&machine=standardLinux32gb&repo=1129033041&ref=main&devcontainer_path=.devcontainer%2Fdevcontainer.json)

2. Wait 2 minutes for the codespace to load on Github's servers.
3. Click on "Allow" if VS Code complains about an unsupported OS version.
4. You are ready to run Creusot!

### Install Creusot locally

1. [Follow these instructions to install Creusot.](https://creusot-rs.github.io/creusot/guide/installation.html)

2. (Optional) For VS Code users, you can install [Creusot IDE](https://github.com/creusot-rs/creusot-ide), a VS Code extension that provides syntax highlighting and buttons to run Creusot in the editor.

    - Note: There are two parts to Creusot IDE: the extension itself, which is available on the VS Code Marketplace, and the language server Creusot LSP, which must currently be installed manually. See the README of Creusot IDE for instructions.
    - Note also: Make sure to install the "Pre-release" version of the extension.

2. Clone this repository.

    ```shell
    git clone https://github.com/creusot-rs/tutorial
    cd tutorial
    ```

## Usage

- In the terminal, run `cargo creusot prove` to check that it works.
    You should see some checkmarks in the output indicating that
    the initial examples have been proved correct.

- If you use VS Code with Rust Analyzer (*e.g.*, if you are on Codespaces), this tutorial repository is already set up to use Creusot to type-check on save. If you also installed the [Creusot IDE](https://github.com/creusot-rs/creusot-ide) extension, you should see buttons appear to the left of `fn` definitions in your editor; clicking on one will
run SMT solvers to attempt to prove that the corresponding function
satisfies its specification.

    - Troubleshooting: If nothing happens in ~10 seconds after `cargo creusot` or `cargo creusot prove`, you can try to relaunch the LSP server with the following commands: `Ctrl+P` (or click the search bar at the top) > Write "> Creusot" (with the `>`!) > Select "Creusot: Restart language server"".

- Otherwise:

    - Run `cargo creusot` for type-checking and compilation to Coma (quick, but no proofs).
    - Run `cargo creusot prove` to run Why3find and dispatch proof obligations to SMT solvers. (This command also implies `cargo creusot` so you don't need to do it separately.)
    - Run `cargo creusot prove $NAME` to only try proving function `$NAME`. For example, `cargo creusot prove gnome_sort`. 
