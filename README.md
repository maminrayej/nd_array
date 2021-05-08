# ndarray-rs
Crate for linear algebra that tries to have a similar api to numpy.

# Creation
### Range of values
```rust
let array = Array::arange(0..10);

assert_eq!(
    array.flat().copied().collect::<Vec<usize>>(),
    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
)
```

### Array full of zeros
```rust
let array = Array::zeros([2, 4]);

assert_eq!(
    array.flat().copied().collect::<Vec<usize>>(),
    vec![0, 0, 0, 0, 0, 0, 0, 0]
)
```

### Array full of zeroes with the shape of another array
```rust
let array = Array::arange(0..8).reshape([2, 4]);

let zeros_like = Array::zeros_like(&array);

assert_eq!(
    zeros_like.flat().copied().collect::<Vec<usize>>(),
    vec![0, 0, 0, 0, 0, 0, 0, 0]
)
```

### Array full of ones
```rust
let array = Array::ones([2, 4]);

assert_eq!(
    array.flat().copied().collect::<Vec<usize>>(),
    vec![1, 1, 1, 1, 1, 1, 1, 1]
)
```

### Array full of ones with the shape of another array
```rust
let array = Array::arange(0..8).reshape([2, 4]);

let ones_like = Array::ones_like(&array);

assert_eq!(
    ones_like.flat().copied().collect::<Vec<usize>>(),
    vec![1, 1, 1, 1, 1, 1, 1, 1]
)
```

### Array full of a defined value
```rust
let array = Array::full(10, [2, 4]);

assert_eq!(
    array.flat().copied().collect::<Vec<usize>>(),
    vec![10, 10, 10, 10, 10, 10, 10, 10]
)
```

### Array full of a defined value with the shape of another array
```rust
let array = Array::arange(0..8).reshape([2, 4]);

let full_like = Array::full_like(10, &array);

assert_eq!(
    full_like.flat().copied().collect::<Vec<usize>>(),
    vec![10, 10, 10, 10, 10, 10, 10, 10]
)
```

# Transformations
### Reshape
```rust
// 2-D array:
// 1 2 3
// 4 5 6
let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

// reshape it to the 3x2 2-D array:
// 1 2
// 3 4
// 5 6
let array = array.reshape([3, 2]);

assert_eq!(array[[0, 0]], 1);
assert_eq!(array[[0, 1]], 2);
assert_eq!(array[[1, 0]], 3);
assert_eq!(array[[1, 1]], 4);
assert_eq!(array[[2, 0]], 5);
assert_eq!(array[[2, 1]], 6);
```

### Transpose
```rust
// 2-D array:
// 1 2 3
// 4 5 6
let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

// tranpose the array to:
// 1 4
// 2 5
// 3 6
let array = array.transpose();

assert_eq!(array[[0, 0]], 1);
assert_eq!(array[[0, 1]], 4);
assert_eq!(array[[1, 0]], 2);
assert_eq!(array[[1, 1]], 5);
assert_eq!(array[[2, 0]], 3);
assert_eq!(array[[2, 1]], 6);
```

### Flip
```rust
// 2-D array:
// 1 2 3
// 4 5 6
let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

// flip axis = 0
// 4 5 6
// 1 2 3
let array = array.flip(0);

assert_eq!(
    array.flat().copied().collect::<Vec<usize>>(),
    vec![4, 5, 6, 1, 2, 3]
);
```

### Swap axis
```rust
// 2-D array:
// 1 2 3
let array = Array::init(vec![1, 2, 3], [1, 3]);

let swapped_array = array.swap_axes(0, 1);

assert_eq!(swapped_array[[0, 0]], 1);
assert_eq!(swapped_array[[1, 0]], 2);
assert_eq!(swapped_array[[2, 0]], 3);
```

### Flatten
```rust
// 2-D array:
// 1 2 3
// 4 5 6
let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

let flatten_array = array.flatten();

assert_eq!(
    flatten_array.flat().copied().collect::<Vec<usize>>(),
    vec![1, 2, 3, 4, 5, 6]
)
```

# Operations
### Negation
``` rust
let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

let neg_array = -array;

assert_eq!(
    neg_array.flat().copied().collect::<Vec<i32>>(),
    vec![-1, -2, -3, -4, -5, -6]
);
```

### Add
```rust
let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);
let array2 = Array::init(vec![6, 5, 4, 3, 2, 1], [2, 3]);

let sum_array = array + array2;

assert_eq!(
    sum_array.flat().copied().collect::<Vec<i32>>(),
    vec![7, 7, 7, 7, 7, 7]
);
```

