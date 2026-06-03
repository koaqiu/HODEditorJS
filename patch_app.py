import re

filepath = "src/App.tsx"
with open(filepath, 'r') as f:
    content = f.read()

# Add import for SetupWizard
import_idx = content.find('import { SettingsModal } from "./components/SettingsModal";')
if import_idx != -1:
    content = content[:import_idx] + 'import { SetupWizard } from "./components/SetupWizard";\n' + content[import_idx:]

# Add import for updateDynamicShaderSlots
import_idx2 = content.find('import { HODTexture } from "./components/Viewport";')
if import_idx2 != -1:
    content = content[:import_idx2] + 'import { updateDynamicShaderSlots } from "./texture_utils";\n' + content[import_idx2:]

# Update the useEffect for load_shader_config
old_effect = """  // Load shader directories from config file on startup
  useEffect(() => {
    invoke<{ shader_directories: string[] }>("load_shader_config")
      .then((config) => {
        if (config.shader_directories.length > 0) {
          setKeeperTxtPaths(config.shader_directories);
        }
      })
      .catch((e) => console.error("Failed to load shader config:", e));
  }, []);"""

new_effect = """  // Load shader directories from config file on startup
  const [configLoaded, setConfigLoaded] = useState(false);

  useEffect(() => {
    invoke<{ shader_directories: string[] }>("load_shader_config")
      .then(async (config) => {
        if (config.shader_directories.length > 0) {
          setKeeperTxtPaths(config.shader_directories);
          try {
            const dynamicShaders = await invoke<{name: string, slots: string[]}[]>("get_dynamic_shader_slots", { keeperPaths: config.shader_directories });
            updateDynamicShaderSlots(dynamicShaders);
          } catch (e) {
            console.error("Failed to load dynamic shader slots", e);
          }
        }
      })
      .catch((e) => console.error("Failed to load shader config:", e))
      .finally(() => setConfigLoaded(true));
  }, []);

  const handleSetupComplete = async (paths: string[]) => {
    setKeeperTxtPaths(paths);
    try {
      const dynamicShaders = await invoke<{name: string, slots: string[]}[]>("get_dynamic_shader_slots", { keeperPaths: paths });
      updateDynamicShaderSlots(dynamicShaders);
    } catch (e) {
      console.error("Failed to load dynamic shader slots", e);
    }
  };"""

content = content.replace(old_effect, new_effect)

# Inject the SetupWizard rendering
# We can put it right inside the main return block.
# Look for <div className="app-container">
container_idx = content.find('<div className="app-container">')
if container_idx != -1:
    wizard_jsx = """      {configLoaded && keeperTxtPaths.length === 0 && (
        <SetupWizard onComplete={handleSetupComplete} />
      )}\n"""
    content = content[:container_idx + len('<div className="app-container">') + 1] + wizard_jsx + content[container_idx + len('<div className="app-container">') + 1:]

with open(filepath, 'w') as f:
    f.write(content)

