#! /bin/bash

age=10

if [[ $age -gt 18 && $age -lt 40 ]]; then
  echo "You are in your 20s"
else
  echo "You are not in your 20s"
fi

if [[ $age -gt 18 || $age -lt 40 ]]; then
  echo "You are in your 20s"
else
  echo "You are not in your 20s"
fi
