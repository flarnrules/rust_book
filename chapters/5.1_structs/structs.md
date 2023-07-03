# Structs

Structs are data structures where you can connect like-kind data.

We declare structs with the `struct` keyword like this:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
```

This creates the User struct, but at the moment there are no *instances* of the user struct. We can create an instance of a User from the above declared struct and store it in a variable user1 like this:

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}
```

Now that we have created an instance of the User struct `user1` we can access values using dot notation. `user1.email` would return the value "someone@example.com".

