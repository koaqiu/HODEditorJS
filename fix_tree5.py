import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

# Disable context menu on assembly child joints
# We need to find the `onContextMenu` in `renderJointNode` and block it if it's an assembly subnode.
# renderJointNode is defined as `const renderJointNode = (jointName: string, depth: number) => {`
# We can find `onContextMenu={(e) => handleContextMenu(e, jointName, "joint")}` and wrap it in a condition.
# Wait, we can just block it in `handleContextMenu` directly!
# Let's modify `handleContextMenu`.

context_target = """  const handleContextMenu = (e: React.MouseEvent, name: string, type: string) => {"""
context_repl = """  const handleContextMenu = (e: React.MouseEvent, name: string, type: string) => {
    if (type === "joint") {
      const wInfo = getWeaponGroupInfo(name);
      if (wInfo && name !== wInfo.baseName) {
        // It's a subnode of an assembly, prevent context menu
        e.preventDefault();
        e.stopPropagation();
        return;
      }
    }"""
content = content.replace(context_target, context_repl)

# Now block dragging for assembly joints.
# In `renderJointNode`, the `draggable` is: `draggable={jointName !== "Root" ? "true" : "false"}`
# Let's change it to: `draggable={jointName !== "Root" && !(getWeaponGroupInfo(jointName) && jointName !== getWeaponGroupInfo(jointName)?.baseName) ? "true" : "false"}`

content = content.replace(
    'draggable={jointName !== "Root" ? "true" : "false"}',
    'draggable={jointName !== "Root" && !(getWeaponGroupInfo(jointName) && jointName !== getWeaponGroupInfo(jointName)?.baseName) ? "true" : "false"}'
)

with open(path, "w") as f:
    f.write(content)
print("Assembly dragging and rename blocked")
