# MEK Reactor and Boiler Properties Calculator
## Features:
* Specify reactor size and cooling mode to calculate maximum burn rate and optimal fuel rod layout
* Specify boiler size and heating mode to calculate maximum output efficiency and optimal construction method
## Usage
1. Input length, width, and height
2. Select calculation target
3. Select cooling mode/heating mode
## Notes:
* The boiler's separator element height includes the casing height. When height is 2, it contacts the bottom.
* Both calculations do not account for environmental heat dissipation
* Reactor burn rate and coolant consumption: (slightly lower due to environmental heat dissipation)
    * Water: burn 1 produces 20,000
    * Sodium: burn 1 produces 200,000
# Principles
## What the reactor does each tick:
1. If activated, burn fuel

        Burn rate = min(burn rate, remaining fuel, (fuel rod count * rate limit per fuel rod))
            Default rate limit per fuel rod is 1
        Heat added to heat capacity = burned fuel * heat generated per unit fuel
            Default heat generated per unit fuel is 1,000,000
        If burned fuel + existing waste > waste tank capacity, overflow occurs (radiation leak, no explosion)
        
2. Process coolant

        Effective heat = boiling efficiency * (current temperature - boiling temperature) * heat capacity
        Current temperature = heat capacity heat / heat capacity
            Heat capacity = casing blocks * 1,000 = (length*width*height - (length-2)*(width-2)*(height-2)) * 1,000
            Boiling efficiency = min(1, fuel assembly surface area / fuel assembly count / 4)
            Boiling temperature = 373.15K
        Effective heat dissipation = coolant thermal conductivity * effective heat
            Water coolant: thermal conductivity 0.5, steam efficiency 0.2
            Gas coolant: thermal conductivity varies (sodium steam 1)
        Subtract effective heat dissipation from heat capacity

3. Simulate environmental cooling

        Cooling coefficient = air coefficient + insulation coefficient + thermal conductivity
            Air coefficient = 10,000
            Insulation coefficient = 10,000  
            Thermal conductivity = 10
        Cooling temperature = (heat capacity temperature - environment temperature) / cooling coefficient
            Environment temperature = 300 + 25 * (average temperature coefficient - 0.8)
            Average temperature coefficient = average of temperature coefficients at 8 corners of multiblock
            Temperature coefficient range [-5, 5], plains = 0.8
        Environmental heat dissipation = cooling temperature * heat capacity
        Subtract environmental heat dissipation from heat capacity

4. Update heat
5. Process reactor damage  
6. Process radiation

## What the boiler does each tick:

1. Environmental heat dissipation
   
        Same as reactor

2. Update heat

3. (If applicable) Consume superheated sodium and convert to heat
   
        Superheated sodium consumption = min(superheated sodium storage * cooling coefficient (0.4) * (1 - boiler temperature / coolant temperature (100,000)), remaining cooled sodium steam capacity)
        Heat increase = superheated sodium consumption * sodium enthalpy (5)

4. Consume heat to boil water
   
        Water boiling heat = (current temperature - boiling temperature) * boiler heat capacity * boiler water thermal conductivity
            Boiler water thermal conductivity default = 0.7
        Effective heat = min(water boiling heat, heating elements * heating element thermal conductivity)
            Heating element thermal conductivity default = 160,000,000
        Steam production = min(steam efficiency (0.2) * effective heat / steam enthalpy (10), water tank water, steam tank remaining capacity)
            Water tank capacity = volume below separator (excluding) * 16,000
            Steam capacity = volume above separator (including) * 160,000
        Heat reduction = steam production * steam enthalpy (10) / steam efficiency (0.2)
        
### Boiler heating modes

1. Direct heating mode:

        Maximum steam production = min(heating element heating rate / 320,000, water tank capacity, steam capacity)

2. Sodium cooling heating mode:

        Heat increase from sodium consumption >= heat decrease from water boiling + environmental heat dissipation (negligible)
        Assuming heat decrease from water boiling = water boiling heat and sodium steam won't overflow
        Temperature T = maximum steam production / heat capacity * 71.4285
        Heat consumption = maximum steam production * 50
        Maximum superheated sodium consumption = min(superheated sodium storage * cooling coefficient (0.4) * (1 - boiler temperature / coolant temperature (100,000)), remaining cooled sodium steam capacity, maximum steam production * 10)