use std::{thread, time::Duration};

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red {
            return ShirtColor::Blue;
        } else {
            return ShirtColor::Red;
        }
    }
}
#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);

    let user_pref2 = None;

    let user1_giveaway = store.giveaway(user_pref1);
    println!(
        "The user1 has a pref of {:?} and got color of {:?}",
        user_pref1, user1_giveaway
    );

    let user2_giveaway = store.giveaway(user_pref2);
    println!(
        "The user2 has a pref of {:?} and got color of {:?}",
        user_pref2, user2_giveaway
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly....!");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(99);

    fn add_one(x: i32) -> i32 {
        x + 1
    }
    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    add_one(8);
    add_one_v2(98);
    add_one_v3(9);
    add_one_v4(89);

    // immutable borrowing
    let list = vec![1, 3, 5];
    // list is immutable, and the below is ok because we can have multiple immutable ref @ the same time.
    println!("Before defining the closure: {list:?}");
    let immutable_borrow = || println!("During the closure definition: {list:?}");
    println!("After closure definition: {list:?}");
    immutable_borrow();
    println!("After closure call: {list:?}");

    // mutable borrowing

    let mut nums = vec![1, 3, 5, 7];
    // below we take an immutable borrow
    println!("Before defining the closure: {nums:?}");
    // The immutable borrow ends here, so we can start a new phase.

    // below, the closure captures a mutable ref to nums.
    // so, we can not have another immutable ref at the same time unless the mutable ref ends
    // That explains we can not have the println! statement before calling the closure
    let mut mutable_borrow = || nums.push(11);

    // below we are using the mutable borrow to modify the nums vector.
    mutable_borrow();
    mutable_borrow();
    // The mutable borrow ends here and hence we can have another immutable ref as below

    mutable_borrow();
    println!("After calling the closure: {nums:?}");
    // The mutable borrow ends and hence we can have immutable borrowing normally.
    // if we tried to use the closure again below, it will not compile.
    // because we are using muatbel and immutable refs at the same time.
    //  mutable_borrow(); --------> will not compile

    // Taking ownership

    let owners = vec![1, 2, 3, 4, 5];

    // Below the closure is taking ownership of the owners vector.
    thread::spawn(move || println!("The owners list new owner: {owners:?}"))
        .join()
        .unwrap();

    // So trying to print the last vector owners will not compile.
    // Because owners ownership has been moved.

    // println!("{owners:?}"); ----------> Will not compile.

    // Moving Captured Values Out of Closures and the Fn Traits
    // ============================================================

    /*
        Moving Captured Values Out of Closures.
        ========================================
    */

    let s = String::from("Hello");
    let moving_closure = || {
        println!("I am taking ownership of the value I am capturing which is {s}");
        s
    };

    let moved_string = moving_closure();

    println!("Now ownership is moved to : {moved_string}");
    /*
        Now if I tried to print s it wont compile
        println!("{s}")  ========> Won't compile
        And if I tried to call that closure again, it will not compile.The ownership of the
        value it captures has been moved, plus it implements FnOnce, called only once.
        moving_closure() ========> Won't compile
    */

    // --------------------------------------------------------------

    /*
        2. The FnOnce Trait
        ========================
        A closure implements FnOnce if it moves captured values out of its body.
        The function call_once has a generic type F which has a constraint that F implements the FnOnce trait.

    */

    let string = String::from("used to be called once");

    let call_once_closure = || println!("I am using a resource that is: {string}");

    call_once(call_once_closure); //The function takes a closure that implements a FnOnce trait.

    /*
        Can not be called again.Becauses it captured a value that is moved out of the closure body
    */

    /*
        3. FnMut
        ====================
        a closure implements FnMut if it mutates captured values but doe not move those values
        out of its body.
        it can be called multiple times as long as the mutable borrow is still valid.

        if we pass that closure to a function and we are planning to call that function more
        than once, the function will try to take ownership of the nutable closure.
        The solution or fix for that is to make the function borrow the closure mutably

        like the function call_mut() we will pass a mutable ref of the closure like below
        call_mut(&mut call_mut_closure);

        But the closure it self can be called multiple times as long as the variable it captures
        which is counter in our example is still valid.
    */

    /*
        4. Fn
        ======================
        a closure implements Fn if it does not move values out of its body and
        does not mutate captured values.

        It can also implement Fn if it does not capture anything from the environment.

        It can be called multiple time without any side effects.

    */

    let mut counter = 0;
    let mut i = 9;

    let mut call_mut_closure = || {
        counter += 1;
        println!(
            "Increment counter by 1: it was {} and now it is {}",
            counter - 1,
            counter
        )
    };

    while i >= 0 {
        call_mut(&mut call_mut_closure);
        i -= 1;
    }

    call_mut_closure();

    call_mut_closure();

    call_mut_closure();

    call_pure_mut(call_mut_closure);

    // call_pure_mut(call_mut_closure);   ===> second call here will complain about moved values

    let str2 = String::from("used by a closure that implements Fn trait");

    let fn_closure = || {
        println!(
            "I am using a closure that implements Fn and the value is {}",
            str2
        )
    };

    /*
        - In the above code, the closure fn_closure captures str2 by immutable borrow (&str2) instead of by
          ownership (str2) or mutable borrow (&mut str2).

        - Closures automatically decide how to capture their environment based on what they need.
          Since the closure only reads the value of str2 without mutating or taking ownership of it,
          Rust optimizes by capturing it as an immutable reference (&str2).
    */

    call_fn(fn_closure);

    // The owner of str2 id the outer scope of the main() function,
    // so there it can be used freely here multiple times.
    println!("{str2}");
    println!("{str2}")
}

fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn call_mut<F>(f: &mut F)
where
    F: FnMut(),
{
    f();
}

fn call_pure_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f()
}

fn call_fn<F>(f: F)
where
    F: Fn(),
{
    f()
}
