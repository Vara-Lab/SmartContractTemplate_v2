
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{Out, InOut,Metadata};



// 1. Create your own Actions
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    
    // Add Actions
    FirstAction,
    SecondAction,
    ThirdAction,
    
}


// 2. Create your own Events
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum  Event {
    
    // Add Events(Example)
    FirstEvent,
    SecondEvent,
    ThirdEvent,
}


// 3. Create your own Struct
#[derive(Default, Encode, Decode, Clone, TypeInfo)]
pub struct CustomStruct {
    firstfield: String,
    secondfield: u128,
    thirdfield: ActorId,
   
}


pub struct ContractMetadata;

// 4. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata{
     type Init = ();
     type Handle = InOut<Action,Event>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Out<CustomStruct>;

}