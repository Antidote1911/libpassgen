//! # libpassgen
//!
//! `libpassgen` crate for generating randoms passwords

use indexmap::set::Iter;
use indexmap::IndexSet;
use rand::Rng;
use std::char::ParseCharError;
use std::fmt;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Collection of unique chars. This is wrapper for [`IndexSet<char>`]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pool(IndexSet<char>);

impl Deref for Pool {
    type Target = IndexSet<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<char> for Pool {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut pool = Pool::new();
        pool.0 = IndexSet::from_iter(iter);

        pool
    }
}

impl Extend<char> for Pool {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl FromStr for Pool {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pool(s.chars().collect::<IndexSet<char>>()))
    }
}

impl fmt::Display for Pool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self::new()
    }
}

impl Pool {
    /// Create new empty pool
    pub fn new() -> Self {
        Pool(IndexSet::new())
    }

    /// Return number of chars in the pool
    ///
    /// # Examples
    /// ```
    /// # use libpassgen::Pool;
    /// let pool: Pool = "0123456789".parse().unwrap();
    ///
    /// assert_eq!(pool.len(), 10)
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Extracts all chars from string and adds them to the pool
    pub fn extend_from_string(&mut self, s: &str) -> &mut Self {
        self.0.extend(s.chars().collect::<IndexSet<char>>());

        self
    }

    /// Returns true if pool contains no elements
    ///
    /// # Examples
    /// ```
    /// # use libpassgen::Pool;
    /// let pool = Pool::new();
    ///
    /// assert!(pool.is_empty())
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get char by index
    pub(crate) fn get(&self, index: usize) -> Option<&char> {
        self.0.get_index(index)
    }

    /// Check if char exists in the pool
    ///
    /// # Examples
    /// ```
    /// # use libpassgen::Pool;
    /// let pool: Pool = "ABCDEFG".parse().unwrap();
    ///
    /// assert!(pool.contains('D'))
    /// ```
    pub fn contains(&self, ch: char) -> bool {
        self.0.contains(&ch)
    }

    /// Returns true if pool contains each char from the string `elements`
    ///
    /// # Examples
    /// ```
    /// # use libpassgen::Pool;
    /// let pool: Pool = "ABCDEFG".parse().unwrap();
    ///
    /// assert!(pool.contains_all("DAG"))
    /// ```
    pub fn contains_all(&self, elements: &str) -> bool {
        self.0
            .is_superset(&elements.chars().collect::<IndexSet<char>>())
    }

    /// Insert char to pool.
    /// If an equivalent char already exists in the pool, then the pool is not changed.
    #[allow(dead_code)]
    pub(crate) fn insert(&mut self, ch: char) {
        self.0.insert(ch);
    }

    /// Returns iterator
    pub fn iter(&self) -> Iter<'_, char> {
        self.0.iter()
    }

    /// Remove char from pool. Like a [Vec::swap_remove]
    pub fn swap_remove(&mut self, ch: &char) -> bool {
        self.0.swap_remove(ch)
    }

    /// Remove char from pool. Like a [Vec::remove]
    pub fn shift_remove(&mut self, ch: &char) -> bool {
        self.0.shift_remove(ch)
    }

    /// Remove all chars of the string `elements` from pool
    pub fn remove_all(&mut self, elements: &str) {
        elements.chars().for_each(|ch| {
            self.swap_remove(&ch);
        });
    }

    /// Sorts the chars in the pool
    ///
    /// # Examples
    /// ```
    /// # use libpassgen::Pool;
    /// # use std::str::FromStr;
    /// let mut pool = Pool::from_str("31524").unwrap();
    /// pool.sort();
    ///
    /// assert_eq!(pool, Pool::from_str("12345").unwrap())
    /// ```
    pub fn sort(&mut self) {
        self.0.sort()
    }
}

/// Generate random password.
///
/// # Examples
/// ```
/// # use libpassgen::{Pool, generate_password};
/// let pool = "0123456789".parse().unwrap();
/// let password = generate_password(&pool, 15);
///
/// assert_eq!(password.chars().count(), 15);
/// ```
///
/// # Panics
/// Panics if `pool` is empty.
pub fn generate_password(pool: &Pool, length: usize) -> String {
    assert!(!pool.is_empty(), "Pool contains no elements!");

    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, pool.len());
            *pool.get(idx).unwrap()
        })
        .collect()
}

