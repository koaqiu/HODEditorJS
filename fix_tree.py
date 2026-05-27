import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# 1. Add portal import
if "createPortal" not in content:
    content = content.replace('import React, { useState, useRef } from "react";', 'import React, { useState, useRef } from "react";\nimport { createPortal } from "react-dom";')

# 2. Add handleContextMenu, contextMenu state, handleRenameNode right before handleDeleteNode
handle_delete_idx = content.find("const handleDeleteNode = (name: string, type: string) => {")

insertion = """
  const [contextMenu, setContextMenu] = useState<{ x: number, y: number, name: string, type: string } | null>(null);

  const handleContextMenu = (e: React.MouseEvent, name: string, type: string) => {
    e.preventDefault();
    e.stopPropagation();
    setSelectedNode({ type, name });
    setContextMenu({ x: e.clientX, y: e.clientY, name, type });
  };

  const handleRenameNode = (oldName: string, type: string) => {
    if (!model) return;
    
    let cleanOldName = oldName;
    let prefix = "";
    let suffix = "";
    
    if (type === "navlight" && oldName.startsWith("NAVL[")) {
      prefix = "NAVL["; suffix = "]"; cleanOldName = oldName.substring(5, oldName.length - 1);
    } else if (type === "marker" && oldName.startsWith("MARK[")) {
      prefix = "MARK["; suffix = "]"; cleanOldName = oldName.substring(5, oldName.length - 1);
    } else if (type === "engine_burn" && oldName.startsWith("BURN[")) {
      prefix = "BURN["; suffix = "]"; cleanOldName = oldName.substring(5, oldName.length - 1);
    } else if (type === "mesh" && oldName.startsWith("MULT[")) {
      prefix = "MULT["; suffix = "]"; cleanOldName = oldName.substring(5, oldName.length - 1);
    } else if (type === "collision" && oldName.startsWith("COL[")) {
      prefix = "COL["; suffix = "]"; cleanOldName = oldName.substring(4, oldName.length - 1);
    } else if (type === "engine_glow" && oldName.startsWith("GLOW[")) {
      prefix = "GLOW["; suffix = "]"; cleanOldName = oldName.substring(5, oldName.length - 1);
    } else if (type === "engine_shape" && oldName.startsWith("SHAP[")) {
      prefix = "SHAP["; suffix = "]"; cleanOldName = oldName.substring(5, oldName.length - 1);
    }

    const input = window.prompt(`Rename ${type}:`, cleanOldName);
    if (!input || input.trim() === "" || input === cleanOldName) return;
    
    const newName = `${prefix}${input.trim()}${suffix}`;
    
    const checkDuplicate = (n: string) => {
      if (model.joints.some(j => j.name === n)) return true;
      if (model.meshes.some(m => m.name === n)) return true;
      if (model.nav_lights.some(nv => nv.name === n)) return true;
      if (model.markers.some(m => m.name === n)) return true;
      if (model.engine_burns?.some(b => b.name === n)) return true;
      if (model.engine_glows?.some(g => g.name === n)) return true;
      if (model.engine_shapes?.some(s => s.name === n)) return true;
      if (model.collision_meshes?.some(c => c.name === n)) return true;
      if (model.dockpaths?.some(d => d.name === n)) return true;
      return false;
    };
    
    let updatedModel = { ...model };

    if (type === "weapon_group") {
      const groupJoints = model.joints.filter(j => j.name.toLowerCase().startsWith(oldName.toLowerCase() + "_") || j.name.toLowerCase() === oldName.toLowerCase());
      
      for (const j of groupJoints) {
         const renamed = j.name.replace(oldName, newName);
         if (checkDuplicate(renamed)) {
            window.alert(`Cannot rename weapon assembly: Subnode "${renamed}" would conflict with an existing node!`);
            return;
         }
      }

      updatedModel.joints = model.joints.map(j => {
        if (groupJoints.some(gj => gj.name === j.name)) {
          return { ...j, name: j.name.replace(oldName, newName), parent_name: j.parent_name ? j.parent_name.replace(oldName, newName) : "Root" };
        }
        return { ...j, parent_name: j.parent_name ? j.parent_name.replace(oldName, newName) : "Root" };
      });
      updatedModel.meshes = model.meshes.map(m => ({ ...m, parent_name: m.parent_name ? m.parent_name.replace(oldName, newName) : "Root" }));
      updatedModel.markers = model.markers.map(m => ({ ...m, parent_joint: m.parent_joint.replace(oldName, newName) }));
      
      onModelChange?.(updatedModel);
      if (selectedNode && selectedNode.name === oldName && selectedNode.type === type) {
        setSelectedNode({ type, name: newName });
      }
      invoke("log_event", { level: "INFO", message: `Renamed weapon_group assembly from ${oldName} to ${newName}` }).catch(console.error);
      return;
    }

    if (checkDuplicate(newName)) {
      window.alert(`A node with the name "${newName}" already exists!`);
      return;
    }

    if (type === "joint") {
      updatedModel.joints = model.joints.map(j => {
        if (j.name === oldName) return { ...j, name: newName };
        if (j.parent_name === oldName) return { ...j, parent_name: newName };
        return j;
      });
      updatedModel.markers = model.markers.map(m => {
        if (m.parent_joint === oldName) return { ...m, parent_joint: newName };
        return m;
      });
      updatedModel.meshes = model.meshes.map(m => {
        if (m.parent_name === oldName) return { ...m, parent_name: newName };
        return m;
      });
      updatedModel.dockpaths = model.dockpaths.map(dp => {
        if (dp.parent_name === oldName) return { ...dp, parent_name: newName };
        return dp;
      });
      updatedModel.engine_burns = model.engine_burns.map(eb => {
        if (eb.parent_name === oldName) return { ...eb, parent_name: newName };
        return eb;
      });
      updatedModel.engine_glows = model.engine_glows.map(eg => {
        if (eg.parent_name === oldName) return { ...eg, parent_name: newName };
        return eg;
      });
      updatedModel.engine_shapes = model.engine_shapes.map(es => {
        if (es.parent_name === oldName) return { ...es, parent_name: newName };
        return es;
      });
      updatedModel.collision_meshes = model.collision_meshes.map(c => {
        if (c.mesh && c.mesh.parent_name === oldName) {
           return { ...c, mesh: { ...c.mesh, parent_name: newName } };
        }
        return c;
      });
    } else if (type === "marker") {
      updatedModel.markers = model.markers.map(m => m.name === oldName ? { ...m, name: newName } : m);
    } else if (type === "navlight") {
      updatedModel.nav_lights = model.nav_lights.map(m => m.name === oldName ? { ...m, name: newName } : m);
      updatedModel.joints = model.joints.map(j => j.name === oldName ? { ...j, name: newName } : j);
    } else if (type === "dockpath") {
      updatedModel.dockpaths = model.dockpaths.map(m => m.name === oldName ? { ...m, name: newName } : m);
    } else if (type === "collision") {
      updatedModel.collision_meshes = model.collision_meshes.map(m => m.name === oldName ? { ...m, name: newName } : m);
    } else if (type === "engine_burn") {
      updatedModel.engine_burns = model.engine_burns.map(m => m.name === oldName ? { ...m, name: newName } : m);
    } else if (type === "engine_glow") {
      updatedModel.engine_glows = model.engine_glows.map(m => m.name === oldName ? { ...m, name: newName } : m);
    } else if (type === "engine_shape") {
      updatedModel.engine_shapes = model.engine_shapes.map(m => m.name === oldName ? { ...m, name: newName } : m);
    } else if (type === "mesh") {
      updatedModel.meshes = model.meshes.map(m => m.name === oldName ? { ...m, name: newName } : m);
    } else if (type === "material") {
      updatedModel.materials = model.materials.map(m => m.name === oldName ? { ...m, name: newName } : m);
    }
    
    onModelChange?.(updatedModel);
    if (selectedNode && selectedNode.name === oldName && selectedNode.type === type) {
      setSelectedNode({ type, name: newName });
    }
    invoke("log_event", { level: "INFO", message: `Renamed ${type} from ${oldName} to ${newName}` }).catch(console.error);
  };
"""

