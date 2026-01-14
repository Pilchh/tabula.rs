use tabulars::dataframe::core::{DataFrame, Series};

fn main() {
    let s_a = Series::from_vec("a", vec![1.0, 2.0, 3.0]);
    let s_b = Series::from_vec("b", vec![1, 2, 3]);

    let s_v = vec![s_a, s_b];
    let df = DataFrame::from_series(s_v).unwrap();
    println!("{}", df);

    let df_a = df.select(&["a"]);
    println!("{}", df_a);
}
