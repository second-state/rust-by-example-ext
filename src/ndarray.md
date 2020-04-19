# N-dimensional array

The N-dimensional array is a widely used data structure for scientific 
computing and data analysis. The [`ndarray`](https://crates.io/crates/ndarray) crate provides support for
N-dimensional array in Rust. It is widely used by other crates.
To use the `ndarray` crate, just do the following in your `Cargo.toml` file.

```
[dependencies]
ndarray = "0.13.0"
```

## Basic operations

To create a 3-D array, and access one of its element by index, do the following.
The example creates a 3x4x5 array, and we access its elements using the 
`[[i, j, k]]` notation, where `i j k` are index positions for the element.

```rust,editable
# extern crate ndarray;

use ndarray::Array3;

fn main() {
    let mut a3 = Array3::<f64>::zeros((3, 4, 5));
    a3[[0, 0, 0]] = 0.0;
    a3[[1, 1, 1]] = 1.0;
    a3[[2, 2, 2]] = 2.0;
    println!("The 3D array is {:?}", a3);
}
```

Here is another example of a 3x3 2D array.

```rust,editable
# extern crate ndarray;

use ndarray::Array2;

fn main() {
    let mut a2 = Array2::<f64>::zeros((3, 3));
    a2[[0, 0]] = 0.0;
    a2[[0, 1]] = 0.5;
    a2[[0, 2]] = -0.5;
    a2[[1, 0]] = 1.0;
    a2[[1, 1]] = 1.5;
    a2[[1, 2]] = -1.5;
    a2[[2, 0]] = 2.0;
    a2[[2, 1]] = 2.5;
    a2[[2, 2]] = -2.5;
    println!("The 2D array is {:?}", a2);
}
```

Manually setting array elements by index positions is tedious. Next, let's see a better way to do this.

## Create arrays

The `arr2` and `arr3` macros allow you to create 2D and 3D arrays quickly.

The example creates a 3x3 2D array.

- The array has 3 rows `[row0, row1, row2]`. For example, `row0` is `[1, 2, 3]`.
- Each row has 3 columns `col0, col1, col2`. For example, `row0 col0` is `1`.

```rust,editable
# extern crate ndarray;

use ndarray::arr2;

fn main() {
    let mut a2 = arr2(&[[1, 2, 3],
                    [4, 5, 6],
                    [7, 8, 9]]);
    a2[[2, 1]] = 10;
    println!("The 2D array is {:?}", a2);
}
```

The example creates a 3x2x2 3D array. 

- The array has 3 rows `[row0, row1, row2]`. For example, `row0` is `[[1, 2], [3, 4]]`.
- Each row has 2 columns `[col0, col1]`. For example, `row0 col0` is `[1, 2]`.
- Each column has 2 levels `lvl0, lvl1`. For example, `row0 col0 lvl0` is `1`. 

```rust,editable
# extern crate ndarray;

use ndarray::arr3;

fn main() {
    let mut a3 = arr3(&[[[1, 2], [3, 4]],
                    [[5, 6], [7, 8]],
                    [[9, 0], [1, 2]]]);
    a3[[2, 1, 1]] = 10;
    println!("The 3D array is {:?}", a3);
}
```

## Iterate through the arrays

Using the `genrows()` function, you can flatten the `n` dimension array into an 
array of rows. Each row contains a simple (one dimension) array of values
along the original array's shortest axis.

Using the `outer_iter()` function, you can deconstruct the `n` dimension array 
into a simple (one dimension) array of `n-1` dimension arrays.

```rust,editable
# extern crate ndarray;

use ndarray::arr3;

fn main() {
    let a3 = arr3(&[[[1, 2], [3, 4]],
                    [[5, 6], [7, 8]],
                    [[9, 0], [1, 2]]]);
    for row in a3.genrows() {
        // Each row is a 1D array
        println!("row is {:?}", row);
    }
    for a2 in a3.outer_iter() {
        println!("2D array is {:?}", a2);
    }
}
```
