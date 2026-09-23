#[derive(Debug)]
pub struct V {
    pub vector: Vec<i32>,
}

fn main() {
    let mut stv = V {
        vector: vec![1, 2, 3],
    };

    push_sum(&mut stv);
    println!("{:?}", stv);
}

pub fn push_sum(stv: &mut V) {
    // your code here
    if stv.vector.len() == 0 {
        stv.vector.push(0);
    }
    else {
        let mut sum = 0;

        for i in 0..stv.vector.len() {
            sum += stv.vector[i];
        }

        stv.vector.push(sum);
    }
}
