extern crate yew;
extern crate z1router;

use yew::prelude::*;
use z1router::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
