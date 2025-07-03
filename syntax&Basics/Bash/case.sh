#! /bin/bash

echo "escoge entre el valor 1 o 2:"
read valor

case $valor in
1)
  echo "escogiste el valor 1"
  ;;
2)
  echo "escogiste el valor 2"
  ;;
*)
  echo "escogiste un valor diferente"
  ;;
esac
