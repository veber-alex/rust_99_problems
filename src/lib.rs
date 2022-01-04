//! 99 problems solved in Rust
//! Based on https://ocaml.org/learn/tutorials/99problems.html

// Working with lists

/// 1. Write a function last : 'a list -> 'a option that returns the last element of a list. (easy)
pub fn last<T: Copy>(slice: &[T]) -> Option<T> {
    match *slice {
        [] => None,
        [.., a] => Some(a),
    }
}

#[test]
fn test_01_last() {
    assert_eq!(last(&["a", "b", "c", "d"]), Some("d"));
    assert_eq!(last::<&str>(&[]), None)
}

/// 2. Find the last but one (last and penultimate) elements of a list. (easy)
pub fn last_two<T: Copy>(slice: &[T]) -> Option<(T, T)> {
    match slice {
        [] | [_] => None,
        &[a, b] => Some((a, b)),
        [_, rest @ ..] => last_two(rest),
    }
}

#[test]
fn test_02_last_two() {
    assert_eq!(last_two(&["a", "b", "c", "d"]), Some(("c", "d")));
    assert_eq!(last_two(&["a"]), None)
}

/// 3. Find the K'th element of a list. (easy)
pub fn at<T: Copy>(slice: &[T], k: usize) -> Option<T> {
    match slice {
        &[a, ..] if k == 1 => Some(a),
        [_, rest @ ..] if k > 1 => at(rest, k - 1),
        _ => None,
    }
}

#[test]
fn test_03_at() {
    assert_eq!(at(&["a", "b", "c", "d", "e"], 3), Some("c"));
    assert_eq!(at(&["a"], 3), None)
}

/// 4. Find the number of elements of a list. (easy)
pub fn length<T>(slice: &[T]) -> usize {
    fn inner<T>(slice: &[T], len: usize) -> usize {
        match slice {
            [] => len,
            [_, rest @ ..] => inner(rest, len + 1),
        }
    }
    inner(slice, 0)
}

#[test]
fn test_04_length() {
    assert_eq!(length(&["a", "b", "c"]), 3);
    assert_eq!(length::<&str>(&[]), 0);
}

/// 5. Reverse a list. (easy)
pub fn rev<T: Copy>(slice: &[T]) -> Vec<T> {
    fn inner<T: Copy>(slice: &[T], mut rlist: Vec<T>) -> Vec<T> {
        match slice {
            [] => rlist,
            [first @ .., a] => {
                rlist.push(*a);
                inner(first, rlist)
            }
        }
    }
    inner(slice, vec![])
}

#[test]
fn test_05_rev() {
    assert_eq!(rev(&["a", "b", "c"]), ["c", "b", "a"]);
}

/// 6. Find out whether a list is a palindrome. (easy)
pub fn is_palindrome<T: PartialEq>(slice: &[T]) -> bool {
    match slice {
        [] | [_] | [_, _] => true,
        [a, middle @ .., b] => {
            if a != b {
                false
            } else {
                is_palindrome(middle)
            }
        }
    }
}

#[test]
fn test_06_is_palindrome() {
    assert!(is_palindrome(&["x", "a", "m", "a", "x"]));
    assert!(is_palindrome(&["a", "b"]))
}

/// There is no nested list type in Rust, so we need to define one first.
/// A node of a nested list is either an element, or a list of nodes.
pub enum Node<T> {
    One(T),
    Many(Vec<Node<T>>),
}

/// 7. Flatten a nested list structure. (medium)
pub fn flatten<T: Copy>(list: &[Node<T>]) -> Vec<T> {
    fn inner<T: Copy>(node: &[Node<T>], mut list: Vec<T>) -> Vec<T> {
        use Node::*;
        match node {
            [] => list,
            [a, rest @ ..] => {
                match a {
                    One(a) => list.push(*a),
                    Many(nodes) => list = inner(nodes, list),
                };
                inner(rest, list)
            }
        }
    }
    inner(list, vec![])
}

