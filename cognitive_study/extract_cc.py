import os
import json
import pandas as pd

def extract_cognitive_complexity(base_dir, project_name, exclude_folders={"unit_test"}):
    records = []
    for root, _, files in os.walk(base_dir):
        for file in files:
            if not file.endswith(".json"):
                continue

            full_path = os.path.join(root, file)

            if project_name in {"rossta-c2rust"} and "src" not in os.path.relpath(full_path, base_dir):
                continue

            try:
                with open(full_path, "r") as f:
                    data = json.load(f)

                    folder = get_logical_folder(root, base_dir)
                    if folder in exclude_folders:
                        continue

                    # === File-level ===
                    if project_name in {"rossta-c2rust", "rosetta_code_gpt"}:
                        total_cognitive = sum(
                            item["metrics"]["cognitive"]["sum"]
                            for item in data.get("spaces", [])
                            if "metrics" in item and "cognitive" in item["metrics"]
                        )

                        records.append({
                            "folder": folder,
                            "file": file,
                            "cognitive_complexity": total_cognitive
                        })

                    # === Function-level ===
                    else:
                        for item in data.get("spaces", []):
                            if "metrics" in item and "cognitive" in item["metrics"]:
                                records.append({
                                    "folder": folder,
                                    "file": file,
                                    "function": item.get("name", ""),
                                    "cognitive_complexity": item["metrics"]["cognitive"]["sum"]
                                })

            except Exception as e:
                print(f"Error processing {full_path}: {e}")
    return pd.DataFrame(records)

def get_logical_folder(path, base):
    parts = os.path.relpath(path, base).split(os.sep)
    if "src" in parts:
        idx = parts.index("src")
        if idx + 1 < len(parts):
            return parts[idx + 1]
    elif len(parts) >= 1:
        return parts[0]
    return "unknown"

if __name__ == "__main__":
    base_directories = [
        "results/bzip2_rs_gpt",
        "results/bzip2-c2rust",
        "results/rosetta_code_gpt",
        "results/rossta-c2rust"
    ]

    for base_dir in base_directories:
        project_name = os.path.basename(base_dir)
        df = extract_cognitive_complexity(base_dir, project_name)
        if not df.empty:
            output_csv = f"cognitive_complexity_{project_name}.csv"
            df.to_csv(output_csv, index=False)
            print(f"Saved {len(df)} entries to {output_csv}")
        else:
            print(f"No data extracted from {base_dir}")
