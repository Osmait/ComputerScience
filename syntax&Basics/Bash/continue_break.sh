#! /bin/bash

for ((i = 0; i < 10; i++)); do
  if [[ $i -eq 3 || $i -eq 4 ]]; then
    continue
  fi

  echo "Number: $i"

  if [ $i -eq 5 ]; then
    break
  fi
done
