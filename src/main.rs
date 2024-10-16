mod grid;
use std::error::Error;

use grid::Grid;
use seq_macro::seq;

use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    /// Size of de grid N*N
    size: usize,
    #[command(flatten)]
    inputs: Inputs,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Inputs {
    /// Path to the grid's CSV file
    #[arg(long)]
    path: Option<std::path::PathBuf>,
    /// grid as a csv string
    #[arg(long)]
    content: Option<String>,
}

seq!(N in 2..=10{
const SIZE_~N: usize = N * N;
}
);

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    if let Some(path) = args.inputs.path {
        seq!(N in 2..=10 {
        match args.size {
            #(SIZE_~N => {
                let mut grid: Grid<SIZE_~N> = Grid::from_csv(
                        &mut csv::ReaderBuilder::new()
                                .has_headers(false)
                                .from_path(path)?,
                        )?;
                        grid.solve();
                        println!("{}", grid);
                    })*
            _ => unimplemented!()
        };
        });
    } else if let Some(content) = args.inputs.content {
        seq!(N in 2..=10 {
            match args.size {
            #(SIZE_~N => {
                let mut grid: Grid<SIZE_~N> = Grid::from_csv(
                        &mut csv::ReaderBuilder::new()
                            .has_headers(false)
                            .from_reader(content.as_bytes()),
                    )?;
                    grid.solve();
                    println!("{}", grid);
                })*
            _ => unimplemented!()
            };
        });
    }
    Ok(())
}
