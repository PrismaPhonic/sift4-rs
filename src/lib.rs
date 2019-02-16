//! Sift4 calculates the sift4 distance between two strings (how many characters are different) using the sift4 "simple" algorithm with a default max_distance of 5.
//! # Algorithm
//! [Sift4](https://siderite.blogspot.com/2014/11/super-fast-and-accurate-string-distance.html) -
//! Super Fast and Accurate string distance algorithm.
//! Unlike Sift3, Sift4 is not only more accurate but more closely reflects what you
//! would get from getting the levenshtein distance between two strings.

/// Simple will take in two strings and return the string distance using the sift4 algorithm. It
/// uses a default max offset of 5.
///
/// ## Examples
/// ```
/// extern crate sift4;
///
/// // Sift4 distance
/// let distance = sift4::simple("London", "Londo");   
/// assert_eq!(1, distance);
/// ```
pub fn simple(s1: &str, s2: &str) -> i32 {
    return sift4_offset(s1, s2, 5);
}

fn min_usize(u1: usize, u2: usize) -> usize {
    if u1 <= u2 {
        u1
    } else {
        u2
    }
}

fn max_usize(u1: usize, u2: usize) -> usize {
    if u1 >= u2 {
        u1
    } else {
        u2
    }
}

fn sift4_offset(s1: &str, s2: &str, max_offset: usize) -> i32 {
    let l1 = s1.len();
    let l2 = s2.len();

    // handle empty strings
    if l1 == 0 {
        if l2 == 0 {
            return 0;
        } else {
            return l2 as i32;
        }
    }

    if l2 == 0 {
        return l1 as i32;
    }

    let s1v: Vec<char> = s1.chars().collect();
    let s2v: Vec<char> = s2.chars().collect();
    let mut c1 = 0; // cursor for string 1
    let mut c2 = 0; // cursor for string 2
    let mut lcss = 0; // largest common subsequence
    let mut local_cs = 0; // local common substring

    while c1 < l1 && c2 < l2 {
        if s1v[c1] == s2v[c2] {
            local_cs += 1;
        } else {
            lcss += local_cs;
            local_cs = 0;
            if c1 != c2 {
                c1 = min_usize(c1, c2);
                c2 = c1; // using min allows the computation of transpositions
            }

            for i in 0..max_offset {
                if (c1 + 1 < l1 || c2 + i < l2) == false {
                    break;
                }

                if c1 + i < l1 && s1v[c1 + i] == s2v[c2] {
                    c1 += i;
                    local_cs += 1;
                    break;
                }
                if (c2 + i < l2) && (s1v[c1] == s2v[c2 + i]) {
                    c2 += i;
                    local_cs += 1;
                    break;
                }
            }
        }
        c1 += 1;
        c2 += 1;
    }
    lcss += local_cs;
    (max_usize(l1, l2) - lcss) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(2, simple("London", "Lond"));
        assert_eq!(2, simple("Chicago", "Chiag"));
        assert_eq!(4, simple("Los Angeles", "Angeles"));
        assert_eq!(2, simple("Bangkok", "Bagrok"));
    }

    #[test]
    fn case() {
        assert_eq!(1, simple("San Francisco", "san Francisco"));
        assert_eq!(1, simple("New York", "new York"));
    }

    #[test]
    fn empty() {
        assert_eq!(13, simple("San Francisco", ""));
        assert_eq!(8, simple("", "New York"));
    }

}
