#![feature(new_uninit)]
mod matrix;

use crate::matrix::Matrix;

fn main() {
    test_add_sub();
    test_transpose();
    test_scale_math();
    test_mul();
    test_square();
    test_big_mul();
}

fn test_add_sub() {
    assert_eq!(
        *Matrix::<i32, 2, 2>::default()
            .add_ref_self(&Matrix::new_unit(1))
            .sub_ref_self(&Matrix::new_unit(-2)),
        Matrix::new([[3, 3], [3, 3]])
    );
    assert_eq!(
        Matrix::<i32, 2, 2>::default() + Matrix::new_unit(1) - Matrix::new_unit(-2),
        Matrix::new([[3, 3], [3, 3]])
    );

    let mut mat = Matrix::<i32, 2, 2>::default();
    mat = mat.add_ref(&Matrix::new_unit(1));
    mat = mat.sub_ref(&Matrix::new_unit(-2));
    assert_eq!(mat, Matrix::new([[3, 3], [3, 3]]));

    let mut mat = Matrix::<i32, 2, 2>::default();
    mat += Matrix::new_unit(1);
    mat -= Matrix::new_unit(-2);
    assert_eq!(mat, Matrix::new([[3, 3], [3, 3]]));
}

fn test_transpose() {
    let mut transpose = Matrix::new([[4, 6, 0, 7], [0, 6, 7, 4], [6, 1, 9, 0], [8, 3, 2, 5]]);
    assert_eq!(
        transpose.transpose(),
        Matrix::new([[4, 0, 6, 8], [6, 6, 1, 3], [0, 7, 9, 2], [7, 4, 0, 5]])
    );
    transpose.transpose_self();
    assert_eq!(
        transpose,
        Matrix::new([[4, 0, 6, 8], [6, 6, 1, 3], [0, 7, 9, 2], [7, 4, 0, 5]])
    );
}

fn test_scale_math() {
    assert_eq!(
        *Matrix::<i32, 2, 2>::default()
            .add_scale_self(&6)
            .mul_scale_self(&2)
            .sub_scale_self(&1)
            .div_scale_self(&3),
        Matrix::new([[3, 3], [3, 3]])
    );

    let mut scale = Matrix::<i32, 2, 2>::default();
    scale += 6;
    scale *= 2;
    scale -= 1;
    scale /= 3;
    assert_eq!(scale, Matrix::new([[3, 3], [3, 3]]));

    let mut scale = Matrix::<i32, 2, 2>::default();
    scale = scale.add_scale(&6);
    scale = scale.mul_scale(&2);
    scale = scale.sub_scale(&1);
    scale = scale.div_scale(&3);
    assert_eq!(scale, Matrix::new([[3, 3], [3, 3]]));

    assert_eq!(
        (((Matrix::<i32, 2, 2>::default() + 6) * 2) - 1) / 3,
        Matrix::new([[3, 3], [3, 3]])
    );
}

fn test_mul() {
    assert_eq!(
        Matrix::new([[4, 0, 6], [6, 6, 1], [0, 7, 9], [7, 4, 0]]).mul_ref(&Matrix::new([
            [4, 0, 6, 6],
            [6, 1, 0, 7],
            [9, 7, 4, 0]
        ])),
        Matrix::new([
            [70, 42, 48, 24],
            [69, 13, 40, 78],
            [123, 70, 36, 49],
            [52, 4, 42, 70]
        ])
    );
    assert_eq!(
        Matrix::new([[4, 0, 6], [6, 6, 1], [0, 7, 9], [7, 4, 0]])
            * Matrix::new([[4, 0, 6, 6], [6, 1, 0, 7], [9, 7, 4, 0]]),
        Matrix::new([
            [70, 42, 48, 24],
            [69, 13, 40, 78],
            [123, 70, 36, 49],
            [52, 4, 42, 70]
        ])
    );

    assert_eq!(
        *Matrix::new([[4, 0, 6, 8], [6, 6, 1, 5], [0, 7, 9, 1], [7, 4, 0, 2]]).mul_ref_self(
            &Matrix::new([[4, 0, 6, 6], [6, 1, 0, 7], [9, 7, 4, 0], [1, 3, 9, 8]])
        ),
        Matrix::new([
            [78, 66, 120, 88],
            [74, 28, 85, 118],
            [124, 73, 45, 57],
            [54, 10, 60, 86]
        ])
    );

    let mut mat = Matrix::new([[4, 0, 6, 8], [6, 6, 1, 5], [0, 7, 9, 1], [7, 4, 0, 2]]);
    mat *= Matrix::new([[4, 0, 6, 6], [6, 1, 0, 7], [9, 7, 4, 0], [1, 3, 9, 8]]);
    assert_eq!(
        mat,
        Matrix::new([
            [78, 66, 120, 88],
            [74, 28, 85, 118],
            [124, 73, 45, 57],
            [54, 10, 60, 86]
        ])
    );
}

fn test_square() {
    assert!(Matrix::new([[1, 0, 0, 0], [1, 1, 0, 0], [1, 1, 1, 0], [1, 1, 1, 1]]).is_trig_inf());
    assert!(!Matrix::new([[4, 0, 6, 8], [6, 6, 1, 3], [0, 7, 9, 2], [7, 4, 0, 5]]).is_trig_inf());
    assert!(Matrix::new([[1, 1, 1, 1], [0, 1, 1, 1], [0, 0, 1, 1], [0, 0, 0, 1]]).is_trig_sup());
    assert!(!Matrix::new([[4, 0, 6, 8], [6, 6, 1, 3], [0, 7, 9, 2], [7, 4, 0, 5]]).is_trig_sup());

    assert!(!Matrix::new([[4, 0, 6, 8], [6, 6, 1, 3], [0, 7, 9, 2], [7, 4, 0, 5]]).is_scale(1));
    assert!(Matrix::<i32, 5, 5>::new_scale(5).is_diagonale());
    assert!(Matrix::<i32, 5, 5>::new_scale(5).is_scale(5));
}

fn test_big_mul() {
    assert_eq!(
        &Matrix::<i32, 4, 4>::default_box(),
        &Box::new(Matrix::<i32, 4, 4>::default())
    );

    const SIDE: usize = 512;
    let mut mat_a = Matrix::<i32, SIDE, SIDE>::new_box_unit(69);
    let mat_b = Matrix::<i32, SIDE, SIDE>::new_box_scale(2);
    assert_eq!(
        mat_a.mul_ref_self(&mat_b),
        Matrix::new_box_unit(138).as_mut()
    );
}
