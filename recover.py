import json
import sys

transcript_path = "/home/guillermo/.gemini/antigravity-cli/brain/c958e19b-8752-4e8a-94b5-d461bfd8db35/.system_generated/logs/transcript_full.jsonl"
out_path = "recovered_hod.rs"

with open(transcript_path, 'r') as f:
    for line in f:
        data = json.loads(line)
        # Search the entire JSON string just in case
        if "pub fn synthesize_engine_nozzles_v1" in line:
            print("Found it!")
            # Extract the raw string that looks like Rust code
            # Let's just dump the pretty-printed JSON to a file so I can grep it!
            with open("dump.json", 'w') as out:
                json.dump(data, out, indent=2)
            sys.exit(0)
