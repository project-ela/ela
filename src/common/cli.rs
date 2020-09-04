use std::env;

#[derive(Default)]
pub struct CompilerConfig {
    pub input_file: String,
    pub output_file: String,
    pub optimize: bool,
    pub dump_tokens: bool,
    pub dump_nodes: bool,
    pub dump_tac: bool,
}

pub fn parse_arguments() -> Result<CompilerConfig, ()> {
    let mut config = CompilerConfig::default();
    let args: Vec<String> = env::args().skip(1).collect();
    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "--optimize" => config.optimize = true,
            "--dump-tokens" => config.dump_tokens = true,
            "--dump-nodes" => config.dump_nodes = true,
            "--dump-tac" => config.dump_tac = true,
            _ => {
                if args.len() != i + 2 {
                    return Err(());
                }
                config.input_file = args[i].to_owned();
                config.output_file = args[i + 1].to_owned();
                return Ok(config);
            }
        }
    }
    Err(())
}
