# Work Management
## Roadmap
- MS1: playable demo
		* E1: combat with options and rewards
		* E2: dungeon generation with themed monsters
## Product Backlog
- R, 0: textwrap -library to wrap text (with hyphenation)
- R, 1: combat options: <clear> + available options (each adds one entry)
- R, 3: finish dungeon generation
- R, 1: add unique identifiers to all identifiable items, characters, monsters, etc.
- R, 0: korvaa option säädöt as_reffeillä (Option<T> -> Option<&T>)
- R, 0: käytä inventoryssä normaalia listaa (siihen saa stylet paremmin)
- R, 0: game::controller::Controller: make views use constants as key
- B: equipping an item while there's no slot for it makes the item disappear

## Legend
- Roadmap
	* milestone -> key: <description>
		* epic -> key: <description>
- Product Backlog
	* requirement -> R, <effort>: <description>
		* [T]ask: <description>
		* [B]ug: <description>
	* user story -> U, <effort>: <desription>
		* [T]ask: <description>
		* [B]ug: <description>
	* <max_effort> = 2^n, n goes from 0 to 4 (hrs: 1, 2, 4, 8, 16)

# Other
## Ideas
- elävöitä combat tekstejä; kirjoita auki hit outcome [dodge/miss, block]
- difficult but fair
	- difficulty: is good, but only when the player may feel that they could've done better
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

