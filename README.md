# Introduction 

This repository is meant to serve as a proof-of-concept and an example of how to use cron jobs, Git, and GitHub to trigger the automated running of a Slurm job. In essence, we want to trigger new build and simulation runs when there is a new commit to the `devel` branch of this repository. More specifically, if we want to trigger a build and simulation run, we do so by making a commit to the `devel` branch with the commit message starting with the following string: `build:`. 

In this particular example, we're using Rust to build a small program that generates Collatz sequences starting from a random number between 1 and 100-million. Obviously, this can be replaced by any script or other program. This would be done by editing the `submit.sh` script, which we use for submitting jobs to Slurm.


# Dependencies 

This example is meant to be used on the Oscar supercomputer at Brown. However, it should also work on a machine with the following software:

  1. Slurm
  2. Git
  3. A module system like Lmod or Tcl/C Environment Modules
  4. Rust 1.69.0

# Instructions
We're going to begin by cloning this repository. We will then proceed to make a small change to the repo, and then commit that change to the `devel` branch and push that to GitHub. After that, we will manually run the `scripts/rebuild_if_needed.sh` script. This should trigger a build and model run executed by Slurm. Finally, we move on to set up a cron job that will run the `scripts/rebuild_if_needed.sh` script every 15 minutes.


  1. Log on to Oscar and open a terminal session.
  
  2. Clone this repo using the command below: 
  ```
  git clone https://github.com/brown-ccv/git-trigger-simulation.git
  ```

  3. Make a small edit to the `README.md` file using the commands below: 
  ```
  cd git-trigger-simulation
  git checkout devel
  echo 'foo' >> README.md
  ```

  4. Commit the changes to the `devel` branch and push them back to GitHub using the commands below:
  ```
  git add -A
  git commit -m "build: updated the readme file"
  git push origin devel
  ```

## Manually Trigger a Build and Simulation
Now that we have pushed changes to the `devel` branch, we can manually trigger a build by running the following command from the top-level of this repo on Oscar. 

```
./scripts/rebuild_if_needed.sh
```


