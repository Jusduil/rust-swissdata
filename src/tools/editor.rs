use super::message::{LinkedMessage, Message, Translated};

struct Editor<S> {
    name: LinkedMessage<S>,
    //datasets: Vec<Box<dyn Dataset<S, Store = dyn ToString>>>,
}
