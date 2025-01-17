#!/bin/bash

http-server &
BACKGROUND_PID=$!
open http://localhost:8080/miner.html
wait $BACKGROUND_PID