# SHADERS.MAP Integration Plan

## Status: Planning

**Created:** 2026-05-28  
**Priority:** High  
**Estimated Effort:** 4-6 hours  

---

## Executive Summary

This document outlines the plan for integrating SHADERS.MAP into the HOD 2.0 creation pipeline. SHADERS.MAP defines how texture channels are mapped to game shaders, which is critical for creating valid HOD 2.0 files from assets.

---

## Current Understanding

### SHADERS.MAP Format

**Location:** `GBXTools/HODOR/SHADERS.MAP`

**Structure:**
```
+pipeline_name
    $param[FORMAT] = default_values
        TEXTURE_ROLE = channel_mapping
```

**Example (ship shader):**
```
+ship,matte,matte2s,monolith,megalith,fxMatte
    $diffuse[DXT1] = 1 1 1 1
        DIFF = R G B 1
    $glow[DXT1]= 0 0 0 1
        GLOW = G G G G
        SPEC = B B B B
        REFL = R R R R
    $team[DXT1] = 1 1 0 1
        TEAM = 1 1 1 r
        STRP = 1 1 1 g
        PAIN = 1 1 1 b
    $normal[DXT1]= 5 5 1 1
        NORM[B] = R G B 1
```

### Key Concepts

1. **Pipeline Names:** `+ship`, `+shipglow`, `+thruster`, etc.
2. **Parameters:** `$diffuse`, `$glow`, `$team`, `$normal`, etc.
3. **Formats:** `DXT1`, `DXT5`, `8888`
4. **Channel Mapping:** `R G B A` with optional modifiers

### Existing Analysis

From `progress_log.md`:
- SHADERS.MAP defines shader parameter mapping for all pipelines
- Confirms STAT parameter names (`$diffuse`, `$glow`, `$normal`) are correct
- Format: `+pipeline_name` block with `$param[FORMAT] = default_values` and channel swizzle rules

---

## Integration Requirements

### Requirement 1: Parse SHADERS.MAP

**Goal:** Read and parse SHADERS.MAP file into structured data

**Output:**
```rust
struct ShaderMapping {
    pipeline_name: String,
    parameters: Vec<ShaderParameter>,
}

struct ShaderParameter {
    name: String,           // "$diffuse", "$glow", etc.
    format: String,         // "DXT1", "DXT5", "8888"
    default_values: Vec<f32>,
    channel_mappings: Vec<ChannelMapping>,
}

struct ChannelMapping {
    texture_role: String,   // "DIFF", "GLOW", "TEAM", etc.
    channels: String,       // "R G B 1", "G G G G", etc.
}
```

### Requirement 2: Map Textures to Shaders

**Goal:** Automatically map TGA textures to shader parameters based on naming conventions

**Mapping Rules:**
- `*_DIFF.TGA` → `$diffuse` parameter
- `*_GLOW.TGA` → `$glow` parameter
- `*_TEAM.TGA` → `$team` parameter
- `*_NORM.TGA` → `$normal` parameter
- `*_SPEC.TGA` → `$spec` parameter

### Requirement 3: Apply Channel Mapping

**Goal:** Apply channel swizzling when creating texture pool

**Example:**
```
$diffuse[DXT1] = 1 1 1 1
    DIFF = R G B 1
```
- Read TGA channels (R, G, B, A)
- Apply mapping: DIFF = R, G, B, 1 (alpha = 1)
- Output to texture pool

### Requirement 4: Select Correct Shader

**Goal:** Automatically select correct shader based on texture set

**Logic:**
```
if has_team_texture:
    shader = "ship"
elif has_glow_texture:
    shader = "shipglow"
else:
    shader = "matte"
```

---

## Implementation Plan

### Phase 1: SHADERS.MAP Parser (2 hours)

**Task 1.1: Create Parser Structure**
- Define data structures for shader mappings
- Implement file reading and parsing
- Handle comments and whitespace

**Task 1.2: Parse Pipeline Blocks**
-识别 `+pipeline_name` blocks
- Extract pipeline names (comma-separated)
- Parse parameter definitions

**Task 1.3: Parse Parameters**
- Extract parameter names (`$diffuse`, `$glow`, etc.)
- Parse format (`DXT1`, `DXT5`, `8888`)
- Parse default values
- Extract channel mappings

