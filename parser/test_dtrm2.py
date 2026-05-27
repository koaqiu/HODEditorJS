import struct
import sys

with open("/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium.hod_generated.hod", "rb") as f:
    data = f.read()

def parse_chunks(data, start, end, indent=""):
    pos = start
    while pos < end - 8:
        cid = data[pos:pos+4].decode('ascii', errors='ignore')
        size = struct.unpack('<I', data[pos+4:pos+8])[0]
        
        if cid == 'FORM':
            form_type = data[pos+8:pos+12].decode('ascii', errors='ignore')
            if form_type == 'HIER':
                print(f"{indent}FORM HIER found! Size: {size}")
                first_val = struct.unpack('<I', data[pos+12:pos+16])[0]
                print(f"{indent}  first_val: 0x{first_val:08X}")
                
                # first joint
                jname_len = struct.unpack('<I', data[pos+16:pos+20])[0]
                jpos = pos + 20 + jname_len
                jpos = (jpos + 3) & ~3 # align
                rx, ry, rz = struct.unpack('<fff', data[jpos+16:jpos+28])
                print(f"{indent}  joint rot: ({rx}, {ry}, {rz})")
                
            parse_chunks(data, pos+12, pos+8+size, indent+"  ")
        pos += (size + 8 + 3) & ~3

parse_chunks(data, 0, len(data))
