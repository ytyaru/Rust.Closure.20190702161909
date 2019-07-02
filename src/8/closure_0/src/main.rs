fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // 所有権のムーブ
    println!("can't use x here: {:?}", x); // error[E0382]: borrow of moved value: `x`
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

