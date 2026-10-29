import toml

def bumpver(path: str):
    di = toml.load(f"./{path}/Cargo.toml")

    original_version_str = di["package"]["version"]

    version = list(map(int, original_version_str.split(".")))

    version[-1] += 1

    version_str = ".".join(map(str, version))

    with open(f"./{path}/Cargo.toml") as file:
        src = file.read()

    with open(f"./{path}/Cargo.toml", "w") as file:
        file.write(src.replace(original_version_str, version_str))

bumpver("xsc-cli")
bumpver("xsc-lsp")
