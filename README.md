# vp (vim pipe)

Spawn `$EDITOR` for use in a pipe command. If the editor exits with nonzero
status (`:cq` in `vim`), the pipeline is aborted. This is very much like the
`vipe` program in `moreutils`, except it forwards its arguments to the editor.
It also supports (n)vim shorthand for setting the filetype (see example below).

# Examples

```
$ curl https://get.docker.com | vp | sh
```

View bash script before executing.

```
$ cat query-template.sql | vp +"/{" | sqlite3 file.db
```

This argument tells `vim` (the configured editor) to jump to the template marker
'{' so that a query template can be filled in and passed to sqlite.

```
$ cat data.json | vp json | jq '.foo.bar'
```

Shorthand for `vp +"set ft=json"`. Setting the filetype in vim enables syntax
highlighting, linting, and so on. Modelines are sometimes used to do this, but
not all files have them, and many people like to disable them for security
reasons. Also, filetype detection often does not work when no file extension is
available.

# Installation

$ cargo install vp
