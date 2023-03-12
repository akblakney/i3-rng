use i3ipc::I3EventListener;
use i3ipc::I3Connection;
use i3ipc::Subscription;
use i3ipc::event::Event;
use std::time::{Instant};//, SystemTime, Duration};
use blake2::{Digest};
use std::sync::{Arc, Mutex};//mpsc;
use crate::structs::{HashObj};
use crate::constants::{MIN_ENTROPY};

pub fn i3_listener(hash_mut: Arc<Mutex<HashObj>>) {

  let mut start_time = std::time::Instant::now();

  // setup conn and listener
  let mut connection = I3Connection::connect().unwrap();
  println!("i3 conn version: {} set up successfully", connection.get_version().unwrap().human_readable);
  let mut listener = I3EventListener::connect().unwrap();
  let subs = [Subscription::Mode, Subscription::Binding];
  listener.subscribe(&subs).unwrap();

  for event in listener.listen() {
    match event.unwrap() {

      // binding events
      Event::BindingEvent(e) => {
        handle_binding_event(e.binding.command, &mut start_time, &hash_mut);
      },
      Event::ModeEvent(e) => println!("new mode: {}", e.change),

      // misc event
      _ => unreachable!()
//      _ => (),
    }
  }

}


pub fn handle_binding_event(command: String, prev_time: &mut Instant, hash_mut: &Arc<Mutex<HashObj>>) {

  println!("received i3 command: {}", command);

  // unlock hasher mutex object
  let mut hash_obj = hash_mut.lock().unwrap();

  let lsb_time: [u8; 1] = [(prev_time.elapsed().as_nanos() % 256).try_into().unwrap()];

//  println!("adding {:?} to entropy pool", lsb_time);
  hash_obj.hasher.update(lsb_time);
//  println!("adding {} to entropy pool", command);
  hash_obj.hasher.update(command.as_bytes());

  if hash_obj.entropy_est < MIN_ENTROPY {
    hash_obj.entropy_est += 2;
  }
  println!("entropy est: {}", hash_obj.entropy_est);

  println!();

  *prev_time = std::time::Instant::now();
}
