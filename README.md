![alt text](https://github.com/amixtum/dangerous-deliveries/blob/couch/dd.png?raw=true)

If you have Rust installed already just type:

    cargo run

Otherwise install Rust first

<h3>How to play:</h3>

* Display/Hide Help = 0 (zero)

* Look = Semicolon (Press a direction key afterwards to look in that direction)

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

* Exit = Esc

* Don't fall over (or into a bottomless pit of spikes)!

<h3>Configuring</h3>

The config directory contains all data files necessary to compute the game

If you would like to add an lsystem, create a new file (adhering to the format in the existing lsystem files) in the config directory and set:

    lsystem <filename>

in config/game.txt

additionally config/table.txt contains values that affect how the lsystem is used for map generation 

All other values are commented to explain how they affect the model (explained more in design/doc.txt)
