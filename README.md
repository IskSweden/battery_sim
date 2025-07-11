 # Battery Simulation Overview

 ## Goal
 Simulate a grid-connected battery system that:
 - Reduces peak consumption ("Peak Shaving")
 - Participates in the SRL reserve energy market
 - Tracks battery state of charge (SoC) and physical limits
 - Accounts for economics, transformer limits, and grid interaction

 ## Input (per tick from `MergedTick`)
 - timestamp: DateTime
 - power_kw: real-world consumption/load (+) or feed-in (−)
 - srl_pos_kwh: requested discharge (grid needs power)
 - srl_neg_kwh: requested charge (grid wants absorption)
 - srl_pos_price_eur_mwh, srl_neg_price_eur_mwh: optional, for revenue calc

 ## Simulation Configuration (constant across run)
 - battery.capacity_kwh
 - battery.c_rate (charge/discharge power)
 - battery.efficiency (round-trip %)
 - battery.min_soc_frac (e.g. 10%)
 - battery.initial_soc_frac (e.g. 50%)
 - reserve_fraction: percent of usable capacity kept aside for SRL
 - transformer_limit_kw: max allowed grid draw/feed

 ## What Is Simulated (per tick)
 1. Update SoC based on SRL (if any):
     - deliver SRL pos if SoC ≥ min and enough power
     - absorb SRL neg if SoC ≤ max and room to charge
     - reserve part of capacity just for SRL use

 2. Apply Peak Shaving:
     - if power_kw > 0: discharge battery to reduce grid usage
     - if power_kw < 0: charge battery to absorb excess feed-in
     - must respect SoC and SRL reserve

 3. Update SoC after both steps
 4. Compute grid_net_kw = power_kw ± battery ± srl
 5. Track transformer limit violations
 6. Track economic values:
     - SRL energy delivered/absorbed
     - energy cost savings
     - potential revenue

 ## Output
 - Vector of per-tick data (SoC, charge/discharge, srl, grid_net_kw)
 - Total:
     - peak reduction
     - SRL energy delivered/absorbed
     - number of transformer limit violations
     - number of full battery cycles
     - estimated revenue / cost savings