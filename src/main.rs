// mod arrays;
// mod print;
// mod strings;
// mod tuples;
// mod vectors;
mod parser;

// command line:
// cargo clean
// cargo run

fn main() {
    let data: String = String::from("https://tauri.localhost/#access_token=BQBRFS3PWbtt3E4IloYrZ3Qxxs-ewqJ6izmVb3IlhkDK19yrlAupUsIImANnU92E9Mrwo1Y7CdoxkJuU3OpEjvRICHnui9bddVj1c6qm7mhigHJjGMXEAiXgk5ulKe5G1kcUyrfm0ZowEk-QMPnBlrmPr-KdYzGKkYP589ijK12J5X_iKueDm1mLPHRTxiRarb5C_64X&token_type=Bearer&expires_in=3600");
    let result = parser::parse_url(data);
    println!("{}", result);
}
