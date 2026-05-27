import struct
import sys

def find_chunk(data, chunk_id, start_pos=0):
    pos = start_pos
    while pos < len(data) - 8:
        cid = data[pos:pos+4].decode('ascii', errors='ignore')
        size = struct.unpack('<I', data[pos+4:pos+8])[0]
        if cid == chunk_id:
            return pos, size
        pos += (size + 3) & ~3
    return -1, -1

with open("/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium.hod_generated.hod", "rb") as f:
    data = f.read()

dtrm_pos, dtrm_size = find_chunk(data, "DTRM")
if dtrm_pos == -1:
    print("DTRM not found!")
    sys.exit(1)

print(f"DTRM found at {dtrm_pos}, size {dtrm_size}")
dtrm_data = data[dtrm_pos+12:dtrm_pos+dtrm_size] # skip FORM DTRM size
pos = 0
burn_count = 0
hier_first_val = None

while pos < len(dtrm_data) - 8:
    cid = dtrm_data[pos:pos+4].decode('ascii', errors='ignore')
    size = struct.unpack('<I', dtrm_data[pos+4:pos+8])[0]
    ctype = dtrm_data[pos+8:pos+12].decode('ascii', errors='ignore')
    if cid == "HIER":
        print(f"Found HIER, size {size}, type {ctype}")
        if ctype == "FORM":
            hier_first_val = struct.unpack('<I', dtrm_data[pos+12:pos+16])[0]
            print(f"  first_val: {hex(hier_first_val)}")
            
            # Read first joint rotation
            jname_len = struct.unpack('<I', dtrm_data[pos+16:pos+20])[0]
            jpos = pos + 20 + jname_len
            jpos = (jpos + 3) & ~3
            rx, ry, rz = struct.unpack('<fff', dtrm_data[jpos+16:jpos+28])
            print(f"  first joint rotation: ({rx}, {ry}, {rz})")
            
    elif cid == "BURN":
        burn_count += 1
        print(f"Found BURN {burn_count}, size {size}")
    
    pos += (size + 3) & ~3

print(f"Total BURN chunks: {burn_count}")
