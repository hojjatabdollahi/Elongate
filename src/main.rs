use anyhow::{anyhow, Context, Result};
use clap::{CommandFactory, Parser};
use execute::Execute;
use std::env;
use std::process::{Command, Stdio};
use twelf::{config, Layer};

const FFMPEG_PATH: &str = "ffmpeg";
const SOUND_STRETCH_PATH: &str = "soundstretch";

fn convert(input_file: &str) -> Result<()> {
    let mut ffmpeg = Command::new(FFMPEG_PATH);
    // ffmpeg.arg("-version");
    ffmpeg.arg("-y");
    ffmpeg.arg("-i");
    ffmpeg.arg(input_file);
    ffmpeg.arg("-acodec");
    ffmpeg.arg("pcm_s16le");
    ffmpeg.arg("/tmp/1.wav");

    ffmpeg.stdout(Stdio::piped());
    ffmpeg.stderr(Stdio::piped());

    let output = ffmpeg.execute_output().context("Failed to run ffmepg")?;

    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Ok: ");
            println!("{}", String::from_utf8(output.stdout).unwrap());
        } else {
            println!("Failed:");
            return Err(anyhow!("{}", String::from_utf8(output.stderr).unwrap()));
        }
    } else {
        println!("Interrupted");
        return Err(anyhow!("{}", String::from_utf8(output.stderr).unwrap()));
    }

    Ok(())
}

fn stretch(tempo: i32, output_file: &str) -> Result<()> {
    let mut soundstretch = Command::new(SOUND_STRETCH_PATH);
    soundstretch.arg("/tmp/1.wav");
    soundstretch.arg(output_file);
    soundstretch.arg(format!("-tempo={}", tempo));

    soundstretch.stdout(Stdio::piped());
    soundstretch.stderr(Stdio::piped());

    let output = soundstretch
        .execute_output()
        .context("Failed to run soundstretch")?;

    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Ok: ");
            println!("{}", String::from_utf8(output.stdout).unwrap());
        } else {
            println!("Failed:");
            return Err(anyhow!("{}", String::from_utf8(output.stderr).unwrap()));
        }
    } else {
        println!("Interrupted");
        return Err(anyhow!("{}", String::from_utf8(output.stderr).unwrap()));
    }

    Ok(())
}

fn stretch_alignment(input_file: &str, tempo: i32, output_file: &str) -> Result<()> {
    let input_file = input_file.replace(".wav", ".ali");
    let output_file = output_file.replace(".wav", ".ali");
    let timings =
        std::fs::read_to_string(input_file).context("Failed to read the alignment file")?;
    let num_of_timings = timings.lines().count();
    let mut timings_vec = Vec::with_capacity(num_of_timings);

    let multiplier = 1.0 - (tempo as f32 / 100.0);

    for timing in timings.lines() {
        if timing.trim().is_empty() {
            continue;
        }
        let cells: Vec<_> = timing.trim().split_whitespace().collect();
        if cells.len() != 8 {
            return Err(anyhow!("This line is not correct: {}", timing));
        }
        timings_vec.push(
            vec![
                (((cells[0]
                    .parse::<i32>()
                    .context("Failed to parse the first cell to i32")? as f32)
                    * multiplier) as i32)
                    .to_string(),
                format!("{}  {}  {}  {}", cells[1], cells[2], cells[3], cells[4]),
                (((cells[5]
                    .parse::<i32>()
                    .context("Failed to parse the 6th cell to i32")? as f32)
                    * multiplier) as i32)
                    .to_string(),
                format!("{}  {}", cells[6], cells[7]),
            ]
            .join("  "),
        );
    }

    std::fs::write(output_file, timings_vec.join("\n"))
        .context("Couldn't write to the output file for ali")?;
    Ok(())
}

#[config]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// The new tempo for the audio, use negative numbers to slow down the audio
    #[clap(short, long, default_value_t = -15)]
    tempo: i32,

    #[clap(
        short,
        long,
        default_value_t = String::from("in.wav")
    )]
    input_file: String,

    #[clap(
        short,
        long,
        default_value_t = String::from("out.wav")
    )]
    output_file: String,
}

fn main() -> Result<()> {
    // let args = Args::parse();
    let matches = Args::command().get_matches();
    let mut config_layers = vec![Layer::Env(None)];
    let mut config_path = env::current_exe()?;
    config_path.pop();
    config_path.push("elongate.toml");
    if config_path.exists() {
        config_layers.push(Layer::Toml("elongate.toml".into()));
    } else {
        println!("Configfile not found");
    }
    config_layers.push(Layer::Clap(matches));
    let config = Args::with_layers(&config_layers).unwrap();

    println!("tempo: {}", config.tempo);
    println!("input file: {}", config.input_file);
    println!("output file: {}", config.output_file);
    convert(&config.input_file)?;
    stretch(config.tempo, &config.output_file)?;
    stretch_alignment(&config.input_file, config.tempo, &config.output_file)?;
    Ok(())
}
