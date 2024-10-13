# Antics!
Simulates ants 🐜 making their way through the world 🌎 looking for food 🍔

## The Ant Brain 🧠
### Input
|field             |data type|description|
|------------------|---------|-----------|
|is_carrying_food  |`boolean`|is the ant currently carrying food?|
|is_food_on_ground |`boolean`|is there food on the ground?|
|is_in_nest        |`boolean`|is the ant currently within the nest of its colony?|
|pheromone         |`Option<{scent: integer, my_colony: boolean}>`|pheromone on the ground, if any. `scent` is any non-negative number and can be used to distinguish different pheromones; `my_colony` is `true` if and only if this pheromone marker has been placed by a member of the ant's own colony|

### Action

### Formica, the Ant Language 
🐜

🗯️
|instruction   |`FLAG` |description|
|--------------|-------|-----------|
|`TURN_L`      |       |turn left  |
|`TURN_R`      |       |turn right |
|`ADVANCE`     |*write*|move forward; clear `FLAG` on success, set on failure|
|`PICKUP`      |*write*|pick up food; clear `FLAG` on success, set on failure|
|`DROP`        |*write*|drop carried food; clear `FLAG` on success, set on failure|
|`RELEASE_PH i`|       |release pheromone with scent *i* (any integer)|
|`ERASE_PH`    |       |erase any pheromone present|
|`CHECK_FOOD`  |*write*|set `FLAG` to amount of food present|
|`CHECK_PH`    |*write*|set `FLAG` to detected scent, *0* if none present|
|`CHECK_NEST`  |*write*|set `FLAG` if inside own colony's nest, clear otherwise|
|`JMP l`       |*read* |jump to instruction labelled *l*|
|`JZ l`        |*read* |jump to instruction labelled *l* if register `FLAG` contains zero|
|`JNZ l`       |*read* |jump to instruction labelled *l* unless register `FLAG` contains zero|