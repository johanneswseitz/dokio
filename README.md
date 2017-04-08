# Dokio 

Dokio is a Wiki for Agile Software Documentation that lives in your source code repository and is super easy to use. Why not version control your documentation along with your software and keep it in sync with Dokio's advanced features that enable living documentation and automatic drift checks.

## Documentation

Dokio is documented using Dokio. Run it with `cargo run` in order to get the best out if this documentation:
[doc/index.md](doc/index.md)

## Configuration

Dokio is configured using a `Dokiofile` at the root of your Repository.

Here's an example `Dokiofile`:

    port = 4000; // default is 3000
    default_page = "usage.md"; // default is README.md
    theme = "my_company_theme"; // default is dokio

