use std::fmt;
use std::ops::{Add, Mul, Index, IndexMut};
// region BoolVec
use std::ops::Index;
const BITS: usize = usize::BITS as usize;
use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Clone)]
struct BoolVec {
	data: Vec<usize>,
	len: usize,
	true_count: usize,  // O(1) count tracking
}

impl BoolVec {
	fn new() -> Self {
		BoolVec { data: Vec::new(), len: 0, true_count: 0 }
	}

	fn with_capacity(capacity: usize) -> Self {
		let chunks = capacity.div_ceil(BITS);
		BoolVec {
			data: Vec::with_capacity(chunks),
			len: 0,
			true_count: 0,
		}
	}

	fn filled(len: usize, val: bool) -> Self {
		if len == 0 {
			return BoolVec::new();
		}
		let chunks = len.div_ceil(BITS);
		let fill: usize = if val { usize::MAX } else { 0 };
		let mut data = vec![fill; chunks];
		if val {
			let leftover = len % BITS;
			if leftover != 0 {
				let mask = (1 << leftover) - 1;
				*data.last_mut().unwrap() &= mask;
			}
		}
		BoolVec {
			data,
			len,
			true_count: if val { len } else { 0 },
		}
	}

	fn push(&mut self, val: bool) {
		let bit_index = self.len % BITS;
		if bit_index == 0 {
			self.data.push(0);
		}
		if val {
			let chunk_index = self.len / BITS;
			self.data[chunk_index] |= 1 << bit_index;
			self.true_count += 1;
		}
		self.len += 1;
	}

	fn get(&self, index: usize) -> Option<bool> {
		if index >= self.len {
			return None;
		}
		let chunk_index = index / BITS;
		let bit_index = index % BITS;
		Some((self.data[chunk_index] >> bit_index) & 1 == 1)
	}

	fn set(&mut self, index: usize, val: bool) {
		assert!(index < self.len, "index out of bounds");
		let chunk_index = index / BITS;
		let bit_index = index % BITS;
		let current = (self.data[chunk_index] >> bit_index) & 1 == 1;
		// only update true_count if the value actually changes
		match (current, val) {
			(false, true) => self.true_count += 1,
			(true, false) => self.true_count -= 1,
			_ => {} // no change
		}
		if val {
			self.data[chunk_index] |= 1 << bit_index;
		} else {
			self.data[chunk_index] &= !(1 << bit_index);
		}
	}

	fn len(&self) -> usize {
		self.len
	}

	fn is_empty(&self) -> bool {
		self.len == 0
	}

	fn count_true(&self) -> usize {
		self.true_count
	}

	fn count_false(&self) -> usize {
		self.len - self.true_count
	}
}

// AND: bv1 & bv2
impl BitAnd for &BoolVec {
	type Output = BoolVec;
	fn bitand(self, rhs: &BoolVec) -> BoolVec {
		assert_eq!(self.len, rhs.len, "BoolVec lengths must match for AND");
		let data: Vec<usize> = self.data.iter().zip(rhs.data.iter()).map(|(a, b)| a & b).collect();
		let true_count = data.iter().map(|x| x.count_ones() as usize).sum();
		BoolVec { data, len: self.len, true_count }
	}
}

// OR: bv1 | bv2
impl BitOr for &BoolVec {
	type Output = BoolVec;
	fn bitor(self, rhs: &BoolVec) -> BoolVec {
		assert_eq!(self.len, rhs.len, "BoolVec lengths must match for OR");
		let data: Vec<usize> = self.data.iter().zip(rhs.data.iter()).map(|(a, b)| a | b).collect();
		let true_count = data.iter().map(|x| x.count_ones() as usize).sum();
		BoolVec { data, len: self.len, true_count }
	}
}

// XOR: bv1 ^ bv2
impl BitXor for &BoolVec {
	type Output = BoolVec;
	fn bitxor(self, rhs: &BoolVec) -> BoolVec {
		assert_eq!(self.len, rhs.len, "BoolVec lengths must match for XOR");
		let data: Vec<usize> = self.data.iter().zip(rhs.data.iter()).map(|(a, b)| a ^ b).collect();
		let true_count = data.iter().map(|x| x.count_ones() as usize).sum();
		BoolVec { data, len: self.len, true_count }
	}
}

