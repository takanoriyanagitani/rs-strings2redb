#!/bin/bash

set -u

wsm='./strings2redb.wasm'

outname='output.redb'
hostd="${PWD}"
guestd='/guest.d'

out4wasi="${guestd}/${outname}"

input1(){
  echo helo
  echo wrld
  echo hello
  echo world
}

input2(){
  seq 1 1048576
}

input3(){
  seq 1 1048576 |
    sort -n -r
}

run_wasi(){
  cat /dev/stdin |
    wasmtime \
      run \
      --dir "${hostd}::${guestd}" \
      --env ENV_OUTPUT_PATH="${out4wasi}" \
      "${wsm}"
}

echo simple strings
time input1 | run_wasi
echo

echo more strings
time input2 | run_wasi
echo

echo more strings, reverse order
time input3 | run_wasi
echo
