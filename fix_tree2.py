import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# Fix the duplicate attributes
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, jointName, "joint"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, jointName, "joint"\)\}', r'onContextMenu={(e) => handleContextMenu(e, jointName, "joint")}', content)
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, marker\.name, "marker"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, marker\.name, "marker"\)\}', r'onContextMenu={(e) => handleContextMenu(e, marker.name, "marker")}', content)
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, meshKey, "mesh"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, meshKey, "mesh"\)\}', r'onContextMenu={(e) => handleContextMenu(e, meshKey, "mesh")}', content)
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, nav\.name, "navlight"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, nav\.name, "navlight"\)\}', r'onContextMenu={(e) => handleContextMenu(e, nav.name, "navlight")}', content)
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, burn\.name, "engine_burn"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, burn\.name, "engine_burn"\)\}', r'onContextMenu={(e) => handleContextMenu(e, burn.name, "engine_burn")}', content)
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, glow\.name, "engine_glow"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, glow\.name, "engine_glow"\)\}', r'onContextMenu={(e) => handleContextMenu(e, glow.name, "engine_glow")}', content)
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, shape\.name, "engine_shape"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, shape\.name, "engine_shape"\)\}', r'onContextMenu={(e) => handleContextMenu(e, shape.name, "engine_shape")}', content)
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, col\.name, "collision"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, col\.name, "collision"\)\}', r'onContextMenu={(e) => handleContextMenu(e, col.name, "collision")}', content)
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, path\.name, "dockpath"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, path\.name, "dockpath"\)\}', r'onContextMenu={(e) => handleContextMenu(e, path.name, "dockpath")}', content)
content = re.sub(r'onContextMenu=\{\(e\) => handleContextMenu\(e, baseName, "weapon_group"\)\}\s*onContextMenu=\{\(e\) => handleContextMenu\(e, baseName, "weapon_group"\)\}', r'onContextMenu={(e) => handleContextMenu(e, baseName, "weapon_group")}', content)

with open(path, "w") as f:
    f.write(content)

print("Duplicates fixed")
