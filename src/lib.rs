use pyo3::prelude::*;
use std::time::{Instant};

#[pyfunction]
fn quicksort(m: Vec<u32>) -> PyResult<Vec<u32>> {
    let mut temp = m.clone();
    let size = temp.len();
    //let now = Instant::now();
    quicksort_call(&mut temp, 0, size as i32 - 1);
    //println!("RUST QUICKSORT\t{}MS", now.elapsed().as_millis());
    Ok(temp)
}

fn quicksort_call(m: &mut Vec<u32>, lo: i32, hi: i32) {
    if lo < hi {
        let p = partition(m, lo, hi);
        quicksort_call(m, lo, p-1);
        quicksort_call(m, p+1, hi);
    }
}

fn partition(m: &mut Vec<u32>, lo: i32, hi: i32) -> i32 {
    let pivot = m[lo as usize];
    let mut i = lo - 1;
    
    for j in lo..hi {
        if m[j as usize] < pivot {
            i += 1;
            m.swap(i as usize, j as usize);
        }
    }
    i += 1;
    m.swap(i as usize, hi as usize);
    return i;
}

#[pyfunction]
fn mergesort(m: Vec<u32>) -> PyResult<Vec<u32>> {
    let mut temp = m.clone();
    let size = temp.len();
    //let now = Instant::now();
    mergesort_call(&mut temp, 0, size - 1);
    //println!("RUST MERGESORT\t{}MS", now.elapsed().as_millis());
    Ok(temp)
}

fn mergesort_call(m: &mut Vec<u32>, i: usize, j: usize) {
    let k = (i + j) / 2;
    if j -  i > 2 {
        mergesort_call(m, i, k);
        mergesort_call(m, k, j);
    }
    merge(m, i, k, j);
}

fn merge(m: &mut Vec<u32>, i: usize, k: usize, j: usize) {
    let m1 = m[i..k].to_vec();
    let m2 = m[k..j].to_vec();
    let mut mn1 = 0;
    let mut mn2 = 0;
    for n in i..j {
        if mn1 < m1.len() && mn2 < m2.len() {
            if m1[mn1] < m2[mn2] {
                m[n] = m1[mn1];
                mn1 += 1;
            }else{
                m[n] = m2[mn2];
                mn2 += 1;
            }
        } else if mn1 < m1.len() {
            m[n] = m1[mn1];
            mn1 += 1;
        } else{
            m[n] = m2[mn2];
            mn2 += 1;
        }
    }
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(quicksort, m)?)?;
    m.add_function(wrap_pyfunction!(mergesort, m)?)?;
    Ok(())
}
/*
fn main() {
    let mut rng = rand::thread_rng();
    let count = 200000;
    let numbers: Vec<u32> = (0..count).map(|_| rng.gen_range(0..10000)).collect();
}
*/