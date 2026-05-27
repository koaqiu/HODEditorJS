import struct

with open("/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium.hod_generated.hod", "rb") as f:
    data = f.read()

idx = data.find(b'HIER')
if idx != -1:
    pos = idx - 8  # assuming 'FORM' + size + 'HIER'
    cid = data[pos:pos+4]
    if cid == b'FORM':
        size = struct.unpack('<I', data[pos+4:pos+8])[0]
        first_val = struct.unpack('<I', data[pos+12:pos+16])[0]
        print(f"FORM HIER size: {size}")
        print(f"first_val: 0x{first_val:08X}")
        
        jname_len = struct.unpack('<I', data[pos+16:pos+20])[0]
        jname = data[pos+20:pos+20+jname_len]
        print(f"joint name: {jname}")
        
        jpos = pos + 20 + jname_len
        jpos = (jpos + 3) & ~3
        rx, ry, rz = struct.unpack('<fff', data[jpos+16:jpos+28])
        print(f"joint rot: ({rx}, {ry}, {rz})")
