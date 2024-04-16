/*!
# nal-stats

**nal-stats** is a statistics extension of the linear algebra library, *nalgebra* written for Rust targeting:

* General-purpose statistical analysis. Lacks almost all features currently.

## Using **nal-stats**
You will need the last stable build of the [rust compiler](https://www.rust-lang.org)
and the official package manager: [cargo](https://github.com/rust-lang/cargo).

Simply add the following to your `Cargo.toml` file:

```ignore
[dependencies]
// TODO: replace the * by the latest version.
nal_stats = "*"
```
Most useful functionalities of **nal-stats** are grouped in the root module `nal-stats::`.

However, the recommended way to use **nal-stats** is to import types and traits
explicitly.

*/

pub mod round;
pub use round::Round;

pub mod cor2;
pub use cor2::Cor2;

pub mod randn;
pub use randn::Randn;

pub mod cov;
pub use cov::Stats;
