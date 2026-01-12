use tabulars::dataframe::core::Series;

fn main() {
    let s = Series::from_vec("a", vec![1.0, 2.0, 3.0]);
    println!("{}", s);
}