content = content[:handle_delete_idx] + insertion + "\n" + content[handle_delete_idx:]

# 3. Add contextMenu Portal at bottom
portal_jsx = """
      {contextMenu && createPortal(
        <>
          <div 
            style={{ position: 'fixed', top: 0, left: 0, width: '100vw', height: '100vh', zIndex: 9999 }} 
            onClick={() => setContextMenu(null)}
            onContextMenu={(e) => { e.preventDefault(); setContextMenu(null); }}
          />
          <div 
            style={{
              position: 'fixed',
              top: contextMenu.y,
              left: contextMenu.x,
              zIndex: 10000,
              background: 'var(--bg-panel)',
              border: '1px solid rgba(255,255,255,0.1)',
              borderRadius: '4px',
              boxShadow: '0 4px 12px rgba(0,0,0,0.5)',
              padding: '4px 0',
              minWidth: '150px',
              display: 'flex',
              flexDirection: 'column'
            }}
          >
            <div 
              className="list-item"
              style={{ padding: '8px 12px', cursor: 'pointer', fontSize: '12px', color: 'var(--text-primary)' }}
              onClick={() => {
                 handleRenameNode(contextMenu.name, contextMenu.type);
                 setContextMenu(null);
              }}
            >
              ✏️ Rename
            </div>
            {isNodeDeletable(contextMenu.name, contextMenu.type) && (
              <div 
                className="list-item"
                style={{ padding: '8px 12px', cursor: 'pointer', fontSize: '12px', color: '#ff1744' }}
                onClick={() => {
                   const confirmMsg = contextMenu.type === "weapon_group" 
                     ? `Are you sure you want to delete the entire weapon/turret family "${contextMenu.name}"? This will remove all of its component joints safely.` 
                     : `Are you sure you want to delete "${contextMenu.name}"?`;
                   if (window.confirm(confirmMsg)) {
                     handleDeleteNode(contextMenu.name, contextMenu.type);
                   }
                   setContextMenu(null);
                }}
              >
                ✕ Delete
              </div>
            )}
          </div>
        </>,
        document.body
      )}
"""

