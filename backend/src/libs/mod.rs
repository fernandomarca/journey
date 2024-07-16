mod client_prisma;

pub mod mail;
#[allow(warnings, unused)]
pub mod prisma;

pub use client_prisma::prisma;
pub use prisma::*;
