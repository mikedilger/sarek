# Sarek

THIS CRATE IS IN VERY EARLY DEVELOPMENT and may not persist, so use at your own risk!

This crate provides a high-level rust interface to the Khronos Group's Vulkan API.
It uses Dennis Hamester's `vk-sys` (https://gitlab.com/dennis-hamester/vk-sys) for the
low-level interface.

Like most high-level interfaces in rust, it attempts to expose all functionality without
making any decisions for you.

* Low-level unsafe types and function calls are abstracted away. You should not need to write
  any unsafe code.
* Types use rust-friendly snake case naming. In many cases, these are bit-for-bit with the C
  types and memory is simply transmuted (a compiler sugar no-op). However in some cases the
  data is modified to be more rust friendly (for example: String) or to save memory (for
  exaple, a bit vector may be used rather than a long struct of many 32-bit boolean fields).
* Some functions are re-organized as methods of objects where it is logical to do so.
* Names are sometimes changed, usually shortened, especially where methods would be repeating
  the name of the object they are called on.
* Fields which cannot be otherwise are, where possible, hidden from you (e.g. the sType fields,
  the pNext field because we will help handle the extensions, etc).
* Where code couldn't reasonably be otherwise, we handle the details for you (e.g. we call
  into VkEnumeratePhysicalDevices twice, first to get a count, and again after we allocate
  a place for the results, but we expose this as a single call returning a Vec).

We do NOT attempt to meet ALL of your Vulkan API responsibilites. The Vulkan API is rather
complex, and many requirements and invariants are stated in the standard. In order to ensure
adherance to all of these rules, more complicated data structure organisation and type system
tricks would be required. This is no small job, and we punt on it. If you want such a thing,
I direct your attention to the `vulkano` (https://github.com/tomaka/vulkano) crate, where
such work is ongoing.

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
