use crate::receiver::Builder;

#[test]
fn denon() {
    use std::vec::Vec;

    let dists = &[
        0, 136, 65, 18, 16, 18, 15, 19, 49, 18, 15, 19, 48, 19, 15, 19, 48, 19, 15, 18, 15, 19, 48,
        19, 15, 19, 14, 19, 49, 18, 49, 18, 15, 19, 15, 19, 14, 19, 15, 19, 15, 18, 15, 19, 15, 19,
        14, 19, 48, 19, 15, 19, 49, 18, 16, 17, 16, 18, 16, 18, 16, 17, 16, 18, 16, 18, 15, 18, 49,
        18, 16, 17, 17, 17, 16, 18, 16, 17, 16, 18, 16, 18, 15, 18, 16, 17, 17, 17, 16, 18, 16, 18,
        15, 18, 16, 18, 49, 18, 16, 18, 2939, 138, 65, 17, 16, 18, 16, 18, 49, 18, 16, 17, 50, 18,
        15, 18, 50, 17, 16, 18, 16, 17, 50, 17, 16, 18, 16, 17, 18, 49, 18, 16, 16, 17, 18, 16, 18,
        16, 17, 16, 19, 15, 18, 15, 18, 16, 17, 50, 19, 15, 19, 48, 17, 16, 19, 15, 18, 15, 19, 15,
        18, 16, 18, 15, 19, 15, 19, 48, 19, 14, 19, 15, 19, 15, 19, 14, 19, 15, 19, 14, 19, 15, 19,
        14, 19, 15, 19, 15, 18, 15, 19, 15, 19, 14, 20, 47, 19, 15, 19, 2940, 136, 65, 19, 15, 18,
        15, 19, 49, 18, 15, 19, 48, 19, 15, 19, 48, 19, 15, 18, 15, 19, 48, 19, 15, 18, 15, 19, 49,
        18, 49, 18, 15, 19, 15, 19, 14, 19, 15, 19, 15, 18, 15, 19, 15, 19, 14, 19, 48, 19, 15, 19,
        48, 19, 15, 18, 17, 16, 19, 15, 19, 15, 18, 15, 19, 49, 19, 15, 18, 16, 17, 16, 18, 16, 18,
        15, 18, 16, 19, 14, 18, 16, 18, 16, 17, 16, 18, 16, 18, 15, 18, 16, 18, 49, 18, 16, 18,
    ];
    let mut brecv = Builder::new()
        .denon()
        .resolution(40_000)
        .buffer(dists)
        .build();
    let cmds = brecv.iter().collect::<Vec<_>>();

    for cmd in &cmds {
        println!("{:?}", cmd);
    }
}
