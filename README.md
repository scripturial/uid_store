## UID UidStore

Generate random uid strings containing letters, numbers, or base62 values.

### Functions to generate random UID's

Generate a random string of a fixed length using any roman letter or numeral.

    let uid = random_string(8);

Generate a random string of a fixed length using any roman letter or numeral
except letters often confused by people due to difficult to read fonts.
Excludes the letters 0 o O l L i I.

    let uid = human_random_string(8);

Convert a number to a base62 encoded string:

    let uid = number_to_uid(1000);

Convert a base62 encoded string to a number. Returns none if the string is
not a valid base62 string.

    let uid = uid_to_number(1000).unwrap();

### Generate random UID's guaranteed to be unique.

If you are using a short UID, there is a high chance of collision. Use the
UidStore to hold previous UID values to ensure a value is not generated
twice.

Generate a sequence of 10 character UID strings that are guaranteed to
to be unique:

    let mut u = UidStore::new();
    let uid = u.next(10);

Generate a sequence of UID strings that are limited in size to fit
within a unsigned integer type.

    let mut u = UidStore::new();
    let uid = u.next_u16();
    let uid = u.next_u32();
    let uid = u.next_u64();

Generate a new uid if this uid is already in the bucket.

    let mut u = UidStore::new();
    if let Some(uid) = u.make_unique("0123456789") {
        println!("New UID generated. {}", uid);
    }
