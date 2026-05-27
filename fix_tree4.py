import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# Add duplicate checking into handleAddNode
# Find handleAddNode start
start_add_node = content.find("const handleAddNode = () => {")

insertion = """
    const checkDuplicate = (n: string) => {
      if (model.joints.some(j => j.name === n)) return true;
      if (model.meshes.some(m => `${m.name}_lod_${m.lod}` === n || m.name === n)) return true;
      if (model.nav_lights.some(nv => nv.name === n)) return true;
      if (model.markers.some(m => m.name === n)) return true;
      if (model.engine_burns?.some(b => b.name === n)) return true;
      if (model.engine_glows?.some(g => g.name === n)) return true;
      if (model.engine_shapes?.some(s => s.name === n)) return true;
      if (model.collision_meshes?.some(c => c.name === n)) return true;
      if (model.dockpaths?.some(d => d.name === n)) return true;
      return false;
    };
"""

target = "let updatedModel = { ...model };"
idx = content.find(target, start_add_node)
content = content[:idx] + insertion + """
    if (checkDuplicate(name)) {
      window.alert(`A node with the name "${name}" already exists! Please choose a unique name.`);
      return;
    }
    
    """ + content[idx:]

# Also fix the duplicate name detection in handleRenameNode to include _lod_ meshes
# We'll just run a regex on handleRenameNode's checkDuplicate
content = re.sub(
    r'if \(model\.meshes\.some\(m => m\.name === n\)\) return true;',
    r'if (model.meshes.some(m => `${m.name}_lod_${m.lod}` === n || m.name === n)) return true;',
    content
)

# And ensure "type !== weapon_group" points are not renamed/deleted from their nodes but only from parent Assembly
# To achieve this:
# In handleContextMenu, block if it's an assembly subnode.
# "User shouldn't be able to drag or rename the joint nodes for Assemblies"
# So let's block the context menu for assembly joints, and block dragging.
# Actually, the user says "User shouldn't be able to drag or rename the joint nodes for Assemblies (can only rename and delete via the assembly representation node itself, as well as drag the whole assembly)."

with open(path, "w") as f:
    f.write(content)
print("Duplicate checks added")
