// 定義 enum
enum TrafficLight {
    Red,
    Yellow,
    Green { duration: u32 },
}
fn light_action(light: TrafficLight) -> String {
    match light {
        TrafficLight::Red => String::from("Stop!"),
        TrafficLight::Yellow => "Caution!".to_string(),
        TrafficLight::Green { duration } => format!("Go! ({} seconds left)", duration),
    }
}

fn next_light(current: TrafficLight) -> TrafficLight {
    match current {
        TrafficLight::Red => TrafficLight::Green { duration: 30 },
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Green { .. } => TrafficLight::Yellow,
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("{}", light_action(light));
    // 輸出: Stop!

    let light = TrafficLight::Green { duration: 25 };
    println!("{}", light_action(light));
    // 輸出: Go! (25 seconds left)

    let next = next_light(TrafficLight::Yellow);
    println!("{}", light_action(next));
    // next 應該是 Green { duration: 30 }
}
