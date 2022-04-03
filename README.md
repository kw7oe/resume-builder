This is a simple resume builder written in Rust, using [`handlerbars`][0] and
[`serde`][1].

The code read the data from `data.yml` during compile time _(since
we use `include_str!`)_, convert it from `yaml` to `json` and pass it to
`handlerbars` to generate the final HTML.

## Usage

1. Rename the `sample_data.yml` to `data.yml`.
2. Update the data in the new `data.yml` file.
3. Run `cargo run`.
4. View the generated HTMl at `target/resume.html`

Currently, there's no way to export it as PDF. To do that, open the HTML file
in any browser and use the print function and save it as PDF.

In Firefox, it can be done by performing the steps below:

1. Open `target/resume.html`.
2. Press `CMD + P` to trigger printing.
3. Uncheck the `Print headers and footers` option.
4. Choose `Save to PDF` in the Destination input.
5. Click `Save`.

## Styling

I'm using Tailwind CSS to rapidly style my resume. Currently, we pull all the CSS
from the CDN. Purging is not implemented as it's designed to be run locally,
where the CSS size is not a concern.

## Non Goals

The design of the resume is not meant to fit everyone need. The goal is to just
share the how.

[0]: https://github.com/sunng87/handlebars-rust
[1]: https://github.com/serde-rs/serde
