#! /bin/bash

val=0
enabled=1

while IFS="" read -r p || [ -n "$p" ]
do
  matches=($(echo "$p" | grep -Eo "mul\(\d+,\d+\)|do\(\)|don\'t\(\)"))

  for m in "${matches[@]}";
  do
    if [ $m = "don't()" ]; then
      enabled=0
    fi
    if [ $m = "do()" ]; then
      enabled=1
    fi

    if [[ $m = "mul("* && $enabled = 1 ]]; then
      nums=($(echo "$m" | grep -Eo "[0-9]+"))
      l=${#nums[@]}

      for ((i = 0; i < $l -1; i += 2)); do
        val=$((val + $((${nums[$i]} * ${nums[$i + 1]}))))
      done
    fi
  done
done < inputs.txt

echo $val