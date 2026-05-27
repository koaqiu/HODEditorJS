import re

path = "docs/ui-source-of-truth/02-node-creation-and-types.md"
with open(path, "r") as f:
    content = f.read()

content = content.replace(
    "UI Tree should show those structures as a main Assembly node (not specifically a HOD node) to represent the weapon subtree",
    "UI Tree should show those structures as a main Assembly node (not specifically a HOD node) to represent the assembly subtree"
)
content = content.replace(
    "Duplicate names are rejected by rename logic and should be avoided by creation flows.",
    "Duplicate names are strictly restricted across the entire tree during node creation and renaming. The UI enforces uniqueness for all newly created Nodes or Meshes to prevent internal naming collisions."
)

with open(path, "w") as f:
    f.write(content)

path2 = "docs/ui-source-of-truth/09-node-type-rulings.md"
with open(path2, "r") as f:
    content2 = f.read()

content2 = content2.replace(
    "Direct subnode rename is forbidden.",
    "Direct subnode rename is forbidden (the Context Menu is natively blocked for these subnodes)."
)
content2 = content2.replace(
    "Direct subnode delete is forbidden.",
    "Direct subnode delete is forbidden (the Context Menu is natively blocked for these subnodes)."
)

with open(path2, "w") as f:
    f.write(content2)

print("Docs updated")
