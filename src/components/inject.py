import re

path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# Make renderDeleteButton return null
content = re.sub(
    r"const renderDeleteButton = \(name: string, type: string\) => \{\n\s+if \(!isDeleteModeActive\) return null;",
    "const renderDeleteButton = (name: string, type: string) => {\n    return null;",
    content
)

# Remove the delete mode button entirely (lines around 1905)
# Actually, we can just leave the button there but hide it or remove it.
# Let's remove the toolbar button for delete mode.
content = re.sub(
    r'\{/\* Delete Mode Toggle \*/\}.*?</div>',
    '',
    content,
    flags=re.DOTALL
)

# Inject the context menu JSX at the very end of the file right before the last </div>
menu_jsx = """
      {contextMenu && (
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
        </>
      )}
"""

if "{contextMenu && (" not in content:
    content = content.replace("    </div>\n  );\n}\n\nexport default HierarchyTree;", menu_jsx + "\n    </div>\n  );\n}\n\nexport default HierarchyTree;")

# Inject onContextMenu into the onClick divs for all elements
replacements = [
    (r'(onClick=\{.*\bsetSelectedNode\(\{ type: "marker", name: marker\.name \}\).*?\}\s*)', r'\1onContextMenu={(e) => handleContextMenu(e, marker.name, "marker")}\n'),
    (r'(onClick=\{.*\bsetSelectedNode\(\{ type: "mesh", name: meshKey \}\).*?\}\s*)', r'\1onContextMenu={(e) => handleContextMenu(e, meshKey, "mesh")}\n'),
    (r'(onClick=\{.*\bsetSelectedNode\(\{ type: "navlight", name: nav\.name \}\).*?\}\s*)', r'\1onContextMenu={(e) => handleContextMenu(e, nav.name, "navlight")}\n'),
    (r'(onClick=\{.*\bsetSelectedNode\(\{ type: "engine_burn", name: burn\.name \}\).*?\}\s*)', r'\1onContextMenu={(e) => handleContextMenu(e, burn.name, "engine_burn")}\n'),
    (r'(onClick=\{.*\bsetSelectedNode\(\{ type: "engine_glow", name: glow\.name \}\).*?\}\s*)', r'\1onContextMenu={(e) => handleContextMenu(e, glow.name, "engine_glow")}\n'),
    (r'(onClick=\{.*\bsetSelectedNode\(\{ type: "engine_shape", name: shape\.name \}\).*?\}\s*)', r'\1onContextMenu={(e) => handleContextMenu(e, shape.name, "engine_shape")}\n'),
    (r'(onClick=\{.*\bsetSelectedNode\(\{ type: "collision", name: col\.name \}\).*?\}\s*)', r'\1onContextMenu={(e) => handleContextMenu(e, col.name, "collision")}\n'),
    (r'(onClick=\{.*\bsetSelectedNode\(\{ type: "dockpath", name: path\.name \}\).*?\}\s*)', r'\1onContextMenu={(e) => handleContextMenu(e, path.name, "dockpath")}\n'),
    (r'(onClick=\{.*\bsetSelectedNode\(\{ type: "weapon_group", name: baseName \}\).*?\}\s*)', r'\1onContextMenu={(e) => handleContextMenu(e, baseName, "weapon_group")}\n'),
]

for pat, repl in replacements:
    content = re.sub(pat, repl, content)

# For joints, the onClick is slightly different, it just says onClick={() => setSelectedNode({ type: "joint", name: jointName })}
# but it also has drag events. Let's find: `onClick={() => setSelectedNode({ type: "joint", name: jointName })}`
content = re.sub(
    r'(onClick=\{\(\) => setSelectedNode\(\{ type: "joint", name: jointName \}\)\}\s*)',
    r'\1onContextMenu={(e) => handleContextMenu(e, jointName, "joint")}\n',
    content
)

with open(path, "w") as f:
    f.write(content)
print("Injected context menu")