// NOT: !bv
impl Not for &BoolVec {
	type Output = BoolVec;
	fn not(self) -> BoolVec {
		let mut data: Vec<usize> = self.data.iter().map(|a| !a).collect();
		let leftover = self.len % BITS;
		if leftover != 0 {
			let mask = (1 << leftover) - 1;
			if let Some(last) = data.last_mut() {
				*last &= mask;
			}
		}
		let true_count = self.len - self.true_count; // flipped
		BoolVec { data, len: self.len, true_count }
	}
}

impl Index<usize> for BoolVec {
	type Output = bool;
	fn index(&self, index: usize) -> &Self::Output {
		match self.get(index) {
			Some(true) => &true,
			Some(false) => &false,
			None => panic!("index {index} out of bounds (len = {})", self.len),
		}
	}
}

impl std::fmt::Display for BoolVec {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[")?;
		for i in 0..self.len {
			if i > 0 { write!(f, ", ")?; }
			write!(f, "{}", if self[i] { 1 } else { 0 })?;
		}
		write!(f, "]")
	}
}
// endregion

//region Matrix

// ── Struct ────────────────────────────────────────────────────────────────────

/// A generic 2-D matrix with contiguous, row-major heap storage.
///
/// # Type parameter
///
/// `T` may be any type — numeric primitives, `String`, user-defined structs,
/// etc.  Only the methods that perform arithmetic require additional bounds
/// such as [`Add`] or [`Mul`].
///
/// # Memory layout
///
/// Elements are stored in a single `Vec<T>` in row-major order: all elements
/// of row 0 come first, then row 1, and so on.  The element at logical
/// position `(r, c)` lives at flat index `r * cols + c`.
///
/// # Panics
///
/// All index and shape checks panic immediately with a descriptive message
/// rather than propagating `Option` or `Result`.  This mirrors the behaviour
/// of Rust's built-in slice indexing.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
	rows: usize,
	cols: usize,
	data: Vec<T>,
}

// ── Construction ──────────────────────────────────────────────────────────────

impl<T: Clone> Matrix<T> {
	/// Create a new matrix where every element is initialised to `value`.
	///
	/// # Arguments
	///
	/// * `rows` — number of rows.
	/// * `cols` — number of columns.
	/// * `value` — the fill value; cloned into every cell.
	///
	/// # Examples
	///
	/// ```rust
	/// let m: Matrix<i32> = Matrix::new(3, 4, 0);
	/// assert_eq!(m.shape(), (3, 4));
	/// assert_eq!(m[(2, 3)], 0);
	/// ```
	pub fn new(rows: usize, cols: usize, value: T) -> Self {
		Self {
			rows,
			cols,
			data: vec![value; rows * cols],
		}
	}

	/// Construct a matrix from a flat, row-major `Vec<T>`.
	///
	/// The elements are interpreted in row-major order: the first `cols`
	/// elements form row 0, the next `cols` elements form row 1, and so on.
	///
	/// # Panics
	///
	/// Panics if `data.len() != rows * cols`.
	///
	/// # Examples
	///
	/// ```rust
	/// // 2 × 3 matrix: [[1, 2, 3], [4, 5, 6]]
	/// let m = Matrix::from_vec(2, 3, vec![1, 2, 3, 4, 5, 6]);
	/// assert_eq!(m[(1, 1)], 5);
	/// ```
	pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Self {
		assert_eq!(
			data.len(),
			rows * cols,
			"data length {} != rows({}) * cols({})",
			data.len(),
			rows,
			cols
		);
		Self { rows, cols, data }
	}

	/// Construct a matrix from a `Vec` of row `Vec`s.
	///
	/// Each inner `Vec` represents one row.  All rows must have the same
	/// length.
	///
	/// # Panics
	///
	/// * Panics if `rows` is empty.
	/// * Panics if any inner `Vec` has a different length than the first one.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![
	///     vec![1, 2, 3],
	///     vec![4, 5, 6],
	/// ]);
	/// assert_eq!(m.shape(), (2, 3));
	/// assert_eq!(m[(0, 2)], 3);
	/// ```
	pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
		let nrows = rows.len();
		assert!(nrows > 0, "matrix must have at least one row");
		let ncols = rows[0].len();
		assert!(
			rows.iter().all(|r| r.len() == ncols),
			"all rows must be the same length"
		);
		let data = rows.into_iter().flatten().collect();
		Self { rows: nrows, cols: ncols, data }
	}
}

