import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np

# === Step 1: Load four CSVs ===
csv_files = {
    "BZip2_RustMap": "cognitive_complexity_bzip2_rs_gpt.csv",
    "BZip2_C2Rust": "cognitive_complexity_bzip2-c2rust.csv",
    "Rosetta_RustMap": "cognitive_complexity_rosetta_code_gpt.csv",
    "Rosetta_C2Rust": "cognitive_complexity_rossta-c2rust.csv"
}

data = []
for label, path in csv_files.items():
    try:
        df = pd.read_csv(path)
        project, method = label.split("_")
        for val in df["cognitive_complexity"]:
            data.append({
                "project": project,
                "method": method,
                "complexity": val
            })
    except Exception as e:
        print(f"Failed to load {path}: {e}")

df_all = pd.DataFrame(data)

# 仅查看 Rosetta 部分
rosetta_values = df_all[df_all["project"] == "Rosetta"]["complexity"]

# 输出数值统计摘要
print(rosetta_values.describe())

# === Step 2: Plot ===
sns.set(style="whitegrid")
fig, axes = plt.subplots(1, 2, figsize=(10, 6), sharey=True)

# Subplot (a): BZip2
sns.violinplot(
    data=df_all[df_all["project"] == "BZip2"],
    x="method", y="complexity",
    ax=axes[0],
    inner="box", density_norm="width"
)
axes[0].set_title("(a) BZip2")
axes[0].set_ylabel("Cognitive Complexity")
axes[0].set_xlabel("")
axes[0].set_yscale("log")
axes[0].set_ylim(0.001, 1000) 

# Subplot (b): Rosetta
sns.violinplot(
    data=df_all[df_all["project"] == "Rosetta"],
    x="method", y="complexity",
    ax=axes[1],
    inner="box"
)
axes[1].set_title("(b) Rosetta")
axes[1].set_xlabel("")
axes[1].set_ylabel("")
axes[1].set_yscale("log")
axes[1].set_ylim(0.001, 1000)  


# Final formatting
fig.suptitle("Figure 7: Comparison of Complexity between RustMap and C2Rust")
plt.tight_layout(rect=[0, 0.03, 1, 0.95])
output_file = "violin_comparison_trimmed_rosetta.png"
plt.savefig(output_file)
plt.show()

print(f"Plot saved to {output_file}")
