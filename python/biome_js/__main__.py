import os
import sys
import sysconfig


# Based on Ruff's find_ruff_bin(): https://github.com/astral-sh/ruff/blob/main/python/ruff/__main__.py
def find_biome_bin() -> str:
    """Return the Biome binary path."""

    biome_exe = "biome" + (sysconfig.get_config_var("EXE") or "")

    scripts_path = os.path.join(sysconfig.get_path("scripts"), biome_exe)
    if os.path.isfile(scripts_path):
        return scripts_path

    if sys.version_info >= (3, 10):
        user_scheme = sysconfig.get_preferred_scheme("user")
    elif os.name == "nt":
        user_scheme = "nt_user"
    elif sys.platform == "darwin" and getattr(sys, "_framework", None):
        user_scheme = "osx_framework_user"
    else:
        user_scheme = "posix_user"

    user_path = os.path.join(
        sysconfig.get_path("scripts", scheme=user_scheme), biome_exe
    )
    if os.path.isfile(user_path):
        return user_path

    # Search in `bin` adjacent to package root (as created by `pip install --target`).
    pkg_root = os.path.dirname(os.path.dirname(__file__))
    target_path = os.path.join(pkg_root, "bin", biome_exe)
    if os.path.isfile(target_path):
        return target_path

    raise FileNotFoundError(scripts_path)


if __name__ == "__main__":
    biome = find_biome_bin()
    os.execvp(biome, [biome, *sys.argv[1:]])
