

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use {
        anchor_lang::prelude::*,
        anchor_lang::solana_program::{system_instruction, system_program, sysvar},
        anchor_spl::token::{Mint, MintTo},
        spl_associated_token_account
    };
    #[test]
    fn hmm() {
        #[derive(AnchorSerialize, Clone, AnchorDeserialize)]
        struct Thing {
            y: Vec<u8>,
            z: Vec<u8>, // dynamically sized!
        }
        
        let mut vec_1 = Vec::<u8>::with_capacity(10);
        vec_1.push(1);
        vec_1.push(2);
        let vec_2 = Vec::<u8>::with_capacity(10);
        
        let istance =  Thing {
            y: vec_1,
            z: vec_2,
        };
        eprintln!("Before ser... vec_1 cap: {}, vec_2 cap: {}", istance.y.capacity(), istance.z.capacity());

        let mut serialized_data = istance.try_to_vec().unwrap();
        
        eprintln!("data size: {}", serialized_data.len());
        eprintln!("{:?}", serialized_data);
        
        let deserialized_thing: Thing = Thing::deserialize(&mut &serialized_data[..]).unwrap();
        eprintln!("Deserialized... vec_1 cap: {}, vec_2 cap: {}", deserialized_thing.y.capacity(), deserialized_thing.z.capacity());
   }
}
