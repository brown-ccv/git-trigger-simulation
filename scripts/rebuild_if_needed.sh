#!/usr/bin/env bash

git checkout devel
git pull origin devel

NUM_COMMITS_TO_CHECK=20

# Get the latest commit hash, and check if we've already built from that commit 
# LASTHASH will be the empty string if we haven't built from this commit 
LAST_BUILD_HASH=`git --no-pager log --pretty=oneline -n $NUM_COMMITS_TO_CHECK | grep build: | head -n 1 | cut -d ' ' -f 1 | xargs -I {} grep {} ./logs/previous_build_hashes.txt`

echo $LAST_BUILD_HASH 

if [ -z "$LAST_BUILD_HASH" ]; then
    echo "We need to rebuild!!"
    sbatch submit.sh
    git --no-pager log --pretty=oneline -n $NUM_COMMITS_TO_CHECK | grep build: | head -n 1 | cut -d ' ' -f 1 >> ./logs/previous_build_hashes.txt
else
    echo "No need to rebuild; the last commit has already been built."
fi
