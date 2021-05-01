#!/bin/bash

if [ "$1" = "-l" ]; then
  if [ "$2" = "/" ]; then
    echo "It's dangerous."
    exit
  fi
  if [ "$2" = "" ]; then
    exit
  fi
  while true
  do
    pid=`lsof -t $2`
    if [ "$pid" != "" ]; then
      kill $pid
    fi
    if [ "$3" != "" ]; then
      sleep $3
    fi
  done
  exit
fi

if [ "$1" = "/" ]; then
  echo "It's dangerous."
  exit
fi
if [ "$1" = "" ]; then
  exit
fi
pid=`lsof -t $1`
if [ "$pid" != "" ]; then
  kill $pid
fi
