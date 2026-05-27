import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# Update getWeaponGroupInfo to handle all point groups
# We'll just search and replace the function exactly
start_idx = content.find("const getWeaponGroupInfo = (name: string) => {")
end_idx = content.find("};", start_idx) + 2

new_get_group = """const getWeaponGroupInfo = (name: string) => {
    // Check weapon and turret assemblies
    const weaponMatch = name.match(/^(Weapon_[A-Za-z0-9_]+|Turret_[A-Za-z0-9_]+)_(Position|Direction|Muzzle\\d*|Rest|Latitude|Pitch|Yaw|Barrel\\d*)$/i);
    if (weaponMatch) return { baseName: weaponMatch[1], suffix: weaponMatch[2], type: weaponMatch[1].startsWith('Turret') ? 'turret_group' : 'weapon_group' };
    
    // Check point groups (Capture, Repair, Salvage, Hardpoint)
    const pointMatch = name.match(/^(CapturePoint\\d+|RepairPoint\\d+|SalvagePoint\\d+|Hardpoint_\\d+)_(Heading|Left|Up|Position|Direction|Rest)$/i);
    if (pointMatch) {
      let type = "point_group";
      if (pointMatch[1].startsWith("Hardpoint")) type = "hardpoint_group";
      else if (pointMatch[1].startsWith("Capture")) type = "capture_point_group";
      else if (pointMatch[1].startsWith("Repair")) type = "repair_point_group";
      else if (pointMatch[1].startsWith("Salvage")) type = "salvage_point_group";
      return { baseName: pointMatch[1], suffix: pointMatch[2], type };
    }
    
    // Check base nodes for point groups (they don't always have a suffix for the root)
    const exactPointMatch = name.match(/^(CapturePoint\\d+|RepairPoint\\d+|SalvagePoint\\d+)$/i);
    if (exactPointMatch) {
      let type = "point_group";
      if (exactPointMatch[1].startsWith("Capture")) type = "capture_point_group";
      else if (exactPointMatch[1].startsWith("Repair")) type = "repair_point_group";
      else if (exactPointMatch[1].startsWith("Salvage")) type = "salvage_point_group";
      return { baseName: exactPointMatch[1], suffix: "", type };
    }

    return null;
  };"""

content = content[:start_idx] + new_get_group + content[end_idx:]

# Also rename getUniqueWeaponGroups -> getUniqueAssemblyGroups
content = content.replace("getUniqueWeaponGroups", "getUniqueAssemblyGroups")

# And update the render logic for WeaponGroupNode to be AssemblyNode
content = content.replace("renderWeaponGroupNode", "renderAssemblyNode")

# We'll just find the exact display text block and replace it
target_display = '{isTurret ? "Turret: " : "Weapon: "}{baseName}'
replacement_display = """{(() => {
                const info = getWeaponGroupInfo(baseName + "_Position") || getWeaponGroupInfo(baseName + "_Heading") || getWeaponGroupInfo(baseName);
                const type = info?.type || "weapon_group";
                if (type === "turret_group") return "Turret: " + baseName;
                if (type === "weapon_group") return "Weapon: " + baseName;
                if (type === "hardpoint_group") return "Hardpoint: " + baseName;
                return "Point: " + baseName;
              })()}"""
content = content.replace(target_display, replacement_display)

with open(path, "w") as f:
    f.write(content)

print("Groups updated")
