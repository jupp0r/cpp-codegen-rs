Generate Code from C++ Classes [![Build Status](https://travis-ci.org/jupp0r/cpp-codegen-rs.svg?branch=master)](https://travis-ci.org/jupp0r/cpp-codegen-rs)[![Build status](https://ci.appveyor.com/api/projects/status/nov0lxhgce7dwjvl/branch/master?svg=true)](https://ci.appveyor.com/project/jupp0r/cpp-codegen-rs/branch/master)[![Coverage Status](https://coveralls.io/repos/github/jupp0r/cpp-codegen-rs/badge.svg?branch=master)](https://coveralls.io/github/jupp0r/cpp-codegen-rs?branch=master)
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

Usage
-----
cpp-codegen-rs is a source-to-source compiler. While the
input has to be very specific (C++ classes), the output can be
anything ranging from more C++ code, bindings for other languages to
documentation.

The following example creates
[GoogleTest](https://github.com/google/googletest) mock objects for
C++ classes. Usually code generation is run before the actual
compilation.

Given the following C++ header:

``` c++
#pragma once

struct Interface {
    virtual ~Interface() = default;
    virtual void method(int foo) = 0;
    virtual int foo(double) = 0;
};
```

The corresponding [GoogleTest](https://github.com/google/googletest)
mock object would look like this:

``` c++
#pragma once
#include <gmock/gmock.h>

class MockInterface : public Interface {
  MOCK_METHOD1(method, void(int));
  MOCK_METHOD1(foo, void(double));
};
```

cpp-codegen-rs parses the C++ header, and creates a `Model` object out of the abstract syntax tree defined by the header. The `Model` is then passed to a template file. To achieve the desired transformation, a suitable template looks like this:


``` c++
// THIS FILE IS GENERATED, CHANGING IT IS FUTILE
#pragma once
#include <gmock/gmock.h>

{{#each interfaces ~}}
class Mock{{name}} : public {{name}} {
{{#each methods ~}}
    MOCK_METHOD{{len arguments}}({{name}}, {{return_type}}({{#each arguments}}{{argument_type}}{{#unless @last}}, {{/unless}}{{/each}}));
{{/each ~}}
};
{{/each ~}}
```

The templating language is based on the
[HandleBars Rust](https://github.com/sunng87/handlebars-rust) library,
which should be consulted for documentation on how to write
templates. A complete template for
[GoogleTest](https://github.com/google/googletest) mock objects, which
also deals with namespaces and class templates, can be found in
[gmock.hbs](templates/gmock.hbs).

In order to perform the actual compilation, cpp-codegen-rs is invoked with the following parameters

``` bash
cpp_codegen interface.h -t templates/gmock.hbs
```

Generated code is written to stdout.

Distribution
------------
My goal is to supply statically linked (against libclang) release
binaries for Linux, OS X and Windows to ease deployment.

License
-------
MIT