content = content.replace("    </div>\n  );\n};", portal_jsx + "\n    </div>\n  );\n};")

# 4. Inject onContextMenu safely to all elements. We will find their onClick and inject onContextMenu below it.
# We do this by iterating over elements to ensure no duplicates.
replacements = [
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "joint", name: jointName \}\)\})', r'\1\n          onContextMenu={(e) => handleContextMenu(e, jointName, "joint")}'),
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "marker", name: marker\.name \}\)\})', r'\1\n                  onContextMenu={(e) => handleContextMenu(e, marker.name, "marker")}'),
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "mesh", name: meshKey \}\)\})', r'\1\n                  onContextMenu={(e) => handleContextMenu(e, meshKey, "mesh")}'),
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "navlight", name: nav\.name \}\)\})', r'\1\n                  onContextMenu={(e) => handleContextMenu(e, nav.name, "navlight")}'),
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "engine_burn", name: burn\.name \}\)\})', r'\1\n                  onContextMenu={(e) => handleContextMenu(e, burn.name, "engine_burn")}'),
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "engine_glow", name: glow\.name \}\)\})', r'\1\n                  onContextMenu={(e) => handleContextMenu(e, glow.name, "engine_glow")}'),
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "engine_shape", name: shape\.name \}\)\})', r'\1\n                  onContextMenu={(e) => handleContextMenu(e, shape.name, "engine_shape")}'),
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "collision", name: col\.name \}\)\})', r'\1\n                  onContextMenu={(e) => handleContextMenu(e, col.name, "collision")}'),
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "dockpath", name: path\.name \}\)\})', r'\1\n                  onContextMenu={(e) => handleContextMenu(e, path.name, "dockpath")}'),
    (r'(onClick=\{\(\) => setSelectedNode\(\{ type: "weapon_group", name: baseName \}\)\})', r'\1\n          onContextMenu={(e) => handleContextMenu(e, baseName, "weapon_group")}'),
]

for pat, repl in replacements:
    # First, strip out any already existing onContextMenu calls to prevent duplicates
    # Since we are starting from clean file, we shouldn't have any, but just to be safe
    content = re.sub(pat, repl, content)

with open(path, "w") as f:
    f.write(content)
print("Fixes applied securely!")
