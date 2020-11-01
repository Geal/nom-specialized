# Specialized combinators for nom

This library contains specialized versions of combinators from the
[nom parser combinators library](https://github.com/geal/nom), and additional
tooling.

They are designed to provide better performance, but come with tradeoffs that
make them less suitable for the main library.


## Compilation options

compilation line to activate SSE4.2:

`RUSTFLAGS="-C target-feature=+sse2" cargo build --release`


## License

Copyright (C) 2020 Geoffroy Couprie

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
