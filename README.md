# fmtapp: Contrast Rust `format!()` with `to_string() +`
Bart Massey 2021

This Criterion benchmark tries to capture the difference
between the following ways of appending two `&str`s to get a
`String`:

    format!("{}{}", s1, s2)
    s1.to_string() + s2

The data shows roughly a constant 20ns difference on my
box. For strings above 30 characters or so, both methods
perform roughly constant-time at about 60ns for `format!()`
and 80ns for `to_string() +`. For empty strings, `format!()`
takes about 20ns and `to_string() +` takes about 10ns.

tl;dr: If you are really concerned about performance, use
`to_string() +`.

To measure on your machine, install `cargo-criterion` and
run this with `cargo criterion`.
