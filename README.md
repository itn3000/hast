# HAsh tool written by ruST

This is file hash calculation tool inspired by [rash](https://github.com/themadprofessor/rash)

# Features

* calculating file hash
* check file hash

## Supported algorithm

* [MD5](https://en.wikipedia.org/wiki/MD5)
* [SHA1](https://en.wikipedia.org/wiki/SHA-1)
* [SHA2](https://en.wikipedia.org/wiki/SHA-2)
* [SHA3(including Shake)](https://en.wikipedia.org/wiki/SHA-3)
* [Blake2](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2)

# Installation

## download from releases

you can download precompiled single binaries from [github release page](https://github.com/itn3000/hast/releases)

## install from cargo

if you want to install via cargo, you need compilation environment.

### From source

1. checkout source from [github](https://github.com/itn3000/hast)
2. move checkout directory
3. run `cargo install`

### From github repository

1. run `cargo install --git https://github.com/itn3000/hast.git --tag [tag_version]`

# Usage

you can get help with `hast help` or `hast [subcommand] --help` command.

## calculating hash

you can get hash by `hast calc [algorithm] [options] [files...]`,
and you will get string like `[filepath],[hash]`, one file per line.
you can specify file path by globbing like `**/*.txt`
if you don't specify file, stdin used as data source and filename is "-".

### example

```
> hast calc sha2 README.md
README.md,8eefeb014f5bfe970f069a2a23433d65a7004a10cb8cb43e385f1bdfa7f077cb
> hast calc sha2 "src/*.rs"
src\calc.rs,910746602c1c94f40565573b4e7988402b9465a116e123d5eec20d2c3b301f13
src\check.rs,c36343758d644a651887b81384e7db839695ed7403dc69d0eeb1bb263594db91
src\command.rs,66da805ae20d7465cb99f251c677f39033894bb034786745871091b3dca59cdf
src\digestutil.rs,068147bfa5fcbb4f09e4ca22a05da937b43cf993df58c25e9134e1d98ebdc3b3
src\error.rs,0df5c313420c4b1c0339e1f084a72f0cc989bae1dbf25d5547d8b81a25eafc30
src\ioutil.rs,d00bd83e0ed93d2595aa632f849bfed39907cbd713f084d5539f71376ab6a92f
src\main.rs,1b6927fc80923413e4407e2a75413eb364768b7acb3809beded57bb34de05753
```

## check hash

you can check file hash between calculated value and real file.
`hast check [algorithm] [options] [file output by hast calc]`
you can specify base search directory by `-b` option.

### example

```
> hast calc sha2 README.md > result.csv
> hast check sha2 result.csv
(exit with no output if succeeded, or you will get error message)
```
