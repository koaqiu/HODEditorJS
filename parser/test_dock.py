import struct

path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_orion/ter_orion.hod"

with open(path, "rb") as f:
    data = f.read()

idx = data.find(b"DOCK")
if idx != -1:
    size = struct.unpack(">I", data[idx+4:idx+8])[0]
    payload = data[idx+8:idx+8+size]
    
    offset = 0
    first_val = struct.unpack("<I", payload[offset:offset+4])[0]
    offset += 4
    count = first_val
    if first_val >= 10:
        count = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        
    print(f"Paths count: {count}")
    for p in range(count):
        print(f"\nPath {p} starting at offset {offset}")
        
        name_len = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        name = payload[offset:offset+name_len].decode("ascii")
        offset += name_len
        print(f"  Name: {name}")
        
        parent_len = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        parent = payload[offset:offset+parent_len].decode("ascii")
        offset += parent_len
        print(f"  Parent: {parent}")
        
        val1 = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        val2 = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        val3 = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        val4 = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        val5 = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        print(f"  Vals: {val1}, {val2}, {val3}, {val4}, {val5}")
        
        ships_len = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        ships = payload[offset:offset+ships_len].decode("ascii")
        offset += ships_len
        print(f"  Compatible ships: {ships}")
        
        pad1 = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        pad2 = struct.unpack("<I", payload[offset:offset+4])[0]
        offset += 4
        print(f"  Pads: {pad1}, {pad2}")
        
        num_points = struct.unpack("<i", payload[offset:offset+4])[0]
        offset += 4
        print(f"  Num points: {num_points}")
        
        for pt in range(num_points):
            print(f"    Point {pt} at offset {offset}")
            px, py, pz = struct.unpack("<fff", payload[offset:offset+12])
            offset += 12
            m = []
            for _ in range(9):
                m.append(struct.unpack("<f", payload[offset:offset+4])[0])
                offset += 4
            tol = struct.unpack("<f", payload[offset:offset+4])[0]
            offset += 4
            spd = struct.unpack("<f", payload[offset:offset+4])[0]
            offset += 4
            extra1 = struct.unpack("<I", payload[offset:offset+4])[0]
            offset += 4
            extra2 = struct.unpack("<I", payload[offset:offset+4])[0]
            offset += 4
            print(f"      Pos: {px}, {py}, {pz} | Extra: {extra1}, {extra2}")
else:
    print("DOCK not found")