impl<T: Default + Clone> Matrix<T> {
	/// Construct a matrix filled with [`Default::default()`].
	///
	/// For numeric types this is `0`; for `String` it is `""`.
	///
	/// Requires `T: Default + Clone`.
	///
	/// # Examples
	///
	/// ```rust
	/// let m: Matrix<f64> = Matrix::zeros(2, 2);
	/// assert_eq!(m[(0, 0)], 0.0);
	///
	/// let s: Matrix<String> = Matrix::zeros(1, 3);
	/// assert_eq!(s[(0, 0)], "");
	/// ```
	pub fn zeros(rows: usize, cols: usize) -> Self {
		Self::new(rows, cols, T::default())
	}
}

impl<T> Matrix<T> {
	/// Construct a matrix by calling a closure `f(i, j)` for every cell.
	///
	/// The closure receives the **row index** `i` and the **column index** `j`
	/// of the cell being filled, both zero-based, and must return the value
	/// that should be stored there.  Cells are visited in row-major order
	/// (row 0 left-to-right, then row 1, …), so a `FnMut` closure may
	/// accumulate state across calls if needed.
	///
	/// No trait bounds are required on `T` beyond what the closure itself
	/// produces, making this the most flexible constructor in the API.
	///
	/// # Arguments
	///
	/// * `rows` — number of rows in the new matrix.
	/// * `cols` — number of columns in the new matrix.
	/// * `f`    — a closure `(i: usize, j: usize) -> T` called once per cell.
	///
	/// # Examples
	///
	/// ```rust
	/// // Identity matrix — 1 on the diagonal, 0 elsewhere
	/// let eye = Matrix::from_builder(3, 3, |i, j| if i == j { 1 } else { 0 });
	/// assert_eq!(eye[(0, 0)], 1);
	/// assert_eq!(eye[(0, 1)], 0);
	/// assert_eq!(eye[(2, 2)], 1);
	///
	/// // Multiplication table
	/// let table = Matrix::from_builder(4, 4, |i, j| (i + 1) * (j + 1));
	/// assert_eq!(table[(2, 3)], 12);  // (2+1) * (3+1)
	///
	/// // String labels from indices
	/// let labels = Matrix::from_builder(2, 3, |i, j| format!("({i},{j})"));
	/// assert_eq!(labels[(1, 2)], "(1,2)");
	///
	/// // Running counter via FnMut
	/// let mut n = 0_u32;
	/// let counter = Matrix::from_builder(2, 3, |_, _| { n += 1; n });
	/// assert_eq!(counter.as_slice(), &[1, 2, 3, 4, 5, 6]);
	/// ```
	pub fn from_builder<F>(rows: usize, cols: usize, mut f: F) -> Self
	                       where
		                       F: FnMut(usize, usize) -> T,
	{
		let data = (0..rows)
			.flat_map(|i| (0..cols).map(move |j| (i, j)))
			.map(|(i, j)| f(i, j))
			.collect();
		Self { rows, cols, data }
	}
}

// ── Dimensions & accessors ────────────────────────────────────────────────────

impl<T> Matrix<T> {
	/// Return the number of rows.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
	/// assert_eq!(m.rows(), 3);
	/// ```
	pub fn rows(&self) -> usize { self.rows }

	/// Return the number of columns.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![1, 2, 3]]);
	/// assert_eq!(m.cols(), 3);
	/// ```
	pub fn cols(&self) -> usize { self.cols }

	/// Return the shape of the matrix as `(rows, cols)`.
	///
	/// # Examples
	///
	/// ```rust
	/// let m: Matrix<i32> = Matrix::zeros(4, 7);
	/// assert_eq!(m.shape(), (4, 7));
	/// ```
	pub fn shape(&self) -> (usize, usize) { (self.rows, self.cols) }

	/// Return the total number of elements (`rows * cols`).
	///
	/// # Examples
	///
	/// ```rust
	/// let m: Matrix<i32> = Matrix::zeros(3, 4);
	/// assert_eq!(m.len(), 12);
	/// ```
	pub fn len(&self) -> usize { self.data.len() }

	/// Return `true` if the matrix contains no elements (either dimension is
	/// zero).
	///
	/// In practice this can only arise if a matrix was constructed via
	/// [`Matrix::from_vec`] with `rows = 0` or `cols = 0`.
	///
	/// # Examples
	///
	/// ```rust
	/// let m: Matrix<i32> = Matrix::from_vec(0, 0, vec![]);
	/// assert!(m.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool { self.data.is_empty() }

