#!/bin/bash

cargo build --release 
    sudo rm -rf /usr/local/bin/store
sudo mv target/release/store /usr/local/bin



