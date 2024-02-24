use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{io, thread, time};
use std::sync::{Arc, Mutex};
use rand::Rng;

const NUM_GUESTS: usize = 10;
const LOQUACIOUS: bool = true;
const SPACING_MILLIS: u64 = 1;

fn guest_strategy(cupcake_lock: Arc<Mutex<bool>>, exited_labyrinth_signal: Arc<AtomicBool>, announcement: Arc<AtomicBool>) {
    let mut have_eaten_cupcake = false;
    loop {
        // Wait until the Minotaur lets us into the labyrinth
        thread::park();

        // If the announcement to the Minotaur has been made, the game is over
        if announcement.load(Ordering::SeqCst) { break };

        let mut cupcake = cupcake_lock.lock().unwrap();
        if have_eaten_cupcake {
            if LOQUACIOUS { println!("\tI've already eaten a cupcake! I'm leaving without causing a scene.") };
        } else {
            if LOQUACIOUS { println!("\tI haven't eaten a cupcake yet...") };
            if *cupcake {
                if LOQUACIOUS { println!("\t...and there's one here! Bingo!") };
                *cupcake = false;
                have_eaten_cupcake = true;
            } else {
                if LOQUACIOUS { println!("\t...but there isn't a cupcake here for me.") };
            }
        };

        // Let the Minotaur know we've exited the labyrinth.
        exited_labyrinth_signal.store(true, Ordering::SeqCst);
    }
}

fn leader_strategy(cupcake_lock: Arc<Mutex<bool>>, exited_labyrinth_signal: Arc<AtomicBool>, announcement: Arc<AtomicBool>) {
    let mut headcount = 0;
    loop {
        // Wait until the Minotaur lets us into the labyrinth
        thread::park();

        let mut cupcake = cupcake_lock.lock().unwrap();
        if !*cupcake {
            if LOQUACIOUS { println!("\tSomeone's eaten a cupcake! Another guest must've gone through since the last time I've been in here.") };
            headcount += 1;
            if headcount == NUM_GUESTS - 1 { break };
            *cupcake = true;
            if LOQUACIOUS { 
                let plural_s = if headcount == 1 { "" } else { "s" };
                println!("\tThat makes {headcount} guest{plural_s}. I'm replacing the cupcake.")
            };
        } else {
            if LOQUACIOUS { println!("\tThere's a cupcake here. I'll leave without doing anything.") };
        }

        // Let the Minotaur know we've exited the labyrinth.
        exited_labyrinth_signal.store(true, Ordering::SeqCst);
    }
    if LOQUACIOUS { println!("\tThat's all {NUM_GUESTS} guests (including me)!") };

    // Announce to the Minotaur that we are sure that all guests have entered the labyrinth.
    announcement.store(true, Ordering::SeqCst);
    exited_labyrinth_signal.store(true, Ordering::SeqCst);
}

fn main() {
    println!("*********\nProblem 1\n*********");

    let cupcake = Arc::new(Mutex::new(true));
    let announcement = Arc::new(AtomicBool::new(false));
    let mut guests = vec![];
    let mut num_times_labyrinth_entered = 0; 
    let mut completion_signals = vec![];

    // Assign each guest a strategy and give them means to communicate with the Minotaur (initialize the threads)
    for id in 0..NUM_GUESTS {
        let cupcake_clone = cupcake.clone();
        let announcement_clone = announcement.clone();
        let signal = Arc::new(AtomicBool::new(false));
        let signal_clone = signal.clone();
        let handle = if id == 0 {
            thread::spawn(move || leader_strategy(cupcake_clone, signal_clone, announcement_clone))
        } else {
            thread::spawn(move || guest_strategy(cupcake_clone, signal_clone, announcement_clone))
        };
        guests.push(handle);
        completion_signals.push(signal.clone());
    }

    // The Minotaur summons guests into the labyrinth while no one has made an announcement to end the game
    while !announcement.load(Ordering::SeqCst) {
        let next_guest = rand::thread_rng().gen_range(0..NUM_GUESTS);
        if LOQUACIOUS { println!("The Minotaur has selected guest {next_guest}") };

        // Let the next guest know that it's their turn to enter the labyrinth.
        guests[next_guest].thread().unpark();
        num_times_labyrinth_entered += 1;

        // Wait until the guest lets us know they've exited the labyrinth.
        while !completion_signals[next_guest].load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(SPACING_MILLIS));
        }
        completion_signals[next_guest].store(false, Ordering::SeqCst);
    }

    // The game is over! Let the remaining guests know that they can stop waiting to enter.
    for guest in guests.into_iter() {
        guest.thread().unpark();
        guest.join().unwrap();
    };

    println!("\nThe game is over! The guests entered the labyrinth a total of {num_times_labyrinth_entered} times.\n");
    println!("Press enter to continue...");
    let mut input = [0; 1];
    io::stdin().read_exact(&mut input).expect("Failed to read input");


    // ===========================================================================================================================


    println!("*********\nProblem 2\n*********");

    let showroom = Arc::new(Mutex::new(Showroom {}));
    let stop_signal = Arc::new(AtomicBool::new(false));
    let mut guests = vec![];
    for id in 0..NUM_GUESTS {
        let showroom_clone = showroom.clone();
        let stop_signal_clone = stop_signal.clone();
        let handle = thread::spawn(move || bumble_around_but_try_to_enter_the_showroom_every_now_and_then(id, showroom_clone, stop_signal_clone));
        guests.push(handle);
    }

    // Give the guests some time to mingle (and opportunistically enter the showroom)
    thread::sleep(Duration::from_millis(SPACING_MILLIS * 1000));

    // The Minotaur calls an end to the party.
    println!();
    stop_signal.store(true, Ordering::SeqCst);
    for guest in guests.into_iter() { guest.join().unwrap() };
}

struct Showroom { }

// The guests mingle in the party.
// Every 0 to SPACING_MILLIS * 50 milliseconds, the guest attempts to enter the showroom.
// If the showroom is unoccupied, they will occupy it and admire the crystal vase for 0 to SPACING_MILLIS * 10 milliseconds.
// If the showroom is occupied, they will go back to mingling and try again in another 0 to SPACING_MILLIS * 50 milliseconds.
// The Minotaur will let the guests know when to wrap up.
fn bumble_around_but_try_to_enter_the_showroom_every_now_and_then(id: usize, showroom_lock: Arc<Mutex<Showroom>>, stop_bumbling: Arc<AtomicBool>) {
    let mut num_times_entered_room = 0;
    let mut num_times_failed_to_enter = 0;
    loop {
        let loitering_time = time::Duration::from_millis(rand::thread_rng().gen_range(0..50));
        thread::sleep(loitering_time);
        if stop_bumbling.load(Ordering::SeqCst) { break }
        match showroom_lock.try_lock() {
            Ok(_) => {
                if LOQUACIOUS { println!("Guest {id} has entered the showroom.") };
                let vase_admiration_time = time::Duration::from_millis(rand::thread_rng().gen_range(0..10));
                thread::sleep(vase_admiration_time);
                num_times_entered_room += 1;
                if stop_bumbling.load(Ordering::SeqCst) { break };
                if LOQUACIOUS { println!("Guest {id} has exited the showroom.") };
            },
            Err(_) => {
                if LOQUACIOUS { println!("\tGuest {id} attempted to enter the showroom, but it was occupied.") };
                num_times_failed_to_enter += 1;
            },
        }
    }
    println!("Guest {id} entered the showroom {num_times_entered_room} times (and failed to enter {num_times_failed_to_enter} times)");
}