### Sub
```rust
let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);
let array2 = Array::init(vec![6, 5, 4, 3, 2, 1], [2, 3]);

let sub_array = array - array2;

assert_eq!(
    sub_array.flat().copied().collect::<Vec<i32>>(),
    vec![-5, -3, -1, 1, 3, 5]
);
```

### Mul
```rust
let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

let mul_array = array * 2;

assert_eq!(
    mul_array.flat().copied().collect::<Vec<i32>>(),
    vec![2, 4, 6, 8, 10, 12]
);
```

### Div
```rust
let array = Array::init(vec![2, 4, 6, 8, 10, 12], [2, 3]);

let div_array = array / 2;

assert_eq!(
    div_array.flat().copied().collect::<Vec<i32>>(),
    vec![1, 2, 3, 4, 5, 6]
);
```

# Calculations
### Max
```rust
// 2-D array:
// 0 1
// 2 3
let array = Array::init(vec![0, 1, 2, 3], [2, 2]);

// Find max
assert_eq!(array.max().unwrap(), 3);
// Find index of max value if array was flattened
assert_eq!(array.arg_max()[0], 3);

// Find max values across a specific axis
assert_eq!(array.max_across(1), vec![Some(2), Some(3)]);
assert_eq!(array.max_across(0), vec![Some(1), Some(3)]);

// Find indices of max values across a specific axis
assert_eq!(array.arg_max_across(1), vec![Some(1), Some(1)]);
assert_eq!(array.arg_max_across(0), vec![Some(1), Some(1)]);
```

### Min
```rust
// 2-D array:
// 0 1
// 2 3
let array = Array::init(vec![0, 1, 2, 3], [2, 2]);

// Find min
assert_eq!(array.min().unwrap(), 0);
// Find index of min value if array was flattened
assert_eq!(array.arg_min()[0], 0);

// Find min values across a specific axis
assert_eq!(array.min_across(1), vec![Some(0), Some(1)]);
assert_eq!(array.min_across(0), vec![Some(0), Some(2)]);

// Find indices of min values across a specific axis
assert_eq!(array.arg_min_across(1), vec![Some(0), Some(0)]);
assert_eq!(array.arg_min_across(0), vec![Some(0), Some(0)]);
```

### Clip
```rust
// Array:
// 0 1 2 3 4 5 6 7 8 9
let array = Array::arange(0..10);

// Clip the values so that all of them are between 1 and 8
let clipped = array.clip(&1, &8);

assert_eq!(
    clipped.flat().copied().collect::<Vec<i32>>(),
    vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 8]
);
```

### PTP (Peak to Peak)
```rust
// Array:
// 4 9 2 10
// 6 9 7 12
let array = Array::init(vec![4, 9, 2, 10, 6, 9, 7, 12], [2, 4]);

// Find max - min across all members of the array
assert_eq!(array.ptp().unwrap(), 10)

// Find max - min across a specific axis
assert_eq!(array.ptp_across(0), vec![Some(8), Some(6)]);
assert_eq!(
    array.ptp_across(1),
    vec![Some(2), Some(0), Some(5), Some(2)]
)
```

### Sum
```rust
// Array:
// 1 2
// 3 4
let array = Array::arange(1..5).reshape([2, 2]);

// Calculate sum of all elements in the array
assert_eq!(array.sum(), 10);

// Calculate sum of elements across a specific axis.
assert_eq!(array.sum_across(0), vec![3, 7]);
assert_eq!(array.sum_across(1), vec![4, 6]);
```

### Product
```rust
// Array:
// 1 2
// 3 4
let array = Array::arange(1..5).reshape([2, 2]);

// Calculate product of all elements in the array
assert_eq!(array.prod(), 24);

// Calculate product of elements across a specific axis
assert_eq!(array.prod_across(0), vec![2, 12]);
assert_eq!(array.prod_across(1), vec![3, 8]);
```

### Mean
```rust
// Array:
// 1 2
// 3 4
let array = Array::arange(1..5).reshape([2, 2]);

// Calculate mean of all elements in the array
assert_eq!(array.mean(), 2);

// Calculate mean of elements across a specific axis
assert_eq!(array.mean_across(0), vec![1, 3]);
assert_eq!(array.mean_across(1), vec![2, 3]);
```

### Variance
```rust
// Array:
// 1 2
// 3 4
let array = Array::init(vec![1.0, 2.0, 3.0, 4.0], [2, 2]);

// Calculate variance of all elements in the array
assert_eq!(array.var(), 1.25);

// Calculate variance of elements across a specific axis
assert_eq!(array.var_across(0), vec![0.25, 0.25]);
assert_eq!(array.var_across(1), vec![1.0, 1.0]);
```