// anim
//
// A framework independent animation library for rust, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use anim::{timeline::Status, Animatable, Options, Timeline};
use std::time::Duration;

#[derive(Clone, Debug)]
struct MyModel {
    a: f32, //animatable
    b: i64, //animatable
}

// make it animatable, do not forget to derive Clone
impl Animatable for MyModel {
    fn animate(&self, to: &Self, time: f64) -> Self {
        let a = self.a.animate(&to.a, time);
        let b = self.b.animate(&to.b, time);
        MyModel { a, b }
    }
}

// once it's animatable, you can use it with anim::timeline::Options;

fn main() {
    let from = MyModel { a: 0.0, b: 32 };
    let to = MyModel { a: 100.0, b: 100 };
    let mut timeline: Timeline<_> = Options::new(from, to)
        .duration(Duration::from_secs(2))
        .times(1.5)
        .into();

    println!("start animation");
    timeline.begin();

    loop {
        let status = timeline.update();
        if status == Status::Completed {
            break;
        }
        let value = timeline.value();
        println!("animated: {:?}", value);
    }
}
