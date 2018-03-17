struct Matrix {
    rows: i32,
    cols: i32,
    values: Vec<i32>,
}

struct Vector {
    rows: i32,
    values: Vec<i32>,
}

fn print_values(mat: Matrix){
    for val in &mat.values {
        println!("{}",val)
    }
}

fn main() {
    let rows = 2;
    let cols = 2;
    let values = vec![1,0,0,1];
    let identity = Matrix { rows, cols, values };
    print_values(identity);
}
