pub trait Throwable {}

pub struct MyException;

impl Throwable for MyException {}

pub struct Exception {}

impl Throwable for Exception {}
