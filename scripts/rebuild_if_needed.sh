#!/usr/bin/env bash


git checkout devel
git pull origin devel

# Get the latest commit hash, and check if we've already built from that commit 
LASTHASH=`git --no-pager log --pretty=oneline -n1 | grep 'build:' | cut -d' ' -f 1 | xargs -I {} grep {} ./logs/previous_build_hashes.txt`

# LASTHASH will be the empty string if we haven't built from this commit 
echo $LASTHASH

if [ -z "$LASTHASH" ]; then
    echo "We need to rebuild!!"
   
    sbatch submit.sh
    
    git --no-pager log --pretty=oneline -n1 | cut -d' ' -f 1 >> ./logs/previous_build_hashes.txt
else
    echo "No need to rebuild; the last commit has already been built."
fi
