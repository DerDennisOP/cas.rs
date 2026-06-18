use cas_exec::Evaluator;
use cas_io::Repl;

fn main() -> std::io::Result<()> {
    Repl::new(Evaluator).run()
}
