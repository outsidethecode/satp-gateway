#!/bin/bash

buf build ../protos --as-file-descriptor-set -o descriptor_set.pb --verbose
envoy -c envoy.yaml
