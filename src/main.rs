// use std::collections::HashSet;

fn main()
{
    // Create a buffer to store our inputs
    let mut buffer: [[char; 5]; 6] = [['_'; 5]; 6];

    // TODO: Get word of the day
    // let mut _wordlist: HashSet<String> = HashSet::new();

    let _word_to_find: &'static str = "harsh";

    buffer[1][1] = 'a';

    for elem in buffer
    {
        // println!("{:?}", elem);

        for char in elem
        {
            // TODO: 1. Check if the character matches the word_to_find
                // if yes: print green
                // if no:
                    // if does exist: print yellow
                    // else: print gray
            print!("{}", char);
            print!(" ");
        }
        println!("");
    }
}
