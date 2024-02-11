use crate::{
    notes::{
        step1_4::{
            STEP1_NOTES, STEP2_NOTES1, STEP2_NOTES2, STEP3_NOTES, STEP4_NOTES1, STEP4_NOTES2,
        },
        step5::{STEP5_NOTES1, STEP5_NOTES2},
    },
    raw_ptr::RawMutPtr,
};
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use std::{
    error::Error,
    io::{stdin, stdout, Write},
    thread::{self, sleep},
    time::Duration,
};

pub fn play() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Midi Out Stream")?;

    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };

    let mut conn_out = midi_out.connect(out_port, "大哉乾元 - 洛天依")?;

    println!(
        "{}",
        "-------------------------Start!-------------------------".blue()
    );
    for notes in STEP1_NOTES {
        for notes in notes.iter() {
            for note in notes {
                println!("{:?}", *note as u8);
                play_note(&mut conn_out, *note as u8, 1);
            }
        }
    }
    println!(
        "{}",
        "                                                        ".on_blue()
    );
    let ptr = RawMutPtr::new(&mut conn_out);
    let ptr_clone = ptr.clone();
    for handle in [
        thread::spawn(move || {
            let conn_out = ptr.clone();
            for notes in STEP2_NOTES1 {
                for notes in notes.iter() {
                    for note in notes {
                        println!("{:?}", *note as u8);
                        play_note(unsafe { conn_out.0.as_mut().unwrap() }, *note as u8, 1);
                    }
                }
            }
        }),
        thread::spawn(move || {
            let temp = ptr_clone;
            for notes in STEP2_NOTES2 {
                for notes in notes.iter() {
                    for note in notes {
                        println!("{:?}", *note as u8);
                        play_note(unsafe { temp.0.as_mut().unwrap() }, *note as u8, 1);
                    }
                }
            }
        }),
    ] {
        handle.join().unwrap();
    }
    sleep(Duration::from_secs(6));
    println!(
        "{}",
        "                                                        ".on_blue()
    );
    for notes in STEP3_NOTES {
        for notes in notes.iter() {
            for note in notes {
                println!("{:?}", *note as u8);
                play_note(&mut conn_out, *note as u8, 1);
            }
        }
    }
    println!(
        "{}",
        "                                                        ".on_blue()
    );
    let ptr = RawMutPtr::new(&mut conn_out);
    let ptr_clone = ptr.clone();
    for handle in [
        thread::spawn(move || {
            let conn_out = ptr.clone();
            for notes in STEP4_NOTES1 {
                for notes in notes.iter() {
                    for note in notes {
                        println!("{:?}", *note as u8);
                        play_note(unsafe { conn_out.0.as_mut().unwrap() }, *note as u8, 1);
                    }
                }
            }
        }),
        thread::spawn(move || {
            let temp = ptr_clone;
            for notes in STEP4_NOTES2 {
                for notes in notes.iter() {
                    for note in notes {
                        println!("{:?}", *note as u8);
                        play_note(unsafe { temp.0.as_mut().unwrap() }, *note as u8, 1);
                    }
                }
            }
        }),
    ] {
        handle.join().unwrap();
    }
    sleep(Duration::from_secs(46));
    println!(
        "{}",
        "                                                        ".on_blue()
    );
    let ptr = RawMutPtr::new(&mut conn_out);
    let ptr_clone = ptr.clone();
    for handle in [
        thread::spawn(move || {
            let conn_out = ptr.clone();
            for notes in STEP5_NOTES1 {
                for notes in notes.iter() {
                    for note in notes {
                        println!("{:?}", *note as u8);
                        play_note(unsafe { conn_out.0.as_mut().unwrap() }, *note as u8, 1);
                    }
                }
            }
        }),
        thread::spawn(move || {
            let temp = ptr_clone;
            for notes in STEP5_NOTES2 {
                for notes in notes.iter() {
                    for note in notes {
                        println!("{:?}", *note as u8);
                        play_note(unsafe { temp.0.as_mut().unwrap() }, *note as u8, 1);
                    }
                }
            }
        }),
    ] {
        handle.join().unwrap();
    }
    println!(
        "{}",
        "--------------------------Done!-------------------------".blue()
    );

    Ok(())
}

fn play_note(conn_out: &mut MidiOutputConnection, note: u8, duration: u64) {
    if note != 0x0 {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
        sleep(Duration::from_millis(duration * 87));
        let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
    } else {
        sleep(Duration::from_millis(duration * 87));
    }
}
