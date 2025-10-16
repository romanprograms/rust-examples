/// Minimal DB facade to illustrate features without heavy deps.
#[cfg(feature = "postgres")]
pub fn connect_info() -> &'static str {
    "db://postgres@localhost:5432"
}

#[cfg(not(feature = "postgres"))]
pub fn connect_info() -> &'static str {
    "db://memory"
}