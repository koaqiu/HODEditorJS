import sys

filepath = "docs/hod2-reverse-engineering/PROGRESS.md"

with open(filepath, 'r') as f:
    content = f.read()

new_bullet = """- ✅ Restored UI Functionality for Textures: Added back the individual sub-texture "Toggle Y-Flip" and "Delete" buttons to the `Inspector` panel (which had been lost during the group refactor). Also added a right-click context menu in the `HierarchyTree` to allow deleting an entire Texture Group at once (which safely removes all its component textures and clears them from material slots).
"""

insert_idx = content.find("## Current Status\n") + len("## Current Status\n")
new_content = content[:insert_idx] + new_bullet + content[insert_idx:]

with open(filepath, 'w') as f:
    f.write(new_content)
