import re

path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/src/components/HierarchyTree.tsx"

with open(path, "r") as f:
    content = f.read()

# 1. Add react-dom import
if "import { createPortal } from \"react-dom\";" not in content:
    content = content.replace("import React, { useState, useRef } from \"react\";", "import React, { useState, useRef } from \"react\";\nimport { createPortal } from \"react-dom\";")

# 2. Fix Context Menu Portal
if "createPortal(" not in content:
    content = content.replace("{contextMenu && (", "{contextMenu && createPortal(")
    content = content.replace("</>\n      )}\n\n    </div>", "</>\n      , document.body)}\n\n    </div>")

# 3. Add handleRenameNode duplicates check & Weapon Assembly logic
old_handle_rename = r'const handleRenameNode = \(oldName: string, type: string\) => \{\n\s+if \(!model\) return;\n\s+let cleanOldName = oldName;.*?if \(type === "joint"\) \{'

new_handle_rename = """const handleRenameNode = (oldName: string, type: string) => {
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
          return { ...j, name: j.name.replace(oldName, newName), parent_name: j.parent_name.replace(oldName, newName) };
        }
        return { ...j, parent_name: j.parent_name.replace(oldName, newName) };
      });
      updatedModel.meshes = model.meshes.map(m => ({ ...m, parent_name: m.parent_name.replace(oldName, newName) }));
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

    if (type === "joint") {"""

content = re.sub(old_handle_rename, new_handle_rename, content, flags=re.DOTALL)


# 4. Modify Navlight States to string
content = content.replace('const [navLightSize, setNavLightSize] = useState(1.0);', 'const [navLightSize, setNavLightSize] = useState("1.0");')
content = content.replace('const [navLightPhase, setNavLightPhase] = useState(0.0);', 'const [navLightPhase, setNavLightPhase] = useState("0.0");')
content = content.replace('const [navLightFreq, setNavLightFreq] = useState(1.0);', 'const [navLightFreq, setNavLightFreq] = useState("1.0");')

# Adjust handleAddNode parsing
content = content.replace('size: navLightSize,', 'size: parseFloat(navLightSize) || 1.0,')
content = content.replace('phase: navLightPhase,', 'phase: parseFloat(navLightPhase) || 0.0,')
content = content.replace('frequency: navLightFreq,', 'frequency: parseFloat(navLightFreq) || 1.0,')

# Adjust inline onChange
content = content.replace('onChange={(e) => setNavLightSize(parseFloat(e.target.value) || 1.0)}', 'onChange={(e) => setNavLightSize(e.target.value)}')
content = content.replace('onChange={(e) => setNavLightPhase(parseFloat(e.target.value) || 0.0)}', 'onChange={(e) => setNavLightPhase(e.target.value)}')
content = content.replace('onChange={(e) => setNavLightFreq(parseFloat(e.target.value) || 1.0)}', 'onChange={(e) => setNavLightFreq(e.target.value)}')


# 5. Weapon Assembly Subnode Rendering Suffix Fix
# In renderJointNode
# Original: <span style={{ overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>\n              {jointName}\n            </span>
# We'll replace {jointName} with display logic
suffix_logic = """{(() => {
                const wInfo = getWeaponGroupInfo(jointName);
                return wInfo && jointName !== wInfo.baseName ? jointName.substring(wInfo.baseName.length) : jointName;
              })()}"""
content = re.sub(
    r'<span style=\{\{\s*overflow:\s*"hidden",\s*textOverflow:\s*"ellipsis",\s*whiteSpace:\s*"nowrap"\s*\}\}>\s*\{jointName\}\s*</span>',
    f'<span style={{{{ overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}}}>\n              {suffix_logic}\n            </span>',
    content
)


with open(path, "w") as f:
    f.write(content)
print("Fixes injected successfully.")
