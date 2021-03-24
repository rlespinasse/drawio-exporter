pub type App = clap::App<'static, 'static>;
pub type Arg = clap::Arg<'static, 'static>;

pub fn arg(name: &'static str) -> Arg {
    Arg::with_name(name)
}
