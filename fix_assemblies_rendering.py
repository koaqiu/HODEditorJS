import re

path = "src/components/HierarchyTree.tsx"
with open(path, "r") as f:
    content = f.read()

target1 = """    const childWeaponGroups = getUniqueAssemblyGroups().filter(baseName => {
      const groupJoints = model.joints.filter(j => j.name.toLowerCase().startsWith(baseName.toLowerCase() + "_"));"""

repl1 = """    const childWeaponGroups = getUniqueAssemblyGroups().filter(baseName => {
      const groupJoints = model.joints.filter(j => j.name.toLowerCase().startsWith(baseName.toLowerCase() + "_") || j.name.toLowerCase() === baseName.toLowerCase());"""

content = content.replace(target1, repl1)

target2 = """  const rootWeaponGroups = getUniqueAssemblyGroups().filter(baseName => {
    const groupJoints = model.joints.filter(j => j.name.toLowerCase().startsWith(baseName.toLowerCase() + "_"));"""

repl2 = """  const rootWeaponGroups = getUniqueAssemblyGroups().filter(baseName => {
    const groupJoints = model.joints.filter(j => j.name.toLowerCase().startsWith(baseName.toLowerCase() + "_") || j.name.toLowerCase() === baseName.toLowerCase());"""

content = content.replace(target2, repl2)

with open(path, "w") as f:
    f.write(content)

print("Assembly rendering fix applied.")
