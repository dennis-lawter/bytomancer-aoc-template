# Purpose
Using this repository template,
you can develop your own solutions for Advent of Code in Rust.

# Environment
## NixOS Flake
My recommendation would be to launch a persistent editing environment, using either:
- `nix develop --command zellij` for neovim or other command line development
- `nix develop --command codium .` for a vscode style environment

The nix flake includes a Lazyvim default setup of neovim!
If you wish to use your system's neovim, be sure to remove this from the flake.

## Other Systems
You will need the following dependencies:
- https://rustup.rs/
- `cargo install bacon`

# Setup Guide
- Copy `.env.example` to `.env`
- Open https://www.adventofocode.com/ and logout then login (cookies last 30 days)
- Press f12 in your browser of choice, open storage, and copy the value from your `session` cookie.
- Edit your .env file.
- Paste the cookie value into the `SESSION` env var.
- Set the `USERAGENT` to your repository's URL (this is by request from Eric Wastl).
- Set the `YEAR` to the year you're developing against.

# Daily Solving
I plan to improve this somehow in the future with a wrapped runner...
Until then, follow these steps.

## Generate code
We'll use day 1 as an example, but replace 01 with the current day you're solving...
- Generate the daily first solution file with: `cargo run -- d01s1 -g`
- Generate the dialy second solution file with: `cargo run -- d01s2 -g`
As a note, you can also generate arbitrary endings,
such as `d01s1rev1` for revisions or `d01s1vis` for visualizers, etc,
but you MUST follow the `dXXsY` pattern at the start.

## Writing a solution
You'll notice you now have a file matching the name you gave in `./src/solutions/`.
Open the appropriate solution file, and get to it.

You are provided an example `input()` function which is where you can massage the input into lists, structs, etc.
The function `raw_input()` generates 1 massive string from your input file.
The `input()` function is public and is shared for the `s2` file.

You are also given the `solve()` function.
This function calls `input()` to get your data collected into whatever way you felt best suited the solution.
At the end, you'll see a call to `final_answer()` where the first argument is any displayable value (String, &str, any numeric value...)
Do not edit the 2nd & 3rd arguments to `final_anser()`.

## Live debugging
Be sure to edit `bacon.toml` and replace the `d01s1` and `d01s2` with your current solutions you're working on.
Once setup, simply run `bacon`.
By pressing **1**, you will keep a live update running for solution 1.
When you start up `bacon` it defaults to solution 1.
You can swap to solution 2 by pressing **2**.
Saving your rust source files should auto-compile and run the code without swapping your current solution.
You can apply rustfmt auto formatting to your code at any time by pressing **f** but be sure you have all files saved first.

## Automatic upload
You can attempt to use the framework to submit your answer for you.
I only recommend you do this if you are trying for the leaderboard,
as the website provides better feedback.
Either way, simply run `cargo run -- d01s1 -s` to submit your answer.
AOC has a 60 second delay between submissions,
but this tool will not track time between submissions.
Please respect the AOC website and do not use bacon with the -s flag!

## Using example files
This framework can attempt to parse an example from the problem.
When the page has multiple examples, the file it generates can sometimes not be what you expected.
Either way, you can attempt to use this feature knowing that you may need to manually edit the example file.
To attempt to automatically pull the example and run the code against that,
execute `cargo run -- d01s1 -e`.s
A file should be created in `./_cache/2025/day/1/example.txt` (assuming day 1, 2025).
You can manually edit this file to change the example input file.
Re-running with the `-e` flag will use whatever file is in the correct example path.
You can also edit your `bacon.toml` file to include the `-e` argument to have automatic debugging on the example file.

# Future Updates
This is a port of my initial solving framework from 2023.
I recognize it has some usability flaws, and I'd like to reimagine it in the future.
I imagine an executable wrapper that manages how bacon works would be much more powerful.
The framework would expect you to create (and still help codegen) a cargo submodule...
So you'd generate a day 1 ELF / EXE independent from other days.
The framework would build a custom bacon runner for that day,
and provide jobs for switching between example files,
proper input files,
and even submitting to the website (with a filelock timer to prevent frequent requests).

Until then, this repo should still be a servicable solution for
automating your Advent of Code in Rust experience.
