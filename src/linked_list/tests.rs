use super::{LinkedList, macros::list};


#[test]
fn push_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);
    list.push_front(5);
    assert_eq!(list, list![5, 4, 3, 2, 1]);
    assert_eq!(list.len(), 5);
}

#[test]
fn push_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    assert_eq!(list, list![1, 2, 3, 4, 5]);
    assert_eq!(list.len(), 5);
}

#[test]
fn append() {
    let mut list_one: LinkedList<&str> = list!["One", "Two", "Three"];
    let mut list_two: LinkedList<&str> = list!["Four", "Five", "Six"];
    list_one.append(&mut list_two);
    assert_eq!(list_one, list!["One", "Two", "Three", "Four", "Five", "Six"]);
    assert_eq!(list_one.len(), 6);
    assert!(list_two.is_empty());
}

#[test]
fn remove() {
    let mut list: LinkedList<&str> = list!["One", "Two", "Three"];

    list.remove_back();
    assert_eq!(list, list!["One", "Two"]);

    list.remove_front();
    assert_eq!(list, list!["Two"]);

    list.push_back("<-");
    list.push_front("->");
    assert_eq!(list, list!["->", "Two", "<-"]);
}

#[test]
fn pop() {
    let mut list: LinkedList<i32> = list![1, 2, 3, 4, 5];
    
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_back(), Some(5));

    list.push_back(0);
    list.push_front(0);
    assert_eq!(list, list![0, 2, 3, 4, 0]);
}

#[test]
fn clear() {
    let mut list: LinkedList<i32> = list![6, 6, 6];
    list.clear();
    assert_eq!(list, LinkedList::<i32>::new());
}

#[test]
fn is_empty() {
    let mut list: LinkedList<&str> = list!["I", "am", "not", "empty."];
    assert_eq!(list.is_empty(), false);
    list.clear();
    assert_eq!(list.is_empty(), true);
}

#[test]
fn get() {
    let list: LinkedList<i32> = list![1, 2, 3, 4, 5];
    assert_eq!(list.get(2), Some(&3));
}

#[test]
fn get_mut() {
    let mut list: LinkedList<f32> = list![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(list.get_mut(2), Some(&mut 3.0));
    let got = list.get_mut(2).unwrap();
    *got = 5.0;
    assert_eq!(list.get_mut(2), Some(&mut 5.0));
}

#[test]
fn search() {
    let list: LinkedList<&str> = list!["Search", "Idk", "Maybe", "This?"];
    assert_eq!(list.search("This?"), Some(3));
}

#[test]
fn eq() {
    assert_eq!(list![3, 2, 1], list![3, 2, 1]);
}

#[test]
fn ne() {
    assert_ne!(list![4, 0, 4], list![4, 0, 0]);
    assert_ne!(list![4, 0, 4], list![4, 0, 4, 0]);
}

#[test]
fn index() {
    let list: LinkedList<&str> = list!["Hey", "this", "is", "a", "Linked", "List"];
    assert_eq!(list[0], "Hey");
    assert_eq!(list[1], "this");
    assert_eq!(list[2], "is");
    assert_eq!(list[3], "a");
    assert_eq!(list[4], "Linked");
    assert_eq!(list[5], "List");
}

#[test]
fn index_mut() {
    let mut list: LinkedList<i32> = list![3, 3, 3];
    assert_eq!(list, list![3, 3, 3]);
    list[0] = 6;
    list[1] = 6;
    list[2] = 6;
    assert_eq!(list, list![6, 6, 6]);
}

#[test]
fn default() {
    let list: LinkedList<u8> = LinkedList::default();
    assert_eq!(list, LinkedList::new());
}

#[test]
fn length() {
    assert_eq!(list![1, 2, 3].len(), 3);
}

#[test]
fn display() {
    assert_eq!(format!("{}", list![1, 3, 3, 7]), "[1, 3, 3, 7]");
    assert_eq!(format!("{}", LinkedList::<i32>::new()), "[]");
}

#[test]
fn from_vec() {
    let list = LinkedList::from(vec![1, 2, 3]);
    assert_eq!(list, list![1, 2, 3]);
}

#[test]
fn from_slice() {
    let slice: &[i32] = &[1, 2, 3];
    let list = LinkedList::from(slice);
    assert_eq!(list, list![1, 2, 3]);
}

#[test]
fn from_array() {
    let arr: [i32; 3] = [1, 2, 3];
    let list = LinkedList::from(arr);
    assert_eq!(list, list![1, 2, 3]);
}

#[test]
fn from_str() {
    let x = "5 4 3 2 1";
    assert_eq!(x.parse::<LinkedList<i32>>().unwrap(), list![5, 4, 3, 2, 1]);
}

#[test]
fn as_vector() {
    let list = list![1, 3, 3, 7];
    assert_eq!(list.as_vector(), vec![1, 3, 3, 7]);
}

#[test]
fn insert() {
    let mut list = list![1, 1];
    list.insert(2, 1);

    assert_eq!(list, list![1, 2, 1]);
    assert_eq!(list.length, 3);
    assert_eq!(list[1], 2);

    list.push_back(0);
    list.push_front(0);
    assert_eq!(list, list![0, 1, 2, 1, 0]);
    assert_eq!(list.length, 5);
    assert_eq!(list[2], 2);

    list.insert(0, 2);
    assert_eq!(list, list![0, 1, 0, 2, 1, 0]);
    assert_eq!(list.length, 6);
    assert_eq!(list[2], 0);
}

#[test]
fn iterator() {
    let mut sum = 0;

    for i in list![1, 3, 3, 7, 3, 0, 0, 5] {
        sum += i;
    }

    assert_eq!(sum, 22);
}

#[test]
fn from_iterator() {
    assert_eq!(vec![1, 2, 3].into_iter().collect::<LinkedList<i32>>(), list![1, 2, 3]);
}