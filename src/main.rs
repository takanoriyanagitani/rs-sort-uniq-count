use std::process::ExitCode;

use std::io;

use std::io::BufRead;
use std::io::BufReader;

use std::io::BufWriter;
use std::io::Write;

use std::collections::BTreeMap;

use rs_sort_uniq_count::count::btree::strings2count_limited;
use rs_sort_uniq_count::count::btree::UNIQUE_ELEMENT_COUNT_LIMIT_DEFAULT;

use rs_sort_uniq_count::OutputMode;
use rs_sort_uniq_count::OUT_MODE_DEFAULT;

fn stdin2lines() -> impl Iterator<Item = Result<String, io::Error>> {
    let i = io::stdin();
    let il = i.lock();
    let br = BufReader::new(il);
    br.lines()
}

fn envkey2val(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(io::Error::other)
}

fn unique_element_count_limit() -> usize {
    envkey2val("ENV_UNIQUE_ELEMENT_LIMIT")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(UNIQUE_ELEMENT_COUNT_LIMIT_DEFAULT)
}

fn map2writer<W>(m: &BTreeMap<String, u32>, mut wtr: W) -> Result<(), io::Error>
where
    W: FnMut(&str, u32) -> Result<(), io::Error>,
{
    for pair in m {
        let (key, val) = pair;
        let ks: &str = key;
        let v: u32 = *val;
        wtr(ks, v)?;
    }
    Ok(())
}

fn map2stdout(m: &BTreeMap<String, u32>) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();

    let mut bw = BufWriter::new(&mut ol);

    map2writer(m, |key: &str, val: u32| {
        writeln!(&mut bw, "{val}	{key}")?;
        Ok(())
    })?;

    bw.flush()?;
    drop(bw);

    ol.flush()?;
    Ok(())
}

fn map2stdout_json(m: &BTreeMap<String, u32>) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();

    let mut bw = BufWriter::new(&mut ol);

    map2writer(m, |key: &str, val: u32| {
        #[derive(serde::Serialize)]
        struct Pair<'a> {
            pub value: &'a str,
            pub count: u32,
        }
        let p: Pair = Pair {
            value: key,
            count: val,
        };
        serde_json::to_writer(&mut bw, &p)?;
        writeln!(&mut bw)?;
        Ok(())
    })?;

    bw.flush()?;
    drop(bw);

    ol.flush()?;
    Ok(())
}

fn stdin2stdout_json() -> Result<(), io::Error> {
    let lines = stdin2lines();
    let limit: usize = unique_element_count_limit();
    let m: BTreeMap<String, u32> = strings2count_limited(lines, limit)?;
    map2stdout_json(&m)?;
    Ok(())
}

fn stdin2stdout_plain() -> Result<(), io::Error> {
    let lines = stdin2lines();
    let limit: usize = unique_element_count_limit();
    let m: BTreeMap<String, u32> = strings2count_limited(lines, limit)?;
    map2stdout(&m)?;
    Ok(())
}

fn output_mode() -> OutputMode {
    match std::env::var("ENV_OUTPUT_MODE").ok() {
        None => OUT_MODE_DEFAULT,
        Some(s) => str::parse(s.as_str()).ok().unwrap_or(OUT_MODE_DEFAULT),
    }
}

fn stdin2stdout() -> Result<(), io::Error> {
    match output_mode() {
        OutputMode::Plain => stdin2stdout_plain(),
        OutputMode::Json => stdin2stdout_json(),
    }
}

fn main() -> ExitCode {
    stdin2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
