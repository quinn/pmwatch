# pmwatch
```
Usage: pmwatch --watch <PATH> [COMMAND]...

Arguments:
  [COMMAND]...  

Options:
  -w, --watch <PATH>  
  -h, --help          Print help information
  -V, --version       Print version information
```

watch `PATH` for changes, restart `COMMAND`. Uses inotify, only works on linux. 

## TODO
* terminate the program if the watched process exits for any reason. 
