pub mod lambda;
pub mod events;

use events::aws_lambda;
use lambda::event_loop;

pub fn start(lambda: fn(&aws_lambda::LambdaEvent) -> String) {
    event_loop::start_loop(lambda);
}
