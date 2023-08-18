# openweather-async2
A library building off of
[kilroyjones/openweather-async](https://github.com/kilroyjones/openweather-async) with
some updates to support API changes upstream and more recent dependencies.

Made for use in a Rust version of the
[openweathermap-fullfeatured](https://github.com/polybar/polybar-scripts/tree/master/polybar-scripts/openweathermap-fullfeatured)
weather script for [polybar](https://github.com/polybar/polybar).  I have ripped out a
lot of old methods from the upstream, while adding basic support for the `/forecast`
(<https://openweathermap.org/forecast5#data>) API 2.5 endpoint, as I just don't want to
support something I'm not using.

Don't know if I will upload this as a crate yet.  Needs polishing once I'm done with it.
