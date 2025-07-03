#! /bin/bash

arg=("$@")
len=$#
echo "Number of arguments: $len"
echo "Arguments: ${arg[1]}"

for i in ${arg[@]}; do
  echo $i
done
