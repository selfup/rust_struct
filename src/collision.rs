pub fn alive_or_dead(o_p: &mut Vec<i32>, t_p: &mut Vec<i32>, status: &mut Vec<i32>) {
    for x in 0..o_p.len() - 1 {
        if t_p[x] == o_p[x] && t_p[x + 1] == o_p[x + 1]{
            status.pop();
            status.push(0);
        } else {
            status.pop();
            status.push(1);
        }
    }
}

#[test]
fn it_can_tell_if_a_bike_died() {
    let mut one_pos = vec![];
    let mut two_pos = vec![];
    let mut bike_stats = vec![];

    one_pos.push(0);
    one_pos.push(50);
    two_pos.push(0);
    two_pos.push(50);

    alive_or_dead(&mut one_pos, &mut two_pos, &mut bike_stats);

    assert_eq!(0, bike_stats[0]);
}

#[test]
fn it_can_tell_if_a_bike_is_alive() {
    let mut one_pos = vec![];
    let mut two_pos = vec![];
    let mut bike_stats = vec![];

    one_pos.push(100);
    one_pos.push(50);
    two_pos.push(0);
    two_pos.push(50);

    alive_or_dead(&mut one_pos, &mut two_pos, &mut bike_stats);

    assert_eq!(1, bike_stats[0]);
}
