import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# Update handleRenameNode to accept all point_group types instead of just "weapon_group"
target = 'if (type === "weapon_group") {'
repl = 'if (type.endsWith("_group")) {'

content = content.replace(target, repl)

with open(path, "w") as f:
    f.write(content)

path2 = "src/App.tsx"
with open(path2, "r") as f:
    content2 = f.read()
    
target2 = 'if (type === "weapon_group") {'
repl2 = 'if (type.endsWith("_group")) {'
content2 = content2.replace(target2, repl2)

with open(path2, "w") as f:
    f.write(content2)

print("Fixed rename and delete cascades for groups")
