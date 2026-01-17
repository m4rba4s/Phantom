import json
import os
import sys

def unmap(map_file, output_dir):
    print(f"[+] Loading map file: {map_file}")
    
    try:
        with open(map_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
    except Exception as e:
        print(f"[-] Failed to load map: {e}")
        return

    sources = data.get('sources', [])
    contents = data.get('sourcesContent', [])

    if not sources or not contents:
        print("[-] Map file missing 'sources' or 'sourcesContent'.")
        return

    print(f"[+] Found {len(sources)} source files.")

    for i, source_path in enumerate(sources):
        if i >= len(contents):
            break
            
        content = contents[i]
        if not content:
            continue

        # Normalize path
        # source_path often starts with webpack:// or similar
        clean_path = source_path.replace('webpack:///', '').replace('webpack://', '')
        
        # Prevent traversal
        clean_path = clean_path.replace('..', '__')
        
        full_path = os.path.join(output_dir, clean_path)
        os.makedirs(os.path.dirname(full_path), exist_ok=True)

        try:
            with open(full_path, 'w', encoding='utf-8') as out:
                out.write(content)
        except Exception as e:
            print(f"[-] Error writing {full_path}: {e}")

    print(f"[+] Extraction complete to {output_dir}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 unmap.py <file.js.map> [output_dir]")
    else:
        out = sys.argv[2] if len(sys.argv) > 2 else "src_restored"
        unmap(sys.argv[1], out)
