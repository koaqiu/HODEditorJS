import re

filepath = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/src/components/Inspector.tsx"

with open(filepath, 'r') as f:
    content = f.read()

# 1. Add TextInput definition
text_input_def = """interface TextInputProps {
  value: string;
  onChange: (val: string) => void;
  style?: React.CSSProperties;
  placeholder?: string;
  className?: string;
}

const TextInput: React.FC<TextInputProps> = ({
  value,
  onChange,
  style,
  placeholder,
  className
}) => {
  const [localValue, setLocalValue] = useState(value);
  const isFocusedRef = useRef(false);

  useEffect(() => {
    if (!isFocusedRef.current) {
      setLocalValue(value);
    }
  }, [value]);

  const handleBlur = () => {
    isFocusedRef.current = false;
    onChange(localValue);
    setLocalValue(value); // Resets if parent rejected the change
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      e.currentTarget.blur();
    }
  };

  return (
    <input
      type="text"
      value={localValue}
      onFocus={() => { isFocusedRef.current = true; }}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
      onChange={(e) => setLocalValue(e.target.value)}
      style={style}
      placeholder={placeholder}
      className={className}
    />
  );
};
"""

insert_idx = content.find("const SHADER_SLOTS")
if insert_idx != -1:
    content = content[:insert_idx] + text_input_def + "\n" + content[insert_idx:]

# 2. Replace <input type="text" ... />
# NavLight style
content = content.replace("""            <input
              type="text"
              value={nav.style}
              onChange={(e) => handleNavLightChange("style", e.target.value)}
              style={{ fontSize: "12px" }}
            />""", """            <TextInput
              value={nav.style}
              onChange={(val) => handleNavLightChange("style", val)}
              style={{ fontSize: "12px", width: "100%", background: "rgba(0,0,0,0.2)", border: "1px solid rgba(255,255,255,0.1)", color: "var(--text-primary)", padding: "6px 10px", borderRadius: "4px" }}
            />""")

# Texture Group Base Name
content = content.replace("""              <input
                type="text"
                value={group.baseName}
                onChange={(e) => handleGroupNameChange(e.target.value)}
                style={{ width: "100%", background: "rgba(0,0,0,0.2)", border: "1px solid rgba(255,255,255,0.1)", color: "var(--text-primary)", padding: "6px 10px", borderRadius: "4px", fontSize: "12px", fontFamily: "var(--font-mono)" }}
              />""", """              <TextInput
                value={group.baseName}
                onChange={(val) => handleGroupNameChange(val)}
                style={{ width: "100%", background: "rgba(0,0,0,0.2)", border: "1px solid rgba(255,255,255,0.1)", color: "var(--text-primary)", padding: "6px 10px", borderRadius: "4px", fontSize: "12px", fontFamily: "var(--font-mono)" }}
              />""")

# Material Name
content = content.replace("""              <input
                type="text"
                value={material.name}
                onChange={(e) => handleMaterialNameChange(e.target.value)}
              />""", """              <TextInput
                value={material.name}
                onChange={(val) => handleMaterialNameChange(val)}
                style={{ width: "100%", background: "rgba(0,0,0,0.2)", border: "1px solid rgba(255,255,255,0.1)", color: "var(--text-primary)", padding: "6px 10px", borderRadius: "4px", fontSize: "12px", fontFamily: "var(--font-mono)" }}
              />""")

with open(filepath, 'w') as f:
    f.write(content)
