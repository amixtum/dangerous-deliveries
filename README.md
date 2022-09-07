![alt text](https://github.com/amixtum/dangerous-deliveries/blob/couch/dd.png?raw=true)

If you have Rust installed already just type:

    cargo run

Otherwise install Rust first

<h3>How to play:</h3>

* Display/Hide Menu = Esc 

* Apply Cellular Automata = G

* Look = Semicolon (Press a direction key afterwards to look in that direction) It's a good idea to look at what you're standing on by pressing Semicolon, then Period or Tab

* Move Left = A or H

* Move Right = D or L 

* Move Up = W or K

* Move Down = S or J 

* Move Northwest = Q or Y

* Move Northeast = E or U

* Move Southwest = Z or B

* Move Southeast = C or N 

* Wait = Period or Tab

* Restart = Enter

* Don't fall over (or into a bottomless pit of spikes)! 

* Tip: You can press G to modify the level while you are playing.

* Movement is calculated by adding an instantaneous velocity to your speed when you press a key.

<h3>Configuring</h3>

The config directory contains all data files necessary to compute the game

If you would like to add an [lsystem](https://en.wikipedia.org/wiki/L-system), or use a different one, create a new file (adhering to the format described in lsystem0.txt) in the config directory and set:

    lsystem <filename>

in config/window.txt
