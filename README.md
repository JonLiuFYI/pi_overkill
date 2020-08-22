# pi_overkill 0.2.2
© Copyright 2020 JonLiuFYI

Discover the value of pi the long way. Fun fact: the probability of two random positive integers being coprime is `6 / pi^2`.

Made in Rust.

## Usage
* Change the parameters at the top of the `main` function of main.rs to the values you want.
* Build and run this project. Use `cargo run --release` in your terminal.
* Rerun this any number of times and see what this program thinks pi is.

## License
pi_overkill source code is licensed under GNU GPLv3+. See LICENSE.

pi_overkill is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

pi_overkill is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see https://www.gnu.org/licenses/.

## Changelog
### 0.2.2
* Removed an unnecessary instance of cloning the Sender

### 0.2.1
* Fixed off-by-one error that made one too many Senders
* Display more information about the pi estimate
  * Total iterations
  * Number of threads
  * Percentage error relative to Rust's builtin `std::f64::consts::PI`

### 0.2
* Added multithreading with message passing, but it's hardcoded
  * Enjoy running 12 threads with 4 million iterations each