	/// Return `true` if the matrix is square (rows == cols).
	///
	/// # Examples
	///
	/// ```rust
	/// let sq: Matrix<i32> = Matrix::zeros(3, 3);
	/// assert!(sq.is_square());
	///
	/// let rect: Matrix<i32> = Matrix::zeros(2, 3);
	/// assert!(!rect.is_square());
	/// ```
	pub fn is_square(&self) -> bool { self.rows == self.cols }

	/// Return a shared slice of the underlying row-major storage.
	///
	/// Elements are in row-major order: index `r * cols + c` corresponds to
	/// element `(r, c)`.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
	/// assert_eq!(m.as_slice(), &[1, 2, 3, 4]);
	/// ```
	pub fn as_slice(&self) -> &[T] { &self.data }

	// ── Private helpers ───────────────────────────────────────────────────────

	/// Convert a `(row, col)` logical index into a flat `Vec` index.
	///
	/// # Panics
	///
	/// Panics if `row >= self.rows` or `col >= self.cols`.
	fn idx(&self, row: usize, col: usize) -> usize {
		assert!(row < self.rows, "row index {row} out of bounds (rows={})", self.rows);
		assert!(col < self.cols, "col index {col} out of bounds (cols={})", self.cols);
		row * self.cols + col
	}

	// ── Element access ────────────────────────────────────────────────────────

	/// Return a shared reference to the element at `(row, col)`.
	///
	/// Prefer the `m[(r, c)]` index syntax for brevity; this method is the
	/// underlying implementation of [`Index<(usize, usize)>`].
	///
	/// # Panics
	///
	/// Panics if `row >= self.rows` or `col >= self.cols`.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![10, 20], vec![30, 40]]);
	/// assert_eq!(m.get(1, 0), &30);
	/// // Equivalent sugar:
	/// assert_eq!(m[(1, 0)], 30);
	/// ```
	pub fn get(&self, row: usize, col: usize) -> &T {
		&self.data[self.idx(row, col)]
	}

	/// Return an exclusive reference to the element at `(row, col)`.
	///
	/// Prefer the `m[(r, c)] = …` index syntax for brevity; this method is
	/// the underlying implementation of [`IndexMut<(usize, usize)>`].
	///
	/// # Panics
	///
	/// Panics if `row >= self.rows` or `col >= self.cols`.
	///
	/// # Examples
	///
	/// ```rust
	/// let mut m: Matrix<i32> = Matrix::zeros(2, 2);
	/// *m.get_mut(0, 1) = 99;
	/// assert_eq!(m[(0, 1)], 99);
	/// // Equivalent sugar:
	/// m[(1, 0)] = 7;
	/// assert_eq!(m[(1, 0)], 7);
	/// ```
	pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
		let i = self.idx(row, col);
		&mut self.data[i]
	}

	// ── Row / column views ────────────────────────────────────────────────────

	/// Return a shared slice of the elements in row `r`.
	///
	/// The slice is a zero-copy view into the underlying storage because rows
	/// are stored contiguously in memory.
	///
	/// # Panics
	///
	/// Panics if `r >= self.rows`.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![1, 2, 3], vec![4, 5, 6]]);
	/// assert_eq!(m.row(1), &[4, 5, 6]);
	/// ```
	pub fn row(&self, r: usize) -> &[T] {
		assert!(r < self.rows, "row index {r} out of bounds (rows={})", self.rows);
		let start = r * self.cols;
		&self.data[start..start + self.cols]
	}

	/// Return a `Vec` of shared references to the elements in column `c`.
	///
	/// Unlike [`row`](Self::row), this cannot return a contiguous slice
	/// because columns are not stored adjacently in row-major layout.
	///
	/// # Panics
	///
	/// Panics if `c >= self.cols`.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
	/// let col: Vec<&i32> = m.col(0);
	/// assert_eq!(col, vec![&1, &3, &5]);
	/// ```
	pub fn col(&self, c: usize) -> Vec<&T> {
		assert!(c < self.cols, "col index {c} out of bounds (cols={})", self.cols);
		(0..self.rows).map(|r| &self.data[r * self.cols + c]).collect()
	}

	// ── Iterators ─────────────────────────────────────────────────────────────

	/// Return an iterator over all elements in row-major order.
	///
	/// The iterator yields shared references `&T`.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
	/// let sum: i32 = m.iter().sum();
	/// assert_eq!(sum, 10);
	/// ```
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.data.iter()
	}

	/// Return an iterator that yields `(row, col, &value)` triples in
	/// row-major order.
	///
	/// This is useful when you need the position of each element alongside
	/// its value without manually computing indices.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![10, 20], vec![30, 40]]);
	/// for (r, c, v) in m.indexed_iter() {
	///     println!("({r},{c}) = {v}");
	/// }
	/// // prints: (0,0)=10  (0,1)=20  (1,0)=30  (1,1)=40
	/// ```
	pub fn indexed_iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
		self.data
		    .iter()
		    .enumerate()
		    .map(|(i, v)| (i / self.cols, i % self.cols, v))
	}
}