#[test]
fn test_07_flatten() {
    use Node::*;
    assert_eq!(
        flatten(&[
            One("a"),
            Many(vec![One("b"), Many(vec![One("c"), One("d")])]),
            One("e")
        ]),
        ["a", "b", "c", "d", "e"]
    );
}

/// 8. Eliminate consecutive duplicates of list elements. (medium)
pub fn compress<T: Copy + PartialEq>(slice: &[T]) -> Vec<T> {
    fn inner<T: Copy + PartialEq>(slice: &[T], mut list: Vec<T>) -> Vec<T> {
        match slice {
            [] => list,
            [a, rest @ ..] => {
                match list.as_slice() {
                    [] => list.push(*a),
                    [.., b] => {
                        if a != b {
                            list.push(*a);
                        }
                    }
                };
                inner(rest, list)
            }
        }
    }

    inner(slice, vec![])
}

#[test]
fn test_08_compress() {
    assert_eq!(
        compress(&["a", "a", "a", "a", "b", "c", "c", "a", "a", "d", "e", "e", "e", "e",]),
        ["a", "b", "c", "a", "d", "e"]
    );
}

/// 9. Pack consecutive duplicates of list elements into sublists. (medium)
pub fn pack<T: Copy + PartialEq>(slice: &[T]) -> Vec<Vec<T>> {
    fn inner<T: Copy + PartialEq>(slice: &[T], mut list: Vec<Vec<T>>) -> Vec<Vec<T>> {
        match slice {
            [] => list,
            [a, rest @ ..] => {
                match list.as_mut_slice() {
                    [] => list.push(vec![*a]),
                    [.., v] => match v.as_slice() {
                        &[.., b] if *a == b => v.push(b),
                        _ => list.push(vec![*a]),
                    },
                }
                inner(rest, list)
            }
        }
    }
    inner(slice, vec![])
}

#[test]
fn test_09_pack() {
    assert_eq!(
        pack(&["a", "a", "a", "a", "b", "c", "c", "a", "a", "d", "d", "e", "e", "e", "e"]),
        [
            vec!["a", "a", "a", "a"],
            vec!["b"],
            vec!["c", "c"],
            vec!["a", "a"],
            vec!["d", "d"],
            vec!["e", "e", "e", "e"]
        ]
    );
}

/// 10. Run-length encoding of a list. (easy)
pub fn encode<T: Copy + PartialEq>(slice: &[T]) -> Vec<(usize, T)> {
    fn inner<T: Copy + PartialEq>(slice: &[T], mut list: Vec<(usize, T)>) -> Vec<(usize, T)> {
        match slice {
            [] => list,
            [a, rest @ ..] => {
                match list.as_mut_slice() {
                    [.., (i, b)] if *a == *b => *i += 1,
                    _ => list.push((1, *a)),
                };
                inner(rest, list)
            }
        }
    }
    inner(slice, vec![])
}

#[test]
fn test_10_encode() {
    assert_eq!(
        encode(&["a", "a", "a", "a", "b", "c", "c", "a", "a", "d", "e", "e", "e", "e"]),
        [(4, "a"), (1, "b"), (2, "c"), (2, "a"), (1, "d"), (4, "e")]
    )
}

/// A type to hold both single elements and sub-lists
#[derive(Debug, PartialEq)]
pub enum Rle<T> {
    One(T),
    Many(usize, T),
}

/// 11. Modified run-length encoding. (easy)
pub fn mod_encode<T: Copy + PartialEq>(slice: &[T]) -> Vec<Rle<T>> {
    fn inner<T: Copy + PartialEq>(slice: &[T], mut list: Vec<Rle<T>>) -> Vec<Rle<T>> {
        use Rle::*;
        match slice {
            [] => list,
            [a, rest @ ..] => {
                match list.as_mut_slice() {
                    [] => list.push(One(*a)),
                    [.., rle] => match rle {
                        One(b) if *a == *b => *rle = Many(2, *a),
                        Many(i, b) if *a == *b => *i += 1,
                        _ => list.push(One(*a)),
                    },
                };
                inner(rest, list)
            }
        }
    }
    inner(slice, vec![])
}

