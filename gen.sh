#!/usr/bin/env bash

if [ $# -eq 0 ]
  then
    echo "Please specify a day."
	exit 1
fi

cp -r "./.template/" "./day_$1"
sed -i "s/^]/\t\"day_$1\",\n\]/" "./Cargo.toml"
sed -i "s/day_x/day_$1/" "./day_$1/Cargo.toml"
