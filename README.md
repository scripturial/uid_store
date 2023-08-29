## UID UidStore

Generate a random string of a fixed length using any roman letter or numeral.

    let uid = random_string(8);

Generate a random string of a fixed length using any roman letter or numeral
except letters often confused by people due to difficult to read fonts.
Excludes the letters 0 o O l L i I.

    let uid = human_random_string(8);

Create a bucket that can be used to generate a random string that does not
match any previously generated string:

    let mut u = UidStore::new(10);
    let id = u.next().to_string();

Generate a new uid if this uid is already in the bucket.

    let uid = "0123456789";
    let o = u.make_unique(uid);
    if o.is_some() {
        println!("New UID generated. {}", o.unwrap());
    }
