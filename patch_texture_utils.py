import re

filepath = "src/texture_utils.ts"
with open(filepath, 'r') as f:
    content = f.read()

# Replace const SHADER_SLOTS and KNOWN_TYPES to allow updates
content = content.replace("export const KNOWN_TYPES = ", "export let KNOWN_TYPES = ")
content = content.replace("export const SHADER_SLOTS: Record<string, string[]> = {", "export let SHADER_SLOTS: Record<string, string[]> = {")

new_func = """
export function updateDynamicShaderSlots(dynamicShaders: {name: string, slots: string[]}[]) {
  const newTypes = new Set(KNOWN_TYPES);
  for (const shader of dynamicShaders) {
    const formattedSlots = shader.slots.map(slot => {
      // e.g. "diffuse" -> "Diffuse (DIFFUSE)", "glow" -> "Glow (GLOW)"
      const suffix = "_" + slot.toUpperCase();
      newTypes.add(suffix);
      return `${slot.charAt(0).toUpperCase() + slot.slice(1)} Map (${slot.toUpperCase()})`;
    });
    // Merge or replace
    SHADER_SLOTS[shader.name.toLowerCase()] = formattedSlots;
  }
  KNOWN_TYPES = Array.from(newTypes);
}
"""

content += "\n" + new_func

with open(filepath, 'w') as f:
    f.write(content)
