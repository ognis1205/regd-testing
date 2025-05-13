// Copyright 2025 Shingo OKAWA. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module contains a set of testing utilities of random value generators.

use std::fs;

use rand::Rng;
use rand::distr::uniform::{SampleRange, SampleUniform};
use rand::distr::{Alphanumeric, StandardUniform};
use rand::prelude::Distribution;

/// Generates a random value of type `T`.
///
/// This function uses the default random number generator to produce a value of type `T`.
/// The type `T` must implement the `Distribution` trait for `StandardUniform`.
///
/// # Returns
/// - A randomly generated value of type `T`.
///
/// # Examples
/// ```
/// use regd_testing;
///
/// let x: u32 = regd_testing::rand::generate();
///
/// println!("Generated value: {}", x);
///
/// // Output (example):
/// // Generated value: 1739284512  // actual value will vary
/// ```
///
/// # Panics
/// - This function may panic if `T` does not implement `Distribution` for `StandardUniform`.
pub fn generate<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    let mut rng = rand::rng();
    rng.random::<T>()
}

/// Generates a random value of type `T` within the specified range.
///
/// This function returns a randomly selected value of type `T` from the provided range.
/// The type `T` must implement `SampleUniform`, and the range must implement `SampleRange<T>`.
///
/// # Parameters
/// - `range`: The range from which to generate a random value.
///
/// # Returns
/// - A randomly generated value of type `T` within the specified range.
///
/// # Examples
/// ```
/// use regd_testing;
///
/// let x: i32 = regd_testing::rand::generate_range(10..20);
///
/// println!("Generated value: {}", x);
///
/// let y: f64 = regd_testing::rand::generate_range(1.0..5.0);
///
/// println!("Generated value: {}", y);
///
/// // Output (example):
/// // Generated value: 13
/// // Generated value: 3.2748123901  // actual values will vary
/// ```
///
/// # Panics
/// - This function will panic if the provided range is empty.
pub fn generate_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    assert!(!range.is_empty(), "cannot sample empty range");
    let mut rng = rand::rng();
    rng.random_range(range)
}

/// Generates a vector of random bytes of the specified length.
///
/// This function returns a `Vec<u8>` filled with random byte values (`u8`)
/// generated using the thread-local random number generator.
///
/// # Parameters
/// - `length`: The number of random bytes to generate.
///
/// # Returns
/// - A `Vec<u8>` containing `length` random bytes.
///
/// # Examples
/// ```
/// use regd_testing;
///
/// let x = regd_testing::rand::generate_bytes(16);
///
/// assert_eq!(x.len(), 16);
///
/// println!("Generated value: {:?}", x);
///
/// // Output (example):
/// // Generated value: [152, 34, 78, 200, 15, 99, 142, 4, 11, 87, 201, 190, 62, 250, 73, 12]
/// // (actual value will vary)
/// ```
pub fn generate_bytes(length: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    (0..length).map(|_| rng.random::<u8>()).collect()
}

/// Generates a random alphanumeric string of the specified length.
///
/// This function creates a string consisting of randomly selected characters from the
/// alphanumeric set (`A-Z`, `a-z`, `0-9`) using the thread-local random number generator.
///
/// # Parameters
/// - `length`: The length of the generated string.
///
/// # Returns
/// - A `String` containing `length` randomly chosen alphanumeric characters.
///
/// # Examples
/// ```
/// use regd_testing;
///
/// let x = regd_testing::rand::generate_alphanumeric(12);
///
/// assert_eq!(x.len(), 12);
///
/// println!("Generated value: {}", x);
///
/// // Output (example):
/// // Generated value: "aZ8kD9fQwL2x"  // actual value will vary
/// ```
pub fn generate_alphanumeric(length: usize) -> String {
    let rng = rand::rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generates a random alphanumeric filename that does not exist in the current directory.
///
/// This function creates a random alphanumeric string of the specified length,
/// checks whether a file with that name already exists in the current working directory,
/// and returns it only if the name is **not** already used. This ensures that the generated
/// filename can safely be used for temporary files or testing without clashing with existing files.
///
/// # Parameters
/// - `length`: The length of the generated filename. Must be greater than 0.
///
/// # Returns
/// - A `String` representing a randomly generated, non-existent filename.
///
/// # Examples
/// ```
/// use regd_testing;
///
/// let x = regd_testing::rand::generate_badfile(12);
///
/// assert!(std::fs::metadata(&x).is_err()); // Confirm file does not exist
///
/// println!("Generated unique filename: {}", x);
///
/// // Output (example):
/// // Generated unique filename: "nonexistent_file_X8v9L1qKzW0a"
/// // (path is unique and file is guaranteed not to exist)
/// ```
///
/// # Panics
/// - This function will panic if `length == 0`.
///
/// # Notes
/// - The function uses a loop and may retry multiple times if name collisions occur,
///   although with a reasonable `length` (e.g., ≥8), collisions are very unlikely.
/// - The check is limited to the **current working directory**.
pub fn generate_badfile(length: usize) -> String {
    assert!(length > 0, "cannot sample empty file name");
    loop {
        let rng = rand::rng();
        let filename: String = rng
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}
