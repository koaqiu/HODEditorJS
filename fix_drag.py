import re

path = "src/App.tsx"
with open(path, "r") as f:
    content = f.read()

target = '    } else if (nodeType === "navlight") {'

insertion = """    } else if (nodeType.endsWith("_group")) {
      const groupJoints = model.joints.filter(j => j.name.toLowerCase().startsWith(nodeName.toLowerCase() + "_") || j.name.toLowerCase() === nodeName.toLowerCase());
      const rootGroupJoints = groupJoints.filter(j => {
        const hasParentInGroup = groupJoints.some(other => other.name === j.parent_name);
        return !hasParentInGroup;
      });

      // Prevent cyclical parenting for assemblies
      const isDescendant = (parent: string, child: string): boolean => {
        if (parent === child) return true;
        const currentJoint = model.joints.find(j => j.name === parent);
        if (!currentJoint || !currentJoint.parent_name || currentJoint.parent_name === "Root") return false;
        return isDescendant(currentJoint.parent_name, child);
      };

      for (const root of rootGroupJoints) {
         if (isDescendant(newParentName, root.name)) {
            setErrorMsg("Error: Cyclical parenting is not allowed (cannot parent an assembly under its own descendants).");
            return;
         }
      }

      updatedModel.joints = model.joints.map((joint) => {
        if (rootGroupJoints.some(rj => rj.name === joint.name)) {
          return { ...joint, parent_name: newParentName };
        }
        return joint;
      });
"""

content = content.replace(target, insertion + target)

with open(path, "w") as f:
    f.write(content)

path2 = "src/components/HierarchyTree.tsx"
with open(path2, "r") as f:
    content2 = f.read()

# Need to make AssemblyNode draggable
# Search for renderAssemblyNode's list-item div:
# <div
#   className={`list-item ${isSelected ? "active" : ""}`}
#   onClick={() => setSelectedNode({ type: "weapon_group", name: baseName })}
#   onContextMenu={(e) => handleContextMenu(e, baseName, "weapon_group")}
#   style={{

target_div = """        <div
          className={`list-item ${isSelected ? "active" : ""}`}
          onClick={() => setSelectedNode({ type: "weapon_group", name: baseName })}
          onContextMenu={(e) => handleContextMenu(e, baseName, "weapon_group")}"""

insertion_div = """        <div
          className={`list-item ${isSelected ? "active" : ""}`}
          onClick={() => setSelectedNode({ type: "weapon_group", name: baseName })}
          onContextMenu={(e) => handleContextMenu(e, baseName, "weapon_group")}
          draggable="true"
          onDragStart={(e) => {
            e.stopPropagation();
            handleDragStart(e, baseName, "weapon_group"); // We just use weapon_group as the general type for assembly dragging, or dynamically type it
          }}"""

# Actually, the baseName could be capture point. The type passed to setSelectedNode is hardcoded as "weapon_group".
# Let's fix that too! We should use the actual type.
target_div = """        <div
          className={`list-item ${isSelected ? "active" : ""}`}
          onClick={() => setSelectedNode({ type: "weapon_group", name: baseName })}
          onContextMenu={(e) => handleContextMenu(e, baseName, "weapon_group")}"""

insertion_div = """        <div
          className={`list-item ${isSelected ? "active" : ""}`}
          onClick={() => {
            const info = getWeaponGroupInfo(baseName + "_Position") || getWeaponGroupInfo(baseName + "_Heading") || getWeaponGroupInfo(baseName);
            setSelectedNode({ type: info?.type || "weapon_group", name: baseName });
          }}
          onContextMenu={(e) => {
            const info = getWeaponGroupInfo(baseName + "_Position") || getWeaponGroupInfo(baseName + "_Heading") || getWeaponGroupInfo(baseName);
            handleContextMenu(e, baseName, info?.type || "weapon_group");
          }}
          draggable="true"
          onDragStart={(e) => {
            e.stopPropagation();
            const info = getWeaponGroupInfo(baseName + "_Position") || getWeaponGroupInfo(baseName + "_Heading") || getWeaponGroupInfo(baseName);
            handleDragStart(e, baseName, info?.type || "weapon_group");
          }}"""

content2 = content2.replace(target_div, insertion_div)

# Also fix the context menu delete/rename types to be dynamic
# In the right click context menu, `contextMenu.type` is correct because we passed the correct type to it now.

with open(path2, "w") as f:
    f.write(content2)

print("Draggable fixes added")
