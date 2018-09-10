# sagz
search and gzip

A simple gzip utility written in pure Rust

```
 
EXAMPLES:
    sagz -p ./ -a 10m -e .log
    Locate and gzip files in current directory older than 10 minutes,
    with extension .log
    sagz -p /home/magnus/slask/ -a 100d -d
    Locate all files (no -e switch) in /home/magnus/slask/, older than 100 days,
    do not gzip (-d flag), just print which files would be processed.
    
NOTES:
    When running without extension, you will be prompted as an extra
    precaution, since this means you are processing all files in the
    path.
    
    Like GNU gzip, timestamps are preserved upon creating gzip.
    If you want to disable this, use the -n (--nongnu) flag.

USAGE:
    sagz [FLAGS] [OPTIONS] --age <TIME> --path <PATH>

FLAGS:
    -d, --dryrun     Don't compress. Just print which files
                     would be compressed.
    -h, --help       Prints help information
    -k, --keep       Don't delete the original file.
    -n, --nongnu     Don't preserve timestamps.
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

<h2>Examples</h2>

*Compress files in `/home/magnus/slask/logs/` with extension `.log` older than 100 days:*

```
$ sagz -p /home/magnus/slask/logs/ -e .log -a 100d
```

<h5>Video example (on Debian 9)</h5>

https://asciinema.org/a/HWQmLIqw44QkVqkWDkcqkewvR

<h2>Notes</h2>

Using `sagz` is similar to running:

```find <PATH> -maxdepth 1 -type f -name "*.log" -mtime +100 -exec gzip "{}" \+```

On a GNU/Linux OS.

Hopefully, `sagz` is somewhat easier than remembering all those switches (and knowing the difference between `-mtime` and `-mmin` etc.). And, since it is written in pure Rust without OS bindings, it works on Windows as well.

Written just for fun.
