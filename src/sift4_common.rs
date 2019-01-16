pub fn sift4(s1: &str, s2: &str) -> i32 {
    return sift4_offset(s1, s2, 5, 5);
}

#[derive(Copy, Clone)]
struct Offset {
    c1: usize,
    c2: usize,
    trans: bool,
}

impl Offset {
    fn new(c1: usize, c2: usize, trans: bool) -> Offset {
        Offset { c1, c2, trans }
    }
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

fn sift4_offset(s1: &str, s2: &str, max_offset: usize, max_distance: usize) -> i32 {
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
    let mut trans = 0; // number of transpositions ("ab" vs "ba")
    let mut offset_vec: Vec<Offset> = Vec::new(); // offset pair array, for computing transpositions

    while c1 < l1 && c2 < l2 {
        if s1v[c1] == s2v[c2] {
            local_cs += 1;
            let mut is_trans = false;

            // see if current match is a transposition
            let mut i = 0;
            while i < offset_vec.len() {
                let mut ofs = offset_vec[i];
                if c1 <= ofs.c1 || c2 <= ofs.c2 {
                    is_trans =
                        if (c2 as i32 - c1 as i32).abs() >= (ofs.c2 as i32 - ofs.c1 as i32).abs() {
                            true
                        } else {
                            false
                        };

                    if is_trans == true {
                        trans += 1;
                    } else {
                        if ofs.trans == false {
                            ofs.trans = true;
                            trans += 1;
                        }
                    }
                    break;
                } else {
                    if c1 > ofs.c2 && c2 > ofs.c1 {
                        offset_vec.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
            let offset = Offset::new(c1, c2, is_trans);
            offset_vec.push(offset);
        } else {
            lcss += local_cs;
            local_cs = 0;
            if c1 != c2 {
                c1 = min_usize(c1, c2);
                c2 = c1; // using min allows the computation of transpositions
            }

            // if matching characters are found, remove 1 from both cursors so
            // we only have one code block handling matches

            for i in 0..max_offset {
                if !(c1 + i < l1 || c2 + i < l2) {
                    break;
                }

                if c1 + i < l1 && s1v[c1 + i] == s2v[c2] {
                    c1 += i - 1;
                    c2 -= 1;
                    break;
                }
                if (c2 + i < l2) && (s1v[c1] == s2v[c2 + i]) {
                    c1 -= 1;
                    c2 += i - 1;
                    break;
                }
            }
        }
        c1 += 1;
        c2 += 1;

        if max_distance > 0 {
            let temporary_distance = max_usize(c1, c2) - lcss + trans;

            if temporary_distance >= max_distance {
                return temporary_distance as i32;
            };
        };

        if c1 >= l1 || c2 >= l2 {
            lcss += local_cs;
            local_cs = 0;
            c1 = min_usize(c1, c2);
            c2 = c1;
        }
    }
    lcss += local_cs;
    (max_usize(l1, l2) - lcss + trans) as i32
}

#[cfg(test)]
mod tests {
    use super::sift4;

    #[test]
    fn basic() {
        assert_eq!(1, sift4("London", "Londo"));
    }
}
