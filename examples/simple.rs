use ll::{LinkedList, Bump};


fn main() {
    let bump = Bump::new();

    let ll = LinkedList::new_with_bump(1, &bump);

    ll.push_to_end(2);
    ll.push_to_end(3);

    ll.push_to_front(0);

    for value in ll.iter() {
        println!("{value}");
    };
}