**Task 1.4: Test Parser**
- Parse existing SHADERS.MAP file
- Verify parsed data matches expected structure
- Create unit tests

### Phase 2: Texture Mapping (1 hour)

**Task 2.1: Implement Naming Convention Detection**
- Scan texture files for naming patterns
- Map `*_DIFF.TGA` to `$diffuse`
- Map `*_GLOW.TGA` to `$glow`
- Map `*_TEAM.TGA` to `$team`
- Map `*_NORM.TGA` to `$normal`
- Map `*_SPEC.TGA` to `$spec`

**Task 2.2: Auto-Select Shader**
- Analyze available textures
- Select appropriate shader pipeline
- Default to "ship" if unclear

**Task 2.3: Test Mapping**
- Test with pebble_0 textures
- Verify correct shader selection
- Verify correct parameter mapping

### Phase 3: Channel Mapping (1 hour)

**Task 3.1: Implement Channel Swizzling**
- Read TGA texture data
- Apply channel mapping rules
- Handle format conversion (TGA → DXT)

**Task 3.2: Integrate with Texture Pool**
- Apply channel mapping during texture compression
- Verify output matches RODOH behavior
- Test with various shader types

**Task 3.3: Test Channel Mapping**
- Test with ship shader
- Test with thruster shader
- Test with background shader
- Verify visual quality

### Phase 4: Integration & Testing (2 hours)

**Task 4.1: Integrate with Replicate Testing**
- Update replicate_testing to use SHADERS.MAP
- Verify generated HOD files are valid
- Compare with vanilla HOD files

**Task 4.2: Create Documentation**
- Document SHADERS.MAP format
- Document mapping rules
- Create usage examples

**Task 4.3: Validate in Game**
- Generate HOD files with SHADERS.MAP integration
- Test in Homeworld Remastered
- Verify rendering quality

---

## Technical Details

### SHADERS.MAP Parsing Algorithm

```rust
fn parse_shaders_map(content: &str) -> Vec<ShaderMapping> {
    let mut mappings = Vec::new();
    let mut current_pipeline = None;
    
    for line in content.lines() {
        let line = line.trim();
        
        // Skip comments and empty lines
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        
        // Pipeline block
        if line.starts_with('+') {
            let pipeline_names = line[1..].split(',')
                .map(|s| s.trim().to_string())
                .collect();
            current_pipeline = Some(ShaderMapping {
                pipeline_name: pipeline_names.join(","),
                parameters: Vec::new(),
            });
            mappings.push(current_pipeline.as_ref().unwrap().clone());
            continue;
        }
        
        // Parameter definition
        if line.starts_with('$') && current_pipeline.is_some() {
            let param = parse_parameter(line);
            current_pipeline.as_mut().unwrap().parameters.push(param);
        }
    }
    
    mappings
}
```

### Texture Mapping Algorithm

```rust
fn map_textures_to_shader(textures: &[String], mappings: &[ShaderMapping]) -> (String, Vec<TextureMapping>) {
    // Analyze texture names to determine shader type
    let has_diff = textures.iter().any(|t| t.contains("_DIFF"));
    let has_glow = textures.iter().any(|t| t.contains("_GLOW"));
    let has_team = textures.iter().any(|t| t.contains("_TEAM"));
    let has_norm = textures.iter().any(|t| t.contains("_NORM"));
    let has_spec = textures.iter().any(|t| t.contains("_SPEC"));
    
    // Select shader pipeline
    let shader_name = if has_team {
        "ship"
    } else if has_glow {
        "shipglow"
    } else {
        "matte"
    };
    
    // Find mapping for selected shader
    let mapping = mappings.iter()
        .find(|m| m.pipeline_name.contains(shader_name))
        .unwrap();
    
    // Map textures to parameters
    let mut texture_mappings = Vec::new();
    for texture in textures {
        if let Some(param) = find_parameter_for_texture(texture, mapping) {
            texture_mappings.push(TextureMapping {
                texture_name: texture.clone(),
                parameter: param.name.clone(),
                format: param.format.clone(),
            });
        }
    }
    
    (shader_name.to_string(), texture_mappings)
}
```

