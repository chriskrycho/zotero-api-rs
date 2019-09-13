# zotero-api-rs

An implementation of the API *types* for Zotero. Note that this does *not* implement access to *read* the <abbr>API</abbr>; it simply provides deserializations and serializations of Zotero's core types, which may be used in conjunction with another library which returns <abbr>JSON</abbr> for Zotero's endpoints.

(This was built to support https://rewrite.software – which intentionally does *not* implement its data access in Rust, but on native platforms.)
