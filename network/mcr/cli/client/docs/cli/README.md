# Command-Line Help for `mcr-network-client`

This document contains the help content for the `mcr-network-client` command-line program.

**Command Overview:**

* [`mcr-network-client`↴](#mcr-network-client)
* [`mcr-network-client markdown`↴](#mcr-network-client-markdown)
* [`mcr-network-client markdown generate`↴](#mcr-network-client-markdown-generate)
* [`mcr-network-client markdown file`↴](#mcr-network-client-markdown-file)
* [`mcr-network-client markdown print`↴](#mcr-network-client-markdown-print)
* [`mcr-network-client markdown workspace`↴](#mcr-network-client-markdown-workspace)

## `mcr-network-client`

The `mcr-network-client` CLI

**Usage:** `mcr-network-client [COMMAND]`

###### **Subcommands:**

* `markdown` — ???



## `mcr-network-client markdown`

???

**Usage:** `mcr-network-client markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mcr-network-client markdown generate`

Generate and update the documentation

**Usage:** `mcr-network-client markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mcr-network-client markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mcr-network-client markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mcr-network-client markdown print`

Print the documentation in the shell

**Usage:** `mcr-network-client markdown print`



## `mcr-network-client markdown workspace`

Generate the documentation for the workspace

**Usage:** `mcr-network-client markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
