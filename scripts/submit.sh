#!/bin/bash 

#SBATCH --time=10:00
#SBATCH --nodes=1
#SBATCH --cpus-per-task=1 
#SBATCH --partition=batch
#SBATCH --mem=4G
#SBATCH --job-name collatz-sim
#SBATCH --output logs/collatz-sim-%j.out
#SBATCH --error logs/collatz-sim-%j.out
#SBATCH --constraint cascade


module load rust/1.64.0

./script/rebuild_if_needed.sh
