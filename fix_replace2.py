import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# Fix the mesh/marker replacement logic
target = """
      updatedModel.meshes = model.meshes.map(m => ({ ...m, parent_name: m.parent_name ? m.parent_name.replace(oldName, newName) : "Root" }));
      updatedModel.markers = model.markers.map(m => ({ ...m, parent_joint: m.parent_joint.replace(oldName, newName) }));
"""

repl = """
      updatedModel.meshes = model.meshes.map(m => {
         const pNameRegex = new RegExp(`^${oldName}(_.*)?$`, 'i');
         return { ...m, parent_name: m.parent_name ? m.parent_name.replace(pNameRegex, (_match, suffix) => `${newName}${suffix || ""}`) : "Root" };
      });
      updatedModel.markers = model.markers.map(m => {
         const pNameRegex = new RegExp(`^${oldName}(_.*)?$`, 'i');
         return { ...m, parent_joint: m.parent_joint.replace(pNameRegex, (_match, suffix) => `${newName}${suffix || ""}`) };
      });
"""

content = content.replace(target, repl)

with open(path, "w") as f:
    f.write(content)
print("Mesh/Marker rename logic fixed")
