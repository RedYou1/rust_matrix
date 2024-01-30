#![feature(new_uninit)]
mod matrix;

use crate::matrix::Matrix;

fn main() {
    test_add_sub_rem();
    test_transpose();
    test_scale_math();
    test_mul();
    test_square();
    test_big_mul();
    test_neg();
    test_not();
    test_inv();
}

fn test_add_sub_rem() {
    assert_eq!(
        *Matrix::<i32, 2, 2>::default()
            .add_ref_self(&Matrix::new_unit(4))
            .sub_ref_self(&Matrix::new_unit(-3))
            .rem_ref_self(&Matrix::new_unit(4)),
        Matrix::new([[3, 3], [3, 3]])
    );
    assert_eq!(
        (Matrix::<i32, 2, 2>::default() + Matrix::new_unit(4) - Matrix::new_unit(-3))
            % Matrix::new_unit(4),
        Matrix::new([[3, 3], [3, 3]])
    );

    let mut mat = Matrix::<i32, 2, 2>::default();
    mat = mat.add_ref(&Matrix::new_unit(4));
    mat = mat.sub_ref(&Matrix::new_unit(-3));
    mat = mat.rem_ref(&Matrix::new_unit(-4));
    assert_eq!(mat, Matrix::new([[3, 3], [3, 3]]));

    let mut mat = Matrix::<i32, 2, 2>::default();
    mat += Matrix::new_unit(4);
    mat -= Matrix::new_unit(-3);
    mat %= Matrix::new_unit(-4);
    assert_eq!(mat, Matrix::new([[3, 3], [3, 3]]));
}

fn test_transpose() {
    let transpose = Matrix::new([[4, 6, 0, 7], [0, 6, 7, 4], [6, 1, 9, 0]]);
    assert_eq!(
        transpose.transpose(),
        Matrix::new([[4, 0, 6], [6, 6, 1], [0, 7, 9], [7, 4, 0]])
    );
    let mut transpose = Matrix::new([[4, 6, 0, 7], [0, 6, 7, 4], [6, 1, 9, 0], [8, 3, 2, 5]]);
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

    assert!(!Matrix::new([[4, 0, 6, 8], [6, 6, 1, 3], [0, 7, 9, 2], [7, 4, 0, 5]]).is_scale(&1));
    assert!(Matrix::<i32, 5, 5>::new_scale(5).is_diagonale());
    assert!(Matrix::<i32, 5, 5>::new_scale(5).is_scale(&5));
}

fn test_big_mul() {
    const SIDE: usize = 512;

    assert_eq!(
        &Matrix::<i32, 4, 4>::default_box(),
        &Box::<Matrix::<i32, 4, 4>>::default()
    );

    assert_eq!(
        &Matrix::<usize, 4, 4>::new_box_fn(|x, y| x * y),
        &Box::new(Matrix::<usize, 4, 4>::new([
            [0, 0, 0, 0],
            [0, 1, 2, 3],
            [0, 2, 4, 6],
            [0, 3, 6, 9]
        ]))
    );
    assert_eq!(
        &Matrix::<usize, 10, 10>::new_box_fn(|x, y| x * y),
        &Box::new(Matrix::<usize, 10, 10>::new_fn(|x, y| x * y))
    );

    let mut mat_a = Matrix::<i32, SIDE, SIDE>::new_box_unit(69);
    let mat_b = Matrix::<i32, SIDE, SIDE>::new_box_scale(2);
    assert_eq!(
        mat_a.mul_ref_self(&mat_b),
        Matrix::new_box_unit(138).as_mut()
    );
}

fn test_neg() {
    assert_eq!(-Matrix::<i32, 5, 5>::new_unit(1), Matrix::new_unit(-1));
    assert_eq!(-Matrix::<f64, 5, 5>::new_unit(1.0), Matrix::new_unit(-1.0));
}

fn test_not() {
    assert_eq!(!Matrix::<u8, 5, 5>::new_unit(1), Matrix::new_unit(!1));
    assert_eq!(!Matrix::<i64, 5, 5>::new_unit(450), Matrix::new_unit(!450));
}

fn test_inv() {
    let mat_a = Matrix::<f64, 2, 2>::new([[7.1, 3.6], [1.9, 3.1]]);
    let mat_b = mat_a.inverse().expect("cant inverse mat_a");
    let mat_i = Matrix::new_scale(1.0);
    assert_eq!(mat_a.mul_ref(&mat_b), mat_i);
    assert_eq!(mat_b.mul_ref(&mat_a), mat_i);

    let [[a1, b1], [c1, d1]] = mat_b.as_ref();
    let mat_a = Matrix::<f32, 2, 2>::new([[7.1, 3.6], [1.9, 3.1]]);
    let mat_b = mat_a.inverse().expect("cant inverse mat_a");
    let [[a2, b2], [c2, d2]] = mat_b.as_ref();

    assert!((*a1 - f64::from(*a2)).abs() < f64::from(f32::EPSILON));
    assert!((*b1 - f64::from(*b2)).abs() < f64::from(f32::EPSILON));
    assert!((*c1 - f64::from(*c2)).abs() < f64::from(f32::EPSILON));
    assert!((*d1 - f64::from(*d2)).abs() < f64::from(f32::EPSILON));
}
