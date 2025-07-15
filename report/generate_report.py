import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import os

sns.set(style="whitegrid")
OUTPUT_DIR = "data/output"
CSV_PATH = os.path.join(OUTPUT_DIR, "simulation.results.csv")

print("Reading data...")
df = pd.read_csv(CSV_PATH, parse_dates=["timestamp"])

# Plot 1: SoC over time
plt.figure(figsize=(12, 4))
plt.plot(df["timestamp"], df["soc_kwh"], color="green")
plt.title("Battery State of Charge (SoC)")
plt.xlabel("Time")
plt.ylabel("kWh")
plt.tight_layout()
plt.savefig(os.path.join(OUTPUT_DIR, "soc_over_time.png"))
plt.close()

# Plot 2: Net grid power
plt.figure(figsize=(12, 4))
plt.plot(df["timestamp"], df["grid_net_kw"], label="Grid Net kW", color="blue")
plt.title("Net Grid Power Over Time")
plt.xlabel("Time")
plt.ylabel("Power (kW)")
plt.tight_layout()
plt.savefig(os.path.join(OUTPUT_DIR, "grid_power_over_time.png"))
plt.close()

# Plot 3: Battery in/out
plt.figure(figsize=(12, 4))
plt.plot(df["timestamp"], df["battery_in_kw"], label="Charge", color="orange")
plt.plot(df["timestamp"], df["battery_out_kw"], label="Discharge", color="red")
plt.title("Battery Power Flow")
plt.xlabel("Time")
plt.ylabel("kW")
plt.legend()
plt.tight_layout()
plt.savefig(os.path.join(OUTPUT_DIR, "battery_io.png"))
plt.close()

# Plot 4: SRL in/out energy
plt.figure(figsize=(12, 4))
plt.plot(df["timestamp"], df["srl_energy_in_kwh"], label="SRL Charge", color="purple")
plt.plot(df["timestamp"], df["srl_energy_out_kwh"], label="SRL Discharge", color="brown")
plt.title("SRL Participation Over Time")
plt.xlabel("Time")
plt.ylabel("Energy (kWh)")
plt.legend()
plt.tight_layout()
plt.savefig(os.path.join(OUTPUT_DIR, "srl_over_time.png"))
plt.close()

print("Plots saved to:", OUTPUT_DIR)