// ── Transformations (T: Clone) ────────────────────────────────────────────────

impl<T: Clone> Matrix<T> {
	/// Return the transpose of this matrix.
	///
	/// The transpose of an `m × n` matrix is an `n × m` matrix whose `(i, j)`
	/// element equals the original `(j, i)` element.
	///
	/// A new allocation is always made; the original matrix is not consumed.
	///
	/// **Complexity:** O(rows × cols) time and space.
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![1, 2, 3], vec![4, 5, 6]]);
	/// let t = m.transpose();  // shape (3, 2)
	/// assert_eq!(t.shape(), (3, 2));
	/// assert_eq!(t[(0, 1)], 4);  // was m[(1, 0)]
	/// assert_eq!(t[(2, 0)], 3);  // was m[(0, 2)]
	/// ```
	pub fn transpose(&self) -> Self {
		let mut data = Vec::with_capacity(self.rows * self.cols);
		for c in 0..self.cols {
			for r in 0..self.rows {
				data.push(self.data[r * self.cols + c].clone());
			}
		}
		Self { rows: self.cols, cols: self.rows, data }
	}

	/// Apply a function `f` to every element and return a new matrix of the
	/// results.
	///
	/// The output element type `U` may differ from the input type `T`, making
	/// this suitable for type-changing transforms (e.g. `i32` → `f64` or
	/// numeric → `String`).
	///
	/// # Examples
	///
	/// ```rust
	/// let m = Matrix::from_rows(vec![vec![1, 2, 3], vec![4, 5, 6]]);
	///
	/// // Double every element (same type)
	/// let doubled = m.map(|x| x * 2);
	/// assert_eq!(doubled[(0, 2)], 6);
	///
	/// // Convert to f64 (type change)
	/// let floats: Matrix<f64> = m.map(|x| *x as f64 * 0.5);
	/// assert_eq!(floats[(1, 1)], 2.5);
	/// ```
	pub fn map<U, F: Fn(&T) -> U>(&self, f: F) -> Matrix<U> {
		Matrix {
			rows: self.rows,
			cols: self.cols,
			data: self.data.iter().map(f).collect(),
		}
	}

	/// Combine two same-shape matrices element-wise using a binary function.
	///
	/// For each position `(r, c)` the output contains `f(&self[(r,c)],
	/// &other[(r,c)])`.  The output element type `V` may differ from both
	/// input types.
	///
	/// This is the building block used by the [`Add`] and [`Mul`] operator
	/// impls.
	///
	/// # Panics
	///
	/// Panics if `self.shape() != other.shape()`.
	///
	/// # Examples
	///
	/// ```rust
	/// let a = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
	/// let b = Matrix::from_rows(vec![vec![10, 20], vec![30, 40]]);
	///
	/// // Compute element-wise maximum
	/// let max = a.zip_map(&b, |x, y| (*x).max(*y));
	/// assert_eq!(max[(0, 0)], 10);
	/// assert_eq!(max[(1, 1)], 40);
	/// ```
	pub fn zip_map<U, V, F>(&self, other: &Matrix<U>, f: F) -> Matrix<V>
	                        where
		                        U: Clone,
		                        F: Fn(&T, &U) -> V,
	{
		assert_eq!(self.shape(), other.shape(), "shape mismatch in zip_map");
		Matrix {
			rows: self.rows,
			cols: self.cols,
			data: self.data.iter().zip(other.data.iter()).map(|(a, b)| f(a, b)).collect(),
		}
	}

	/// Consume `self` and return a new matrix with different dimensions but
	/// the same elements in the same row-major order.
	///
	/// No data is copied; only the `rows` and `cols` fields are changed.
	///
	/// # Panics
	///
	/// Panics if `rows * cols != self.len()`.
	///
	/// # Examples
	///
	/// ```rust
	/// // 2×3 → 3×2
	/// let m = Matrix::from_rows(vec![vec![1, 2, 3], vec![4, 5, 6]]);
	/// let r = m.reshape(3, 2);
	/// assert_eq!(r.shape(), (3, 2));
	/// assert_eq!(r[(1, 0)], 3);  // element index 2 in row-major order
	/// ```
	pub fn reshape(self, rows: usize, cols: usize) -> Self {
		assert_eq!(
			rows * cols,
			self.data.len(),
			"reshape: element count must match (got {} * {} = {}, need {})",
			rows, cols, rows * cols, self.data.len()
		);
		Self { rows, cols, data: self.data }
	}

	/// Stack `other` directly below `self`, returning a new matrix.
	///
	/// The two matrices must have the same number of columns.  The resulting
	/// matrix has `self.rows + other.rows` rows and `self.cols` columns.
	///
	/// # Panics
	///
	/// Panics if `self.cols != other.cols`.
	///
	/// # Examples
	///
	/// ```rust
	/// let top    = Matrix::from_rows(vec![vec![1, 2, 3]]);
	/// let bottom = Matrix::from_rows(vec![vec![4, 5, 6]]);
	/// let m = top.vstack(&bottom);
	/// assert_eq!(m.shape(), (2, 3));
	/// assert_eq!(m[(1, 2)], 6);
	/// ```
	pub fn vstack(&self, other: &Self) -> Self {
		assert_eq!(
			self.cols, other.cols,
			"vstack: column counts must match (self={}, other={})",
			self.cols, other.cols
		);
		let mut data = self.data.clone();
		data.extend_from_slice(&other.data);
		Self { rows: self.rows + other.rows, cols: self.cols, data }
	}

	/// Stack `other` to the right of `self`, returning a new matrix.
	///
	/// The two matrices must have the same number of rows.  The resulting
	/// matrix has `self.rows` rows and `self.cols + other.cols` columns.
	///
	/// # Panics
	///
	/// Panics if `self.rows != other.rows`.
	///
	/// # Examples
	///
	/// ```rust
	/// let left  = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
	/// let right = Matrix::from_rows(vec![vec![5], vec![6]]);
	/// let m = left.hstack(&right);
	/// assert_eq!(m.shape(), (2, 3));
	/// assert_eq!(m[(0, 2)], 5);
	/// assert_eq!(m[(1, 2)], 6);
	/// ```
	pub fn hstack(&self, other: &Self) -> Self {
		assert_eq!(
			self.rows, other.rows,
			"hstack: row counts must match (self={}, other={})",
			self.rows, other.rows
		);
		let mut data = Vec::with_capacity((self.cols + other.cols) * self.rows);
		for r in 0..self.rows {
			data.extend_from_slice(self.row(r));
			data.extend_from_slice(other.row(r));
		}
		Self { rows: self.rows, cols: self.cols + other.cols, data }
	}
}

