import struct

with open("/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium.hod_generated.hod", "rb") as f:
    data = f.read()

import re
for match in re.finditer(b'FORM....HIER', data, re.DOTALL):
    pos = match.start()
    size = struct.unpack('<I', data[pos+4:pos+8])[0]
    first_val = struct.unpack('<I', data[pos+12:pos+16])[0]
    print(f"FORM HIER size: {size}")
    print(f"first_val: 0x{first_val:08X}")
    
    jname_len = struct.unpack('<I', data[pos+16:pos+20])[0]
    jname = data[pos+20:pos+20+jname_len].decode('ascii', errors='ignore')
    print(f"joint name: {jname}")
    
    jpos = pos + 20 + jname_len
    jpos = (jpos + 3) & ~3
    
    # Read transform
    tx, ty, tz = struct.unpack('<fff', data[jpos+4:jpos+16])
    rx, ry, rz = struct.unpack('<fff', data[jpos+16:jpos+28])
    sx, sy, sz = struct.unpack('<fff', data[jpos+28:jpos+40])
    
    print(f"joint pos: ({tx}, {ty}, {tz})")
    print(f"joint rot: ({rx}, {ry}, {rz})")
    print(f"joint scale: ({sx}, {sy}, {sz})")
