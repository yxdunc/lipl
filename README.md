# LIve PLot
[![Actions Status](https://github.com/yxdunc/lipl/workflows/Tests_OSX/badge.svg)](https://github.com/yxdunc/lipl/actions)
[![Actions Status](https://github.com/yxdunc/lipl/workflows/Tests_Linux/badge.svg)](https://github.com/yxdunc/lipl/actions)

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

![main example screenshot](../media/screen_shots/lipl.gif?raw=true)


## Install

### homebrew (osx)

```
brew tap yxdunc/tools
brew install lipl
```

## Arguments

### positional argument:

A string containing a bash command.

If the bash command returns a number a plot will be shown (ex: `ls -1 | wc -l`)

If the bash command returns anything else the command shows the output and 
follow the same behaviour as [watch](https://en.wikipedia.org/wiki/Watch_(Unix))

### named arguments:
`-n/--refresh-rate`: the refresh rate in seconds

`-t/--target`: a target value that will be used to show a progress bar based on
 the command outputs. A simple linear regression is used.

`-l/--history-len`: the number of results from the given command that are stored
 and plotted.  

`--show-regression-line`: when true shows the regression line used to compute
the ETA.

`--show-target-line`: when true shows an horizontal line representing the target
value.

## Sample usages:

🗃Plot number of files in `/tmp`
```
lipl -n 0.5 "ls -1 /tmp | wc -l"
```

♨️ Plot cpu usage of a given PID
```
lipl -n 0.1 "ps -p ${PID} -o %cpu | tail -1"
```

🗂Plot mem usage of a given PID
```
lipl -n 0.1 "ps -p ${PID} -o %mem | tail -1"
```

🐍Plot number of python processes running
```
lipl -n 0.5 "pgrep python | wc -l"
```

👩‍🚀Plot number of people in space
```
lipl -n 1 'echo "curl -s http://api.open-notify.org/astros.json | jq .number" | sh'
```

🔥Plot load of most cpu intensive process
```
lipl -n 0.01 "ps -eo pcpu | sort -n | tail -1"
```

💻Plot sum of all processes cpu load
```
lipl -n 0.01 'ps -eo pcpu | grep -v CPU | sed "s/  //" | paste -sd "+" - | bc'
```

⛓Plot bitcoin price
```
lipl -n 5 'curl -s https://api.coindesk.com/v1/bpi/currentprice.json | jq .bpi.EUR.rate_float'
```

🎢Plot polynomial
```
lipl -n 1 'echo "x=$(($(date +%s) % 30 - 15)); echo $(($x * $x * $x + $x * $x + $x))" | sh'
```

```
lipl -n 1 'echo "x=$(($(date +%s) % 30 - 15)); echo $(($x * $x + $x))" | sh'
```
