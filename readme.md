# cprs

This tool is heavily inspired by
[rust-competitive-helper](https://github.com/rust-competitive-helper/rust-competitive-helper)
and
[rust-bundler-cp](https://github.com/Endle/rust-bundler-cp)

> [!IMPORTANT]  
> **I wrote this for my specific need, it only works in Linux and zsh, use at your own risk**

## Install

To install, you must have `python`, `rust` pre-installed.

Clone the repo. From the repo root, run the install script:

```bash
python install.py
```

It will:

- Install the `cprs_cli` cmd
- Generate default config file
- Add init command to `~/.zshrc`

### Usage

- To download problems, [competitive-companion](https://github.com/jmerle/competitive-companion) is needed. Invoke the command below to listen to the addon:

  ```bash
  cprs listen
  ```

- To show recent downloaded problems:

  ```bash
  cprs list 5
  ```

  Sample output:

  ```bash
  ❯ cprs list 5
  Showing 5 latest tasks:
  Id  0: Task `G2. Division + LCP (hard version)`, from `https://codeforces.com/problemset/problem/1968/G2`
  Id  1: Task `Z Algorithm`, from `https://judge.yosupo.jp/problem/zalgorithm`
  Id  2: Task `B2. Exact Neighbours (Medium)`, from `https://codeforces.com/problemset/problem/1970/B2`
  Id  3: Task `G2. Min-Fund Prison (Medium)`, from `https://codeforces.com/problemset/problem/1970/G2`
  Id  4: Task `F. Non-academic Problem`, from `https://codeforces.com/contest/1986/problem/F`
  ```

- To change directory into specific problem directory id:

  ```bash
  cprs cd 0
  ```

- To run test and submit (It actually only copies bundled source file into clipboard)

  ```bash
  cprs submit
  ```

### Bonus

If you want to modify to works with other shells or other platforms, please see:

- [init.rs](./cprs_cli/src/cli/init.rs) which creates `zsh` function
- [install.py](./install.py) for modifying `~/.zshrc`
