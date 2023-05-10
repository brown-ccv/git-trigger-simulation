#!/usr/bin/env bash

git checkout devel
git pull origin devel; echo "git pull exit code: ${?}"

NUM_COMMITS_TO_CHECK=50

# Get the latest commit hash, and check if we've already built from that commit 
# LAST_BUILD_COMMIT will be the empty string if we haven't built from this commit 
LAST_BUILD_COMMIT=`git --no-pager log --pretty=oneline -n $NUM_COMMITS_TO_CHECK | grep build: | head -n 1 | cut -d ' ' -f 1`
grep -q $LAST_BUILD_COMMIT ./logs/previous_build_hashes.txt
GREP_EXIT_STATUS=$? # get grep exit status, which will be `0` if it found LAST_BUILD_COMMIT 

echo -e "\n\nThe last commit hash is: $LAST_BUILD_COMMIT\n\n" 

if [ $GREP_EXIT_STATUS -ne 0 ]; then
    echo -e "We need to rebuild!!\n\n"
    sbatch submit.sh
    git --no-pager log --pretty=oneline -n $NUM_COMMITS_TO_CHECK | grep build: | head -n 1 | cut -d ' ' -f 1 >> ./logs/previous_build_hashes.txt
else
    echo "No need to rebuild; the last commit has already been built."
fi
