#[cfg(test)]
pub mod tests;

use std::cmp::PartialOrd;
pub fn bubble_sort<T>(v: &mut [T])
where
    T: PartialOrd,
{
    let mut n = 0;
    while {
        !pass(v, &n)
    }
    {
        n += 1;
    }
    fn pass<T>(v: &mut [T], n: &usize) -> bool
    where
        T: PartialOrd,
    {
        let mut done = true;
        for i in 1..v.len() - n {
            if v[i] < v[i - 1] {
                done = false;
                v.swap(i, i - 1);
            }
        }
        done
    }
}
pub fn bubble_sort_preserve_index<T>(v: &mut [T]) -> Vec<u16>
where
    T: PartialOrd,
{
    let mut k = Vec::with_capacity(v.len());
    for i in 0..v.len() {
        k.push(i as u16);
    }

    let mut n = 0;
    while {
        !pass(v, &n, &mut k)
    }
    {
        n += 1;
    }
    fn pass<T>(v: &mut [T], n: &usize, k: &mut Vec<u16>) -> bool
    where
        T: PartialOrd,
    {
        let mut done = true;
        for i in 1..v.len() - n {
            if v[i] < v[i - 1] {
                done = false;
                v.swap(i, i - 1);
                k.swap(i, i - 1);
            }
        }
        done
    }
    k
}

pub fn insertion_sort<T>(v: &mut [T])
where
    T: PartialOrd,
{

    //Loop over the vector, inserting data in correct order
    for i in 1..v.len() {
        //Loop over the already sorted subvector and insert i at right place
        for j in 0..i {
            //Element i is smaller than element j, insert i before j and break loop

            if v[j] > v[i] {
                //Move element i back in vector by swapping until it reaches correct position
                for k in (j..i).rev() {
                    v.swap(k + 1, k);
                }

                //No need to keep looping j, break out
                break;
            }
        }
    }
}
pub fn selection_sort<T>(v: &mut [T])
where
    T: PartialOrd,
{
    let mut n: usize = 0;
    while n < v.len() {
        pass(v, &mut n);
    }

    fn pass<T>(v: &mut [T], n: &mut usize)
    where
        T: PartialOrd,
    {
        let mut lowest: usize = *n;
        for i in *n + 1..v.len() {
            if v[i] < v[lowest] {
                lowest = i;
            }
        }
        v.swap(*n, lowest);
        *n += 1;
    }
}
pub fn quicksort_1(min: usize, max: usize, v: &mut Vec<i32>) {
    let pivot = v[max - 1];
    let mut wall = min;
    for i in min..max {
        if v[i] < pivot {
            v.swap(i, wall);
            wall += 1;
        }
    }
    v.swap(wall, max - 1);
    if wall > min {
        quicksort_1(min, wall, v);
    }
    if max > wall + 1 {
        quicksort_1(wall + 1, max, v);
    }
}



pub fn quicksort_2(left: usize, right: usize, v: &mut Vec<i32>) {
    let pivot_pos = quicksort_partition(left, right, v);
    if pivot_pos - left > 1 {
        quicksort_2(left, pivot_pos - 1, v)

    }
    if right - pivot_pos > 1 {
        quicksort_2(pivot_pos + 1, right, v)
    }
    fn quicksort_partition(left: usize, right: usize, array: &mut Vec<i32>) -> usize {
        print!("{}-{} ", left, right);
        let mut lt = left;
        let mut rt = right;
        let mut dir = true;
        let pivot = array[left];
        while lt < rt {
            if dir {
                if array[rt] > pivot {
                    rt = rt - 1;
                } else {
                    array[lt] = array[rt];
                    lt = lt + 1;
                    dir = false;
                }
            } else {
                if array[lt] < pivot {
                    lt = lt + 1;
                } else {
                    array[rt] = array[lt];
                    rt = rt - 1;
                    dir = true;
                }
            }

        }
        array[lt] = pivot;
        lt

    }
}