// ── Numeric operations ────────────────────────────────────────────────────────

impl<T: Default + Clone> Matrix<T> {
	/// Compute the matrix product `self × rhs` using caller-supplied
	/// `product` and `addition` operations.
	///
	/// Uses the standard O(n³) triple-loop algorithm.  If `self` is `m × k`
	/// and `rhs` is `k × n`, the result is `m × n`.
	///
	/// Rather than hard-coding the `*` and `+` operators, this method accepts
	/// two explicit closures.  This makes `matmul` work for any type `T` that
	/// has a notion of multiplication and addition — including types that do
	/// not implement the standard [`Mul`] / [`Add`] traits (e.g. a custom
	/// fixed-point number, a modular-arithmetic wrapper, or a semiring
	/// element).  The accumulator cells are initialised with
	/// [`Default::default()`], which must act as the additive identity (zero)
	/// for the chosen `addition` operation.
	///
	/// For element-wise (Hadamard) multiplication, use the `*` operator
	/// instead.
	///
	/// # Type parameters
	///
	/// * `P` — the type of the `product` closure: `FnMut(T, T) -> T`.
	/// * `A` — the type of the `addition` closure: `FnMut(T, T) -> T`.
	///
	/// # Arguments
	///
	/// * `rhs`      — the right-hand matrix.  Must satisfy `self.cols == rhs.rows`.
	/// * `product`  — called as `product(a, b)` to multiply two elements;
	///                corresponds to the `·` in the standard dot-product formula.
	/// * `addition` — called as `addition(acc, prod)` to accumulate the
	///                running sum; corresponds to the `+` in the dot-product.
	///
	/// # Panics
	///
	/// Panics if `self.cols != rhs.rows`.
	///
	/// # Examples
	///
	/// ```rust
	/// // Standard integer matrix multiplication
	/// let a = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
	/// let b = Matrix::from_rows(vec![vec![5, 6], vec![7, 8]]);
	/// let c = a.matmul(&b, |x, y| x * y, |acc, p| acc + p);
	/// // [[1·5+2·7, 1·6+2·8], [3·5+4·7, 3·6+4·8]] = [[19,22],[43,50]]
	/// assert_eq!(c[(0, 0)], 19);
	/// assert_eq!(c[(1, 1)], 50);
	///
	/// // Tropical (min-plus) semiring — shortest-path matrix multiplication.
	/// // Here "product" is addition and "addition" is minimum.
	/// let inf = i32::MAX / 2;
	/// let d = Matrix::from_rows(vec![
	///     vec![0,   3,   inf],
	///     vec![inf, 0,   1  ],
	///     vec![inf, inf, 0  ],
	/// ]);
	/// // One step of repeated squaring gives shortest paths of length ≤ 2.
	/// let d2 = d.matmul(&d,
	///     |a, b| if a == inf || b == inf { inf } else { a + b },
	///     |a, b| a.min(b),
	/// );
	/// assert_eq!(d2[(0, 2)], 4);  // 0 →(3)→ 1 →(1)→ 2
	/// ```
	pub fn matmul<P, A>(&self, rhs: &Self, mut product: P, mut addition: A) -> Self
	                    where
		                    P: FnMut(T, T) -> T,
		                    A: FnMut(T, T) -> T,
	{
		assert_eq!(
			self.cols, rhs.rows,
			"matmul: lhs cols ({}) != rhs rows ({})",
			self.cols, rhs.rows
		);
		let mut out = Self::zeros(self.rows, rhs.cols);
		for r in 0..self.rows {
			for k in 0..self.cols {
				for c in 0..rhs.cols {
					let prod = product(self[(r, k)].clone(), rhs[(k, c)].clone());
					let cur  = out[(r, c)].clone();
					out[(r, c)] = addition(cur, prod);
				}
			}
		}
		out
	}
}

