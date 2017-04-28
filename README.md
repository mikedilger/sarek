# Sarek

THIS CRATE IS IN VERY EARLY DEVELOPMENT and may not persist, so use at your own risk!

This crate provides a high-level rust interface to the Khronos Group's Vulkan API.
It uses Dennis Hamester's `vk-sys` (https://gitlab.com/dennis-hamester/vk-sys) for the
low-level interface.

Like most high-level interfaces in rust, it attempts to expose all functionality without
making any decisions for you.

Unlike the popular Vulkan interface `vulkano` (https://github.com/tomaka/vulkano), it does
not attempt to keep you perfectly safe, and thus it is much more simply organized. Vulkan
is a complex API with many complex requirements and rules. In order to expose all functionality
while ensuring adherence to all of the rules, complex structures and type system techniques
are required. Such work is ongoing in the vulkano crate, but that work is incomplete (and will
probably remain incomplete for some time). Herein, we head out on a different tack, hiding
the lowest level details in a safe manner, but not ensuring the higher level rules are adhered
to.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE)
    or http://www.apache.org/licenses/LICENSE-2.0)

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
