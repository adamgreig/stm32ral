#[cfg(any(feature="doc", feature="armv6_m", feature="armv7e_m", feature="armv7_m"))]
pub mod dcb;

#[cfg(any(feature="doc", feature="armv6_m", feature="armv7e_m", feature="armv7_m"))]
pub mod dwt;

#[cfg(any(feature="doc", feature="armv6_m", feature="armv7e_m", feature="armv7_m"))]
pub mod syst;

#[cfg(any(feature="doc", feature="armv7e_m", feature="armv7_m"))]
pub mod cpb;

#[cfg(any(feature="doc", feature="armv7e_m", feature="armv7_m"))]
pub mod cpuid;

#[cfg(any(feature="doc", feature="armv7e_m", feature="armv7_m"))]
pub mod fpb;

#[cfg(any(feature="doc", feature="armv7e_m", feature="armv7_m"))]
pub mod itm;

#[cfg(any(feature="doc", feature="armv7e_m", feature="armv7_m"))]
pub mod tpiu;

