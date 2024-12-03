#! /bin/bash

val=0

while IFS="" read -r p || [ -n "$p" ]
do
  matches=$(echo "$p" | grep -Eo "mul\([0-9]+,[0-9]+\)")

  nums=($(echo "$matches" | grep -Eo "[0-9]+"))

  l=${#nums[@]}
  for ((i = 0; i < $l -1; i += 2)); do
    p=$((${nums[$i]} * ${nums[$i + 1]}))
    # echo "${nums[$i]} ${nums[$i + 1]}: $p"
    val=$((val + p))
  done
  
  
done < inputs.txt

echo $val