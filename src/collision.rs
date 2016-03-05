pub fn alive_or_dead(o_p: &mut Vec<i32>, t_p: &mut Vec<i32>) {
    for x in 0..o_p.len() - 1 {
        let k = x + 1;
        if t_p[x] == o_p[x] && t_p[k] == o_p[k]{
            println!("SOMEONE WON!");
        } else {
            true;
        }
    }
}
