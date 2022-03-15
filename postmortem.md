# What features you hit
Game1 has a menu, movement controls, collision detection, animation keyframes on sprites, doors and scene changes, text rendering, and an inventory. The interactive fiction game has a menu, movement controls, collision detection, animation keyframes on sprites, doors and scene changes, text rendering, and long animation sequences.
# A brief description of the game
Game1 is a exploration game, where the player finds themselves by a mysterious house that they are able to explore. The challenges involve finding keys to unlock things. But be careful what you unlock! Navigate with the arrow keys and press space to interact with an object.
Game2 "Disintegration" is an interactive fiction piece about the world possibly ending. Use the arrow keys to navigate.
# What went well, what went poorly
It was difficult to begin making the engine, because the rendering code was pretty confusing. Once we were able to set up the engine and figure out how to bitblit, things moved a lot faster. Another difficult thing was figuring out the right collision boxes for each object, because a lot of our objects were shaped weirdly. Finally, text rendering and pattern matching made our compile time a lot longer. We also had a confusing time figuring out the best way to pass and share objects (clone? reference? cope? just make a new one? mutable?)
# What you learned from playtesting
We learned that a lot of our collision boxes were off. Also that we needed to make sure that we only change the room once when the space bar is pressed, and check that the prev_key space is different than the now_key one. Otherwise we would oscillate between rooms forever.
# What you learned from coding it up (e.g. for your second game, how easily could you reuse code meant for the first game?).
The second game was made in less than 24 hours. The first took weeks to make. Our two games were pretty similar, although one is an exploration game and one is an interactive fiction piece. We were able to reuse most of the code but added a way to do longer animation and had to add a few new fields to some of the structs. 
