#!/bin/bash

cargo build --release 
sudo rm -rf /usr/local/bin/fstore
sudo mv target/release/fstore /usr/local/bin



