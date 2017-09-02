# Work Management
## Roadmap
- MS1: playable demo
		* E1: combat with options and rewards
		* E2: dungeon generation with themed monsters
- MS2: social dynamics prototype
		* E: social options
		* E: serialization
## Product Backlog
* R: allow player to proceed to next room from the loot swap scene
* R, 2: a couple of consumable items
* U, 1: 10 encounters, 1 reward each
* R: create reward item from loot table and display it for the player
* R: claim or ignore reward + move to next encounter
* R, 1: combat options: (clear +) attack + evade
* R, 1: implement stamina
* R: 3: deliver to #substance/#natsukoodi -> gather feedback on UI and mechanics
=== MS 1 ^
* R: Obsolete combat-options
* R, 2: disallow equipping and unequipping items for free (disallow nav command while in combat);
		* T: handle input should return advance:bool
		* T: convert controller state to enum
		* T: equip: set nav_state to "inventory"
		* T: unequip: set nav_state to "character"
* R: write monster pools to a file, and read them from it
* R: write item pools to a file, and read them from it
* US: create multiple keywords per area, use the same keywords for all rooms in an area
* US: generate lots of monster variants based on simple definitions of templates
* R, 1: add unique identifiers to all identifiable items, characters, monsters, etc.
* R, 0: korvaa option säädöt as_reffeillä (`Option<T> -> Option<&T>`)
* R, 0: split rpglib's character.rs into smaller files
* R, 0: käytä inventoryssä normaalia listaa (siihen saa stylet paremmin)
* R, 0: game::controller::Controller: make views use constants as key
* B: equipping an item while there's no slot for it makes the item disappear
* R, 0: aja RLS:n deglob kaikille tiedostoille
* R: save game state each turn
* R: itembuilder: consider removing requirement for effects in constructor
* R: destroy save file on player death
* R: load game state on startup
* R: implement better logging
* R: collect styles as static and reference those where needed
* US: allow built-in difficulty changer mechanic

## Legend
* Roadmap
	* milestone -> key: <description>
		* epic -> key: <description>
* Product Backlog
	* requirement -> R, <effort>: <description>
		* [T]ask: <description>
		* [B]ug: <description>
	* user story -> U, <effort>: <desription>
		* [T]ask: <description>
		* [B]ug: <description>
	* <max_effort> = 2^n, n goes from 0 to 4 (hrs: 1, 2, 4, 8, 16)

# Other
## Ideas
- difficulty: difficult but fair
- difficulty: is good, but only when the player may feel that they could've done better
- difficulty: remember to use the curve (decent frequency of high-low)
- crafting, disassembling things into materials
- combat: AP determines player actions
- Trait: organized => inventory organizes itself automatically
- useful unicode: https://github.com/globalcitizen/zomia/blob/master/USEFUL-UNICODE.md

## Wild ideas
- allow player to gain the Group modifier
- low-hp monsters may plead for mercy, unless *fearless*
- monster type keyword for determining "visuals"
	- organic: blood, entrails
	- mechanical: parts, wood, metal
- corruption
- spellcasting must be hard: blood magic, consumables
- items have main attributes, that effect the amount of damage done by the item
- it should be fun to execute the most basic interactions; this is core
- defensive cooldowns are great in a hardcore game
- crafting: collect treasure or valuable materials (greeding due to inventory slots)
- social system
- social: posture {friendly, scared, hostile)
- social: personality
- social: Plead {P: "You ask the goblin for help. The goblin gives you a food ration.", e: "You ask the goblin for help. The goblin denies your request.", N: "You ask the goblin for help, the goblin hits you with a club."}
- social: Hail {F: "You greet the goblin, the goblin greets you back.", H: "You greet the goblin. The goblin hits you with a club."}
