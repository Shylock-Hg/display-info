use display_info::DisplayInfo;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let display_infos = DisplayInfo::all().unwrap();
    let display_info = display_infos[0].clone();
    println!("scale is: {}", display_info.scale);
    for display_info in display_infos {
        println!("display_info {display_info:?}");
    }
    let display_info = DisplayInfo::from_point(100, 100).unwrap();
    println!("display_info {display_info:?}");
    println!("运行耗时: {:?}", start.elapsed());
}
