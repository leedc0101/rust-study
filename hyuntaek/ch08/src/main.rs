fn main(){
    

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores =
        teams.into_iter().zip(initial_scores.into_iter());

    println!("{:?}",scores);
}