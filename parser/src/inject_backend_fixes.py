import re

path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/parser/src/hod.rs"

with open(path, "r") as f:
    content = f.read()

# We need to inject additional cleanup into `clean_hierarchy`.
# It currently loops over joints. We'll append loops for the other structs right after the joints loop.

old_clean_hierarchy = r"""            // Apply updates
            for \(idx, new_parent, new_transform\) in updates \{
                self\.joints\[idx\]\.parent_name = new_parent;
                self\.joints\[idx\]\.local_transform = new_transform;
            \}
        \}
    \}"""

new_clean_hierarchy = """            // Apply updates
            for (idx, new_parent, new_transform) in updates {
                self.joints[idx].parent_name = new_parent;
                self.joints[idx].local_transform = new_transform;
            }

            // --- FUNCTIONAL NODES CLEANUP ---
            // If any functional node has its parent_name pointing to a special endpoint, we un-nest it.
            // Functional nodes do not carry a full hierarchical local_transform matrix to adapt in the same way,
            // they simply inherit their parent's absolute space.
            
            let mut check_and_update = |parent_name_ref: &mut String| {
                if special_prefixes.iter().any(|&p| parent_name_ref.starts_with(p) || parent_name_ref == &p[..p.len()-1]) {
                    if let Some(parent_joint) = self.joints.iter().find(|j| &j.name == parent_name_ref) {
                        if let Some(new_p) = &parent_joint.parent_name {
                            *parent_name_ref = new_p.clone();
                            changed = true;
                        }
                    }
                }
            };

            for burn in &mut self.engine_burns {
                check_and_update(&mut burn.parent_name);
            }
            if let Some(glows) = &mut self.engine_glows {
                for glow in glows { check_and_update(&mut glow.parent_name); }
            }
            if let Some(shapes) = &mut self.engine_shapes {
                for shape in shapes { check_and_update(&mut shape.parent_name); }
            }
            if let Some(paths) = &mut self.dockpaths {
                for path in paths { check_and_update(&mut path.parent_name); }
            }
            for mesh in &mut self.meshes {
                check_and_update(&mut mesh.parent_name);
            }
            for marker in &mut self.markers {
                check_and_update(&mut marker.parent_joint);
            }
        }
    }"""

content = re.sub(old_clean_hierarchy, new_clean_hierarchy, content)

with open(path, "w") as f:
    f.write(content)

print("Backend hierarchy fix injected.")
