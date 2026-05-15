import toml

di = toml.load("./xsc-cli/Cargo.toml")

original_version_str = di["package"]["version"]

version = list(map(int, original_version_str.split(".")))

version[-1] += 1

version_str = ".".join(map(str, version))

with open("./xsc-cli/Cargo.toml") as file:
    src = file.read()

with open("./xsc-cli/Cargo.toml", "w") as file:
    file.write(src.replace(original_version_str, version_str))
