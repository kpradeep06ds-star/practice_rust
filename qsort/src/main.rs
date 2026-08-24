// quicksort

// logic:
// pick a pivot
//
// IMPORTANT:
// choosing a pivot does NOT magically create:
//      smaller | pivot | larger
//
// partition() is what rearranges the SAME array to create those regions.
//
// after partition:
//
//      elements < pivot | pivot | elements >= pivot
//
// the elements on either side are NOT necessarily sorted.
//
// recursively do exactly the same thing on the left and right regions.
//
// important difference:
// quicksort -> partitioning happens while moving DOWN the recursion
// merge sort -> merging/sorting happens while moving UP the recursion
//
// also VERY IMPORTANT:
// this version is IN PLACE.
// NOT creating left_array and right_array.
// working on smaller slices of the SAME underlying array.


fn partition(a: &mut [i32], p: usize) -> usize {
    let n = a.len();

    // Picked pivot is moved to the end temporarily.
    // This makes partitioning easier because we can scan everything before it.
    a.swap(p, n - 1);

    let pivot = a[n - 1];

    // This was one of the places I was confused.
    //
    // l does NOT simply mean "left".
    //
    // l means:
    // "the next position where an element smaller than the pivot should go"
    //
    // Therefore everything BEFORE l is guaranteed to be < pivot.
    let mut l = 0;

    for i in 0..n - 1 {

        if a[i] < pivot {

            // ound an element that belongs in the smaller region.
            //
            // Put it at position l.
            //
            // If i == l, swap effectively does nothing.
            //
            // If i > l, it means some >= pivot elements have accumulated
            // between l and i, so this swap pulls the smaller element
            // back into the correct region.
            a.swap(l, i);

            // Now the known-smaller region has grown by one.
            l += 1;
        }
    }

    // At this point:
    //
    // indices 0..l       are < pivot
    // index l            is where pivot belongs
    // indices l..n-1     are >= pivot
    //
    // So move the pivot from the end into its final position.
    a.swap(l, n - 1);

    // Return the FINAL index of the pivot.
    l
}


fn quicksort(a: &mut [i32]) {
    // Base case:
    //
    // Notice the ORIGINAL array does not become smaller.
    // The SLICE/range we are working on becomes smaller.
    if a.len() <= 1 {
        return;
    }

    // Choose middle element as pivot.
    //
    // This is the INDEX, not the pivot value.
    let pivot_index = a.len() / 2;

    // partition modifies THIS SAME slice.
    //
    // It also tells us where the pivot finally landed.
    let r = partition(a, pivot_index);

    // Now we conceptually have:
    //
    // left region | pivot | right region
    //
    // But these are NOT new arrays.
    // They are regions of the SAME underlying array.
    //
    // split_at_mut is Rust's way of safely giving us
    // two non-overlapping mutable regions.

    let (left, pivot_and_right) = a.split_at_mut(r);

    // Remove the pivot itself from further recursion.
    let (_, right) = pivot_and_right.split_at_mut(1);

    quicksort(left);
    quicksort(right);
}


fn main() {
    let mut x = vec![10, 2, -1, 0, 11, 8, 7, 9, 2, 12];

    quicksort(&mut x);

    println!("{:?}", x);
}