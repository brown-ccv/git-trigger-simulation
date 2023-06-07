# Introduction 

This repository is meant to serve as a proof-of-concept and an example of how to use cron jobs, Git, and GitHub to trigger the automated running of a SLURM job. 

In essence, we want to trigger new build and simulation runs when there is a new commit merged into the `devel` branch of this repository. More specifically, if we want to trigger a build and simulation run, we do so by creating a "feature branch" and making a commit that is merged into the `devel` branch with the commit message starting with the following string: `"build:"`. 

In this particular example, we're using Rust to build a small program that generates Collatz sequences starting from a random number between 1 and 100-million. Obviously, this can be replaced by any script or other program. This would be done by editing the `submit.sh` script, which we use for submitting jobs to SLURM.


# Dependencies 

This example is meant to be used on the Oscar supercomputer at Brown. However, it should also work on a machine with the following software:

  1. SLURM
  2. Git
  3. A module system like PyModules, Lmod, or Tcl/C Environment Modules
  4. Rust 1.69.0

# Instructions

**Note:** The instructions that follow pre-suppose that the user has access to the `brown-ccv` GitHub organization, and sufficient permisions on this repo. If that is not the case, then one should begin by first forking this repo to your personal GitHub or your organization's GitHub. 

We begin by cloning this repo to Oscar. We will then create a new feature branch called `add-small-file` and proceed to make a small change. Next, we commit the change to our new branch and push to GitHub. After that, we submit a Pull Request on GitHub from `add-small-file` to the `devel` branch. Once that PR is merged into the `devel` branch, we will manually run the `scripts/rebuild_if_needed.sh` script. This should trigger a build and model run executed by SLURM. 

  #### 1. Log on to Oscar and open a terminal session.
  
  #### 2. Clone this repo using the command below: 
  ```
  git clone git@github.com:brown-ccv/git-trigger-simulation.git 
  ```

  #### 3. Create a small file using the commands below: 
  ```
  cd git-trigger-simulation
  git checkout -b add-small-file 
  echo 'foo' > deleteme.txt 
  ```

  #### 4. Commit the changes to the `add-small-file` branch and push them back to GitHub using the commands below:
  ```
  git add -A
  git commit -m "build: add new deleteme file"
  git push origin add-small-file 
  ```

  #### 5. Go to GitHub and create a Pull Request from the `add-small-file` branch to the `devel` branch

  #### 6. Merge the Pull Request 

## Manually Trigger a Build and Simulation
Now that we have merged our changes to the `devel` branch, we can manually trigger a build by running the following command from the top-level of this repo on Oscar. 

```
./scripts/rebuild_if_needed.sh
```

The above script will perform several operations. First, it switches branches to the `devel` branch of our repo. Next, it pulls in the changes (if any) that were made to the `devel` branch on the remote repo. Finally, it checks whether there have been any _new_ commits merged in to `devel` that have included `"build:"` at the start of the commit message. And if there _have_ been new commits that include `"build:"` as part of the commit message, the script above proceeds to re-compile our code, and run the resulting executable using SLURM.

### Importance of `logs/previous_build_hashes.txt` File
In the above section, you might have noticed that we only rebuild and run our simulation when there are **new** commits merged into the `devel` branch. The script above knows that a commit is _new_ if that commit's hash does not appear in the `logs/previous_build_hashes.txt` file. Therefore, this text file is essential to the proper functioning of this repo's building behavior. Before any new build and simulation takes place, our script above checks if we've already built and run that commit's code; and if we have, it doesn't rebuild or re-run the simulation. But if we haven't built and run from the commit, then we proceed to do so, and once complete, we add the commit's hash to the `logs/previous_build_hashes.txt` file.  

## Automating with Cron Job
Finally, we move on to set up a cron job that will run the `scripts/rebuild_if_needed.sh` script every 5 minutes.

This is done by connecting to Oscar's login node, and then opening your crontab editor using `crontab -e`, and then placing something like the following line in that file:

```
*/5 * * * * /users/pstey/projects/git-trigger-simulation/scripts/rebuild_if_needed.sh >> /users/pstey/projects/git-trigger-simulation/cron.log 2>&1
```

Note that the path `/users/pstey/projects...` above will need to be changed to reflect your own account on Oscar and the path to the directory.
