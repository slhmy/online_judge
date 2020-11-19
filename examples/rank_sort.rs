#[derive(Debug, Clone)]
struct OwnerPreview {
    id: i32,
    username: String,
}

#[derive(Debug, Clone)]
struct ProblemPreview {
    id: i32,
    title: String,
    state: String,
    is_accepted: bool,
    try_times: i32,
    penalty: i32,
}

#[derive(Debug, Clone)]
struct RankColume {
    //owner: OwnerPreview,
    total_accepted: i32,
    total_penalty: i32,
    //problem_previews: Vec<ProblemPreview>,
}

fn main() {
    let mut rank_columes = vec![
        RankColume {
            total_accepted: 1,
            total_penalty: 20*2 + 40,
        },
        RankColume {
            total_accepted: 1,
            total_penalty: 60,
        },
        RankColume {
            total_accepted: 0,
            total_penalty: 0,
        }
    ];

    let slice = rank_columes.as_mut_slice();
    slice.sort_by(|colume_a, colume_b| {
        if colume_a.total_accepted != colume_b.total_accepted {
            colume_a.total_accepted.cmp(&colume_b.total_accepted).reverse()
        } else {
            colume_a.total_penalty.cmp(&colume_b.total_penalty)
        }
    });
    println!("{:?}", slice.to_vec());
}