/// Generate multiple random passwords.
///
/// # Examples
/// ```
/// # use libpassgen::{Pool, generate_n_passwords};
/// let pool = "0123456789".parse().unwrap();
/// let vec_passwords = generate_n_passwords(&pool, 15,5);
///
/// assert!(!vec_passwords[2].is_empty());
/// ```
///
/// # Panics
/// Panics if `pool` is empty.
pub fn generate_n_passwords(pool: &Pool, length: usize, count: usize) -> Vec<String> {
    assert!(!pool.is_empty(), "Pool contains no elements!");

    let mut vec: Vec<String> = Vec::new();
    for n in 0..count {
        let pass = generate_password(pool, length);
        vec.insert(n, pass.clone());
    }
    vec
}

/// Calculates entropy.
///
/// # Examples
/// ```
/// # use libpassgen::calculate_entropy;
///
/// assert_eq!(calculate_entropy(12, 64), 72_f64);
/// ```
pub fn calculate_entropy(length: usize, pool_size: usize) -> f64 {
    length as f64 * (pool_size as f64).log2()
}

/// Calculates the minimum password length required to obtain a given entropy.
///
/// # Examples
/// ```
/// # use libpassgen::calculate_length;
///
/// assert_eq!(calculate_length(128_f64, 64_f64), 22_f64);
/// ```
pub fn calculate_length(entropy: f64, pool_size: f64) -> f64 {
    (entropy / pool_size.log2()).ceil()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_deref_mut() {
        let mut pool = Pool::from_str("12345").unwrap();
        *pool = "abcde".chars().collect::<IndexSet<char>>();

        assert_eq!(*pool, "abcde".chars().collect::<IndexSet<char>>())
    }

    #[test]
    fn pool_deref() {
        let pool = Pool::from_str("12345").unwrap();

        assert_eq!(*pool, "12345".chars().collect::<IndexSet<char>>())
    }

    #[test]
    fn pool_sort() {
        let mut pool = Pool::from_str("31524").unwrap();
        pool.sort();

        assert_eq!(pool, Pool::from_str("12345").unwrap())
    }

    #[test]
    fn pool_extend() {
        let mut pool = Pool::from_str("abc").unwrap();
        pool.extend(vec!['d', 'e']);

        assert_eq!(pool, Pool::from_str("abcde").unwrap())
    }

    #[test]
    fn pool_from_iter() {
        let iter = vec!['a', 'b', 'c'].into_iter();

        assert_eq!(iter.collect::<Pool>(), Pool::from_str("abc").unwrap());
    }

    #[test]
    fn pool_remove_all() {
        let mut pool: Pool = "abcde".parse().unwrap();
        pool.remove_all("ace");

        assert_eq!(pool, "bd".parse::<Pool>().unwrap());
    }

    #[test]
    fn pool_swap_remove() {
        let mut pool: Pool = "abcdefz".parse().unwrap();

        assert!(pool.swap_remove(&'b'));
        assert_eq!(pool.get(1), Some(&'z'));
        assert_eq!(pool.get(6), None);
    }

    #[test]
    fn pool_shift_remove() {
        let mut pool: Pool = "abcdefz".parse().unwrap();

        assert!(pool.shift_remove(&'b'));
        assert_eq!(pool.get(1), Some(&'c'));
        assert_eq!(pool.get(6), None);
    }

    #[test]
    fn pool_iter() {
        let pool: Pool = "abcdefz".parse().unwrap();
        let mut iter = pool.iter();

        assert_eq!(iter.next(), Some(&'a'));
        assert_eq!(iter.next(), Some(&'b'));
        assert_eq!(iter.last(), Some(&'z'));
    }

    #[test]
    fn pool_display() {
        let pool: Pool = "0123456789".parse().unwrap();

        assert_eq!(pool.to_string(), "0123456789".to_owned());
    }

    #[test]
    fn pool_contains_all() {
        let pool: Pool = "0123456789".parse().unwrap();

        assert!(pool.contains_all("2357"));
    }

    #[test]
    fn pool_contains_all_assert_false() {
        let pool: Pool = "0123456789".parse().unwrap();

        assert!(!pool.contains_all("0123F"));
    }

    #[test]
    fn pool_contains() {
        let pool: Pool = "0123456789".parse().unwrap();

        assert!(pool.contains('5'));
    }

    #[test]
    fn pool_contains_assert_false() {
        let pool: Pool = "0123456789".parse().unwrap();

        assert!(!pool.contains('A'));
    }

    #[test]
    fn pool_get() {
        let pool: Pool = "ABCD".parse().unwrap();

        assert_eq!(pool.get(0), Some(&'A'))
    }

    #[test]
    fn pool_is_empty() {
        let pool = Pool::new();

        assert!(pool.is_empty());
    }

    #[test]
    fn pool_is_empty_assert_false() {
        let pool = Pool::from_str("0123456789").unwrap();

        assert!(!pool.is_empty());
    }

    #[test]
    fn pool_len() {
        let pool: Pool = "0123456789".parse().unwrap();

        assert_eq!(pool.len(), 10)
    }

    #[test]
    fn pool_insert() {
        let mut pool = "ABC".parse::<Pool>().unwrap();
        pool.insert('D');

        assert_eq!(pool, "ABCD".parse::<Pool>().unwrap())
    }

    #[test]
    fn pool_extend_from_string() {
        let mut pool = "ABC".parse::<Pool>().unwrap();
        let mut other_pool = pool.clone();

        other_pool.insert('D');
        pool.extend_from_string("D");

        assert_eq!(other_pool, pool)
    }

    #[test]
    fn pool_from_string() {
        let indexset: IndexSet<_> = "0123456789".chars().collect();

        assert_eq!(Pool(indexset), "0123456789".to_owned().parse().unwrap())
    }

    #[test]
    fn pool_from_str() {
        let indexset: IndexSet<_> = "0123456789".chars().collect();

        assert_eq!(Pool(indexset), "0123456789".parse().unwrap())
    }

    #[test]
    fn generate_password_assert_len() {
        let pool = "0123456789".chars().collect::<IndexSet<char>>();
        let password = generate_password(&Pool(pool), 15);

        assert_eq!(password.chars().count(), 15);
    }

    #[test]
    fn generate_n_passwords_assert_count() {
        let pool = "0123456789".chars().collect::<IndexSet<char>>();
        let vec_passwords = generate_n_passwords(&Pool(pool), 15, 100);

        assert_eq!(vec_passwords.len(), 100);
    }

    #[test]
    fn generate_n_passwords_assert_len() {
        let pool = "0123456789".chars().collect::<IndexSet<char>>();
        let vec_passwords = generate_n_passwords(&Pool(pool), 15, 100);

        assert_eq!(vec_passwords[3].len(), 15);
    }

    #[test]
    #[should_panic(expected = "Pool contains no elements!")]
    fn generate_password_passed_empty_pool() {
        let pool = "".chars().collect::<IndexSet<char>>();

        generate_password(&Pool(pool), 15);
    }

    #[test]
    fn calculate_entropy_assert_true() {
        let entropy = calculate_entropy(12, 64);

        assert_eq!(entropy, 72_f64);
    }

    #[test]
    fn calculate_entropy_passed_length_is_0() {
        let entropy = calculate_entropy(0, 64);

        assert_eq!(entropy, 0_f64)
    }

    #[test]
    fn calculate_entropy_passed_pool_size_is_0() {
        let entropy = calculate_entropy(12, 0);

        assert_eq!(entropy, f64::NEG_INFINITY)
    }

    #[test]
    fn calculate_entropy_passed_pool_size_is_1() {
        let entropy = calculate_entropy(12, 1);

        assert_eq!(entropy, 0_f64)
    }

    #[test]
    fn calculate_length_assert_true() {
        let length = calculate_length(128_f64, 64_f64);

        assert_eq!(length, 22_f64);
    }

    #[test]
    fn calculate_length_entropy_is_0() {
        let length = calculate_length(0_f64, 64_f64);

        assert_eq!(length, 0_f64);
    }

    #[test]
    fn calculate_length_pool_size_is_0() {
        let length = calculate_length(128_f64, 0_f64);

        assert_eq!(length, 0_f64);
    }

    #[test]
    fn calculate_length_entropy_and_pool_size_is_0() {
        let length = calculate_length(0_f64, 0_f64);

        assert_eq!(length, 0_f64);
    }

    #[test]
    fn calculate_length_entropy_is_0_and_pool_size_is_1() {
        let length = calculate_length(0_f64, 1_f64);

        assert!(length.is_nan());
    }

    #[test]
    fn calculate_length_entropy_is_1_and_pool_size_is_1() {
        let length = calculate_length(1_f64, 1_f64);

        assert_eq!(length, f64::INFINITY);
    }
}
