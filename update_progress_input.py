import sys

filepath = "docs/hod2-reverse-engineering/PROGRESS.md"

with open(filepath, 'r') as f:
    content = f.read()

new_bullet = """- ✅ Fixed UI focus loss during renaming (`Inspector.tsx`): Replaced global-state-bound `<input type="text">` fields with a new `<TextInput>` component that manages local state while typing and commits the change to the global model on `blur` or when pressing `Enter`. This fixes the bug where renaming a Texture Group or Material would lose cursor focus after typing a single letter.
"""

insert_idx = content.find("## Current Status\n") + len("## Current Status\n")
new_content = content[:insert_idx] + new_bullet + content[insert_idx:]

with open(filepath, 'w') as f:
    f.write(new_content)
