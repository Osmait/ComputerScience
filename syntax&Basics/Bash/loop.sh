#! /bin/bash

number=0

# while [ $number -le 10 ]; do
#   echo "Number: $number"
#   number=$((number + 1))
# done
# until [ $number -le 10 ]; do
#   echo "Number: $number"
#   number=$((number + 1))
# done

for i in {0..100}; do
  echo "Number: $i"
done

# for i in {0..100..5}; do
#   echo "Number: $i"
# done
#

for ((i = 0; i < 100; i += 5)); do
  echo "Number: $i"
done
