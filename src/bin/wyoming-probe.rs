//! Local health check and paced, existing-WAV smoke test. Never records audio.
#[path = "../server/wyoming.rs"]
// Framing is shared with the server; its payload handlers are unused by this client.
// Keep dead-code checks enabled for the server and the rest of this binary.
#[allow(dead_code)]
mod wyoming;
use serde_json::json;
use std::{io::{BufReader, Write}, net::TcpStream, time::{Duration, Instant}};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let smoke = matches!(args.get(1).map(String::as_str), Some("--smoke" | "--smoke-fast"));
    let paced = args.get(1).map(String::as_str) != Some("--smoke-fast");
    let address = if smoke { args.get(2).ok_or("Missing HOST:PORT")?.clone() }
        else { format!("127.0.0.1:{}", std::env::var("PORT").unwrap_or("10321".into())) };
    let mut socket = TcpStream::connect_timeout(&address.parse()?, Duration::from_secs(5))?;
    socket.set_read_timeout(Some(Duration::from_secs(10)))?;
    socket.set_write_timeout(Some(Duration::from_secs(10)))?;
    let mut reader = BufReader::new(socket.try_clone()?);
    wyoming::write_event(&mut socket, "describe", json!({}))?;
    let event = wyoming::read_event(&mut reader)?.ok_or("No Describe response")?;
    if event.kind != "info" || event.data.get("asr").and_then(|v|v.as_array()).map_or(true, |v|v.is_empty()) {
        return Err("Describe did not declare ASR".into());
    }
    println!("{}", json!({"event":"describe_ok","data":event.data}));
    if smoke {
        if !(4..=5).contains(&args.len()) { return Err("Expected one or two WAV paths".into()); }
        for path in &args[3..] {
            let mut wav = hound::WavReader::open(path)?;
            let spec = wav.spec();
            if spec.sample_rate != 16000 || spec.channels != 1 || spec.bits_per_sample != 16 || spec.sample_format != hound::SampleFormat::Int {
                return Err("Smoke WAV must be PCM16 16000 Hz mono".into());
            }
            let samples: Vec<i16> = wav.samples().collect::<Result<_,_>>()?;
            wyoming::write_event(&mut socket,"transcribe",json!({"language":"de"}))?;
            wyoming::write_event(&mut socket,"audio-start",json!({"rate":16000,"width":2,"channels":1}))?;
            let start = Instant::now();
            let mut sent = 0;
            for chunk in samples.chunks(320) {
                let deadline = start + Duration::from_secs_f64(sent as f64 /16000.0);
                if let Some(wait) = deadline.checked_duration_since(Instant::now()).filter(|_|paced) { std::thread::sleep(wait); }
                let payload: Vec<u8> = chunk.iter().flat_map(|s|s.to_le_bytes()).collect();
                let header = format!("{}\n",json!({"type":"audio-chunk","data":{"rate":16000,"width":2,"channels":1},"payload_length":payload.len()}));
                // Deliberately split header and payload across differently sized writes.
                for part in header.as_bytes().chunks(7) { socket.write_all(part)?; }
                for part in payload.chunks(113) { socket.write_all(part)?; }
                sent += chunk.len();
            }
            let last = Instant::now();
            wyoming::write_event(&mut socket,"audio-stop",json!({}))?;
            let result = wyoming::read_event(&mut reader)?.ok_or("Missing transcript")?;
            if result.kind != "transcript" { return Err(format!("Unexpected {}: {:?}",result.kind,result.data).into()); }
            let text = result.data.get("text").and_then(|v|v.as_str()).ok_or("Missing text")?;
            let normalized = text.to_lowercase().split_whitespace().map(|s|s.trim_matches(|c:char|!c.is_alphanumeric())).collect::<Vec<_>>().join(" ");
            let expected = if path.ends_with("01.wav") { "schalte das licht im wohnzimmer ein" }
                else if path.ends_with("03.wav") { "stelle die temperatur im wohnzimmer auf zweiundzwanzig grad" }
                else { return Err("Use the verified neural 01.wav and 03.wav fixtures".into()); };
            println!("{}",json!({"event":"smoke_transcript","mode":if paced {"paced"}else{"fast"},"rtf":if paced {None}else{Some(start.elapsed().as_secs_f64()/(samples.len() as f64/16000.0))},"wav":path,"text":text,"expected":expected,"correct":normalized==expected,"audio_s":samples.len() as f64/16000.0,"end_to_end_s":start.elapsed().as_secs_f64(),"last_chunk_to_transcript_s":last.elapsed().as_secs_f64()}));
            if normalized != expected { return Err("Transcript differs from expected words".into()); }
        }
    }
    Ok(())
}
fn main() { if let Err(error) = run() { eprintln!("Wyoming probe failed: {error}"); std::process::exit(1); } }
