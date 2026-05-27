import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# Fix string replacement bug when renaming an assembly multiple times
target = """
      updatedModel.joints = model.joints.map(j => {
        if (groupJoints.some(gj => gj.name === j.name)) {
          return { ...j, name: j.name.replace(oldName, newName), parent_name: j.parent_name ? j.parent_name.replace(oldName, newName) : "Root" };
        }
        return { ...j, parent_name: j.parent_name ? j.parent_name.replace(oldName, newName) : "Root" };
      });
"""

repl = """
      updatedModel.joints = model.joints.map(j => {
        if (groupJoints.some(gj => gj.name === j.name)) {
          // Only replace prefix/base string exactly if it matches
          const nameRegex = new RegExp(`^${oldName}(_.*)?$`, 'i');
          const newJName = j.name.replace(nameRegex, (match, suffix) => `${newName}${suffix || ""}`);
          
          const pNameRegex = new RegExp(`^${oldName}(_.*)?$`, 'i');
          const newPName = j.parent_name ? j.parent_name.replace(pNameRegex, (match, suffix) => `${newName}${suffix || ""}`) : "Root";
          
          return { ...j, name: newJName, parent_name: newPName };
        }
        
        const pNameRegex = new RegExp(`^${oldName}(_.*)?$`, 'i');
        return { ...j, parent_name: j.parent_name ? j.parent_name.replace(pNameRegex, (match, suffix) => `${newName}${suffix || ""}`) : "Root" };
      });
"""

content = content.replace(target, repl)
with open(path, "w") as f:
    f.write(content)
print("Rename logic fixed")
