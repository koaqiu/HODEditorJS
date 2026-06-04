import { useState } from "react";
import { Move, RefreshCw, Crosshair } from "lucide-react";
import { HODModel } from "./Viewport";
import { NumericInput, getEulerRotation, updateMatrixRotation } from "./Inspector";
import { invoke } from "@tauri-apps/api/core";

export function WeaponGroupInspector({
  model,
  selectedNode,
  onModelChange,
  onPositionChange
}: {
  model: HODModel;
  selectedNode: { type: string, name: string };
  onModelChange?: (model: HODModel) => void;
  onPositionChange: (name: string, type: string, pos: { x: number; y: number; z: number }) => void;
}) {
  const [renameWeaponName, setRenameWeaponName] = useState(selectedNode.name);
  const baseName = selectedNode.name;

  const getJoint = (suffix: string) => {
    let jointName = suffix ? `${baseName}${suffix}` : baseName;
    let jointObj = model.joints.find(j => j.name.toLowerCase() === jointName.toLowerCase());
    if (!jointObj && suffix === "_Muzzle") {
      jointObj = model.joints.find(j => j.name.toLowerCase().startsWith(`${baseName}_Muzzle`.toLowerCase()));
    }
    return jointObj;
  };

  const posJoint = getJoint("_Position");
  const dirJoint = getJoint("_Direction");
  const latJoint = getJoint("_Latitude");
  const muzJoint = getJoint("_Muzzle");
  const restJoint = getJoint("_Rest");

  const hasDirection = !!dirJoint;
  const hasLatitude = !!latJoint;
  const hasMuzzle = !!muzJoint;
  const hasRest = !!restJoint;

  const handleToggleJoint = (suffix: string, defaultParentSuffix: string, defaultOffset: {x:number, y:number, z:number}) => {
    if (!model) return;
    const isAdding = !getJoint(suffix);
    let jointsToSave = [...model.joints];

    if (isAdding) {
      const jointName = `${baseName}${suffix}`;
      let parentName = `${baseName}${defaultParentSuffix}`;
      
      // Special Muzzle reparenting logic when toggling
      if (suffix === "_Muzzle") {
        parentName = hasLatitude ? `${baseName}_Latitude` : `${baseName}_Position`;
      }
      
      jointsToSave.push({
        name: jointName,
        parent_name: parentName,
        local_transform: {
          m: [
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [defaultOffset.x, defaultOffset.y, defaultOffset.z, 1]
          ]
        }
      });

      // If adding Latitude and Muzzle exists, reparent Muzzle to Latitude
      if (suffix === "_Latitude" && hasMuzzle) {
        const mObj = jointsToSave.find(j => j.name === muzJoint!.name);
        if (mObj) mObj.parent_name = jointName;
      }
    } else {
      // Removing
      const jointToRemove = getJoint(suffix);
      if (jointToRemove) {
        jointsToSave = jointsToSave.filter(j => j.name !== jointToRemove.name);
        
        // If removing Latitude and Muzzle exists, reparent Muzzle back to Position
        if (suffix === "_Latitude" && hasMuzzle) {
          const mObj = jointsToSave.find(j => j.name === muzJoint!.name);
          if (mObj) mObj.parent_name = `${baseName}_Position`;
        }
      }
    }

    onModelChange?.({ ...model, joints: jointsToSave });
  };

  const handleRenameWeaponGroup = () => {
    const nextName = renameWeaponName.trim();
    if (!nextName || nextName === baseName) return;

    const updatedJoints = model.joints.map((joint) => {
      let updatedName = joint.name;
      let updatedParent = joint.parent_name;
      
      if (joint.name.toLowerCase() === baseName.toLowerCase()) {
        updatedName = nextName;
      } else if (joint.name.toLowerCase().startsWith(baseName.toLowerCase() + "_")) {
        const suffix = joint.name.substring(baseName.length + 1);
        updatedName = `${nextName}_${suffix}`;
      }
      if (joint.parent_name && joint.parent_name.toLowerCase() === baseName.toLowerCase()) {
        updatedParent = nextName;
      } else if (joint.parent_name && joint.parent_name.toLowerCase().startsWith(baseName.toLowerCase() + "_")) {
        const suffix = joint.parent_name.substring(baseName.length + 1);
        updatedParent = `${nextName}_${suffix}`;
      }

      return { ...joint, name: updatedName, parent_name: updatedParent };
    });

    const updatedMeshes = model.meshes.map((mesh) => {
      let updatedParent = mesh.parent_name;
      if (mesh.parent_name.toLowerCase() === baseName.toLowerCase()) {
        updatedParent = nextName;
      } else if (mesh.parent_name.toLowerCase().startsWith(baseName.toLowerCase() + "_")) {
        const suffix = mesh.parent_name.substring(baseName.length + 1);
        updatedParent = `${nextName}_${suffix}`;
      }
      return { ...mesh, parent_name: updatedParent };
    });

    onModelChange?.({
      ...model,
      joints: updatedJoints,
      meshes: updatedMeshes
    });

    invoke("log_event", { level: "INFO", message: `Renamed weapon group from ${baseName} to ${nextName}` }).catch(console.error);
    selectedNode.name = nextName;
    setRenameWeaponName(nextName);
  };

  const handleWeaponJointCoordChange = (jointName: string, axis: "x" | "y" | "z", valStr: string) => {
    const val = parseFloat(valStr);
    if (isNaN(val)) return;

    if (!model) return;
    const jointObj = model.joints.find(j => j.name === jointName);
    if (!jointObj) return;

    const m = jointObj.local_transform.m;
    const pos = {
      x: (axis === "x") ? val : m[3][0],
      y: (axis === "y") ? val : m[3][1],
      z: (axis === "z") ? val : m[3][2],
    };

    onPositionChange(jointName, "joint", pos);
  };

  const handleWeaponJointRotationChange = (jointName: string, m: number[][], rot: { x: number; y: number; z: number }, axis: "x" | "y" | "z", valStr: string) => {
    const val = parseFloat(valStr);
    if (isNaN(val)) return;
    const newRot = { ...rot, [axis]: val };
    const nextMatrix = updateMatrixRotation(m, newRot);
    
    const updatedJoints = model.joints.map((j) => {
      if (j.name === jointName) {
        return { ...j, local_transform: { m: nextMatrix } };
      }
      return j;
    });
    onModelChange?.({ ...model, joints: updatedJoints });
  };

  const renderJointCard = (label: string, jointObj: any) => {
    if (!jointObj) return null;
    const m = jointObj.local_transform.m;
    const pos = { x: m[3][0], y: m[3][1], z: m[3][2] };
    const rot = getEulerRotation(m);

    return (
      <div key={jointObj.name} style={{ background: "rgba(255,255,255,0.02)", padding: "12px", borderRadius: "6px", border: "1px solid var(--border-color)" }}>
        <div style={{ fontSize: "12px", fontWeight: "600", color: "var(--accent-cyan)", marginBottom: "8px", display: "flex", justifyContent: "space-between" }}>
          <span>{label}</span>
          <span style={{ color: "var(--text-muted)", fontSize: "10px" }}>{jointObj.name}</span>
        </div>

        <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "16px" }}>
          <div>
            <div style={{ display: "flex", alignItems: "center", gap: "4px", fontSize: "10px", color: "var(--text-secondary)", marginBottom: "4px" }}>
              <Move size={12} /> Position
            </div>
            <div style={{ display: "flex", gap: "4px" }}>
              {["x", "y", "z"].map((axis) => (
                <div key={axis} style={{ flex: 1 }}>
                  <NumericInput
                    step="1"
                    value={pos[axis as "x" | "y" | "z"]}
                    onChange={(val) => handleWeaponJointCoordChange(jointObj.name, axis as "x" | "y" | "z", val)}
                    style={{ height: "26px", fontSize: "11px", fontFamily: "var(--font-mono)", background: "#050a12", border: "1px solid var(--border-color)", color: "#fff", padding: "0 6px", width: "100%" }}
                  />
                </div>
              ))}
            </div>
          </div>
          <div>
            <div style={{ display: "flex", alignItems: "center", gap: "4px", fontSize: "10px", color: "var(--text-secondary)", marginBottom: "4px" }}>
              <RefreshCw size={12} /> Rotation
            </div>
            <div style={{ display: "flex", gap: "4px" }}>
              {["x", "y", "z"].map((axis) => (
                <div key={axis} style={{ flex: 1 }}>
                  <NumericInput
                    step="1"
                    value={rot[axis as "x" | "y" | "z"]}
                    onChange={(val) => handleWeaponJointRotationChange(jointObj.name, m, rot, axis as "x" | "y" | "z", val)}
                    style={{ height: "26px", fontSize: "11px", fontFamily: "var(--font-mono)", background: "#050a12", border: "1px solid var(--border-color)", color: "#fff", padding: "0 6px", width: "100%" }}
                  />
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    );
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
      <div>
        <div style={{ fontSize: "10px", textTransform: "uppercase", letterSpacing: "0.15em", color: "var(--text-muted)", marginBottom: "4px", display: "flex", alignItems: "center", gap: "6px" }}>
          <Crosshair size={12} /> Unified Weapon Assembly
        </div>
        <div style={{ display: "flex", gap: "8px", alignItems: "center", marginTop: "8px" }}>
          <input
            value={renameWeaponName}
            onChange={(e) => setRenameWeaponName(e.target.value)}
            style={{ fontSize: "14px", fontWeight: "600", color: "var(--accent-cyan)", background: "rgba(22, 160, 255, 0.05)", border: "1px solid var(--border-color)", borderRadius: "4px", padding: "4px 8px", height: "30px", flex: 1 }}
          />
          <button onClick={handleRenameWeaponGroup} className="secondary" style={{ fontSize: "11px", height: "30px", padding: "0 10px" }}>Rename</button>
        </div>
      </div>

      <hr style={{ border: "none", borderTop: "1px solid var(--border-color)", margin: 0 }} />

      <div style={{ display: "flex", flexDirection: "column", gap: "8px", background: "rgba(255,255,255,0.02)", padding: "12px", borderRadius: "4px", border: "1px solid var(--border-color)" }}>
        <div style={{ fontSize: "11px", fontWeight: "600", color: "var(--text-secondary)", marginBottom: "4px" }}>Toggle Weapon Components</div>
        
        {!posJoint && <div style={{color: "var(--accent-red)", fontSize: "11px"}}>Error: Weapon Position joint is missing!</div>}
        
        <div style={{ display: "flex", gap: "8px", flexWrap: "wrap" }}>
          {hasMuzzle ? 
            <button onClick={() => handleToggleJoint("_Muzzle", "_Position", {x:0, y:0, z:5.0})} style={{ fontSize: "11px", padding: "4px 8px", background: "rgba(255, 50, 50, 0.2)", borderColor: "rgba(255, 50, 50, 0.5)" }}>- Remove Muzzle</button> :
            <button onClick={() => handleToggleJoint("_Muzzle", "_Position", {x:0, y:0, z:5.0})} style={{ fontSize: "11px", padding: "4px 8px" }}>+ Add Muzzle (Spawn Point)</button>}
            
          {hasDirection ? 
            <button onClick={() => handleToggleJoint("_Direction", "_Position", {x:0, y:5.0, z:0})} style={{ fontSize: "11px", padding: "4px 8px", background: "rgba(255, 50, 50, 0.2)", borderColor: "rgba(255, 50, 50, 0.5)" }}>- Remove Direction</button> :
            <button onClick={() => handleToggleJoint("_Direction", "_Position", {x:0, y:5.0, z:0})} style={{ fontSize: "11px", padding: "4px 8px" }}>+ Add Direction (Up Vector)</button>}
            
          {hasLatitude ? 
            <button onClick={() => handleToggleJoint("_Latitude", "_Position", {x:0, y:5.0, z:0})} style={{ fontSize: "11px", padding: "4px 8px", background: "rgba(255, 50, 50, 0.2)", borderColor: "rgba(255, 50, 50, 0.5)" }}>- Remove Latitude</button> :
            <button onClick={() => handleToggleJoint("_Latitude", "_Position", {x:0, y:5.0, z:0})} style={{ fontSize: "11px", padding: "4px 8px" }}>+ Add Latitude (Pitch Axis)</button>}
            
          {hasRest ? 
            <button onClick={() => handleToggleJoint("_Rest", "_Position", {x:0, y:0, z:5.0})} style={{ fontSize: "11px", padding: "4px 8px", background: "rgba(255, 50, 50, 0.2)", borderColor: "rgba(255, 50, 50, 0.5)" }}>- Remove Rest</button> :
            <button onClick={() => handleToggleJoint("_Rest", "_Position", {x:0, y:0, z:5.0})} style={{ fontSize: "11px", padding: "4px 8px" }}>+ Add Rest (Yaw/Idle Vector)</button>}
        </div>
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: "12px" }}>
        {renderJointCard("Position (Pivot / Yaw)", posJoint)}
        {renderJointCard("Direction (Up Vector / Heading)", dirJoint)}
        {renderJointCard("Latitude (Pitch Axis)", latJoint)}
        {renderJointCard("Muzzle (Projectile Spawn)", muzJoint)}
        {renderJointCard("Rest (Idle Vector)", restJoint)}
      </div>
    </div>
  );
}
