use hwr_hod_parser::hod::{save_edits, HODModel};
use std::fs;

fn main() -> Result<(), String> {
    let source_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_orion/ter_orion.hod";
    let target_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_orion/ter_orion2.hod";

    let original_bytes = fs::read(source_path).map_err(|e| e.to_string())?;
    let model = HODModel::parse_with_external(&original_bytes, Some(source_path), None)?;
    let patched_bytes = save_edits(&original_bytes, &model)?;
    fs::write(target_path, &patched_bytes).map_err(|e| e.to_string())?;

    Ok(())
}
