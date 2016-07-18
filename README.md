A Code Generator for C++ Interfaces
===================================
cpp-codegen-rs uses libclang to read C++ class definitions and
generate code. Example use cases include the generation of Google Mock
Classes, Reflection Libraries, (De)Serialization, RPC Frameworks, and
anything else that suffers from a lack of proper compile-time
reflection in C++. The underlying concept is that it's sometimes
preferrable to use actual code as the IDL to generate these things as
supposed to a dedicated IDL.

Project Status
--------------
This is currently alpha. Expect bugs and API changes.

Distribution
------------
My goal is to supply statically linked (against libclang) release
binaries for Linux, OS X and Windows to ease deployment.

License
-------
MIT
