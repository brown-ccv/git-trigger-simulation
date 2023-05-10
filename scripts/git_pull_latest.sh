#!/usr/bin/env bash

git checkout devel
git pull origin devel; echo "git pull exit code: ${?}"

