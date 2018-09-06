# sagz
search and gzip

A simple gzip utility written in pure Rust

```
USAGE:
    sagz [FLAGS] [OPTIONS] --age <TIME> --path <PATH>

FLAGS:
    -d, --dryrun     Don't compress. Just print which files
                     would be compressed, and their age in seconds
    -h, --help       Prints help information
    -k, --keep       Don't delete the original file.
    -V, --version    Prints version information

OPTIONS:
    -a, --age <TIME>     Operate on files older than <TIME>.
                         Valid <TIME> is positive integer,
                         followed by d (days), or h (hours), or m (minutes).
                         Examples: 2d, 13h, 45m
    -e, --ext <EXT>      Operate on files with extension <EXT>.
                         Examples: .log, .bak
                         If not supplied, work on ALL files.
    -p, --path <PATH>    Operate on files in path <PATH>.
                         Examples: ./, /home/backups
  
```

---

<h2>Example</h2>

*Compress files in `/home/magnus/slask/logs/` with extension `.log` older than 100 days:*

```$ sagz -p /home/magnus/slask/logs/ -e .log -a 100d```

<h2>Video example (Debian 9)</h2>
https://asciinema.org/a/UrZ3HVelCRpPn43hdy2tW7vue

<h2>Notes</h2>

Unlike GNU gzip, `sagz` does not (at least not **yet**) preserve timestamps from the original on the compressed files.<br>
Using `sagz` is similar to running:

```find <PATH> -maxdepth 1 -type f -name "*.log" -mtime +100 -exec gzip "{}" \+```

On a GNU/Linux OS.

Hopefully, `sagz` is somewhat easier than remembering all those switches (and knowing the difference between `-mtime` and `-mmin` etc.). And, since it is written in pure Rust without OS bindings, it works on Windows as well.

Written just for fun.
