Using multithreading got time to just over 2 mins on first run on an old laptop.

can easily improve by using one random generation to get 3 results - if x uniform random in [0, 1) then add 3 if x < 1/64, 2 if x < 1/16, 1 if x < 1/4, and 0 otherwise.