impl<T> Matrix<T>
where
	T: Default + Clone + PartialEq,
{
	/// Return `true` if `self` and `other` have the same shape and every
	/// corresponding pair of elements compares equal under [`PartialEq`].
	///
	/// This is equivalent to the derived `PartialEq` impl (`self == other`),
	/// but is provided as an explicit named method for clarity in numeric
	/// contexts where `==` might be confused with element-wise comparison.
	///
	/// # Examples
	///
	/// ```rust
	/// let a = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
	/// let b = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
	/// let c = Matrix::from_rows(vec![vec![1, 2], vec![3, 5]]);
	///
	/// assert!(a.equals(&b));
	/// assert!(!a.equals(&c));
	/// ```
	pub fn equals(&self, other: &Self) -> bool {
		self.shape() == other.shape() && self.data == other.data
	}
}

// ── Operator overloads ────────────────────────────────────────────────────────

/// Element access via `matrix[(row, col)]`.
///
/// Delegates to [`Matrix::get`].  Panics if either index is out of bounds.
impl<T> Index<(usize, usize)> for Matrix<T> {
	type Output = T;

	fn index(&self, (r, c): (usize, usize)) -> &T {
		self.get(r, c)
	}
}

/// Mutable element access via `matrix[(row, col)] = value`.
///
/// Delegates to [`Matrix::get_mut`].  Panics if either index is out of bounds.
impl<T> IndexMut<(usize, usize)> for Matrix<T> {
	fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut T {
		self.get_mut(r, c)
	}
}

/// Element-wise addition of two matrices via the `+` operator.
///
/// Both matrices must have the same shape.  The operator is implemented on
/// *references* (`&Matrix<T> + &Matrix<T>`) so neither operand is consumed.
///
/// # Panics
///
/// Panics if the shapes differ.
///
/// # Examples
///
/// ```rust
/// let a = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
/// let b = Matrix::from_rows(vec![vec![10, 20], vec![30, 40]]);
/// let c = &a + &b;
/// assert_eq!(c[(0, 0)], 11);
/// assert_eq!(c[(1, 1)], 44);
/// ```
impl<T: Clone + Add<Output = T>> Add for &Matrix<T> {
	type Output = Matrix<T>;

