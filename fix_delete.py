import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# Update handleDeleteNode for point groups
# Find `} else if (type === "weapon_group") {`
target = '} else if (type === "weapon_group") {'
repl = '} else if (type.endsWith("_group")) {'

content = content.replace(target, repl)

with open(path, "w") as f:
    f.write(content)
print("Delete cascade fixed")
