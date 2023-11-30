#!/bin/bash

for filename in ./graphs/*.gph; do 
    ./target/release/ex06-492253-sanchez-torres-andres-alam "$filename" 2 45
done