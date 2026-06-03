import sys

filepath = "docs/hod2-reverse-engineering/PROGRESS.md"

with open(filepath, 'r') as f:
    content = f.read()

new_bullet = """- ✅ Refactored TGA Import Location (`HierarchyTree.tsx`, `Inspector.tsx`): Moved the "Import TGA Textures" button and the "Loaded TGA Directories" summary out of the individual Material Inspector panel and into the global "Textures" section of the Hierarchy Tree. This creates a logical workflow where users import their raw TGAs globally, resolve missing suffixes interactively, and immediately see the new files populate their respective Texture Groups underneath, before jumping into Material assignment.
"""

insert_idx = content.find("## Current Status\n") + len("## Current Status\n")
new_content = content[:insert_idx] + new_bullet + content[insert_idx:]

with open(filepath, 'w') as f:
    f.write(new_content)
