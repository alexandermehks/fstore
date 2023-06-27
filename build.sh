#!/bin/bash

cargo build --release 
sudo mv target/release/store /usr/local/bin

