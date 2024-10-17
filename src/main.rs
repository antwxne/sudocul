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
    #[command(flatten)]
    outputs: Outputs,
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

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct Outputs {
    /// Path to the the output CSV file
    #[arg(short)]
    csv_file: Option<std::path::PathBuf>,
    /// print in the terminal as a CSV file
    #[arg(long, default_value_t = false)]
    json_term: bool,

    /// print in the terminal in CSV format
    #[arg(long, default_value_t = false)]
    csv_term: bool,
}

seq!(N in 2..=10{
const SIZE_~N: usize = N * N;
}
);

fn display_grid<const SIZE: usize>(
    grid: Grid<SIZE>,
    outputs: &Outputs,
) -> Result<(), Box<dyn Error>> {
    if let Some(outputfile) = &outputs.csv_file {
        grid.to_csv_file(outputfile)?
    } else if outputs.csv_term {
        grid.print_as_csv()?
    } else if outputs.json_term {
        grid.print_as_json()?
    } else {
        println!("{grid}");
    }
    Ok(())
}

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
                        display_grid(grid, &args.outputs)?
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
                    display_grid(grid, &args.outputs)?
                })*
            _ => unimplemented!()
            };
        });
    }
    Ok(())
}
