//! Source in-place stable sorting used by formatter map snapshots.
// Adapted from Go slices/{sort,zsortanyfunc}.go at go1.26.5.
// Copyright 2022 The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
// The pinned generated Go source is an input; no source generator ran.
use std::cmp::Ordering;

pub(crate) fn source_stable_sort<Value>(values: &mut [Value],
    mut compare: impl FnMut(&Value, &Value) -> Ordering)
{
    let length = values.len(); let mut block = 20;
    let mut begin = 0; let mut end = block;
    while end <= length {
        insertion_sort(values, begin, end, &mut compare);
        begin = end; end += block;
    }
    insertion_sort(values, begin, length, &mut compare);
    while block < length {
        begin = 0; end = 2 * block;
        while end <= length {
            symmetric_merge(values, begin, begin + block, end, &mut compare);
            begin = end; end += 2 * block;
        }
        let middle = begin + block;
        if middle < length { symmetric_merge(values, begin, middle, length, &mut compare); }
        block *= 2;
    }
}

fn insertion_sort<Value>(values: &mut [Value], begin: usize, end: usize,
    compare: &mut impl FnMut(&Value, &Value) -> Ordering)
{
    for index in begin + 1..end {
        let mut current = index;
        while current > begin && compare(&values[current], &values[current - 1]) == Ordering::Less {
            values.swap(current, current - 1); current -= 1;
        }
    }
}

// Internal callers retain the source non-degenerate bounds: a < m < b.
fn symmetric_merge<Value>(values: &mut [Value], a: usize, m: usize, b: usize,
    compare: &mut impl FnMut(&Value, &Value) -> Ordering)
{
    if m - a == 1 {
        let mut first = m; let mut last = b;
        while first < last {
            let middle = first.wrapping_add(last) >> 1;
            if compare(&values[middle], &values[a]) == Ordering::Less { first = middle + 1; }
            else { last = middle; }
        }
        for index in a..first - 1 { values.swap(index, index + 1); }
        return;
    }
    if b - m == 1 {
        let mut first = a; let mut last = m;
        while first < last {
            let middle = first.wrapping_add(last) >> 1;
            if compare(&values[m], &values[middle]) != Ordering::Less { first = middle + 1; }
            else { last = middle; }
        }
        for index in (first + 1..=m).rev() { values.swap(index, index - 1); }
        return;
    }
    let middle = a.wrapping_add(b) >> 1;
    let total = middle + m;
    let (mut start, mut right) = if m > middle { (total - b, middle) } else { (a, m) };
    let pivot = total - 1;
    while start < right {
        let candidate = start.wrapping_add(right) >> 1;
        if compare(&values[pivot - candidate], &values[candidate]) != Ordering::Less { start = candidate + 1; }
        else { right = candidate; }
    }
    let end = total - start;
    if start < m && m < end { rotate(values, start, m, end); }
    if a < start && start < middle { symmetric_merge(values, a, start, middle, compare); }
    if middle < end && end < b { symmetric_merge(values, middle, end, b, compare); }
}

fn rotate<Value>(values: &mut [Value], begin: usize, middle: usize, end: usize) {
    let mut left = middle - begin; let mut right = end - middle;
    while left != right {
        if left > right { swap_range(values, middle - left, middle, right); left -= right; }
        else { swap_range(values, middle - left, middle + right - left, left); right -= left; }
    }
    swap_range(values, middle - left, middle, left);
}
fn swap_range<Value>(values: &mut [Value], left: usize, right: usize, length: usize) {
    for index in 0..length { values.swap(left + index, right + index); }
}
