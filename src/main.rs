mod matrix;

use crate::matrix::Matrix;

fn main() {
    assert_eq!(
        *Matrix::<i32, 2, 2>::default()
            .add_self(&Matrix::new_unit(1))
            .sub_self(&Matrix::new_unit(-2)),
        Matrix::new([[3, 3], [3, 3]])
    );

    let mut mat = Matrix::<i32, 2, 2>::default();
    mat = mat.add(&Matrix::new_unit(1));
    mat = mat.sub(&Matrix::new_unit(-2));
    assert_eq!(mat, Matrix::new([[3, 3], [3, 3]]));

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

    assert_eq!(
        *Matrix::<i32, 2, 2>::default()
            .add_scale_self(&6)
            .mul_scale_self(&2)
            .sub_scale_self(&1)
            .div_scale_self(&3),
        Matrix::new([[3, 3], [3, 3]])
    );

    let mut scale = Matrix::<i32, 2, 2>::default();
    scale = scale.add_scale(&6);
    scale = scale.mul_scale(&2);
    scale = scale.sub_scale(&1);
    scale = scale.div_scale(&3);
    assert_eq!(scale, Matrix::new([[3, 3], [3, 3]]));

    assert_eq!(
        Matrix::new([[4, 0, 6], [6, 6, 1], [0, 7, 9], [7, 4, 0]]).mul(&Matrix::new([
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
}
