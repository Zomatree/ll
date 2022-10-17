use ll::LinkedList;


fn main() {
    // this uses Box::leak to create the bump allocator for us
    let ll = LinkedList::new_leaked(1);

    ll.push_to_end(2);
    ll.push_to_end(3);

    for value in ll.iter() {
        println!("{value}");
    };
}