	fn add(self, rhs: &Matrix<T>) -> Matrix<T> {
		self.zip_map(rhs, |a, b| a.clone() + b.clone())
	}
}

/// Element-wise (Hadamard) multiplication of two matrices via the `*` operator.
///
/// Each element `(r, c)` of the result is `self[(r,c)] * rhs[(r,c)]`.
///
/// > **Note:** this is the *Hadamard product*, not standard matrix
/// > multiplication.  For matrix multiplication use [`Matrix::matmul`].
///
/// The operator is implemented on *references* so neither operand is consumed.
///
/// # Panics
///
/// Panics if the shapes differ.
///
/// # Examples
///
/// ```rust
/// let a = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
/// let b = Matrix::from_rows(vec![vec![5, 6], vec![7, 8]]);
/// let c = &a * &b;               // Hadamard product
/// assert_eq!(c[(0, 0)], 5);      //  1·5
/// assert_eq!(c[(1, 1)], 32);     //  4·8
/// ```
impl<T: Clone + Mul<Output = T>> Mul for &Matrix<T> {
	type Output = Matrix<T>;

	fn mul(self, rhs: &Matrix<T>) -> Matrix<T> {
		self.zip_map(rhs, |a, b| a.clone() * b.clone())
	}
}

// ── Display ───────────────────────────────────────────────────────────────────

/// Pretty-print the matrix.
///
/// Each row is printed on its own line in the form `[  v0,  v1, … ]`.
/// Every value is right-aligned in an 8-character field so that columns line
/// up for typical numbers.
///
/// # Examples
///
/// ```rust
/// let m = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
/// println!("{m}");
/// // [        1,        2 ]
/// // [        3,        4 ]
/// ```
impl<T: fmt::Display> fmt::Display for Matrix<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for r in 0..self.rows {
			write!(f, "[")?;
			for c in 0..self.cols {
				if c > 0 { write!(f, ", ")?; }
				write!(f, "{:>8}", self[(r, c)])?;
			}
			writeln!(f, " ]")?;
		}
		Ok(())
	}
}

// endregion

// region Disjoint Set Union
struct DisjointSetUnion {
	/// parent[i] = parent of node i; if parent[i] == i, then i is a root.
	parent: Vec<usize>,
	/// size[i] = number of elements in the set rooted at i (only valid when i is a root).
	size: Vec<usize>,
	/// Current number of disjoint sets remaining.
	num_sets: usize,
}

impl DisjointSetUnion {
	/// Creates n singleton sets, each element its own parent with size 1.
	#[inline]
	fn new(n: usize) -> Self {
		DisjointSetUnion {
			parent: (0..n).collect(),
			size: vec![1; n],
			num_sets: n,
		}
	}

	/// Finds the root of x, applying path halving to flatten the tree as it walks up.
	#[inline]
	fn find(&mut self, mut x: usize) -> usize {
		while self.parent[x] != x {
			self.parent[x] = self.parent[self.parent[x]];
			x = self.parent[x];
		}
		x
	}

	/// Merges the sets containing x and y; returns false if already merged.  
	/// `on_union(new_parent, old_parent)` lets the caller migrate custom per-set
	/// data (max, min, sum, etc.) at the moment of merge — passed per-call,
	#[inline]
	fn union(&mut self, x: usize, y: usize, mut on_union: impl FnMut(usize, usize)) -> bool {
		let mut rx = self.find(x);
		let mut ry = self.find(y);
		if rx == ry {
			return false;
		}
		if self.size[rx] < self.size[ry] {
			std::mem::swap(&mut rx, &mut ry);
		}
		on_union(rx, ry); // old root's data still intact here
		self.parent[ry] = rx;
		self.size[rx] += self.size[ry];
		self.num_sets -= 1;
		true
	}

	#[inline]
	fn same_set(&mut self, x: usize, y: usize) -> bool {
		self.find(x) == self.find(y)
	}

	#[inline]
	fn get_set_size(&mut self, x: usize) -> usize {
		let root = self.find(x);
		self.size[root]
	}

	#[inline]
	fn count_sets(&self) -> usize {
		self.num_sets
	}
}
// endregion