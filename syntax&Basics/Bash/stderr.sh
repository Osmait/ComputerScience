#!/bin/bash

echo "inicio del script" >>file.txt
ls -l 1>>file.txt 2>>error.txt
echo "fin del script" >>file.txt

ls -l >&file2.txt
