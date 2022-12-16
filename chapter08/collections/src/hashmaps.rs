// Hash Maps

// By default HashMap uses a hashing function called SipHash.
// It's not the fastest algorithm but provides high security (against DoS).
// It's chosen as a compromise between security and drop in performance.
// There exists other choices. (crates.io)

// The type HashMap<K, V> stores a mapping of keys of type K to values of type V using the hashing function, which determines how it places these keys and values into memory.
// Are useful when you want to loop up data using a key rather than an index as in vectors.
// Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type. (different from dicts for instance)

// Ownership
// For types that implement the Copy trait, like i32, the values are copied into the hash map.
// For owned values like String, the values will be moved and the hash map will be the owner of those values.
// If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

// Need to load the HashMap from the collections in the standard library
use std::collections::HashMap;

fn main() {
    // Creating a hash map
    // Empty hash map with new and adding elements with the insert method
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accesing values
    // The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None. This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);
    // Iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Hash maps and ownership
    let field_name = String::from("Favorite colour");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Updating a hash map

    // Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);
    println!("{:?}", scores);

    // Adding a Key and Value Only If a Key Isn’t Present
    // To check whether a key is already in the dictionary, the entry method is used
    // entry returns an Entry enum that represents a value that might or nor exists
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // will add Yellow
    scores.entry(String::from("Blue")).or_insert(50); // Blue exists, won't change
    println!("{:?}", scores);

    // Updating a value based on the old value
    // For instance, the following code counts how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // The or_insert method returns a mutable reference (&mut V) to the value for the specified key. Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*).
        let count = map.entry(word).or_insert(0);
        *count += 1; // this updates the variable and its mutable reference: or_insert
    }
    println!("{:?}", map);
}
