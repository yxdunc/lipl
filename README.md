# LIve PLot

lipl is a command line tool that is similar to [watch](https://en.wikipedia.org/wiki/Watch_(Unix)) but has extended functions for commands outputing a number.

For example `lipl 'ls'` will show the output of `ls` and will refresh the result
every 1 second (by default).

Now if a command outputs a number like `ls -1 | wc -l` it will be able to plot
the result. In this case the plot will show a constant result until another 
process adds or removes a file from the current folder. It can be useful when 
downloading many files to the current folder and you want to follow the
progress. If you know that in total 1000 files will be downloaded, you can 
simply add the option `--target 1000` and a progress bar will be shown along 
with an estimated time of completion.

## Arguments

### positional argument:

A string containing a bash command.

If the bash command returns a number a plot will be shown (ex: `ls -1 | wc -l`)

If the bash command returns anything else the command shows the output and 
follow the same behaviour as [watch](https://en.wikipedia.org/wiki/Watch_(Unix))

### named arguments:
`-n/--refresh-rate`: the refresh rate in seconds

`-t/--target`: a target value that will be used to show a progress bar based on the command outputs. A simple linear regression is used.

`-l/--history-len`: the number of results from the given command that are stored and plotted.  

## Sample usages:

Plot number of files in `/tmp`:
```
cargo run -- -n 0.5 "ls -1 /tmp | wc -l"
```

Plot cpu usage of a given PID
```
cargo run -- -n 0.1 "ps -p ${PID} -o %cpu | tail -1"
```

Plot mem usage of a given PID
```
cargo run -- -n 0.1 "ps -p ${PID} -o %mem | tail -1"
```

Plot number of python processes running:
```
cargo run -- -n 0.5 "pgrep python | wc -l"
```

Plot number of people in space:
```
cargo run -- -n 1 'echo "curl -s http://api.open-notify.org/astros.json | jq .number" | sh'
```

Plot load of most cpu intensive process
```
cargo run -- -n 0.01 "ps -eo pcpu | sort -n | tail -1"
```

Plot sum of all processes cpu load
```
cargo run -- -n 0.01 'ps -eo pcpu | grep -v CPU | sed "s/  //" | paste -sd "+" - | bc'
```

Plot bitcoin price:
```
cargo run -- -n 5 'curl -s https://api.coindesk.com/v1/bpi/currentprice.json | jq .bpi.EUR.rate_float'
```

Plot polynomial:
```
cargo run -- -n 1 'echo "x=$(($(date +%s) % 30 - 15)); echo $(($x * $x * $x + $x * $x + $x))" | sh'
```

```
cargo run -- -n 1 'echo "x=$(($(date +%s) % 30 - 15)); echo $(($x * $x + $x))" | sh'
```
