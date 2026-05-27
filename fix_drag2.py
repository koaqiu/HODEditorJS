import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

target = """          onContextMenu={(e) => {
            const info = getWeaponGroupInfo(baseName + "_Position") || getWeaponGroupInfo(baseName + "_Heading") || getWeaponGroupInfo(baseName);
            handleContextMenu(e, baseName, info?.type || "weapon_group");
          }}
          style={{"""

repl = """          onContextMenu={(e) => {
            const info = getWeaponGroupInfo(baseName + "_Position") || getWeaponGroupInfo(baseName + "_Heading") || getWeaponGroupInfo(baseName);
            handleContextMenu(e, baseName, info?.type || "weapon_group");
          }}
          draggable="true"
          onDragStart={(e) => {
            e.stopPropagation();
            const info = getWeaponGroupInfo(baseName + "_Position") || getWeaponGroupInfo(baseName + "_Heading") || getWeaponGroupInfo(baseName);
            handleDragStart(e, baseName, info?.type || "weapon_group");
          }}
          style={{"""

content = content.replace(target, repl)

with open(path, "w") as f:
    f.write(content)
print("Drag fixed")
