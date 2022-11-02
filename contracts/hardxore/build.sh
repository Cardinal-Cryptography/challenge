#!/bin/bash

set -e 

cargo +nightly contract build --release

cp target/ink/metadata.json ../../metadata/hardxore.json