#[test]
fn test_11_mod_encode() {
    use Rle::*;
    assert_eq!(
        mod_encode(&["a", "a", "a", "a", "b", "c", "c", "a", "a", "d", "e", "e", "e", "e"]),
        [
            Many(4, "a"),
            One("b"),
            Many(2, "c"),
            Many(2, "a"),
            One("d"),
            Many(4, "e")
        ]
    )
}

/// 12. Decode a run-length encoded list. (medium)
pub fn decode<T: Copy>(slice: &[Rle<T>]) -> Vec<T> {
    fn inner<T: Copy>(slice: &[Rle<T>], mut list: Vec<T>) -> Vec<T> {
        use Rle::*;
        match slice {
            [] => list,
            [rle, rest @ ..] => {
                match *rle {
                    One(a) | Many(1, a) => list.push(a),
                    Many(n, a) => {
                        list.push(a);
                        list = inner(&[Many(n - 1, a)], list);
                    }
                }
                inner(rest, list)
            }
        }
    }
    inner(slice, vec![])
}

#[test]
fn test_12_decode() {
    use Rle::*;
    assert_eq!(
        decode(&[
            Many(4, "a"),
            One("b"),
            Many(2, "c"),
            Many(2, "a"),
            One("d"),
            Many(4, "e")
        ]),
        ["a", "a", "a", "a", "b", "c", "c", "a", "a", "d", "e", "e", "e", "e"]
    )
}

/// 13. Run-length encoding of a list (direct solution). (medium)
pub fn encode_direct() {
    // TODO: switch with problem 11
    todo!()
}

/// 14. Duplicate the elements of a list. (easy)
pub fn duplicate<T: Copy>(slice: &[T]) -> Vec<T> {
    fn inner<T: Copy>(slice: &[T], mut list: Vec<T>) -> Vec<T> {
        match slice {
            [] => list,
            [a, rest @ ..] => {
                list.push(*a);
                list.push(*a);
                inner(rest, list)
            }
        }
    }
    inner(slice, vec![])
}

#[test]
fn test_14_duplicate() {
    assert_eq!(
        duplicate(&["a", "b", "c", "c", "d"]),
        ["a", "a", "b", "b", "c", "c", "c", "c", "d", "d"]
    )
}

/// 15. Replicate the elements of a list a given number of times. (medium)
pub fn replicate<T: Copy>(slice: &[T], n: usize) -> Vec<T> {
    fn inner<T: Copy>(slice: &[T], mut list: Vec<T>, n: usize) -> Vec<T> {
        match slice {
            [] => list,
            _ if n == 0 => list,
            [a] => {
                list.push(*a);
                inner(slice, list, n - 1)
            }
            [a, rest @ ..] => {
                list = inner(&[*a], list, n);
                inner(rest, list, n)
            }
        }
    }
    inner(slice, vec![], n)
}

#[test]
fn test_15_replicate() {
    assert_eq!(
        replicate(&["a", "b", "c"], 3),
        ["a", "a", "a", "b", "b", "b", "c", "c", "c"]
    )
}

/// 16. Drop every N'th element from a list. (medium)
pub fn dropn<T: Copy>(slice: &[T], n: usize) -> Vec<T> {
    fn inner<T: Copy>(slice: &[T], mut list: Vec<T>, n: usize, m: usize) -> Vec<T> {
        match (slice, n) {
            ([], _) => list,
            ([a, rest @ ..], 2..) => {
                list.push(*a);
                inner(rest, list, n - 1, m)
            }
            ([_, rest @ ..], _) => inner(rest, list, m, m),
        }
    }
    inner(slice, vec![], n, n)
}

#[test]
fn test_16_dropn() {
    assert_eq!(
        dropn(&["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"], 3),
        ["a", "b", "d", "e", "g", "h", "j"]
    )
}
