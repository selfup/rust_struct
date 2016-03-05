pub fn alive_or_dead(o_p: &mut Vec<i32>, t_p: &mut Vec<i32>) {
    for x in 0..o_p.len() - 1 {
        if t_p[x] == o_p[x] && t_p[x + 1] == o_p[x + 1]{
            println!("DEAD");
        } else {
            println!("ALIVE");
        }
    }
}

#[test]
fn it_can_log_all_positions() {
    let mut two_pos = vec![];
    let mut all_pos = vec![];


}
