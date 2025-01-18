// Not generic, only provides the largest i32
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// not generic, returns the largest char, only difference is the type and name
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// To make it a genetic, we need to use the type name declarations within angled brackets: <>

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // Boolean operators don't work in these generic circumstances
        // which is why we use: std::cmp::PartialOrd trait on the generic type

        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = [34, 50, 25, 100, 65];

    let char_list = ['a', 'z', 'S', 'F', 'Z'];

    // This is the implementation without making a generic function to be re-used.
    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_result = largest(&char_list);
    println!("the largest char is {}", char_result);
}
