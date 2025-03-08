#! /bin/bash

age=9

if [ $age -eq 10 ]; then
  echo "You are 10 years old"
else
  echo "error"
fi

if [ $age -ge 10 ]; then
  echo "You are 10 years old"
fi

if [ $age -le 10 ]; then
  echo "You are 10 years old"
fi

if (($age < 10)); then
  echo "You are 11 years old"
elif (($age == 10)); then
  echo "You are 10 years old"
else
  echo "You are 9 years old"
fi
