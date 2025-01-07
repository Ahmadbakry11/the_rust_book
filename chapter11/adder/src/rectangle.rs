struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


#[test]

fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 19,
        height: 29, 
    };

    let smaller = Rectangle {
        width: 9,
        height: 19,
    };

    assert!(larger.can_hold(smaller));
}

#[test]

fn smaller_can_not_hold_larger(){
    let larger = Rectangle {
        width: 19,
        height: 29, 
    };

    let smaller = Rectangle {
        width: 9,
        height: 19,
    };

    assert!(!smaller.can_hold(larger));
}
