import sys

filepath = "docs/hod2-reverse-engineering/PROGRESS.md"

with open(filepath, 'r') as f:
    content = f.read()

new_bullet = """- ✅ Texture Grouping & UI Rewrite (`App.tsx`, `HierarchyTree.tsx`, `Inspector.tsx`, `texture_utils.ts`): Refactored texture representation to be group-based (by base name, e.g., `Hgn_Carrier`) instead of a flat list. Added `parseTextureGroups` utility to extract base names and sub-texture types. The Hierarchy Tree now lists Texture Groups, and selecting one opens a custom Inspector view to rename the entire group (automatically updating all dependent materials) and manage sub-texture formats (DXT1, DXT3, DXT5, RGBA). The Material inspector was simplified to use a single "Texture Group" dropdown that automatically populates the shader's texture slots (DIFF, GLOW, TEAM, etc.) based on the selected group's available sub-textures. Added an interactive prompt during TGA import to explicitly assign missing type suffixes (e.g., `_DIFF`) when unrecognized names are loaded.
"""

# Insert the new bullet after "## Current Status\n"
insert_idx = content.find("## Current Status\n") + len("## Current Status\n")
new_content = content[:insert_idx] + new_bullet + content[insert_idx:]

with open(filepath, 'w') as f:
    f.write(new_content)
