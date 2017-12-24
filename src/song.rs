
use hound;
use hound::{WavReader, WavWriter, WavSpec};

use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter};
use std::fs::File;

pub struct Song {
    channels: u16,
    sample_rate: u32,
    last_sample: u32,
    reader: WavReader<BufReader<File>>
}

impl Song {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let reader = WavReader::open(path).expect("Invalid path to file");
        let WavSpec { channels, sample_rate, .. } = reader.spec();

        Song {
            channels,
            sample_rate,
            reader: reader,
            last_sample: 0
        }
    }

    pub fn split_by<P: AsRef<Path>>(&mut self, intervals: &[(u32, u32)], file: P) {
        for &(from, to) in intervals {
            let mut writer = self.create_writer(file.as_ref(), from, to);

            let (from, to) = self.translate_interval(from, to);

            let samples = self.reader.samples::<i16>();
            let portion = samples
                .skip_while(|x| *x.as_ref().unwrap() == 0)
                .skip(from as usize)
                .take((to - from) as usize);

            for sample in portion {
                writer.write_sample(sample.unwrap()).unwrap();
            }

            writer.finalize().unwrap();
        }
    }

    pub fn duration(&self) -> u32 {
        self.reader.len()
    }

    fn translate_interval(&mut self, from: u32, to: u32) -> (u32, u32) {
        let from = from * self.sample_rate * self.channels as u32;
        let from = from.checked_sub(self.last_sample).unwrap();
        let to = to * self.sample_rate * self.channels as u32;
        let to = to.checked_sub(self.last_sample).unwrap();
        self.last_sample = to;
        (from, to)
    }

    fn create_writer(&self, file: &Path, from: u32, to: u32) -> WavWriter<BufWriter<File>> {
        let file = gen_file_name(file, from, to);
        WavWriter::create(&file, spec(self.channels, self.sample_rate)).unwrap()
    }
}

fn gen_file_name(file: &Path, from: u32, to: u32) -> PathBuf {
    let file_name = file.file_stem().unwrap().to_str().unwrap();
    let path = file.parent().unwrap();
    let file = format!("{}_{}_{}.wav", file_name, from, to);
    path.join(file)
}

fn spec(channels: u16, sample_rate: u32) -> WavSpec {
    WavSpec {
        channels: channels,
        sample_rate: sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int
    }
}
