import pathlib
import subprocess


def install():
    current_dir = pathlib.Path.cwd()
    cprs_cli_dir = current_dir / "cprs_cli"
    if not cprs_cli_dir.exists():
        raise RuntimeError("Incorrect directory")

    # install and setup script
    subprocess.run(["cargo", "install", "--path", "cprs_cli"])
    subprocess.run(["cprs_cli", "setup"])

    # append init file to zsh
    zsh_path = pathlib.Path.home() / ".zshrc"
    append_string = 'eval "$(cprs_cli init)"'
    with open(zsh_path, "r") as f:
        lines = f.readlines()
    appended = any(append_string in line for line in lines)
    if not appended:
        with open(zsh_path, "a+") as f:
            f.write(f"{append_string}\n")


if __name__ == "__main__":
    install()
