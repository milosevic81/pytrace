mod vector;  // This declares the vector module
pub use vector::Vector3;  // This re-exports Vector3 so it can be used from outside

mod color;
pub use color::Color;
