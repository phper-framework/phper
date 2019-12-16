pub trait ModuleContext {
    fn new() -> Self;

    fn has_ini_entries(&self) -> bool {
        false
    }
}

pub trait RequestContext {}
