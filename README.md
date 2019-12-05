# LIve PLot

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