### Channel Mapping Algorithm

```rust
fn apply_channel_mapping(tga_data: &[u8], mapping: &ChannelMapping) -> Vec<u8> {
    let mut output = Vec::new();
    
    // Parse channel mapping string
    let channels: Vec<&str> = mapping.channels.split_whitespace().collect();
    
    // Apply mapping to each pixel
    for pixel in tga_data.chunks(4) {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let a = pixel[3];
        
        for channel in &channels {
            match *channel {
                "R" => output.push(r),
                "G" => output.push(g),
                "B" => output.push(b),
                "A" => output.push(a),
                "r" => output.push(255 - r), // Inverse
                "g" => output.push(255 - g),
                "b" => output.push(255 - b),
                "a" => output.push(255 - a),
                "0" => output.push(0),
                "1" => output.push(255),
                _ => {}
            }
        }
    }
    
    output
}
```

---

## Testing Strategy

### Test Case 1: SHADERS.MAP Parser

**Input:** `GBXTools/HODOR/SHADERS.MAP`  
**Expected:** Parsed shader mappings  
**Validation:** Compare with manual analysis

### Test Case 2: Texture Mapping

**Input:** pebble_0 textures (DIFF, GLOW, NORM)  
**Expected:** Correct shader selection and parameter mapping  
**Validation:** Verify shader is "ship" and parameters are mapped correctly

### Test Case 3: Channel Mapping

**Input:** TGA texture data with channel mapping rules  
**Expected:** Correctly swizzled texture data  
**Validation:** Compare with RODOH output

### Test Case 4: Integration

**Input:** pebble_0 assets with SHADERS.MAP  
**Expected:** Valid HOD 2.0 file  
**Validation:** Game loads without errors

---

## Risk Assessment

### Risk 1: Parser Complexity

**Risk:** SHADERS.MAP format may have edge cases  
**Mitigation:** Study complete file, handle all variations

### Risk 2: Channel Mapping Accuracy

**Risk:** Channel swizzling may not match RODOH exactly  
**Mitigation:** Compare with RODOH output, adjust as needed

### Risk 3: Shader Selection Logic

**Risk:** Auto-selection may choose wrong shader  
**Mitigation:** Allow manual override, test with various models

---

## Success Criteria

### Parser Success

- [ ] Parse all pipeline blocks correctly
- [ ] Extract all parameters and mappings
- [ ] Handle comments and whitespace
- [ ] Unit tests pass

### Mapping Success

- [ ] Auto-detect shader type from textures
- [ ] Map textures to correct parameters
- [ ] Apply channel mapping correctly
- [ ] Integration tests pass

### Integration Success

- [ ] Generate valid HOD 2.0 files
- [ ] Game loads without errors
- [ ] Visual quality matches RODOH output
- [ ] Documentation complete

---

## Dependencies

### Required Files

- `GBXTools/HODOR/SHADERS.MAP` - Shader mapping definitions
- `parser/src/hod.rs` - HOD parsing and generation
- `parser/src/compiler.rs` - Mesh compilation

### Required Knowledge

- SHADERS.MAP format (documented above)
- Texture compression pipeline
- Channel mapping rules

---

## Timeline

### Day 1 (4 hours)

- Phase 1: SHADERS.MAP Parser (2 hours)
- Phase 2: Texture Mapping (1 hour)
- Phase 3: Channel Mapping (1 hour)

### Day 2 (2 hours)

- Phase 4: Integration & Testing (2 hours)

**Total:** 6 hours

---

## Next Steps

### Immediate

1. **Create SHADERS.MAP parser** - Implement parsing logic
2. **Test parser** - Verify against SHADERS.MAP file
3. **Implement texture mapping** - Auto-detect shader type

### Short Term

1. **Implement channel mapping** - Apply swizzling rules
2. **Integrate with replicate_testing** - Test with pebble_0
3. **Validate in game** - Test generated HOD files

### Long Term

1. **Expand to other shaders** - Support all shader types
2. **Create UI integration** - Allow manual shader selection
3. **Documentation** - Complete usage guide

---

**Plan Version:** 1.0  
**Last Updated:** 2026-05-28  
**Status:** Ready for Implementation