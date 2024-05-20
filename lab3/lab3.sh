#!/bin/bash


read -p "Укажите директорию: " directory
staroe="броколли"
novoe="капусту"

touch otvet.txt

for file in "$directory"/*.txt; do
    if grep -q "$staroe" "$file"; then
        sed -i "s/$staroe/$novoe/g" "$file"
        echo "$file" >> modified_files.txt
    fi
done
