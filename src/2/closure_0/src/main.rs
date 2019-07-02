use std::thread;
use std::time::Duration;

/*
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    // ゆっくり計算します
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
fn generate_workout(intensity: u32, random_number: u32) {
//    let expensive_result = simulated_expensive_calculation(intensity);
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        // 今日は{}回腕立て伏せをしてください！
        println!("Today, do {} pushups!", expensive_closure(intensity));
        // 次に、{}回腹筋をしてください！
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            // 今日は、{}分間走ってください！
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}
/*
fn generate_workout(intensity: u32, random_number: u32) {
//    let expensive_result = simulated_expensive_calculation(intensity);
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        // 今日は{}回腕立て伏せをしてください！
        println!("Today, do {} pushups!", expensive_result);
        // 次に、{}回腹筋をしてください！
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            // 今日は、{}分間走ってください！
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}
*/
