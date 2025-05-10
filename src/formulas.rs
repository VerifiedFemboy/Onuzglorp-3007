use crate::tuforums::clear_info::Judgements;

pub fn acc_by_judgement(
    Judgements(
        early_double,
        early_single,
        e_perfect,
        perfect,
        l_perfect,
        late_single,
        late_double,
    ): Judgements,
) -> f64 {
    let judgement_count =
        (early_double + early_single + e_perfect + perfect + l_perfect + late_single + late_double)
            as f64;
    (perfect as f64
        + (e_perfect as f64 + l_perfect as f64) * 0.75
        + (early_single as f64 + late_single as f64) * 0.4
        + (early_double as f64 + late_double as f64) * 0.2)
        / judgement_count
}

fn x_acc_multiplier(x_acc: f64) -> f64 {
    return match x_acc {
        x if x < 95. => 1.0,
        x if x < 100. => -0.027 / (x_acc / 100. - 1.0054) + 0.513,
        x if x == 100. => 10.,
        _ => 10.,
    };
}

fn speed_multiplier(speed: f64) -> f64 {
    return match speed {
        speed if speed < 1.0 => 0.0,
        speed if speed < 1.1 => -3.5 * (speed - 1.0) + 1.0,
        speed if speed < 1.5 => 0.65,
        speed if speed < 2.0 => 0.7 * (speed - 1.5) + 0.65,
        _ => 1.0,
    };
}

fn score_v2(score: f64, misses: u32, tile_count: u32) -> f64 {
    let am = (misses as f64 - (tile_count as f64 * 10.0 / 315.0).floor() / 10.0).max(0.0);

    let k_one = ((am - 1.0) / 24.5).powf(0.7) * 0.2;
    let k_two = ((50.0 - am) / 24.5).powf(0.7) * 0.2;

    score
        * if am <= 0.0 {
            1.0
        } else if am <= 1.0 {
            0.9
        } else if am <= 25.5 {
            0.9 - k_one
        } else if am <= 50.0 {
            1.0 + k_two - 0.5
        } else {
            0.5
        }
}

pub fn score_final(base_score: f64, x_acc: f64, tile_count: u32, misses: u32, speed: f64) -> f64 {
    let score = base_score
        * x_acc_multiplier(x_acc)
        * speed_multiplier(speed)
        * if misses == 0 { 1.1 } else { 1.0 };
    score_v2(score, misses, tile_count)
}
